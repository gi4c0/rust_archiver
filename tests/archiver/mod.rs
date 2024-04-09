use claims::assert_ok;
use dotenvy::dotenv;
use lib::archiver::opening_balance::create_opening_balance_records;
use lib::archiver::opening_balance::loader::get_last_opening_balance_creation_date;
use lib::connectors::load_connectors;
use lib::consts::OPENING_BALANCE_TABLE_NAME;
use lib::helpers::query_helper::{get_archive_schema_name, get_dynamic_table_name};
use lib::helpers::State;
use lib::types::UserID;
use serde_json::json;
use sqlx::PgPool;
use time::{Duration, OffsetDateTime};
use wiremock::matchers::{method, path, path_regex};
use wiremock::{Mock, ResponseTemplate};

use crate::helper::db::{create_maria_db_test_connection, create_pg_test_connection};
use crate::helper::test_data::prepare_data;

const RS_RESPONSE: &str = r#"LD/bLAL8sNY24+eRG5ZqDzbcL0EfmwpQHHRzAY8FnXE5FdT6AeBBsKfjiO/+YRv+ij/EKCp+X45hXPYDJ0XK5AQlZMrL3UGJOfGtKVZTu1If/lLkUnI5pg1HMZpfpgFAv6P5SPGJ3ZEOkDyf0+Lvp76iyoODlUmiT/a9LeqOpBsi8UtwUY84WZ0j+GQ1zcB87Faei6xwK49Zyavx/SlGkTn6p2RSVjMWigQjAxxj351U/2zJc/3/YAMBqKjieUTpS/wvygssUxloLqWRZbGh6XfPMbbEDlsl"#;

#[tokio::test]
async fn finds_last_opening_balance_and_creates_new_records() {
    dotenv().unwrap();

    let pg_pool = create_pg_test_connection().await;
    let maria_db_pool = create_maria_db_test_connection().await;
    let start_date = OffsetDateTime::now_utc().date() - Duration::days(250);

    let test_data = prepare_data(&pg_pool, &maria_db_pool, start_date).await;

    Mock::given(method("POST"))
        .and(path("/getTransactionHistoryResult"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "0000",
            "desc": "Success",
            "url": "http://localhost"
        })))
        .mount(&test_data.mock_servers.sexy_mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/dms/api"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "error_code": "OK",
            "game_history_url": "http://localhost"
        })))
        .mount(&test_data.mock_servers.ameba_mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/GetGameResult"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "ErrorCode": 0,
            "ErrorMessage": "Success",
            "TimeStamp": "we don't care about god damn time! (and don't parse it :)",
            "Data": {
                "Url": "http://localhost"
            }
        })))
        .mount(&test_data.mock_servers.arcadia_mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/dcs/getReplay"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 1000,
            "msg": "Success",
            "data": {
                "record": "http://localhost"
            }
        })))
        .mount(&test_data.mock_servers.dot_connections_mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path_regex(r"^/history/providers/.+/rounds/.+/users/.+"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "urls": ["http://localhost"]
        })))
        .mount(&test_data.mock_servers.king_maker_mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/OpenHistoryExtended"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "description": "Success",
            "error": 0,
            "url": "http://localhost"
        })))
        .mount(&test_data.mock_servers.pragamtic_mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/Player/GetGameMinDetailURLTokenBySeq"))
        .respond_with(ResponseTemplate::new(200).set_body_string(RS_RESPONSE))
        .mount(&test_data.mock_servers.royal_slot_gaming_mock_server)
        .await;

    let connectors = load_connectors(&pg_pool).await.unwrap();

    let mut state = State::new(connectors, pg_pool, maria_db_pool);

    let result = create_opening_balance_records(&mut state).await;
    assert_ok!(result);

    let credit_user_ids = get_credit_players_ids(&state.pg).await;

    for id in credit_user_ids {
        assert!(state.is_credit_player(id));
    }

    let now = OffsetDateTime::now_utc().date() + Duration::DAY;

    let last_opening_balance_record = get_last_opening_balance_creation_date(
        &state.pg,
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
