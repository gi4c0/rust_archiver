use anyhow::bail;
use sqlx::{MySqlPool, Row};
use time::{Date, Duration, Month, OffsetDateTime};

use crate::{
    db::tables::OPENING_BALANCE_TABLE_NAME,
    utils::{
        get_hong_kong_11_hours,
        query_helper::{get_archive_schema_name, get_dynamic_table_name},
    },
};

pub async fn create_opening_balance_records(pool: &MySqlPool) -> anyhow::Result<()> {
    let last_opening_balance_date = find_last_opening_balance_record(pool).await?;
    let tomorrow = OffsetDateTime::now_utc().date() + Duration::days(1);

    if last_opening_balance_date >= tomorrow {
        // Seems like procedure has already been executed today
        return Ok(());
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
