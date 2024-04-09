use lib::enums::provider::ProviderGameKind;
use sqlx::{Execute, PgPool, Postgres, QueryBuilder};
use strum::VariantArray;

pub async fn create_table_and_seed(pg: &PgPool) {
    sqlx::query(
        r#"
            create table if not exists public.provider_game_kind
            (
                alias    varchar(50)  not null
                    constraint "PK_19ceef9d735203ddd9380aad3f2"
                        primary key,
                label    varchar(100) not null,
                ordering integer      not null
            );
        "#,
    )
    .execute(pg)
    .await
    .expect("Failed to create PG 'provider_game_kind' table");

    seed(pg).await;
}

async fn seed(pg: &PgPool) {
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        r#"
            INSERT INTO public.provider_game_kind (
                alias,
                label,
                ordering
            )
        "#,
    );

    let mut ordering = 1;

    query_builder.push_values(ProviderGameKind::VARIANTS.into_iter(), |mut b, row| {
        b.push_bind(row.to_string())
            .push_bind(row.to_string())
            .push_bind(ordering);

        ordering += 1;
    });

    let mut query = query_builder.build();

    sqlx::query_with(query.sql(), query.take_arguments().unwrap())
        .execute(pg)
        .await
        .expect("Failed to seed provider_game_kind table");
}
