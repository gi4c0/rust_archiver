use anyhow::bail;
use sqlx::PgPool;
use time::{Date, Duration, OffsetDateTime};
use uuid::Uuid;

pub mod loader;

use crate::{
    consts::OPENING_BALANCE_TABLE_NAME,
    helpers::{
        get_hong_kong_11_hours, get_hong_kong_11_hours_from_date,
        query_helper::{get_archive_schema_name, get_dynamic_table_name},
        subtract_one_month, State,
    },
};

use self::loader::{
    get_last_opening_balance_creation_date, get_opening_balance_records, get_player_chunk,
    insert_opening_balance_records, OpeningBalance,
};

pub async fn create_opening_balance_records(state: &mut State) -> anyhow::Result<()> {
    let last_opening_balance_date = find_last_opening_balance_record(&state.pg).await?;
    let tomorrow = OffsetDateTime::now_utc().date() + Duration::days(1);

    if last_opening_balance_date >= tomorrow {
        // Seems like procedure has already been executed today
        return Ok(());
    }

    let limit: i64 = 2000;
    let mut players_offset: i64 = 0;

    loop {
        let players_chunk = get_player_chunk(&state.pg, limit, players_offset).await?;

        let mut user_ids = vec![];
        let players_chunk_len = players_chunk.len();

        for user in players_chunk {
            user_ids.push(user.user_id.clone());

            if user.is_credit {
                state.add_credit_player(user.user_id);
            }
        }

        let opening_balance_records =
            get_opening_balance_records(&state.pg, last_opening_balance_date, user_ids).await?;

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
                &state.pg,
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
    let mut current_date = get_hong_kong_11_hours().date().replace_day(1).unwrap();

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
