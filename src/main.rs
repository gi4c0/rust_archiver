use lib::{archiver, configuration, db, utils::State};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to parse .env");

    let config = configuration::parse_config();
    let mut state = State::default();

    let pg = db::create_pg_connection(&config.pg).await;
    let mysql = db::create_mysql_connection(&config.mysql).await;

    archiver::run(&pg, &mysql, &mut state).await.unwrap();
}
