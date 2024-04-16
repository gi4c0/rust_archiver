pub mod migrations;

use std::env;

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
        .database("alexpan") // TODO: use env var
        .username(&env::var("TYPEORM_USERNAME").expect("TYPEORM_USERNAME is not set"))
        .password(&env::var("TYPEORM_PASSWORD").expect("TYPEORM_PASSWORD is not set"));

    let conn = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await
        .expect("Failed to connect to PostgreSQL DB");

    drop_and_create_pg_public_schema(&conn).await;

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

    drop_and_create_maria_db_public_schema(&conn).await;

    conn
}

async fn drop_and_create_pg_public_schema(pg: &PgPool) {
    sqlx::query("DROP SCHEMA public CASCADE;")
        .execute(pg)
        .await
        .expect("Failed to drop pg public schema");

    sqlx::query("CREATE SCHEMA public;")
        .execute(pg)
        .await
        .expect("Failed to create pg public schema");

    sqlx::query(r#"CREATE EXTENSION IF NOT EXISTS "uuid-ossp";"#)
        .execute(pg)
        .await
        .expect("Failed to create pg uuid extension");
}

async fn drop_and_create_maria_db_public_schema(maria_db: &MySqlPool) {
    sqlx::query("DROP DATABASE public;")
        .execute(maria_db)
        .await
        .expect("Failed to drop maria_db public schema");

    sqlx::query("CREATE DATABASE public;")
        .execute(maria_db)
        .await
        .expect("Failed to create maria_db public schema");
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
