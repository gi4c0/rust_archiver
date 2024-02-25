pub mod query_helper;
mod time;

use std::{collections::HashMap, sync::Arc};

pub use time::*;
use tokio::sync::Mutex;

use crate::types::UserID;

pub struct State {
    credit_players: HashMap<UserID, bool>,
}

impl State {
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

pub type SharedState = Arc<Mutex<State>>;
