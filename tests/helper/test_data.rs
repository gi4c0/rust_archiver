use std::collections::{HashMap, HashSet};

use lib::{
    archiver::{
        bets::loader::Bet,
        opening_balance::loader::{insert_opening_balance_records, OpeningBalance},
    },
    consts::SCHEMA,
    enums::{
        bet::BetStatus,
        provider::{
            GameProvider, LiveCasinoProvider, Lottery, OnlineCasinoProvider, SlotProvider,
            Sportsbook,
        },
        PositionEnum,
    },
    helpers::{
        add_month, get_hong_kong_11_hours_from_date,
        query_helper::{get_archive_schema_name, get_bet_table_name},
    },
    types::{
        BetID, Currency, ProviderBetID, ProviderGameVendorID, ProviderGameVendorLabel, Upline,
        UserID, Username,
    },
};
use sqlx::{Execute, MySqlPool, PgPool, Postgres, QueryBuilder};
use time::{macros::time, Date, Duration, OffsetDateTime};
use uuid::Uuid;
use wiremock::MockServer;

use super::{
    archive_tables::{create_credit_debt_table, create_opening_balance_table},
    db::{
        create_archive_schema, drop_schema,
        migrations::{
            maria_db,
            pg::{
                self,
                provider::provider_game_table::{PROVIDER_GAME_LABEL, PROVIDER_VENDOR_ID},
                MockUrls,
            },
        },
    },
    user::{save_balance, save_users_maria_db, save_users_pg, Balance, User},
};

pub struct TestData {
    pub credit_player: User,
    pub cash_player: User,
    pub bets_by_provider: HashMap<GameProvider, Vec<Bet>>,
    pub mock_servers: MockServers,
}

pub struct MockServers {
    pub sexy_mock_server: MockServer,
    pub ameba_mock_server: MockServer,
    pub arcadia_mock_server: MockServer,
    pub dot_connections_mock_server: MockServer,
    pub king_maker_mock_server: MockServer,
    pub pragamtic_mock_server: MockServer,
    pub royal_slot_gaming_mock_server: MockServer,
}

impl MockServers {
    async fn new() -> Self {
        Self {
            sexy_mock_server: MockServer::start().await,
            dot_connections_mock_server: MockServer::start().await,
            king_maker_mock_server: MockServer::start().await,
            pragamtic_mock_server: MockServer::start().await,
            royal_slot_gaming_mock_server: MockServer::start().await,
            ameba_mock_server: MockServer::start().await,
            arcadia_mock_server: MockServer::start().await,
        }
    }

    fn get_mock_urls(&self) -> MockUrls {
        MockUrls {
            sexy_mock_url: self.sexy_mock_server.uri(),
            arcadia_mock_url: self.arcadia_mock_server.uri(),
            ameba_mock_url: self.ameba_mock_server.uri(),
            dot_connections_mock_url: self.dot_connections_mock_server.uri(),
            king_maker_mock_url: self.king_maker_mock_server.uri(),
            pragamtic_mock_url: self.pragamtic_mock_server.uri(),
            royal_slot_gaming_mock_url: self.royal_slot_gaming_mock_server.uri(),
        }
    }
}

pub async fn prepare_data(
    pg_pool: &PgPool,
    maria_db_pool: &MySqlPool,
    start_date: Date,
) -> TestData {
    let mock_servers = MockServers::new().await;
    pg::create_pg_tables_and_seed(pg_pool, mock_servers.get_mock_urls()).await;
    maria_db::create_maria_db_tables(maria_db_pool).await;

    let (cash_player, credit_player) =
        generate_users_and_return_players(pg_pool, maria_db_pool).await;

    create_initial_balance(
        pg_pool,
        &[cash_player.clone(), credit_player.clone()],
        start_date,
    )
    .await;

    let bets_by_provider = create_bets(
        pg_pool,
        &[cash_player.clone(), credit_player.clone()],
        start_date,
    )
    .await;

    TestData {
        credit_player,
        cash_player,
        bets_by_provider,
        mock_servers,
    }
}

/// Generate upline, save it and return player
async fn generate_users_and_return_players(pg_pool: &PgPool, maria_db: &MySqlPool) -> (User, User) {
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

async fn create_bets(
    pg_pool: &PgPool,
    users: &[User],
    start_date: Date,
) -> HashMap<GameProvider, Vec<Bet>> {
    let providers = vec![
        GameProvider::LiveCasino(LiveCasinoProvider::Sexy),
        GameProvider::Slot(SlotProvider::Ameba),
        GameProvider::OnlineCasino(OnlineCasinoProvider::Arcadia),
        GameProvider::Slot(SlotProvider::Relax), // dot connections
        GameProvider::OnlineCasino(OnlineCasinoProvider::Kingmaker),
        GameProvider::Slot(SlotProvider::Pragmatic),
        GameProvider::Slot(SlotProvider::RoyalSlotGaming),
        GameProvider::Lottery(Lottery::StockDowJones),
        GameProvider::Sport(Sportsbook::SingleNonLive),
    ];

    let mut bets_by_provider: HashMap<GameProvider, Vec<Bet>> = HashMap::new();

    for provider in providers {
        let mut rs_provider_bet_id = 1;
        let mut current_iteration_date = OffsetDateTime::new_utc(start_date, time!(0:00));
        let now = OffsetDateTime::now_utc();

        while current_iteration_date <= now {
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
                        funds_delta: [0, 0, 0, 0, 0, 0, 1],
                        valid_amount: Some(2),
                        transactions: vec![r#"{ "provider": "lol" }"#.to_string()],
                        creation_date: current_iteration_date - Duration::seconds(1),
                        pt_by_position: [0, 0, 0, 0, 0, 0, 1],
                        transaction_ids: vec!["1".to_string(), "2".to_string()],
                        provider_bet_id,
                        commission_amount: [0, 0, 0, 0, 0, 0, 1],
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

            current_iteration_date += Duration::minutes(30);
        }
    }

    for (provider, bets) in &bets_by_provider {
        insert_bets(pg_pool, bets.clone(), *provider).await;
    }

    bets_by_provider
}

async fn create_initial_balance(pg_pool: &PgPool, users: &[User], start_date: Date) {
    let balances: Vec<Balance> = users
        .iter()
        .enumerate()
        .map(|(i, user)| Balance::zero_from_user(user, i % 2 == 0))
        .collect();

    save_balance(pg_pool, balances).await;

    create_archive_tables_for_test(pg_pool, start_date).await;

    let initial_opening_balance: Vec<OpeningBalance> = users[1..] // skip not activated user
        .iter()
        .map(|u| OpeningBalance {
            id: Uuid::new_v4(),
            amount: 1000,
            creation_date: get_hong_kong_11_hours_from_date(start_date),
            user_id: u.id,
        })
        .collect();

    insert_opening_balance_records(pg_pool, initial_opening_balance, start_date)
        .await
        .unwrap()
}

/// Creates opening balance tables from initial date to now
async fn create_archive_tables_for_test(pg_pool: &PgPool, initial_date: Date) {
    let now = OffsetDateTime::now_utc().date().replace_day(1).unwrap();
    let mut current_date = initial_date.replace_day(1).unwrap();

    let mut archives = HashSet::new();
    let mut tables = HashSet::new();

    loop {
        archives.insert(get_archive_schema_name(current_date));
        tables.insert(current_date);

        if current_date >= now {
            break;
        }

        current_date = add_month(current_date);
    }

    for schema in archives {
        drop_schema(pg_pool, &schema).await;
        create_archive_schema(pg_pool, &schema).await;
    }

    for table_date in tables {
        create_opening_balance_table(pg_pool, table_date).await;
        create_credit_debt_table(pg_pool, table_date).await;
    }
}

async fn insert_bets(pg_pool: &PgPool, mut bets: Vec<Bet>, provider: GameProvider) {
    let schema = &*SCHEMA;
    let table = get_bet_table_name(provider);

    let mut chunk = 100;

    if chunk > bets.len() {
        chunk = bets.len();
    }

    loop {
        let mut lottery_kind = "";

        if let GameProvider::Lottery(_) = provider {
            lottery_kind = ", kind";
        }

        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(format!(
            r#"
                INSERT INTO {schema}.{table} (
                    id,
                    creation_date,
                    last_status_change,
                    stake,
                    valid_amount,
                    wl,
                    user_id,
                    username,
                    ip,
                    status,
                    currency,
                    pt_by_position,
                    commission_percent,
                    commission_amount,
                    funds_delta,
                    details,
                    replay,
                    transaction_ids,
                    transactions,
                    provider_bet_id,
                    provider_game_vendor_id,
                    provider_game_vendor_label
                    {lottery_kind}
                )
            "#
        ));

        query_builder.push_values(bets.drain(0..chunk), |mut b, bet| {
            b.push_bind(bet.id)
                .push_bind(bet.creation_date)
                .push_bind(bet.last_status_change)
                .push_bind(bet.stake)
                .push_bind(bet.valid_amount)
                .push_bind(bet.wl)
                .push_bind(bet.user_id)
                .push_bind(bet.username)
                .push_bind(bet.ip)
                .push_bind(bet.status.to_string())
                .push_bind(bet.currency)
                .push_bind(bet.pt_by_position)
                .push_bind(bet.commission_percent)
                .push_bind(bet.commission_amount)
                .push_bind(bet.funds_delta)
                .push_bind(bet.details)
                .push_bind(bet.replay)
                .push_bind(bet.transaction_ids)
                .push_bind(bet.transactions)
                .push_bind(bet.provider_bet_id)
                .push_bind(bet.provider_game_vendor_id)
                .push_bind(bet.provider_game_vendor_label);

            if let GameProvider::Lottery(p) = provider {
                b.push_bind(p.to_string());
            }
        });

        let mut query = query_builder.build();

        sqlx::query_with(
            query.sql(),
            query
                .take_arguments()
                .expect("Failed to take arguments for insert bets"),
        )
        .execute(pg_pool)
        .await
        .expect("Failed to insert bets");

        if bets.len() == 0 {
            break;
        }

        if chunk > bets.len() {
            chunk = bets.len();
        }
    }
}

async fn save_uplines(pg: &PgPool, player_upline: Vec<Upline>) {
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        r#"
            INSERT INTO public.user_upline (
                user_id,
                upline_ids
            )
        "#,
    );

    query_builder.push_values(player_upline.into_iter(), |mut b, row| {
        b.push_bind(row[PositionEnum::Player as usize])
            .push_bind(row);
    });

    let mut query = query_builder.build();

    sqlx::query_with(query.sql(), query.take_arguments().unwrap())
        .execute(pg)
        .await
        .expect("Failed to insert players' upline");
}
