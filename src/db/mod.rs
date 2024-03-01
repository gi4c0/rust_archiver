pub mod tables;

use crate::configuration::DBConfig;
use sqlx::{mysql::MySqlPoolOptions, postgres::PgPoolOptions, MySqlPool, PgPool};

pub async fn create_pg_connection(config: &DBConfig) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect_with(config.get_pg_connection_options())
        .await
        .expect("Failed to connect to PostgreSQL DB")
}

pub async fn create_mysql_connection(config: &DBConfig) -> MySqlPool {
    MySqlPoolOptions::new()
        .max_connections(5)
        .connect_with(config.get_mysql_connection_options())
        .await
        .expect("Failed to connect to PostgreSQL DB")
}
