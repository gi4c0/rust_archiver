use lib::enums::provider::{GameProvider, LiveCasinoProvider, OnlineCasinoProvider, SlotProvider};
use sqlx::{Execute, PgPool, Postgres, QueryBuilder};
use strum::VariantArray;

pub async fn create_table_and_seed(pg: &PgPool) {
    let sql = include_str!("../../../../../../migrations/20240412165556_provider.sql");

    sqlx::query(sql)
        .execute(pg)
        .await
        .expect("Failed to create PG 'provider_config' table");

    seed(pg).await;
}

async fn seed(pg: &PgPool) {
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        r#"
            INSERT INTO public.provider (
                label,
                alias,
                product,
                visible,
                ordering,
                currencies
            )
        "#,
    );

    let providers: Vec<GameProvider> = [
        LiveCasinoProvider::VARIANTS
            .into_iter()
            .map(|p| p.into_game_provider())
            .collect::<Vec<GameProvider>>(),
        OnlineCasinoProvider::VARIANTS
            .into_iter()
            .map(|p| p.into_game_provider())
            .collect(),
        SlotProvider::VARIANTS
            .into_iter()
            .map(|p| p.into_game_provider())
            .collect(),
    ]
    .concat();

    query_builder.push_values(providers.into_iter(), |mut b, row| {
        b.push_bind(row.to_string())
            .push_bind(row.to_string())
            .push_bind(row.get_product().to_string())
            .push_bind(true)
            .push_bind(1)
            .push_bind(["THB".to_string()]);
    });

    let mut query = query_builder.build();

    sqlx::query_with(query.sql(), query.take_arguments().unwrap())
        .execute(pg)
        .await
        .expect("Failed to seed providers");
}
