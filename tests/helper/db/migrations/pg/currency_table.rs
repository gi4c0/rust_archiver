use sqlx::PgPool;

pub async fn create_table_and_seed(pg: &PgPool) {
    let sql = include_str!("../../../../../migrations/20240412106533_currency.sql");
    sqlx::query(sql)
        .execute(pg)
        .await
        .expect("Failed to create PG 'currency' table");

    seed(pg).await;
}

async fn seed(pg: &PgPool) {
    sqlx::query(
        r#"
            INSERT INTO public.currency (label, rate, active, ordering, is_default)
            VALUES
                ('THB', 1000, TRUE, 1, TRUE),
                ('USD', 1000, TRUE, 1, FALSE)

            ON CONFLICT DO NOTHING
        "#,
    )
    .execute(pg)
    .await
    .expect("Failed to seed currency table");
}
