use std::str::FromStr;

use anyhow::{anyhow, Context, Result};
use rustc_hash::FxHashMap;
use sqlx::PgPool;

use crate::enums::provider::{
    GameProvider, LiveCasinoProvider, OnlineCasinoProvider, SlotProvider,
};

use self::royal_slot_gaming::RoyalSlotGamingGameConfig;

pub mod ae;
pub mod ameba;
pub mod arcadia;
pub mod dot_connections;
pub mod king_maker;
pub mod pragmatic;
pub mod royal_slot_gaming;

#[derive(Debug)]
pub struct Connectors {
    pub ae: ae::Connector,
    pub ameba: ameba::Connector,
    pub arcadia: arcadia::Connector,
    pub king_maker: king_maker::Connector,
    pub pragmatic: pragmatic::Connector,
    pub royal_slot_gaming: royal_slot_gaming::Connector,
    pub dot_connections: dot_connections::Connector,
}

pub async fn load_connectors(pg_pool: &PgPool) -> Result<Connectors> {
    let configs = get_provider_configs(pg_pool).await?;

    let mut sexy_config: Option<ae::Config> = None;
    let mut ameba_config: Option<ameba::AmebaConfig> = None;
    let mut arcadia_config: Option<arcadia::ArcadiaConfig> = None;
    let mut king_maker_config: Option<king_maker::KingMakerConfig> = None;
    let mut pragmatic_config: Option<pragmatic::PragmaticConfig> = None;
    let mut royal_slot_config: Option<royal_slot_gaming::RoyalSlotGamingConfig> = None;
    let mut dot_connections_config: Option<dot_connections::DotConnectionsConfig> = None;

    for config in configs {
        match config.game_provider {
            GameProvider::LiveCasino(LiveCasinoProvider::Sexy) => {
                sexy_config = Some(
                    serde_json::from_str(&config.config).context("Failed to parse Sexy config")?,
                );
            }
            GameProvider::Slot(SlotProvider::Ameba) => {
                ameba_config = Some(
                    serde_json::from_str(&config.config).context("Failed to parse Ameba config")?,
                );
            }
            GameProvider::OnlineCasino(OnlineCasinoProvider::Arcadia) => {
                arcadia_config = Some(
                    serde_json::from_str(&config.config)
                        .context("Failed to parse Arcadia config")?,
                );
            }
            GameProvider::OnlineCasino(OnlineCasinoProvider::Kingmaker) => {
                king_maker_config = Some(
                    serde_json::from_str(&config.config)
                        .context("Failed to parse Kingmaker config")?,
                );
            }
            GameProvider::LiveCasino(LiveCasinoProvider::Pragmatic) => {
                pragmatic_config = Some(
                    serde_json::from_str(&config.config)
                        .context("Failed to parse Pragmatic config")?,
                );
            }
            GameProvider::Slot(SlotProvider::RoyalSlotGaming) => {
                royal_slot_config = Some(
                    serde_json::from_str(&config.config)
                        .context("Failed to parse RoyalSlotGaming config")?,
                );
            }
            GameProvider::Slot(SlotProvider::Relax) => {
                dot_connections_config = Some(
                    serde_json::from_str(&config.config)
                        .context("Failed to parse DotConnections config")?,
                );
            }
            _ => {
                return Err(anyhow!(
                    "Loaded invalid provider config: '{}'",
                    config.game_provider.as_ref()
                ))
            }
        }
    }

    let royal_slot_gaming_configs = load_royal_slot_game_configs(pg_pool).await?;
    let mut games_by_vendor_id = FxHashMap::default();

    for item in royal_slot_gaming_configs {
        games_by_vendor_id.insert(item.game_id.clone(), item);
    }

    Ok(Connectors {
        dot_connections: dot_connections::Connector::new(
            dot_connections_config.context("dot_connections config not found")?,
        ),
        ae: ae::Connector::new(sexy_config.context("sexy config not found")?),
        ameba: ameba::Connector::new(ameba_config.context("ameba config not found")?),
        arcadia: arcadia::Connector::new(arcadia_config.context("arcadia config not found")?),
        king_maker: king_maker::Connector::new(
            king_maker_config.context("king_maker config not found")?,
        ),
        pragmatic: pragmatic::Connector::new(
            pragmatic_config.context("pragmatic config not found")?,
        ),
        royal_slot_gaming: royal_slot_gaming::Connector::new(
            royal_slot_config.context("royal_slot_gaming config not found")?,
            games_by_vendor_id,
        ),
    })
}

struct ProviderConfig {
    game_provider: GameProvider,
    config: String,
}

struct RawProviderConfig {
    game_provider: String,
    config: String,
}

async fn get_provider_configs(pg_pool: &PgPool) -> Result<Vec<ProviderConfig>> {
    let db_data = sqlx::query_as!(
        RawProviderConfig,
        r#"
            SELECT
                game_provider,
                config
            FROM public.provider_config
            WHERE game_provider IN (
                'sexy',
                'ameba',
                'arcadia',
                'kingmaker',
                'pragmatic_live_casino',
                'royal_slot_gaming',
                'relax'
            )
        "#
    )
    .fetch_all(pg_pool)
    .await
    .context("Failed to fetch DB configs")?;

    let mut result = vec![];

    for item in db_data {
        result.push(ProviderConfig {
            config: item.config,
            game_provider: GameProvider::from_str(&item.game_provider)?,
        });
    }

    Ok(result)
}

async fn load_royal_slot_game_configs(pg_pool: &PgPool) -> Result<Vec<RoyalSlotGamingGameConfig>> {
    let db_data = sqlx::query!(
        r#"
            SELECT
                config
            FROM public.provider_game_config pc
            JOIN public.provider_game pg ON pc.game_id = pg.id
            WHERE pg.provider = $1
        "#,
        SlotProvider::RoyalSlotGaming.as_ref()
    )
    .fetch_all(pg_pool)
    .await
    .context("Failed to load royal_slot_gaming game configs")?;

    let mut result = vec![];

    for item in db_data {
        let parsed = serde_json::from_str(&item.config)
            .context("Failed to parse royal_slot_gaming game config")?;
        result.push(parsed);
    }

    Ok(result)
}
