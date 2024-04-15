use lib::{
    enums::PositionEnum,
    types::{Upline, UserID, Username},
};
use sqlx::{MySqlPool, PgPool};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::helper::user::{save_users_maria_db, save_users_pg, User};

use super::loader::save_uplines;

/// Generate upline, save it and return player
pub async fn generate_users_and_return_players(
    pg_pool: &PgPool,
    maria_db: &MySqlPool,
) -> (User, User) {
    let mut agents: Vec<User> = vec![];
    let mut upline: Upline = [None, None, None, None, None, None, None];

    let agent_position_username = [
        ("AMB", PositionEnum::Owner),
        ("AA", PositionEnum::Company),
        ("BB", PositionEnum::Shareholder),
        ("CC", PositionEnum::Agent),
    ];

    for (i, (username, position)) in agent_position_username.into_iter().enumerate() {
        let current_username;
        let mut parent_id = None;

        if position == PositionEnum::Owner || position == PositionEnum::Company {
            current_username = username.to_string();
        } else {
            current_username = format!("{}{}", agents[i - 1].username, username);
            parent_id = agents[i - 1].parent_id.clone();
        }

        let agent = User {
            id: UserID(Uuid::new_v4()),
            salt: "".to_string(),
            position,
            login: current_username.clone(),
            username: Username(current_username),
            is_sub: false,
            password: Uuid::new_v4().to_string(),
            parent_id,
            activated_at: Some(OffsetDateTime::now_utc()),
            registered_at: Some(OffsetDateTime::now_utc()),
        };

        upline[position as usize] = Some(agent.id);
        agents.push(agent);
    }

    let parent_agent = agents.last().unwrap();
    let mut players: Vec<User> = vec![];
    let mut player_uplines: Vec<Upline> = vec![];

    for i in 0..3 {
        let username = Username(format!("{}00000{i}", parent_agent.username));

        let player = User {
            id: UserID(Uuid::new_v4()),
            salt: "".to_string(),
            position: PositionEnum::Player,
            login: username.0.clone(),
            username,
            is_sub: false,
            password: Uuid::new_v4().to_string(),
            parent_id: Some(parent_agent.id),
            activated_at: Some(OffsetDateTime::now_utc()),
            registered_at: Some(OffsetDateTime::now_utc()),
        };

        let mut player_upline = upline.clone();
        player_upline[PositionEnum::Player as usize] = Some(player.id);

        player_uplines.push(player_upline);
        players.push(player);
    }

    // Imitate not activated user for tests
    players[0].activated_at = None;

    let all_users = [agents, players.clone()].concat();

    save_users_pg(pg_pool, all_users.clone()).await;
    save_uplines(pg_pool, player_uplines).await;
    save_users_maria_db(maria_db, all_users).await;

    (players.pop().unwrap(), players.pop().unwrap())
}
