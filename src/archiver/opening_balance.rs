use anyhow::{bail, Context};
use sqlx::{Execute, PgPool, Postgres, QueryBuilder, Row};
use time::{Date, Duration, Month, OffsetDateTime};
use uuid::Uuid;

use crate::{
    db::tables::OPENING_BALANCE_TABLE_NAME,
    enums::PositionEnum,
    types::UserID,
    utils::{
        get_hong_kong_11_hours, get_hong_kong_11_hours_from_date,
        query_helper::{get_archive_schema_name, get_dynamic_table_name},
        State,
    },
};

pub async fn create_opening_balance_records(
    pg_pool: &PgPool,
    state: &mut State,
) -> anyhow::Result<()> {
    let last_opening_balance_date = find_last_opening_balance_record(pg_pool).await?;
    let tomorrow = OffsetDateTime::now_utc().date() + Duration::days(1);

    if last_opening_balance_date >= tomorrow {
        // Seems like procedure has already been executed today
        return Ok(());
    }

    let limit: i64 = 100;
    let mut players_offset: i64 = 0;

    loop {
        let players_chunk = get_player_chunk(pg_pool, limit, players_offset).await?;

        let mut user_ids = vec![];
        let players_chunk_len = players_chunk.len();

        for user in players_chunk {
            user_ids.push(user.user_id.clone());

            if user.is_credit {
                state.add_credit_player(user.user_id);
            }
        }

        let opening_balance_records =
            get_opening_balance_records(pg_pool, last_opening_balance_date, user_ids).await?;

        let mut new_opening_balance_date = last_opening_balance_date;

        loop {
            new_opening_balance_date += Duration::days(1);
            let opening_balance_records: Vec<OpeningBalance> = opening_balance_records
                .iter()
                .map(|ob| OpeningBalance {
                    id: Uuid::new_v4(),
                    creation_date: get_hong_kong_11_hours_from_date(new_opening_balance_date),
                    ..ob.clone()
                })
                .collect();

            insert_opening_balance_records(
                pg_pool,
                opening_balance_records,
                new_opening_balance_date,
            )
            .await?;

            if new_opening_balance_date >= tomorrow {
                break;
            }
        }

        if players_chunk_len < limit as usize {
            break;
        }

        players_offset += limit;
    }

    Ok(())
}

async fn find_last_opening_balance_record(pool: &PgPool) -> anyhow::Result<Date> {
    let mut current_date = get_hong_kong_11_hours().date();

    loop {
        if current_date.year() == 2020 {
            bail!("We don't have opening balance records prior this date");
        }

        let last_opening_balance_current_month = get_last_opening_balance_creation_date(
            pool,
            get_archive_schema_name(current_date),
            get_dynamic_table_name(OPENING_BALANCE_TABLE_NAME, current_date),
        )
        .await?;

        if let Some(data) = last_opening_balance_current_month {
            return Ok(data);
        }

        current_date = subtract_one_month(current_date);
    }
}

async fn get_last_opening_balance_creation_date(
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

fn subtract_one_month(date: Date) -> Date {
    let month = date.month();
    let mut year = date.year();

    if month == Month::January {
        year -= 1;
    }

    let month = month.previous();

    Date::from_calendar_date(year, month, 1).unwrap()
}

struct UserInfo {
    user_id: UserID,
    is_credit: bool,
}

async fn get_player_chunk(pool: &PgPool, limit: i64, offset: i64) -> anyhow::Result<Vec<UserInfo>> {
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

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct OpeningBalance {
    pub id: Uuid,
    pub amount: i64,
    pub creation_date: OffsetDateTime,
    pub user_id: Uuid,
}

async fn get_opening_balance_records(
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
        .map_err(|e| {
            dbg!(&e);
            dbg!(&sql);
            e
        })
        .context("Failed to insert opening balance records")?;

    Ok(())
}
