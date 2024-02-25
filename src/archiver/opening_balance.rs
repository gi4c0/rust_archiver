use anyhow::{bail, Context};
use sqlx::{MySqlPool, PgPool, Row};
use time::{Date, Duration, Month, OffsetDateTime};

use crate::{
    db::tables::OPENING_BALANCE_TABLE_NAME,
    enums::PositionEnum,
    types::UserID,
    utils::{
        get_hong_kong_11_hours,
        query_helper::{get_archive_schema_name, get_dynamic_table_name},
    },
};

pub async fn create_opening_balance_records(
    mysql_pool: &MySqlPool,
    pg_pool: &PgPool,
) -> anyhow::Result<()> {
    let last_opening_balance_date = find_last_opening_balance_record(mysql_pool).await?;
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
            user_ids.push(user.user_id);
            // TODO:
            // if (user.isCredit) {
            //   BetArchiverProcedure.creditPlayers.add(user.userID);
            // }
        }

        if players_chunk_len < limit as usize {
            break;
        }

        players_offset += limit;
    }

    Ok(())
}

async fn find_last_opening_balance_record(pool: &MySqlPool) -> anyhow::Result<Date> {
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
    pool: &MySqlPool,
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
    .await?;

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
