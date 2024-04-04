use sqlx::PgPool;

use self::{
    balance_table::create_balance_table, bet_status_table::create_bet_status_table,
    bet_tables::create_provider_bet_tables, currency_table::create_bet_currency_table,
    lottery_bet_table::create_lottery_bet_table, user_table::create_user_table,
};

mod balance_table;
mod bet_status_table;
mod bet_tables;
mod currency_table;
mod lottery_bet_table;
mod provider;
mod user_table;

pub struct MockUrls {
    pub sexy_mock_url: String,
    pub ameba_mock_url: String,
    pub dot_connections_mock_url: String,
    pub king_maker_mock_url: String,
    pub pragamtic_mock_url: String,
    pub royal_slot_gaming_mock_url: String,
}

async fn create_pg_tables(pg: &PgPool) {
    create_user_table(pg).await;
    create_balance_table(pg).await;
    create_bet_status_table(pg).await;
    create_bet_currency_table(pg).await;
    create_provider_bet_tables(pg).await;
    create_lottery_bet_table(pg).await;
    provider::create_tables(pg).await;
}

async fn seed(pg: &PgPool, mock_urls: MockUrls) {
    bet_status_table::seed(pg).await;
    currency_table::seed(pg).await;
    provider::seed(pg, mock_urls).await;
}

pub async fn create_pg_tables_and_seed(pg: &PgPool, mock_urls: MockUrls) {
    create_pg_tables(pg).await;
    seed(pg, mock_urls).await;
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
    .expect(&format!("Failed to create {table_name} table"));
}
