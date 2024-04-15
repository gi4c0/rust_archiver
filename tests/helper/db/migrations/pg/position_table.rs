use lib::enums::PositionEnum;
use sqlx::{Execute, PgPool, Postgres, QueryBuilder};
use strum::VariantArray;

pub async fn create_table_and_seed(pg: &PgPool) {
    let sql = include_str!("../../../../../migrations/20240412106111_position.sql");
    sqlx::query(sql)
        .execute(pg)
        .await
        .expect("Failed to create PG 'position' table");

    seed(pg).await;
}

async fn seed(pg: &PgPool) {
    let mut query_builder: QueryBuilder<Postgres> =
        QueryBuilder::new(r#"INSERT INTO public.position (level, position)"#);

    query_builder.push_values(PositionEnum::VARIANTS.into_iter(), |mut b, row| {
        b.push_bind(*row).push_bind(row.to_string());
    });

    query_builder.push(" ON CONFLICT DO NOTHING ");
    let mut query = query_builder.build();

    sqlx::query_with(query.sql(), query.take_arguments().unwrap())
        .execute(pg)
        .await
        .expect("Failed to seed position table");
}
