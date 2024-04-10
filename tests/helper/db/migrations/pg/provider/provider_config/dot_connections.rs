use lib::{connectors, types::Url};

pub fn get_provider_config(mock_url: String) -> String {
    let config = connectors::dot_connections::DotConnectionsConfig {
        api_url: Url(mock_url),
        api_key: "cert".to_string(),
        ip_list: vec![],
        brand_id: "agent_id".to_string(),
        bet_data_url: Url("data_url".to_string()),
    };

    serde_json::to_string(&config).unwrap()
}
