use sqlx::PgPool;

use self::{
    balance_table::create_balance_table, bet_tables::create_provider_bet_tables,
    lottery_bet_table::create_lottery_bet_table, user_table::create_user_table,
};

mod balance_table;
mod bet_status_table;
mod bet_tables;
mod currency_table;
mod lottery_bet_table;
pub mod provider;
mod user_table;
mod user_upline_table;

pub struct MockUrls {
    pub sexy_mock_url: String,
    pub ameba_mock_url: String,
    pub arcadia_mock_url: String,
    pub dot_connections_mock_url: String,
    pub king_maker_mock_url: String,
    pub pragamtic_mock_url: String,
    pub royal_slot_gaming_mock_url: String,
}

pub async fn create_pg_tables_and_seed(pg: &PgPool, mock_urls: MockUrls) {
    create_user_table(pg).await;
    user_upline_table::create_table(pg).await;
    create_balance_table(pg).await;
    bet_status_table::create_table_and_seed(pg).await;
    currency_table::create_table_and_seed(pg).await;
    create_provider_bet_tables(pg).await;
    create_lottery_bet_table(pg).await;
    provider::create_tables_and_seed(pg, mock_urls).await;
}

async fn create_index(pg: &PgPool, column: &str, table_name: &str) {
    sqlx::query(&format!(
        r#"
            CREATE INDEX IF NOT EXISTS "IDX_{column}_{table_name}"
            ON public."{table_name}" ({column});
        "#,
    ))
    .execute(pg)
    .await
    .expect(&format!(
        "Failed to create index on {column} for '{table_name}' table"
    ));
}
