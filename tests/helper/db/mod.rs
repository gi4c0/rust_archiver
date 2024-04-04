pub mod migrations;

use std::env;

use lib::helpers::{provider::get_game_providers, query_helper::get_bet_table_name};
use sqlx::{
    mysql::{MySqlConnectOptions, MySqlPoolOptions},
    postgres::{PgConnectOptions, PgPoolOptions},
    MySqlPool, PgPool,
};

pub async fn create_pg_test_connection() -> PgPool {
    let connect_options = PgConnectOptions::new()
        .host(&env::var("DB_HOST").expect("DB_HOST is not set"))
        .port(
            env::var("DB_PORT")
                .expect("DB_PORT is not set")
                .parse()
                .expect("DB_PORT is not a number"),
        )
        .database("archiver_rust_test")
        .username(&env::var("TYPEORM_USERNAME").expect("TYPEORM_USERNAME is not set"))
        .password(&env::var("TYPEORM_PASSWORD").expect("TYPEORM_PASSWORD is not set"));

    let conn = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await
        .expect("Failed to connect to PostgreSQL DB");

    truncate_pg_tables(&conn).await;

    conn
}

pub async fn create_maria_db_test_connection() -> MySqlPool {
    let connect_options = MySqlConnectOptions::new()
        .host(&env::var("MARIA_DB_HOST").expect("MARIA_DB_HOST is not set"))
        .port(
            env::var("MARIA_DB_PORT")
                .expect("MARIA_DB_PORT is not set")
                .parse()
                .expect("MARIA_DB_PORT is not a number"),
        )
        .username(&env::var("MARIA_DB_USERNAME").expect("MARIA_DB_USERNAME is not set"))
        .password(&env::var("MARIA_DB_PASSWORD").expect("MARIA_DB_PASSWORD is not set"));

    let conn = MySqlPoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await
        .expect("Failed to connect to MariaDB");

    conn
}

async fn truncate_pg_tables(pg: &PgPool) {
    let table_names: Vec<String> = vec![
        "balance".to_string(),
        "user".to_string(),
        "bet_status".to_string(),
        "currency".to_string(),
        "bet_lottery".to_string(),
        "provider".to_string(),
        "provider_config".to_string(),
    ];

    let bet_table_names: Vec<String> = get_game_providers()
        .into_iter()
        .map(get_bet_table_name)
        .collect();

    for table in [table_names, bet_table_names].concat() {
        sqlx::query(&format!("TRUNCATE TABLE public.{table} CASCADE;"))
            .execute(pg)
            .await
            .expect("Failed to truncate table");
    }
}

async fn truncate_maria_db_tables(pg: &MySqlPool) {
    let table_names = vec!["user_card", "bet", "bet_archive_details"];

    for table in table_names {
        sqlx::query(&format!("TRUNCATE TABLE public.{table};"))
            .execute(pg)
            .await
            .expect("Failed to truncate table");
    }
}

pub async fn drop_schema(pg: &PgPool, schema_name: &str) {
    sqlx::query(&format!(r#"DROP SCHEMA IF EXISTS {schema_name} CASCADE;"#))
        .execute(pg)
        .await
        .expect("Failed to create archive schema");
}

pub async fn create_archive_schema(pg: &PgPool, schema_name: &str) {
    sqlx::query(&format!(r#"CREATE SCHEMA IF NOT EXISTS {schema_name};"#))
        .execute(pg)
        .await
        .expect("Failed to create archive schema");
}
