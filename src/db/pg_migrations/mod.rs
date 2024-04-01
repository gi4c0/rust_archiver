use sqlx::PgPool;

use crate::db::pg_migrations::{
    bet_tables::create_provider_bet_tables, user_table::create_user_table,
};

use self::{
    balance_table::create_balance_table, bet_status_table::create_bet_status_table,
    currency_table::create_bet_currency_table, lottery_bet_table::create_lottery_bet_table,
};

mod balance_table;
mod bet_status_table;
mod bet_tables;
mod currency_table;
mod lottery_bet_table;
mod user_table;

pub async fn create_pg_tables(pg_pool: &PgPool) {
    create_user_table(pg_pool).await;
    create_balance_table(pg_pool).await;
    create_bet_status_table(pg_pool).await;
    create_bet_currency_table(pg_pool).await;
    create_provider_bet_tables(pg_pool).await;
    create_lottery_bet_table(pg_pool).await;
}

pub async fn seed(pg: &PgPool) {
    bet_status_table::seed(pg).await;
    currency_table::seed(pg).await;
}

pub async fn create_pg_tables_and_seed(pg: &PgPool) {
    create_pg_tables(pg).await;
    seed(pg).await;
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
