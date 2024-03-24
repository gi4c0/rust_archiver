use std::collections::HashMap;

use anyhow::{anyhow, Result};
use serde_json::json;
use sqlx::Transaction;
use time::{Date, OffsetDateTime};
use uuid::Uuid;

use crate::{
    consts::OPENING_BALANCE_TABLE_NAME,
    enums::provider::{GameProvider, LiveCasinoProvider, OnlineCasinoProvider, SlotProvider},
    helpers::{
        add_month, get_figures_date, get_hong_kong_11_hours_from_date,
        query_helper::{get_archive_schema_name, get_dynamic_table_name},
        State,
    },
    types::{BetID, Currency, UserID},
};

use self::loader::{
    delete_bets_by_ids, get_upline, insert_bet_details_to_details_table, save_debts, Bet,
    BetDetails, CreditDebt,
};

use super::opening_balance::loader::update_opening_balance_amount;

pub mod loader;

#[derive(Debug)]
struct CurrencyAmount {
    currency: Currency,
    amount: i64,
}

type DebtsByDate = HashMap<Date, HashMap<UserID, CurrencyAmount>>;
type WlByDateByUser = HashMap<Date, HashMap<UserID, i64>>;

pub async fn handle_bet_chunk(
    provider: GameProvider,
    bets: Vec<Bet>,
    state: &mut State,
    pg_transaction: &mut Transaction<'_, sqlx::Postgres>,
) -> anyhow::Result<()> {
    let mut bet_ids: Vec<BetID> = vec![];
    let mut debts: DebtsByDate = HashMap::new();
    let mut wl_by_date_by_user: WlByDateByUser = HashMap::new();
    let mut bet_details = vec![];

    for bet in bets {
        bet_ids.push(bet.id.clone());

        state
            .username_by_user_id
            .entry(bet.user_id.clone())
            .or_insert(bet.username.clone());

        if !state.upline.contains_key(&bet.user_id) {
            let upline = get_upline(&bet.user_id, pg_transaction).await?;
            state.upline.insert(bet.user_id.clone(), upline);
        }

        let figures_date = get_figures_date(bet.last_status_change);

        wl_by_date_by_user
            .entry(figures_date)
            .or_insert_with(HashMap::new)
            .entry(bet.user_id.clone())
            .and_modify(|e| *e += bet.wl.unwrap_or(0))
            .or_insert(bet.wl.unwrap_or(0));

        if !state.credit_players.contains_key(&bet.user_id) {
            let existing_debts = debts.entry(figures_date).or_insert_with(HashMap::new);
            calculate_debt_by_bet(&bet, existing_debts, state);
        }

        if let Some(detail) = extend_bet_with_details(state, &bet, provider).await {
            bet_details.push(detail);
        }
    }

    if bet_details.len() > 0 {
        insert_bet_details_to_details_table(&state.mysql, bet_details).await?;
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
    debts: HashMap<Date, Vec<CreditDebt>>,
    bet_ids: Vec<BetID>,
    wl_by_date_by_user: WlByDateByUser,
) -> Result<()> {
    for (date, debts) in debts.into_iter() {
        save_debts(pg_transaction, debts, date).await?;
    }

    delete_bets_by_ids(bet_ids, provider_or_bet_type, pg_transaction).await?;

    let current_start_of_month = OffsetDateTime::now_utc().date().replace_day(1).unwrap();

    for (date, wl_by_user) in wl_by_date_by_user.into_iter() {
        let mut start_of_month_table = date.replace_day(1).unwrap();

        loop {
            update_opening_balance_amount(
                pg_transaction,
                get_archive_schema_name(start_of_month_table),
                get_dynamic_table_name(OPENING_BALANCE_TABLE_NAME, start_of_month_table),
                start_of_month_table,
                &wl_by_user,
            )
            .await?;

            start_of_month_table = add_month(start_of_month_table);

            if start_of_month_table >= current_start_of_month {
                break;
            }
        }
    }

    Ok(())
}

fn create_credit_debt_models(
    figures: DebtsByDate,
    state: &State,
) -> Result<HashMap<Date, Vec<CreditDebt>>> {
    let mut result = HashMap::new();

    for (date, debts_by_user) in figures.into_iter() {
        let mut debts = vec![];

        for (user_id, debt) in debts_by_user.into_iter() {
            debts.push(CreditDebt {
                id: Uuid::new_v4(),
                username: state
                    .username_by_user_id
                    .get(&user_id)
                    .ok_or_else(|| anyhow!("username was not found for user_id: {}", user_id))?
                    .clone(),
                currency: debt.currency,
                user_id,
                date: get_hong_kong_11_hours_from_date(date),
                debt_amount: debt.amount,
            });
        }

        result.insert(date, debts);
    }

    Ok(result)
}

fn calculate_debt_by_bet(
    bet: &Bet,
    existing_figures: &mut HashMap<UserID, CurrencyAmount>,
    state: &mut State,
) {
    for user in state.upline.get(&bet.user_id).unwrap() {
        state
            .username_by_user_id
            .entry(user.id.clone())
            .or_insert(user.username.clone());

        let total_amount = bet
            .commission_amount
            .get(user.position as usize)
            .expect("commission_amount to have values on valid positions (indexes)")
            + bet
                .funds_delta
                .get(user.position as usize)
                .expect("funds_delta to have values on valid positions (indexes)");

        existing_figures
            .entry(user.id.clone())
            .and_modify(|e| e.amount += total_amount)
            .or_insert_with(|| CurrencyAmount {
                currency: bet.currency.clone(),
                amount: total_amount,
            });
    }
}

async fn extend_bet_with_details(
    state: &mut State,
    bet: &Bet,
    provider: GameProvider,
) -> Option<BetDetails> {
    match provider {
        GameProvider::LiveCasino(LiveCasinoProvider::Sexy) => state
            .connectors
            .ae
            .get_transaction_history_result(&bet.username, &bet.provider_bet_id)
            .await
            .ok()
            .map(|url| BetDetails {
                id: bet.id.clone(),
                details: None,
                replay: Some(url),
            }),

        GameProvider::LiveCasino(LiveCasinoProvider::Pragmatic)
        | GameProvider::Slot(SlotProvider::Pragmatic) => {
            if bet.details.is_none() {
                return state
                    .connectors
                    .pragmatic
                    .get_bet_round_history(&bet)
                    .await
                    .ok()
                    .map(|url| BetDetails {
                        id: bet.id.clone(),
                        details: Some(json!({ "result": url }).to_string()),
                        replay: None,
                    });
            };

            None
        }

        GameProvider::Slot(SlotProvider::RoyalSlotGaming) => state
            .connectors
            .royal_slot_gaming
            .get_game_round_history(&bet, None)
            .await
            .ok()
            .map(|url| BetDetails {
                id: bet.id.clone(),
                details: Some(json!({ "result": url }).to_string()),
                replay: None,
            }),

        GameProvider::Slot(SlotProvider::Ameba) => state
            .connectors
            .ameba
            .get_round_history(&bet.username, &bet.provider_bet_id)
            .await
            .ok()
            .map(|url| BetDetails {
                id: bet.id.clone(),
                details: Some(json!({ "result": url }).to_string()),
                replay: None,
            }),

        GameProvider::OnlineCasino(OnlineCasinoProvider::Arcadia) => state
            .connectors
            .arcadia
            .get_bet_history(&bet.provider_bet_id)
            .await
            .ok()
            .map(|url| BetDetails {
                id: bet.id.clone(),
                details: Some(json!({ "result": url }).to_string()),
                replay: None,
            }),

        GameProvider::OnlineCasino(OnlineCasinoProvider::Kingmaker) => state
            .connectors
            .king_maker
            .get_round_history(&bet.username, &bet.provider_bet_id)
            .await
            .ok()
            .map(|url| BetDetails {
                id: bet.id.clone(),
                details: Some(json!({ "result": url }).to_string()),
                replay: None,
            }),

        GameProvider::Slot(SlotProvider::Relax)
        | GameProvider::Slot(SlotProvider::YGG)
        | GameProvider::Slot(SlotProvider::Hacksaw) => state
            .connectors
            .dot_connections
            .get_bet_history(&bet)
            .await
            .ok()
            .map(|url| BetDetails {
                id: bet.id.clone(),
                details: Some(json!({ "result": url }).to_string()),
                replay: None,
            }),
        _ => None,
    }
}
