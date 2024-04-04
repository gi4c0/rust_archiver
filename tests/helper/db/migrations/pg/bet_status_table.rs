use sqlx::{Execute, PgPool, Postgres, QueryBuilder};

use lib::enums::bet::BetStatus;
use strum::VariantNames;

pub async fn create_bet_status_table(pg: &PgPool) {
    sqlx::query(
        r#"
            create table if not exists public.bet_status
            (
                value varchar(255) not null
                    constraint "PK_b37b319510371f89a193d023b21"
                        primary key,
                label varchar(255) not null
                    constraint "UQ_50e78d9f2cdfc29bd5cd564e048"
                        unique
            );
        "#,
    )
    .execute(pg)
    .await
    .expect("Failed to create PG 'balance' table");
}

pub async fn seed(pg: &PgPool) {
    let mut query_builder: QueryBuilder<Postgres> =
        QueryBuilder::new("INSERT INTO public.bet_status (value, label)");

    query_builder.push_values(BetStatus::VARIANTS.iter(), |mut b, &item| {
        b.push_bind(item)
            .push_bind(capitalize_first_latter(&item.to_lowercase()));
    });

    query_builder.push(r#" ON CONFLICT DO NOTHING "#);

    let mut query = query_builder.build();

    sqlx::query_with(query.sql(), query.take_arguments().unwrap())
        .execute(pg)
        .await
        .expect("Failed to seed default values for bet_status");
}

fn capitalize_first_latter(data: &str) -> String {
    let mut c = data.chars();

    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
