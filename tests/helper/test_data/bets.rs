use std::collections::HashMap;

use lib::{
    archiver::bets::loader::Bet,
    enums::{
        bet::BetStatus,
        provider::{GameProvider, SlotProvider},
    },
    helpers::get_hong_kong_11_hours_from_date,
    types::{BetID, Currency, ProviderBetID, ProviderGameVendorID, ProviderGameVendorLabel},
};
use sqlx::PgPool;
use time::{macros::time, Date, Duration, OffsetDateTime};
use uuid::Uuid;

use crate::helper::{
    db::migrations::pg::provider::provider_game_table::{PROVIDER_GAME_LABEL, PROVIDER_VENDOR_ID},
    user::User,
};

use super::{loader::insert_bets, TEST_PROVIDERS};

pub async fn create_bets(
    pg_pool: &PgPool,
    users: &[User],
    start_date: Date,
    bets_interval_duration: Duration,
) -> HashMap<GameProvider, Vec<Bet>> {
    let mut bets_by_provider: HashMap<GameProvider, Vec<Bet>> = HashMap::new();

    for provider in TEST_PROVIDERS {
        let mut rs_provider_bet_id = 1;
        let mut current_iteration_date = OffsetDateTime::new_utc(start_date, time!(0:00));
        let yesterday11 =
            get_hong_kong_11_hours_from_date(OffsetDateTime::now_utc().date() - Duration::days(1));

        while current_iteration_date <= yesterday11 {
            for user in users {
                // We need numbers as provider bet id for royal_slot_gaming
                let provider_bet_id =
                    if provider == GameProvider::Slot(SlotProvider::RoyalSlotGaming) {
                        rs_provider_bet_id += 1;
                        ProviderBetID(rs_provider_bet_id.to_string())
                    } else {
                        ProviderBetID(Uuid::new_v4().to_string())
                    };

                bets_by_provider
                    .entry(provider)
                    .or_insert(vec![])
                    .push(Bet {
                        id: BetID(Uuid::new_v4()),
                        wl: Some(10),
                        username: user.username.clone(),
                        user_id: user.id,
                        ip: "127.0.0.1".to_string(),
                        stake: 2,
                        status: BetStatus::Closed,
                        last_status_change: current_iteration_date,
                        replay: "".to_string(),
                        details: None,
                        currency: Currency("THB".to_string()),
                        funds_delta: [1, 2, 3, 4, 5, 6, 7],
                        valid_amount: Some(2),
                        transactions: vec![r#"{ "provider": "lol" }"#.to_string()],
                        creation_date: current_iteration_date - Duration::seconds(1),
                        pt_by_position: [0, 0, 0, 0, 0, 0, 1],
                        transaction_ids: vec!["1".to_string(), "2".to_string()],
                        provider_bet_id,
                        commission_amount: [1, 2, 3, 4, 5, 6, 7],
                        commission_percent: [0, 1, 2, 3, 4, 5, 6],
                        provider_game_vendor_id: ProviderGameVendorID(
                            // Need to be the same as provider game that was seeded in migrations
                            // to match then game in connector during 'get bet history' process
                            PROVIDER_VENDOR_ID.to_string(),
                        ),
                        provider_game_vendor_label: ProviderGameVendorLabel(
                            PROVIDER_GAME_LABEL.to_string(),
                        ),
                    });
            }

            current_iteration_date += bets_interval_duration;
        }
    }

    for (provider, bets) in &bets_by_provider {
        insert_bets(pg_pool, bets.clone(), *provider).await;
    }

    bets_by_provider
}
