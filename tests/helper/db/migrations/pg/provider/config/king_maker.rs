use lib::{
    connectors::king_maker,
    enums::provider::{GameProvider, OnlineCasinoProvider},
    types::Url,
};

pub fn get_provider_config(mock_url: String) -> (String, GameProvider) {
    let config = king_maker::KingMakerConfig {
        client_secret: "secret".to_string(),
        game_provider_code: "code".to_string(),
        client_id: "id".to_string(),
        ip_list: vec![],
        api_url: Url(mock_url),
        lobby_url: Url(mock_url),
    };

    (
        serde_json::to_string(&config).expect("Failed to stringify ameba config"),
        OnlineCasinoProvider::Kingmaker.into_game_provider(),
    )
}
