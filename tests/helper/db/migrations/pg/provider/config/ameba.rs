use lib::{
    connectors::ameba,
    enums::provider::{GameProvider, SlotProvider},
    types::Url,
};

pub fn get_provider_config(mock_url: String) -> (String, GameProvider) {
    let config = ameba::AmebaConfig {
        secret_key: "secret".to_string(),
        ip_list: vec![],
        api_url: Url(mock_url),
        site_id: "site_id".to_string(),
    };

    (
        serde_json::to_string(&config).expect("Failed to stringify ameba config"),
        SlotProvider::Ameba.into_game_provider(),
    )
}
