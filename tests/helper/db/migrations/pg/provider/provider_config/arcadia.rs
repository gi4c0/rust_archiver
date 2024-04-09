use lib::{
    connectors::arcadia,
    enums::provider::{GameProvider, OnlineCasinoProvider},
    types::Url,
};

pub fn get_provider_config(mock_url: String) -> (String, GameProvider) {
    let config = arcadia::ArcadiaConfig {
        authentication: "secret".to_string(),
        ip_list: vec![],
        api_url: Url(mock_url),
    };

    (
        serde_json::to_string(&config).expect("Failed to stringify arcadia config"),
        OnlineCasinoProvider::Arcadia.into_game_provider(),
    )
}
