use std::collections::{HashMap, HashSet};

use lib::{
    archiver::{
        bets::loader::Bet,
        opening_balance::loader::{insert_opening_balance_records, OpeningBalance},
    },
    enums::provider::{
        GameProvider, LiveCasinoProvider, Lottery, OnlineCasinoProvider, SlotProvider, Sportsbook,
    },
    helpers::{add_month, get_hong_kong_11_hours_from_date, query_helper::get_archive_schema_name},
};
use sqlx::{MySqlPool, PgPool};
use time::{Date, Duration, OffsetDateTime};
use uuid::Uuid;
use wiremock::MockServer;

use self::{bets::create_bets, players::generate_users_and_return_players};

use super::{
    archive_tables::{create_credit_debt_table, create_opening_balance_table},
    db::{
        create_archive_schema, drop_schema,
        migrations::{
            maria_db,
            pg::{self, MockUrls},
        },
    },
    user::{save_balance, Balance, User},
};

mod bets;
mod loader;
mod players;

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

pub fn get_yesterday_11() -> OffsetDateTime {
    get_hong_kong_11_hours_from_date(OffsetDateTime::now_utc().date() - Duration::days(1))
}

pub async fn prepare_data(
    pg_pool: &PgPool,
    maria_db_pool: &MySqlPool,
    start_date: Date,
    bets_interval_duration: Duration,
) -> TestData {
    let mock_servers = MockServers::new().await;
    // pg::create_pg_tables_and_seed(pg_pool, mock_servers.get_mock_urls()).await;
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
        bets_interval_duration,
    )
    .await;

    TestData {
        credit_player,
        cash_player,
        bets_by_provider,
        mock_servers,
    }
}

pub const TEST_PROVIDERS: [GameProvider; 9] = [
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
            amount: 0,
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
