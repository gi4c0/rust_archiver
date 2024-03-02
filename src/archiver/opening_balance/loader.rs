use anyhow::Context;
use sqlx::{Execute, PgPool, Postgres, QueryBuilder, Row};
use time::{Date, OffsetDateTime};
use uuid::Uuid;

use crate::{
    db::tables::OPENING_BALANCE_TABLE_NAME,
    enums::PositionEnum,
    types::UserID,
    utils::{
        get_hong_kong_11_hours_from_date,
        query_helper::{get_archive_schema_name, get_dynamic_table_name},
    },
};

pub async fn get_last_opening_balance_creation_date(
    pool: &PgPool,
    db_schema: String,
    table_name: String,
) -> anyhow::Result<Option<Date>> {
    let result = sqlx::query(&format!(
        r#"
            SELECT
                creation_date
            FROM {db_schema}.{table_name}
            ORDER BY creation_date DESC
            LIMIT 1
        "#,
    ))
    .fetch_optional(pool)
    .await
    .context("Failed to fetch opening balance date")?;

    if let Some(result) = result {
        let creation_date: OffsetDateTime = result.try_get("creation_date")?;
        return Ok(Some(creation_date.date()));
    }

    Ok(None)
}

pub struct UserInfo {
    pub user_id: UserID,
    pub is_credit: bool,
}

pub async fn get_player_chunk(
    pool: &PgPool,
    limit: i64,
    offset: i64,
) -> anyhow::Result<Vec<UserInfo>> {
    sqlx::query_as!(
        UserInfo,
        r#"
            SELECT
                u.id AS user_id,
                b.credit > 0 AS "is_credit!"
            FROM public.user u
            JOIN balance b ON b.user_id = u.id
            WHERE u.position = $1 AND u.activated_at IS NOT NULL
            ORDER BY registered_at
            LIMIT $2 OFFSET $3
        "#,
        PositionEnum::Player as u8 as i64,
        limit,
        offset
    )
    .fetch_all(pool)
    .await
    .context("Failed to fetch a chunk of players")
}

pub async fn insert_opening_balance_records(
    pool: &PgPool,
    records: Vec<OpeningBalance>,
    date: Date,
) -> anyhow::Result<()> {
    let db_schema = get_archive_schema_name(date);
    let table_name = get_dynamic_table_name(OPENING_BALANCE_TABLE_NAME, date);

    let mut query_build: QueryBuilder<Postgres> = QueryBuilder::new(&format!(
        r#"INSERT INTO {db_schema}.{table_name} AS t(id, amount, creation_date, user_id) "#
    ));

    query_build.push_values(records.into_iter(), |mut b, r| {
        b.push_bind(r.id)
            .push_bind(r.amount)
            .push_bind(r.creation_date)
            .push_bind(r.user_id);
    });

    let mut query = query_build.build();

    let sql = format!(
        r#"
            {}
            ON CONFLICT (creation_date, user_id)
            DO UPDATE SET amount = t.amount + EXCLUDED.amount
        "#,
        query.sql()
    );

    sqlx::query_with(&sql, query.take_arguments().unwrap())
        .execute(pool)
        .await
        .context("Failed to insert opening balance records")?;

    Ok(())
}

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct OpeningBalance {
    pub id: Uuid,
    pub amount: i64,
    pub creation_date: OffsetDateTime,
    pub user_id: Uuid,
}

pub async fn get_opening_balance_records(
    pool: &PgPool,
    date: Date,
    user_ids: Vec<UserID>,
) -> anyhow::Result<Vec<OpeningBalance>> {
    let schema = get_archive_schema_name(date);
    let table = get_dynamic_table_name(OPENING_BALANCE_TABLE_NAME, date);
    let user_ids: Vec<Uuid> = user_ids.into_iter().map(|id| id.0).collect();

    let result: Vec<OpeningBalance> = sqlx::query_as(&format!(
        r#"
            SELECT
                id,
                amount,
                creation_date,
                user_id
            FROM {schema}.{table}
            WHERE creation_date = $1 AND user_id = ANY($2)
        "#
    ))
    .bind(get_hong_kong_11_hours_from_date(date))
    .bind(user_ids)
    .fetch_all(pool)
    .await
    .context("Failed to fetch opening balance records")?;

    Ok(result)
}
