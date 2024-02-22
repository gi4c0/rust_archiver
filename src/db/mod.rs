pub mod tables;

use std::env;

use sqlx::{mysql::MySqlPoolOptions, postgres::PgPoolOptions, MySqlPool, PgPool};

pub async fn create_pg_connection() -> PgPool {
    let host = env::var("PG_HOST").unwrap();
    let port = env::var("PG_PORT").unwrap();
    let user = env::var("PG_USER").unwrap();
    let password = env::var("PG_PASSWORD").unwrap();
    let db_name = env::var("PG_DB_NAME").unwrap();

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&format!(
            "postgres://{user}:{password}@{host}:{port}/{db_name}"
        ))
        .await
        .expect("Failed to connect to PostgreSQL DB")
}

pub async fn create_mysql_connection() -> MySqlPool {
    let host = env::var("MARIADB_HOST").unwrap();
    let port = env::var("MARIADB_PORT").unwrap();
    let user = env::var("MARIADB_USER").unwrap();
    let password = env::var("MARIADB_PASSWORD").unwrap();
    let db_name = env::var("MARIADB_DB_NAME").unwrap();

    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&format!(
            "mariadb://{user}:{password}@{host}:{port}/{db_name}"
        ))
        .await
        .expect("Failed to connect to PostgreSQL DB")
}
