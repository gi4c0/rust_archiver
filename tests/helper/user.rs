use lib::enums::PositionEnum;
use sqlx::{Execute, PgPool, Postgres, QueryBuilder};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub position: PositionEnum,
    pub parent_id: Option<Uuid>,
    pub is_sub: bool,
    pub login: String,
    pub activated_at: Option<OffsetDateTime>,
    pub registered_at: Option<OffsetDateTime>,
    pub salt: String,
}

impl User {
    pub fn random(i: i32) -> User {
        User {
            id: Uuid::new_v4(),
            username: format!("AABB{i}"),
            password: Uuid::new_v4().to_string(),
            position: PositionEnum::Player,
            parent_id: None,
            is_sub: false,
            login: Uuid::new_v4().to_string(),
            activated_at: Some(OffsetDateTime::now_utc()),
            registered_at: Some(OffsetDateTime::now_utc()),
            salt: Uuid::new_v4().to_string(),
        }
    }
}

pub async fn save_users(pg_pool: &PgPool, users: Vec<User>) {
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "INSERT INTO public.user (
            id,
            username,
            password,
            position,
            parent_id,
            is_sub,
            login,
            activated_at,
            registered_at,
            salt
        )",
    );

    query_builder.push_values(users.into_iter(), |mut values, row| {
        values
            .push_bind(row.id)
            .push_bind(row.username)
            .push_bind(row.password)
            .push_bind(row.position as i16)
            .push_bind(row.parent_id)
            .push_bind(row.is_sub)
            .push_bind(row.login)
            .push_bind(row.activated_at)
            .push_bind(row.registered_at)
            .push_bind(row.salt);
    });

    let mut query = query_builder.build();

    sqlx::query_with(query.sql(), query.take_arguments().unwrap())
        .execute(pg_pool)
        .await
        .expect("Failed to save users to DB");
}

pub struct Balance {
    pub id: Uuid,
    pub user_id: Uuid,
    pub state: i64,
    pub credit: i64,
    pub credit_available: i64,
    pub cash: i64,
    pub cash_available: i64,
    pub currency: String,
}

impl Balance {
    pub fn zero_from_user(user: &User, is_credit: bool) -> Balance {
        Balance {
            id: Uuid::new_v4(),
            user_id: user.id,
            state: 0,
            credit: if is_credit { 1000 } else { 0 },
            credit_available: 0,
            cash: 0,
            currency: "THB".to_string(),
            cash_available: 0,
        }
    }
}

pub async fn save_balance(pg_pool: &PgPool, balances: Vec<Balance>) {
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "INSERT INTO public.balance (
            id,
            user_id,
            state,
            credit,
            credit_available,
            cash,
            cash_available,
            currency
        ) ",
    );

    query_builder.push_values(balances.into_iter(), |mut values, row| {
        values
            .push_bind(row.id)
            .push_bind(row.user_id)
            .push_bind(row.state)
            .push_bind(row.credit)
            .push_bind(row.credit_available)
            .push_bind(row.cash)
            .push_bind(row.cash_available)
            .push_bind(row.currency);
    });

    let mut query = query_builder.build();

    sqlx::query_with(query.sql(), query.take_arguments().unwrap())
        .execute(pg_pool)
        .await
        .expect("Failed to save opening balances to DB");
}
