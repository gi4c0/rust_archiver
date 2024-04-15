use lib::enums::provider::{ProviderGameKind, SlotProvider};
use sqlx::PgPool;

use crate::helper::db::migrations::pg::create_index;

pub const PROVIDER_VENDOR_ID: &str = "1";
pub const PROVIDER_GAME_LABEL: &str = "Game label";

pub async fn create_table_and_seed(pg: &PgPool) {
    let sql = include_str!("../../../../../../migrations/20240412165558_provider_game.sql");

    sqlx::query(sql)
        .execute(pg)
        .await
        .expect("Failed to create PG 'provider_game' table");

    for column in ["vendor_id", "provider"] {
        create_index(pg, column, "provider_game").await;
    }

    seed(pg).await;
}

async fn seed(pg: &PgPool) {
    sqlx::query(&format!(
        r#"
            INSERT INTO public.provider_game (
                vendor_id,
                product,
                provider,
                label,
                kind,
                visible
            ) VALUES (
                '{PROVIDER_VENDOR_ID}',
                'SLOT',
                '{}',
                '{PROVIDER_GAME_LABEL}',
                '{}',
                TRUE
            )
        "#,
        SlotProvider::RoyalSlotGaming.to_string(),
        ProviderGameKind::Baccarat.to_string()
    ))
    .execute(pg)
    .await
    .expect("Failed to seed provider_game");
}
