pub mod query_helper;
mod time;

use std::collections::HashMap;

pub use time::*;

use crate::types::UserID;

#[derive(Default, Debug)]
pub struct State {
    credit_players: HashMap<UserID, bool>,
}

impl State {
    pub fn new() -> Self {
        State::default()
    }

    pub fn add_credit_player(&mut self, player_id: UserID) {
        self.credit_players.insert(player_id, true);
    }

    pub fn is_credit_player(&self, player_id: UserID) -> bool {
        match self.credit_players.get(&player_id) {
            Some(user) => *user,
            None => false,
        }
    }
}
