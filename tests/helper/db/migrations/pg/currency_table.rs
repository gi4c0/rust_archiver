use sqlx::PgPool;

pub async fn create_table_and_seed(pg: &PgPool) {
    sqlx::query(
        r#"
            create table if not exists public.currency
            (
                label      varchar(10) not null
                    constraint "PK_8e05f18b5e44565959b408e6d39"
                        primary key,
                rate       integer     not null,
                active     boolean     not null,
                ordering   smallint    not null,
                is_default boolean     not null
            );
        "#,
    )
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
