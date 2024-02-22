mod opening_balance;

use sqlx::{MySqlPool, PgPool};

pub async fn run(pg: &PgPool, mysql: &MySqlPool) -> anyhow::Result<()> {
    Ok(())
}
