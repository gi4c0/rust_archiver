use anyhow::Context;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool, Transaction};
use time::{Date, Duration, OffsetDateTime};

use crate::{
    enums::{bet::BetStatus, PositionEnum},
    types::{
        AmountByPosition, BetID, Currency, ProviderBetID, ProviderGameVendorID, UserID, Username,
    },
    utils::get_hong_kong_11_hours_from_date,
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
        where_query.push("last_status_change > $4");
    }

    let bets: Vec<Bet> = sqlx::query_as(&format!(
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
        where_query.join(" OR ")
    ))
    .bind(BetStatus::Active.to_string())
    .bind(BetStatus::Pending.to_string())
    .bind(yesterday)
    .bind(start_date)
    .fetch_all(pg_pool)
    .await
    .with_context(|| format!("Failed to fetch bet chunk from '{table}'"))?;

    Ok(bets)
}

#[derive(FromRow)]
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
    pub details: String,
    pub replay: String,
    pub transaction_ids: Vec<String>,
    pub transactions: Vec<String>,
    pub provider_bet_id: ProviderBetID,
    pub provider_game_vendor_id: ProviderGameVendorID,
    pub provider_game_vendor_label: String,
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
