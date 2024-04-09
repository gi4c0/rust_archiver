use lib::enums::provider::{ProviderGameKind, SlotProvider};
use sqlx::PgPool;

use crate::helper::db::migrations::pg::create_index;

pub const PROVIDER_VENDOR_ID: &str = "Game_ID #1";
pub const PROVIDER_GAME_LABEL: &str = "Game label";

pub async fn create_table_and_seed(pg: &PgPool) {
    sqlx::query(
        r#"
            create table if not exists public.provider_game
            (
                id        uuid default uuid_generate_v4() not null
                    constraint "PK_d2d465ad8bd45aee8716c5ea094"
                        primary key,
                vendor_id varchar(50),
                product   varchar(255)                    not null
                    constraint "FK_4b1384cdd5f1ee4b92bbb4a9a63"
                        references public.product,
                provider  varchar(255)                    not null
                    constraint "FK_92fbb0a04cf3d78134ab12135f9"
                        references public.provider,
                label     varchar(255)                    not null,
                kind      varchar(50)                     not null
                    constraint "FK_02d8997a76f989c8e0488ef2bb1"
                        references public.provider_game_kind,
                visible   boolean                         not null,
                constraint "UQ_406b78ef35689f135019a334836"
                    unique (provider, label)
            );
        "#,
    )
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
