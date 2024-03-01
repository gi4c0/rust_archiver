pub mod opening_balance;

use sqlx::{MySqlPool, PgPool};

use crate::utils::State;

pub async fn run(pg: &PgPool, _mysql: &MySqlPool, state: &mut State) -> anyhow::Result<()> {
    opening_balance::create_opening_balance_records(pg, state).await?;
    Ok(())
}
