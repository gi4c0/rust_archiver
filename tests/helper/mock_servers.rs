use serde_json::json;
use wiremock::matchers::{method, path, path_regex};
use wiremock::{Mock, ResponseTemplate};

use super::test_data::TestData;

const RS_RESPONSE: &str = r#"LD/bLAL8sNY24+eRG5ZqDzbcL0EfmwpQHHRzAY8FnXE5FdT6AeBBsKfjiO/+YRv+ij/EKCp+X45hXPYDJ0XK5AQlZMrL3UGJOfGtKVZTu1If/lLkUnI5pg1HMZpfpgFAv6P5SPGJ3ZEOkDyf0+Lvp76iyoODlUmiT/a9LeqOpBsi8UtwUY84WZ0j+GQ1zcB87Faei6xwK49Zyavx/SlGkTn6p2RSVjMWigQjAxxj351U/2zJc/3/YAMBqKjieUTpS/wvygssUxloLqWRZbGh6XfPMbbEDlsl"#;

pub async fn mount_mock_servers(t_data: &TestData) {
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
        .expect(1..)
        .named("royal_slot")
        .mount(&t_data.mock_servers.royal_slot_gaming_mock_server)
        .await;
}
