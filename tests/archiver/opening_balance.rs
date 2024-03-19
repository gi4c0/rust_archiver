use claims::assert_ok;
use lib::archiver::opening_balance::create_opening_balance_records;
use lib::archiver::opening_balance::loader::get_last_opening_balance_creation_date;
use lib::db::tables::OPENING_BALANCE_TABLE_NAME;
use lib::helpers::query_helper::{get_archive_schema_name, get_dynamic_table_name};
use lib::helpers::State;
use lib::types::UserID;
use sqlx::PgPool;
use time::{Duration, OffsetDateTime};

use crate::helper::db::create_test_connection;
use crate::helper::test_data::prepare_data;

#[tokio::test]
async fn finds_last_opening_balance_and_creates_new_records() {
    let pg_pool = create_test_connection().await;
    prepare_data(&pg_pool).await;

    let mut state = State::new();

    let result = create_opening_balance_records(&pg_pool, &mut state).await;
    assert_ok!(result);

    let credit_user_ids = get_credit_players_ids(&pg_pool).await;

    for id in credit_user_ids {
        assert!(state.is_credit_player(id));
    }

    let now = OffsetDateTime::now_utc().date() + Duration::DAY;

    let last_opening_balance_record = get_last_opening_balance_creation_date(
        &pg_pool,
        get_archive_schema_name(now),
        get_dynamic_table_name(OPENING_BALANCE_TABLE_NAME, now),
    )
    .await;
    assert_eq!(last_opening_balance_record.unwrap().unwrap(), now);
}

async fn get_credit_players_ids(pg_pool: &PgPool) -> Vec<UserID> {
    let player_ids = sqlx::query!(
        r#"
            SELECT
                user_id
            FROM public.balance b
            JOIN public.user u ON b.user_id = u.id
            WHERE credit > 0 AND u.position = 6
            AND u.activated_at IS NOT NULL;
        "#
    )
    .fetch_all(pg_pool)
    .await
    .unwrap();

    player_ids
        .into_iter()
        .map(|row| row.user_id.into())
        .collect()
}
