use lib::{
    connectors::royal_slot_gaming::RoyalSlotGamingGameConfig,
    enums::provider::ProviderGameKind,
    types::{ProviderGameVendorID, ProviderGameVendorLabel},
};
use sqlx::PgPool;

use super::provider_game_table::{PROVIDER_GAME_LABEL, PROVIDER_VENDOR_ID};

pub async fn create_table_and_seed(pg: &PgPool) {
    sqlx::query(
        r#"
            create table if not exists public.provider_game_config
            (
                game_id uuid not null
                    constraint "PK_4ba0c714256899c77f4ab2176f8"
                        primary key
                    constraint "FK_4ba0c714256899c77f4ab2176f8"
                        references public.provider_game,
                config  text not null
            );
        "#,
    )
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
