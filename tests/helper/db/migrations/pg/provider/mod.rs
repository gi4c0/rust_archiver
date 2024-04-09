use sqlx::PgPool;

use super::MockUrls;

mod product_table;
mod provider_config;
mod provider_game_config_table;
mod provider_game_kind_table;
pub mod provider_game_table;
mod provider_table;

pub async fn create_tables_and_seed(pg: &PgPool, mock_urls: MockUrls) {
    product_table::create_table_and_seed(pg).await;
    provider_table::create_table_and_seed(pg).await;
    provider_config::create_table_and_seed(pg, mock_urls).await;
    provider_game_kind_table::create_table_and_seed(pg).await;
    provider_game_table::create_table_and_seed(pg).await;
    provider_game_config_table::create_table_and_seed(pg).await;
}
