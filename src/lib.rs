use helpers::State;

use crate::helpers::logger::log_error;

pub mod archiver;
pub mod configuration;
pub mod connectors;
pub mod db;
pub mod enums;
pub mod helpers;
pub mod types;

pub async fn launch() {
    dotenvy::dotenv().expect("Failed to parse .env");

    let config = configuration::parse_config();

    let pg = db::create_pg_connection(&config.pg).await;
    let mysql = db::create_mysql_connection(&config.mysql).await;

    let connectors = connectors::load_connectors(&pg).await.unwrap();
    let mut state = State::new(connectors);

    if let Err(e) = archiver::run(&pg, &mysql, &mut state).await {
        log_error(&pg, e).await.unwrap();
    }
}
