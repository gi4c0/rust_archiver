use std::collections::HashMap;

use serde_json::{json, to_string};
use sqlx::Transaction;
use time::{macros::time, Date, Duration, OffsetDateTime};

use crate::{
    enums::provider::{GameProvider, LiveCasinoProvider, OnlineCasinoProvider, SlotProvider},
    helpers::State,
    types::{BetID, Currency, UserID},
};

use self::loader::{get_upline, Bet, BetDetails};

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
    transaction: &mut Transaction<'_, sqlx::Postgres>,
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
            let upline = get_upline(&bet.user_id, transaction).await?;
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

        if let Some(detail) = extend_bet_with_details(state, &bet, &provider).await {
            bet_details.push(detail);
        }
    }

    Ok(())
}

async fn save_all(
    state: &mut State,
    provider_or_bet_type: GameProvider,
    depts: DebtsByDate,
    bet_ids: Vec<BetID>,
    wl_by_date_by_user: WlByDateByUser,
) {
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

fn get_figures_date(bet_date: OffsetDateTime) -> Date {
    let threshold = bet_date.replace_time(time!(3:00));

    if bet_date > threshold {
        return (threshold + Duration::days(1)).date();
    }

    threshold.date()
}

async fn extend_bet_with_details(
    state: &mut State,
    bet: &Bet,
    provider: &GameProvider,
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
