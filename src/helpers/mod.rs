pub mod crypto;
pub mod logger;
pub mod query_helper;
mod time;

use std::collections::HashMap;

use ::time::Date;
pub use time::*;

use crate::{
    archiver::bets::loader::User,
    connectors::Connectors,
    types::{UserID, Username},
};

#[derive(Debug)]
pub struct State {
    pub credit_players: HashMap<UserID, bool>,
    pub username_by_user_id: HashMap<UserID, Username>,
    pub upline: HashMap<UserID, Vec<User>>,
    pub wl_by_date_by_user: HashMap<Date, HashMap<UserID, i64>>,
    pub connectors: Connectors,
}

impl State {
    pub fn new(connectors: Connectors) -> Self {
        Self {
            connectors,
            credit_players: HashMap::new(),
            username_by_user_id: HashMap::new(),
            upline: HashMap::new(),
            wl_by_date_by_user: HashMap::new(),
        }
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