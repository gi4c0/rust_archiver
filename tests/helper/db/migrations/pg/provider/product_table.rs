use lib::enums::provider::Product;
use sqlx::{Execute, PgPool, Postgres, QueryBuilder};
use strum::VariantArray;

pub async fn create_table_and_seed(pg: &PgPool) {
    sqlx::query(
        r#"
            create table if not exists public.product
            (
                alias    varchar(255) not null
                    constraint "PK_afa492329cc228eb3fdd51d86fd"
                        primary key,
                label    varchar(100) not null,
                visible  boolean      not null,
                ordering smallint     not null
            );
        "#,
    )
    .execute(pg)
    .await
    .expect("Failed to create PG 'balance' table");

    seed(pg).await;
}

async fn seed(pg: &PgPool) {
    let mut query_builder: QueryBuilder<Postgres> =
        QueryBuilder::new(r#"INSERT INTO public.product (alias, label, visible, ordering)"#);

    let mut ordering = 1;

    query_builder.push_values(Product::VARIANTS.into_iter(), |mut b, row| {
        b.push_bind(row.to_string())
            .push_bind(row.to_string())
            .push_bind(true)
            .push_bind(1);

        ordering += 1;
    });

    let mut query = query_builder.build();

    sqlx::query_with(query.sql(), query.take_arguments().unwrap())
        .execute(pg)
        .await
        .expect("Failed to seed product table");
}
