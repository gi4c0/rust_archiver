pub mod bets;
pub mod opening_balance;

use sqlx::{MySqlPool, PgPool};

use crate::{db::tables::BET_TABLES, helpers::State};

use self::bets::loader::get_target_data_bench;

pub async fn run(pg: &PgPool, _mysql: &MySqlPool, state: &mut State) -> anyhow::Result<()> {
    opening_balance::create_opening_balance_records(pg, state).await?;

    'provider_bet_for: for table_name in BET_TABLES {
        let bet_chunk = get_target_data_bench(&pg, table_name, None).await?;

        if bet_chunk.len() == 0 {
            continue 'provider_bet_for;
        }

        loop {}
    }

    Ok(())
}
