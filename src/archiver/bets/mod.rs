use anyhow::Result;
use arrayvec::ArrayVec;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use sqlx::Transaction;
use time::{Date, OffsetDateTime};

use crate::{
    consts::OPENING_BALANCE_TABLE_NAME,
    enums::provider::GameProvider,
    helpers::{
        add_month, get_figures_date,
        query_helper::{get_archive_schema_name, get_dynamic_table_name},
        State,
    },
    types::{BetID, ChunkVec, Currency, UserID},
};

use self::{
    debts::{calculate_debt_by_bet, create_credit_debt_models, DEBT_SIZE},
    details::extend_bet_with_details,
    loader::{
        delete_bets_by_ids, get_upline, insert_bet_details_to_details_table, save_debts, Bet,
        CreditDebt,
    },
};

use super::{opening_balance::loader::update_opening_balance_amount, CHUNK_SIZE};

mod debts;
mod details;
pub mod loader;

#[derive(Debug)]
struct CurrencyAmount {
    currency: Currency,
    amount: i64,
}

type DebtsByDate = FxHashMap<Date, FxHashMap<UserID, CurrencyAmount>>;
type WlByDateByUser = FxHashMap<Date, FxHashMap<UserID, i64>>;

pub async fn handle_bet_chunk(
    provider: GameProvider,
    bets: ArrayVec<Bet, CHUNK_SIZE>,
    state: &mut State,
    pg_transaction: &mut Transaction<'_, sqlx::Postgres>,
) -> anyhow::Result<()> {
    let mut bet_ids: ChunkVec<BetID> = ArrayVec::new();
    let mut debts: DebtsByDate = FxHashMap::default();

    let mut wl_by_date_by_user: WlByDateByUser = FxHashMap::default();
    let mut bet_details = vec![];

    for bet in bets {
        bet_ids.push(bet.id);

        state
            .username_by_user_id
            .entry(bet.user_id)
            .or_insert(bet.username.clone());

        if !state.upline.contains_key(&bet.user_id) {
            let upline = get_upline(&bet.user_id, pg_transaction).await?;
            state.upline.insert(bet.user_id, upline);
        }

        let figures_date = get_figures_date(bet.last_status_change);

        wl_by_date_by_user
            .entry(figures_date)
            .or_insert_with(FxHashMap::default)
            .entry(bet.user_id)
            .and_modify(|e| *e += bet.wl.unwrap_or(0))
            .or_insert(bet.wl.unwrap_or(0));

        if state.credit_players.contains_key(&bet.user_id) {
            let existing_debts = debts.entry(figures_date).or_insert_with(FxHashMap::default);
            calculate_debt_by_bet(&bet, existing_debts, state)?;
        }

        if let Some(detail) = extend_bet_with_details(state, &bet, provider).await {
            bet_details.push(detail);
        }
    }

    if bet_details.len() > 0 {
        insert_bet_details_to_details_table(&state.maria_db, bet_details).await?;
    }

    save_all(
        pg_transaction,
        provider,
        create_credit_debt_models(debts, state)?,
        bet_ids,
        wl_by_date_by_user,
    )
    .await?;

    Ok(())
}

async fn save_all(
    pg_transaction: &mut Transaction<'_, sqlx::Postgres>,
    provider_or_bet_type: GameProvider,
    debts: FxHashMap<Date, SmallVec<[CreditDebt; DEBT_SIZE]>>,
    bet_ids: ArrayVec<BetID, CHUNK_SIZE>,
    wl_by_date_by_user: WlByDateByUser,
) -> Result<()> {
    for (date, debts) in debts.into_iter() {
        save_debts(pg_transaction, debts, date).await?;
    }

    delete_bets_by_ids(bet_ids, provider_or_bet_type, pg_transaction).await?;

    let current_start_of_month = OffsetDateTime::now_utc().date().replace_day(1).unwrap();

    for (date, wl_by_user) in wl_by_date_by_user.into_iter() {
        let mut start_of_month_table = date.clone().replace_day(1).unwrap();

        loop {
            update_opening_balance_amount(
                pg_transaction,
                get_archive_schema_name(start_of_month_table),
                get_dynamic_table_name(OPENING_BALANCE_TABLE_NAME, start_of_month_table),
                date,
                &wl_by_user,
            )
            .await?;

            start_of_month_table = add_month(start_of_month_table);

            if start_of_month_table > current_start_of_month {
                break;
            }
        }
    }

    Ok(())
}
