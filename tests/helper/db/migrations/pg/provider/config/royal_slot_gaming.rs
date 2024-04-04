use lib::{
    connectors::royal_slot_gaming,
    enums::provider::{GameProvider, SlotProvider},
    types::Url,
};

pub fn get_provider_config(mock_url: String) -> (String, GameProvider) {
    let config = royal_slot_gaming::RoyalSlotGamingConfig {
        des_iv: "AAK5KLJN".to_string(),
        des_key: "9R88X9NY".to_string(),
        web_id: "NewSportsbook".to_string(),
        client_id: "123".to_string(),
        system_code: "400".to_string(),
        client_secret: "V98KXA46".to_string(),
        ip_list: vec![],
        api_url: Url(mock_url),
    };

    (
        serde_json::to_string(&config).expect("Failed to stringify ameba config"),
        SlotProvider::RoyalSlotGaming.into_game_provider(),
    )
}
