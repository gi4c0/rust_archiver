use config::Config;
use serde::Deserialize;
use sqlx::{mysql::MySqlConnectOptions, postgres::PgConnectOptions};

#[derive(Deserialize)]
pub struct AppConfig {
    pub pg: DBConfig,
    pub mysql: DBConfig,
}

#[derive(Deserialize)]
pub struct DBConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub db_name: String,
}

impl DBConfig {
    pub fn get_pg_connection_options(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .password(&self.password)
            .username(&self.user)
            .host(&self.host)
            .port(self.port)
            .database(&self.db_name)
    }

    pub fn get_mysql_connection_options(&self) -> MySqlConnectOptions {
        MySqlConnectOptions::new()
            .password(&self.password)
            .username(&self.user)
            .host(&self.host)
            .port(self.port)
            .database(&self.db_name)
    }
}

pub fn parse_config() -> AppConfig {
    Config::builder()
        .add_source(config::File::with_name("config/local.json"))
        .add_source(config::Environment::with_prefix("PG"))
        .add_source(config::Environment::with_prefix("MARIA_DB"))
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap()
}
