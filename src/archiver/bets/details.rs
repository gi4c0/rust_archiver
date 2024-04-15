use serde_json::json;

use crate::{
    enums::provider::{GameProvider, LiveCasinoProvider, OnlineCasinoProvider, SlotProvider},
    helpers::State,
};

use super::loader::{Bet, BetDetails};

pub async fn extend_bet_with_details(
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
