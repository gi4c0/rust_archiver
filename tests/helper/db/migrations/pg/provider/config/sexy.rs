use lib::{
    connectors,
    enums::provider::{GameProvider, LiveCasinoProvider},
    types::Url,
};

pub fn get_provider_config(mock_url: String) -> (String, GameProvider) {
    let config = connectors::ae::Config {
        host: Url(mock_url),
        cert: "cert".to_string(),
        ip_list: vec![],
        agent_id: "agent_id".to_string(),
        secret_key: "".to_string(),
    };

    (
        serde_json::to_string(&config).unwrap(),
        LiveCasinoProvider::Sexy.into_game_provider(),
    )
}
