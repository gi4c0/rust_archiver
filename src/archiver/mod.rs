pub mod bets;
pub mod opening_balance;

use sqlx::{MySqlPool, PgPool};

use crate::{
    configuration, connectors,
    db::{self, tables::BET_TABLES},
    helpers::{logger::log_error, State},
};

use self::bets::loader::get_target_data_bench;

pub async fn run(state: &mut State) -> anyhow::Result<()> {
    opening_balance::create_opening_balance_records(state).await?;

    'provider_bet_for: for table_name in BET_TABLES {
        let bet_chunk = get_target_data_bench(&state.pg, table_name, None).await?;

        if bet_chunk.len() == 0 {
            continue 'provider_bet_for;
        }

        loop {}
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
