pub mod bets;
pub mod opening_balance;

use anyhow::Context;
use strum::VariantArray;

use crate::{
    configuration, connectors, db,
    enums::provider::{
        GameProvider, LiveCasinoProvider, Lottery, OnlineCasinoProvider, SlotProvider, Sportsbook,
    },
    helpers::{logger::log_error, query_helper::get_bet_table_name, State},
};

use self::bets::{handle_bet_chunk, loader::get_target_data_bench};

pub async fn run(state: &mut State) -> anyhow::Result<()> {
    opening_balance::create_opening_balance_records(state).await?;

    let providers: Vec<GameProvider> = [
        LiveCasinoProvider::VARIANTS
            .into_iter()
            .map(|p| p.clone().into_game_provider())
            .collect(),
        OnlineCasinoProvider::VARIANTS
            .into_iter()
            .map(|p| p.clone().into_game_provider())
            .collect(),
        SlotProvider::VARIANTS
            .into_iter()
            .map(|p| p.clone().into_game_provider())
            .collect(),
        Lottery::VARIANTS
            .into_iter()
            .map(|p| p.clone().into_game_provider())
            .collect(),
        Sportsbook::VARIANTS
            .into_iter()
            .map(|p| p.clone().into_game_provider())
            .collect::<Vec<GameProvider>>(),
    ]
    .concat();

    'provider_bet_for: for provider in providers {
        let runtime_table_name = get_bet_table_name(&provider);

        loop {
            let bet_chunk = get_target_data_bench(&state.pg, &runtime_table_name, None).await?;

            if bet_chunk.len() == 0 {
                continue 'provider_bet_for;
            }

            let mut pg_transaction = state
                .pg
                .begin()
                .await
                .context("Failed to start PG transaction")?;

            handle_bet_chunk(&provider, bet_chunk, state, &mut pg_transaction).await?;
        }
    }

    Ok(())
}

pub async fn launch() {
    dotenvy::dotenv().expect("Failed to parse .env");

    let config = configuration::parse_config();

    let pg = db::create_pg_connection(&config.pg).await;
    let mysql = db::create_mysql_connection(&config.mysql).await;

    let connectors = connectors::load_connectors(&pg).await.unwrap();
    let mut state = State::new(connectors, pg, mysql);

    if let Err(e) = run(&mut state).await {
        log_error(&state.pg, e).await.unwrap();
    }
}
