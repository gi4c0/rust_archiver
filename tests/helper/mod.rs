use lib::utils::query_helper::get_double_digit_month;
use sqlx::PgPool;
use time::Date;

pub mod db;
pub mod test_data;
pub mod user;

pub async fn create_opening_balance_table(pg_pool: &PgPool, date: impl Into<Date>) {
    let date: Date = date.into();
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
