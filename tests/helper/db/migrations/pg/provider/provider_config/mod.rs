use lib::enums::provider::{LiveCasinoProvider, SlotProvider};
use sqlx::{Execute, PgPool, Postgres, QueryBuilder};

use crate::helper::db::migrations::pg::MockUrls;

mod ameba;
mod arcadia;
mod dot_connections;
mod king_maker;
mod pragamtic;
mod royal_slot_gaming;
mod sexy;

pub async fn create_table_and_seed(pg: &PgPool, mock_urls: MockUrls) {
    let sql = include_str!("../../../../../../../migrations/20240413082655_provider_config.sql");

    sqlx::query(sql)
        .execute(pg)
        .await
        .expect("Failed to create PG 'provider_config' table");

    seed(pg, mock_urls).await;
}

async fn seed(pg: &PgPool, mock_urls: MockUrls) {
    let mut provider_configs = vec![];

    provider_configs.push(arcadia::get_provider_config(mock_urls.arcadia_mock_url));
    provider_configs.push(sexy::get_provider_config(mock_urls.sexy_mock_url));
    provider_configs.push(ameba::get_provider_config(mock_urls.ameba_mock_url));
    provider_configs.push(king_maker::get_provider_config(
        mock_urls.king_maker_mock_url,
    ));
    provider_configs.push(royal_slot_gaming::get_provider_config(
        mock_urls.royal_slot_gaming_mock_url,
    ));

    let dot_connections_config_str =
        dot_connections::get_provider_config(mock_urls.dot_connections_mock_url);

    for dot_connections_provider in [
        SlotProvider::Relax.into_game_provider(),
        SlotProvider::YGG.into_game_provider(),
        SlotProvider::Hacksaw.into_game_provider(),
    ] {
        provider_configs.push((dot_connections_config_str.clone(), dot_connections_provider));
    }

    let pragmatic_config = pragamtic::get_provider_config(mock_urls.pragamtic_mock_url);

    for provider in [
        LiveCasinoProvider::Pragmatic.into_game_provider(),
        SlotProvider::Pragmatic.into_game_provider(),
    ] {
        provider_configs.push((pragmatic_config.clone(), provider));
    }

    let mut query_builder: QueryBuilder<Postgres> =
        QueryBuilder::new("INSERT INTO public.provider_config (game_provider, config)");

    query_builder.push_values(provider_configs.into_iter(), |mut b, (config, provider)| {
        b.push_bind(provider.to_string()).push_bind(config);
    });

    let mut query = query_builder.build();

    sqlx::query_with(query.sql(), query.take_arguments().unwrap())
        .execute(pg)
        .await
        .expect("Failed to insert providers' configs");
}
