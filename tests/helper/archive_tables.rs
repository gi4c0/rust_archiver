use lib::helpers::query_helper::get_double_digit_month;
use sqlx::PgPool;
use time::Date;

pub async fn create_opening_balance_table(pg_pool: &PgPool, date: Date) {
    let year = date.year();
    let month = get_double_digit_month(date);

    sqlx::query(&format!(
        r#"
            create table if not exists archive_{year}.opening_balance_{year}_{month}
            (
                id            uuid                     not null
                    primary key,
                creation_date timestamp with time zone not null,
                amount        bigint                   not null,
                user_id       uuid                     not null,
                unique (user_id, creation_date)
            );
        "#,
    ))
    .execute(pg_pool)
    .await
    .unwrap();
}

pub async fn create_credit_debt_table(pg_pool: &PgPool, date: Date) {
    let year = date.year();
    let month = get_double_digit_month(date);

    sqlx::query(&format!(
        r#"
            create table if not exists archive_{year}.credit_debt_{year}_{month}
            (
                id          uuid                     not null
                    primary key,
                date        timestamp with time zone not null,
                debt_amount bigint                   not null,
                currency    varchar(3)               not null,
                username    varchar(100)             not null,
                user_id     uuid                     not null,
                unique (user_id, date)
            );
        "#,
    ))
    .execute(pg_pool)
    .await
    .unwrap();
}
