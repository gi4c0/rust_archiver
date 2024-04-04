use sqlx::PgPool;

use self::{config::create_provider_config_table, provider_table::create_provider_table};

use super::MockUrls;

mod config;
mod provider_table;

pub async fn create_tables(pg: &PgPool) {
    create_provider_table(pg).await;
    create_provider_config_table(pg).await;
}

pub async fn seed(pg: &PgPool, mock_urls: MockUrls) {
    provider_table::seed(pg).await;

    config::seed(pg, mock_urls).await;
}
