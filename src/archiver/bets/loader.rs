use std::str::FromStr;

use anyhow::{Context, Result};
use sqlx::{
    prelude::FromRow, Execute, MySql, MySqlPool, PgPool, Postgres, QueryBuilder, Transaction,
};
use time::{Date, Duration, OffsetDateTime};
use uuid::Uuid;

use crate::{
    consts::{BET_DETAIL_REPORT_TABLE_NAME, CREDIT_DEBT_TABLE_NAME, MARIA_DB_SCHEMA, SCHEMA},
    enums::{bet::BetStatus, provider::GameProvider, PositionEnum},
    helpers::{
        get_hong_kong_11_hours_from_date,
        query_helper::{get_archive_schema_name, get_bet_table_name, get_dynamic_table_name},
    },
    types::{
        AmountByPosition, BetID, Currency, ProviderBetID, ProviderGameVendorID,
        ProviderGameVendorLabel, Url, UserID, Username,
    },
};

pub async fn get_target_data_bench(
    pg_pool: &PgPool,
    table: &str,
    start_date: Option<Date>,
) -> anyhow::Result<Vec<Bet>> {
    let yesterday =
        get_hong_kong_11_hours_from_date(OffsetDateTime::now_utc().date() - Duration::days(1));

    let mut where_query = vec!["last_status_change < $3"];

    if start_date.is_some() {
        where_query.push("last_status_change >= $4");
    }

    let raw_bets: Vec<RawBet> = sqlx::query_as(&format!(
        r#"
            SELECT
                id,
                creation_date,
                last_status_change,
                stake,
                valid_amount,
                wl,
                user_id,
                username,
                ip,
                status,
                currency,
                pt_by_position,
                commission_percent,
                commission_amount,
                funds_delta,
                details,
                replay,
                transaction_ids,
                transactions,
                provider_bet_id,
                provider_game_vendor_id,
                provider_game_vendor_label
            FROM
                public.{table}
            WHERE
                status NOT IN ($1, $2)
            AND
                ({})
        "#,
        where_query.join(" AND ")
    ))
    .bind(BetStatus::Active.to_string())
    .bind(BetStatus::Pending.to_string())
    .bind(yesterday)
    .bind(start_date)
    .fetch_all(pg_pool)
    .await
    .with_context(|| format!("Failed to fetch bet chunk from '{table}'"))?;

    let mut bets = vec![];

    for bet in raw_bets {
        bets.push(bet.try_into_bet()?);
    }

    Ok(bets)
}

#[derive(FromRow, Clone)]
struct RawBet {
    id: BetID,
    creation_date: OffsetDateTime,
    last_status_change: OffsetDateTime,
    stake: i64,
    valid_amount: Option<i64>,
    wl: Option<i64>,
    user_id: UserID,
    username: Username,
    ip: String,
    status: String,
    currency: Currency,
    pt_by_position: AmountByPosition,
    commission_percent: AmountByPosition,
    commission_amount: AmountByPosition,
    funds_delta: AmountByPosition,
    details: Option<String>,
    replay: String,
    transaction_ids: Vec<String>,
    transactions: Vec<String>,
    provider_bet_id: ProviderBetID,
    provider_game_vendor_id: ProviderGameVendorID,
    provider_game_vendor_label: ProviderGameVendorLabel,
}

impl RawBet {
    fn try_into_bet(self) -> Result<Bet> {
        let bet = Bet {
            status: BetStatus::from_str(&self.status).context("Invalid status from DB")?,
            id: self.id,
            creation_date: self.creation_date,
            last_status_change: self.last_status_change,
            stake: self.stake,
            valid_amount: self.valid_amount,
            wl: self.wl,
            user_id: self.user_id,
            username: self.username,
            ip: self.ip,
            currency: self.currency,
            pt_by_position: self.pt_by_position,
            commission_percent: self.commission_percent,
            commission_amount: self.commission_amount,
            funds_delta: self.funds_delta,
            details: self.details,
            replay: self.replay,
            transaction_ids: self.transaction_ids,
            transactions: self.transactions,
            provider_bet_id: self.provider_bet_id,
            provider_game_vendor_id: self.provider_game_vendor_id,
            provider_game_vendor_label: self.provider_game_vendor_label,
        };

        Ok(bet)
    }
}

#[derive(FromRow, Clone)]
pub struct Bet {
    pub id: BetID,
    pub creation_date: OffsetDateTime,
    pub last_status_change: OffsetDateTime,
    pub stake: i64,
    pub valid_amount: Option<i64>,
    pub wl: Option<i64>,
    pub user_id: UserID,
    pub username: Username,
    pub ip: String,
    pub status: BetStatus,
    pub currency: Currency,
    pub pt_by_position: AmountByPosition,
    pub commission_percent: AmountByPosition,
    pub commission_amount: AmountByPosition,
    pub funds_delta: AmountByPosition,
    pub details: Option<String>,
    pub replay: String,
    pub transaction_ids: Vec<String>,
    pub transactions: Vec<String>,
    pub provider_bet_id: ProviderBetID,
    pub provider_game_vendor_id: ProviderGameVendorID,
    pub provider_game_vendor_label: ProviderGameVendorLabel,
}

#[derive(Debug)]
pub struct BetDetails {
    pub id: BetID,
    pub details: Option<String>,
    pub replay: Option<Url>,
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserID,
    pub username: Username,
    pub position: PositionEnum,
}

pub async fn get_upline(
    user_id: &UserID,
    transaction: &mut Transaction<'_, sqlx::Postgres>,
) -> anyhow::Result<Vec<User>> {
    sqlx::query_as!(
        User,
        r#"
            SELECT
                id,
                username AS "username!",
                position
            FROM
                public.user u
            JOIN
                public.user_upline uu ON u.id = uu.user_id
            WHERE
                u.id = $1
        "#,
        user_id.0,
    )
    .fetch_all(&mut **transaction)
    .await
    .context("Failed to fetch upline")
}

pub struct CreditDebt {
    pub id: Uuid,
    pub user_id: UserID,
    pub currency: Currency,
    pub date: OffsetDateTime,
    pub username: Username,
    pub debt_amount: i64,
}

pub async fn save_debts(
    pg_transaction: &mut Transaction<'_, Postgres>,
    debts: Vec<CreditDebt>,
    date: Date,
) -> Result<()> {
    let db_schema = get_archive_schema_name(date);
    let table_name = get_dynamic_table_name(CREDIT_DEBT_TABLE_NAME, date);

    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(&format!(
        r#"
            INSERT INTO {db_schema}.{table_name} AS t(
                id,
                user_id,
                date,
                username,
                debt_amount,
                currency
            )
        "#
    ));

    query_builder.push_values(debts.into_iter(), |mut b, r| {
        b.push_bind(r.id)
            .push_bind(r.user_id)
            .push_bind(r.date)
            .push_bind(r.username)
            .push_bind(r.debt_amount)
            .push_bind(r.currency);
    });

    let mut query = query_builder.build();

    let sql = format!(
        r#"
            {}
            ON CONFLICT (user_id, date)
            DO UPDATE SET debt_amount = t.debt_amount + EXCLUDED.debt_amount
        "#,
        query.sql()
    );

    sqlx::query_with(
        &sql,
        query
            .take_arguments()
            .context("Failed to take arguments for save_debts query")?,
    )
    .execute(&mut **pg_transaction)
    .await
    .context("Failed to save debts")?;

    Ok(())
}

pub async fn delete_bets_by_ids(
    bet_ids: Vec<BetID>,
    provider: GameProvider,
    transaction: &mut Transaction<'_, sqlx::Postgres>,
) -> Result<()> {
    let table_name = get_bet_table_name(provider);
    let schema = &*SCHEMA;

    let mut query_builder: QueryBuilder<Postgres> =
        QueryBuilder::new(format!("DELETE FROM {schema}.{table_name} WHERE id IN ("));

    let mut separated = query_builder.separated(",");

    for id in bet_ids.iter() {
        separated.push_bind(id.as_ref());
    }

    separated.push_unseparated(")");

    let mut query = query_builder.build();

    sqlx::query_with(
        query.sql(),
        query
            .take_arguments()
            .context("Failed to take arguments for delete_bets_by_ids query")?,
    )
    .execute(&mut **transaction)
    .await
    .context("Failed to delete list of bets")?;

    Ok(())
}

pub async fn insert_bet_details_to_details_table(
    mysql: &MySqlPool,
    details: Vec<BetDetails>,
) -> Result<()> {
    let schema = &*MARIA_DB_SCHEMA;

    let mut query_builder: QueryBuilder<MySql> = QueryBuilder::new(format!(
        "INSERT INTO {schema}.{BET_DETAIL_REPORT_TABLE_NAME} (id, details, replay)"
    ));

    query_builder.push_values(details.into_iter(), |mut b, r| {
        b.push_bind(r.id.to_string())
            .push_bind(r.details)
            .push_bind(r.replay);
    });

    let mut query = query_builder.build();

    sqlx::query_with(
        query.sql(),
        query
            .take_arguments()
            .context("Failed to take arguments for insert_bet_details_to_details_table query")?,
    )
    .execute(mysql)
    .await
    .context("Failed to insert bet details to details table")?;

    Ok(())
}

pub async fn update_bet_details(mysql: &MySqlPool) -> Result<()> {
    let schema = &*MARIA_DB_SCHEMA;

    sqlx::query(&format!(
        r#"
            UPDATE {schema}.bet bet
            JOIN {schema}.bet_archive_details details ON bet.id = details.id
            SET bet.details = details.details,
            bet.replay = details.replay
        "#
    ))
    .execute(mysql)
    .await
    .context("Failed to update details in column DB")
    .map(|_| ())
}

pub async fn truncate_maria_db_table(conn: &MySqlPool, table: &str) -> Result<()> {
    let schema = &*MARIA_DB_SCHEMA;

    sqlx::query(&format!("TRUNCATE TABLE {schema}.{table}"))
        .execute(conn)
        .await
        .with_context(|| format!("Fail to truncate '{table}' table"))?;

    Ok(())
}
