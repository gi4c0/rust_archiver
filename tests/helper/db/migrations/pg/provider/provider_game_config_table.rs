use lib::{
    connectors::royal_slot_gaming::RoyalSlotGamingGameConfig,
    enums::provider::ProviderGameKind,
    types::{ProviderGameVendorID, ProviderGameVendorLabel},
};
use sqlx::PgPool;

use super::provider_game_table::{PROVIDER_GAME_LABEL, PROVIDER_VENDOR_ID};

pub async fn create_table_and_seed(pg: &PgPool) {
    let sql = include_str!("../../../../../../migrations/20240412165559_provider_game_config.sql");

    sqlx::query(sql)
        .execute(pg)
        .await
        .expect("Failed to create PG 'balance' table");

    seed(pg).await;
}

async fn seed(pg: &PgPool) {
    let rs_game_config = RoyalSlotGamingGameConfig {
        game_id: ProviderGameVendorID(PROVIDER_VENDOR_ID.to_string()),
        game_type: ProviderGameKind::Baccarat,
        game_label: ProviderGameVendorLabel(PROVIDER_GAME_LABEL.to_string()),
    };

    let json_config = serde_json::to_string(&rs_game_config).unwrap();

    sqlx::query!(
        r#"
            INSERT INTO public.provider_game_config (
                game_id,
                config
            ) VALUES (
                (SELECT id FROM public.provider_game LIMIT 1),
                $1
            )
        "#,
        json_config
    )
    .execute(pg)
    .await
    .expect("Failed to seed provider_game_config table");
}
