use std::collections::HashSet;

use lib::{
    archiver::opening_balance::loader::{insert_opening_balance_records, OpeningBalance},
    helpers::{add_month, get_hong_kong_11_hours_from_date, query_helper::get_archive_schema_name},
};
use sqlx::PgPool;
use time::{Date, Duration, OffsetDateTime};
use uuid::Uuid;

use super::{
    create_opening_balance_table,
    db::{create_archive_schema, drop_schema},
    user::{save_balance, save_users, Balance, User},
};

pub async fn prepare_data(pg_pool: &PgPool) {
    let mut users = vec![];

    for i in 0..3 {
        users.push(User::random(i));
    }

    // Imitate not activated user for tests
    users[0].activated_at = None;
    save_users(pg_pool, users.clone()).await;

    let balances: Vec<Balance> = users
        .iter()
        .enumerate()
        .map(|(i, user)| Balance::zero_from_user(user, i % 2 == 0))
        .collect();

    save_balance(pg_pool, balances).await;

    let last_date = OffsetDateTime::now_utc().date() - Duration::days(31 * 5);
    create_archive_tables_for_test(pg_pool, last_date).await;

    let initial_opening_balance: Vec<OpeningBalance> = users[1..] // skip not activated user
        .iter()
        .map(|u| OpeningBalance {
            id: Uuid::new_v4(),
            amount: 1000,
            creation_date: get_hong_kong_11_hours_from_date(last_date),
            user_id: u.id,
        })
        .collect();

    insert_opening_balance_records(pg_pool, initial_opening_balance, last_date)
        .await
        .unwrap()
}

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
    }
}
