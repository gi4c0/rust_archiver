use std::collections::HashMap;

use anyhow::{anyhow, Context, Result};
use time::Date;
use uuid::Uuid;

use crate::{
    helpers::{get_hong_kong_11_hours_from_date, State},
    types::UserID,
};

use super::{
    loader::{Bet, CreditDebt},
    CurrencyAmount,
};

use super::DebtsByDate;

pub fn create_credit_debt_models(
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

pub fn calculate_debt_by_bet(
    bet: &Bet,
    existing_figures: &mut HashMap<UserID, CurrencyAmount>,
    state: &mut State,
) -> Result<()> {
    let bet_user_upline = state
        .upline
        .get(&bet.user_id)
        .with_context(|| format!("Not found upline for user: {}", &bet.user_id))?;

    for user in bet_user_upline {
        state
            .username_by_user_id
            .entry(user.id)
            .or_insert(user.username.clone());

        let total_amount = bet
            .commission_amount
            .get(user.position as usize)
            .with_context(|| {
                format!(
                    "commission_amount don't have values on valid positions (indexes) for bet {}",
                    &bet.id
                )
            })?
            + bet
                .funds_delta
                .get(user.position as usize)
                .with_context(|| {
                    format!(
                        "funds_delta don't have values on valid positions (indexes) in bet {}",
                        &bet.id
                    )
                })?;

        existing_figures
            .entry(user.id)
            .and_modify(|e| e.amount += total_amount)
            .or_insert_with(|| CurrencyAmount {
                currency: bet.currency.clone(),
                amount: total_amount,
            });
    }

    Ok(())
}
