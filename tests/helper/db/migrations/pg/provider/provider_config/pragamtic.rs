use lib::{connectors::pragmatic::PragmaticConfig, types::Url};

pub fn get_provider_config(mock_url: String) -> String {
    let config = PragmaticConfig {
        secret_key: "secret".to_string(),
        ip_list: vec![],
        api_url: Url(mock_url),
        username: "user".to_string(),
        casino_name: "casino".to_string(),
        provider_id: "123".to_string(),
        secure_login: "login".to_string(),
        game_server_domain: Url("local".to_string()),
    };

    serde_json::to_string(&config).expect("Failed to stringify pragmatic config")
}
