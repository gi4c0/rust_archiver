use anyhow::Result;
use utils::State;

pub mod archiver;
pub mod configuration;
pub mod connectors;
pub mod db;
pub mod enums;
pub mod types;
pub mod utils;

async fn prepare_data_and_run() -> Result<()> {
    dotenvy::dotenv().expect("Failed to parse .env");

    let config = configuration::parse_config();

    let pg = db::create_pg_connection(&config.pg).await;
    let mysql = db::create_mysql_connection(&config.mysql).await;

    let connectors = connectors::load_connectors(&pg).await.unwrap();
    let mut state = State::new(connectors);

    archiver::run(&pg, &mysql, &mut state).await
}

pub async fn launch() {
    if let Err(e) = prepare_data_and_run().await {
        // TODO: log
        panic!("{e}");
    }
}
