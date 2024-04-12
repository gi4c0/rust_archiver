use claims::assert_ok;
use dotenvy::dotenv;
use lib::archiver::run;
use lib::connectors::load_connectors;
use lib::consts::{CREDIT_DEBT_TABLE_NAME, OPENING_BALANCE_TABLE_NAME};
use lib::enums::provider::GameProvider;
use lib::enums::PositionEnum;
use lib::helpers::query_helper::{
    get_archive_schema_name, get_bet_table_name, get_dynamic_table_name,
};
use lib::helpers::{add_month, get_hong_kong_11_hours_from_date, State};
use lib::types::UserID;
use serde_json::json;
use sqlx::prelude::FromRow;
use sqlx::{PgPool, Row};
use time::{Date, Duration, OffsetDateTime};
use wiremock::matchers::{method, path, path_regex};
use wiremock::{Mock, ResponseTemplate};

use crate::helper::db::{create_maria_db_test_connection, create_pg_test_connection};
use crate::helper::test_data::{get_yesterday_11, prepare_data, TEST_PROVIDERS};

const RS_RESPONSE: &str = r#"LD/bLAL8sNY24+eRG5ZqDzbcL0EfmwpQHHRzAY8FnXE5FdT6AeBBsKfjiO/+YRv+ij/EKCp+X45hXPYDJ0XK5AQlZMrL3UGJOfGtKVZTu1If/lLkUnI5pg1HMZpfpgFAv6P5SPGJ3ZEOkDyf0+Lvp76iyoODlUmiT/a9LeqOpBsi8UtwUY84WZ0j+GQ1zcB87Faei6xwK49Zyavx/SlGkTn6p2RSVjMWigQjAxxj351U/2zJc/3/YAMBqKjieUTpS/wvygssUxloLqWRZbGh6XfPMbbEDlsl"#;

#[tokio::test]
async fn finds_last_opening_balance_and_creates_new_records() {
    dotenv().unwrap();
    env_logger::init();

    let pg_pool = create_pg_test_connection().await;
    let maria_db_pool = create_maria_db_test_connection().await;
    let start_date = OffsetDateTime::now_utc().date() - Duration::days(10);

    let t_data = prepare_data(&pg_pool, &maria_db_pool, start_date).await;

    Mock::given(method("POST"))
        .and(path("/getTransactionHistoryResult"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "0000",
            "desc": "Success",
            "url": "http://localhost"
        })))
        // .expect(1..)
        .named("sexy")
        .mount(&t_data.mock_servers.sexy_mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/dms/api"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "error_code": "OK",
            "game_history_url": "http://localhost"
        })))
        // .expect(1..)
        .named("ameba")
        .mount(&t_data.mock_servers.ameba_mock_server)
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
        // .expect(1..)
        .named("arcadia")
        .mount(&t_data.mock_servers.arcadia_mock_server)
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
        // .expect(1..)
        .named("dot_connections")
        .mount(&t_data.mock_servers.dot_connections_mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path_regex(r"^/history/providers/.+/rounds/.+/users/.+"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "urls": ["http://localhost"]
        })))
        // .expect(1..)
        .named("king_maker")
        .mount(&t_data.mock_servers.king_maker_mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/OpenHistoryExtended"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "description": "Success",
            "error": 0,
            "url": "http://localhost"
        })))
        // .expect(1..)
        .named("pragmatic")
        .mount(&t_data.mock_servers.pragamtic_mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/Player/GetGameMinDetailURLTokenBySeq"))
        .respond_with(ResponseTemplate::new(200).set_body_string(RS_RESPONSE))
        // .expect(1..)
        .named("royal_slot")
        .mount(&t_data.mock_servers.royal_slot_gaming_mock_server)
        .await;

    let connectors = load_connectors(&pg_pool).await.unwrap();
    let mut state = State::new(connectors, pg_pool, maria_db_pool);

    let result = run(&mut state).await;
    assert_ok!(result);

    // Check opening balance
    let mut total_wl: i64 = 0;
    let yesterday11 =
        get_hong_kong_11_hours_from_date(OffsetDateTime::now_utc().date() - Duration::days(1));

    let mut all_credit_bets = vec![];

    for (_, bets) in &t_data.bets_by_provider {
        for bet in bets {
            if bet.user_id == t_data.credit_player.id && bet.last_status_change < yesterday11 {
                all_credit_bets.push(bet.clone());
                total_wl += bet.wl.unwrap_or(0);
            }
        }
    }

    let last_opening_balance_amount =
        get_last_opening_balance_amount(&state.pg, t_data.credit_player.id).await;

    assert_eq!(total_wl, last_opening_balance_amount);

    for provider in TEST_PROVIDERS {
        let count = count_bets_before_yesterday11(&state.pg, provider).await;
        assert_eq!(count, 0);
    }

    let debts = get_debts_from_date(&state.pg, start_date).await;

    for debt in debts {
        let start_debt_date = debt.date - Duration::days(1);
        let position = debt.position as usize;
        let mut expected_debt_amount = 0;

        for bet in &all_credit_bets {
            if bet.last_status_change >= start_debt_date
                && bet.last_status_change < debt.date
                && bet.user_id == t_data.credit_player.id
            {
                expected_debt_amount += bet.commission_amount[position] + bet.funds_delta[position];
            }
        }

        if expected_debt_amount != debt.debt_amount {
            dbg!(debt.date);
        }

        assert_eq!(expected_debt_amount, debt.debt_amount);
    }
}

async fn get_last_opening_balance_amount(pg: &PgPool, user_id: UserID) -> i64 {
    let yesterday = OffsetDateTime::now_utc().date() - Duration::days(1);
    let schema = get_archive_schema_name(yesterday);

    let table = get_dynamic_table_name(OPENING_BALANCE_TABLE_NAME, yesterday);
    let opening_balance_date = get_hong_kong_11_hours_from_date(yesterday);

    let row = sqlx::query(&format!(
        r#"
            SELECT amount FROM {schema}.{table}
            WHERE user_id = $1 AND creation_date = $2
        "#
    ))
    .bind(user_id)
    .bind(opening_balance_date)
    .fetch_one(pg)
    .await
    .expect(&format!(
        "Failed to fetch last opening balance for {}",
        user_id
    ));

    let amount: i64 = row
        .try_get("amount")
        .expect("Failed to get amount from 'get_last_opening_balance_amount' query");

    amount
}

async fn count_bets_before_yesterday11(pg: &PgPool, provider: GameProvider) -> i64 {
    let table = get_bet_table_name(provider);
    let yesterday = get_yesterday_11();

    let result = sqlx::query(&format!(
        r#"
            SELECT COUNT(*) as count FROM public.{table}
            WHERE last_status_change < $1
        "#
    ))
    .bind(yesterday)
    .fetch_optional(pg)
    .await
    .unwrap();

    let count: i64 = result
        .unwrap()
        .try_get("count")
        .expect("Failed to get count from 'count_bets'");

    count
}

#[derive(FromRow)]
struct UserDebt {
    date: OffsetDateTime,
    debt_amount: i64,
    position: PositionEnum,
}

async fn get_debts_from_date(pg: &PgPool, start_date: Date) -> Vec<UserDebt> {
    let mut queries = vec![];
    let current_month = OffsetDateTime::now_utc().date().replace_day(1).unwrap();
    let mut iteration_date = start_date.replace_day(1).unwrap();

    while iteration_date <= current_month {
        let schema = get_archive_schema_name(iteration_date);
        let table = get_dynamic_table_name(CREDIT_DEBT_TABLE_NAME, iteration_date);

        queries.push(format!(
            r#"
                SELECT
                    d.date,
                    d.debt_amount,
                    u.position
                FROM {schema}.{table} d
                JOIN public.user u ON u.id = d.user_id
            "#
        ));
        iteration_date = add_month(iteration_date);
    }

    let query = format!("{} ORDER BY date", queries.join(" UNION ALL "));

    let result: Vec<UserDebt> = sqlx::query_as(&query)
        .fetch_all(pg)
        .await
        .expect("Failed to fetch debts");

    result
}
