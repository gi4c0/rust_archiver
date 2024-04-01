use std::env;
pub mod maria_db_migrations;
pub mod pg_migrations;

use sqlx::{
    mysql::{MySqlConnectOptions, MySqlPoolOptions},
    postgres::{PgConnectOptions, PgPoolOptions},
    MySqlPool, PgPool,
};

use self::{maria_db_migrations::create_maria_db_tables, pg_migrations::create_pg_tables_and_seed};

pub async fn create_pg_connection() -> PgPool {
    let connect_options = PgConnectOptions::new()
        .host(&env::var("DB_HOST").expect("DB_HOST is not set"))
        .port(
            env::var("DB_PORT")
                .expect("DB_PORT is not set")
                .parse()
                .expect("DB_PORT is not a number"),
        )
        .username(&env::var("TYPEORM_USERNAME").expect("TYPEORM_USERNAME is not set"))
        .password(&env::var("TYPEORM_PASSWORD").expect("TYPEORM_PASSWORD is not set"));

    let conn = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await
        .expect("Failed to connect to PostgreSQL DB");

    create_pg_tables_and_seed(&conn).await;
    conn
}

pub async fn create_mysql_connection() -> MySqlPool {
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
        .expect("Failed to connect to PostgreSQL DB");

    create_maria_db_tables(&conn).await;
    conn
}
