use lib::enums::provider::{LiveCasinoProvider, SlotProvider};
use sqlx::{Execute, PgPool, Postgres, QueryBuilder};

use crate::helper::migrations::pg::MockUrls;

mod ameba;
mod arcadia;
mod dot_connections;
mod king_maker;
mod pragamtic;
mod royal_slot_gaming;
mod sexy;

pub async fn create_provider_config_table(pg: &PgPool) {
    sqlx::query(
        r#"
            create table if not exists public.provider_config
            (
                game_provider varchar(255) not null
                    constraint "PK_49e30eb371654ab9bfbb63ebb02"
                        primary key
                    constraint "FK_49e30eb371654ab9bfbb63ebb02"
                        references public.provider,
                config        text         not null
            )
        "#,
    )
    .execute(pg)
    .await
    .expect("Failed to create PG 'provider_config' table");
}

pub async fn seed(pg: &PgPool, mock_urls: MockUrls) {
    let mut provider_configs = vec![];

    provider_configs.push(sexy::get_provider_config(mock_urls.sexy_mock_url));
    provider_configs.push(ameba::get_provider_config(mock_urls.ameba_mock_url));
    provider_configs.push(king_maker::get_provider_config(
        mock_urls.king_maker_mock_url,
    ));
    provider_configs.push(royal_slot_gaming::get_provider_config(
        mock_urls.royal_slot_gaming_mock_url,
    ));
    provider_configs.push(dot_connections::get_provider_config(
        mock_urls.dot_connections_mock_url,
    ));

    let pragmatic_config = pragamtic::get_provider_config(mock_urls.pragamtic_mock_url);

    for provider in [
        LiveCasinoProvider::Pragmatic.into_game_provider(),
        SlotProvider::Pragmatic.into_game_provider(),
    ] {
        provider_configs.push((pragmatic_config.clone(), provider));
    }

    let mut query_builder: QueryBuilder<Postgres> =
        QueryBuilder::new("INSERT INTO public.provider_config VALUES (game_provider, config)");

    query_builder.push_values(provider_configs.into_iter(), |mut b, (config, provider)| {
        b.push_bind(provider.to_string()).push_bind(config);
    });

    let mut query = query_builder.build();

    sqlx::query_with(query.sql(), query.take_arguments().unwrap())
        .execute(pg)
        .await
        .expect("Failed to insert providers' configs");
}
