pub mod crypto;
pub mod logger;
pub mod provider;
pub mod query_helper;
mod time;

use ::time::Date;
use rustc_hash::FxHashMap;
use sqlx::{MySqlPool, PgPool};
pub use time::*;

use crate::{
    archiver::bets::loader::User,
    connectors::Connectors,
    types::{UserID, Username},
};

#[derive(Debug)]
pub struct State {
    pub credit_players: FxHashMap<UserID, bool>,
    pub username_by_user_id: FxHashMap<UserID, Username>,
    pub upline: FxHashMap<UserID, Vec<User>>,
    pub wl_by_date_by_user: FxHashMap<Date, FxHashMap<UserID, i64>>,
    pub connectors: Connectors,
    pub pg: PgPool,
    pub maria_db: MySqlPool,
}

impl State {
    pub fn new(connectors: Connectors, pg: PgPool, mysql: MySqlPool) -> Self {
        Self {
            connectors,
            credit_players: FxHashMap::default(),
            username_by_user_id: FxHashMap::default(),
            upline: FxHashMap::default(),
            wl_by_date_by_user: FxHashMap::default(),
            pg,
            maria_db: mysql,
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
