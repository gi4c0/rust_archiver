use lib::enums::provider::{GameProvider, LiveCasinoProvider, OnlineCasinoProvider, SlotProvider};
use sqlx::{Execute, PgPool, Postgres, QueryBuilder};
use strum::VariantArray;

pub async fn create_provider_table(pg: &PgPool) {
    sqlx::query(
        r#"
            create table if not exists public.provider
            (
                label      varchar(255)                                             not null,
                alias      varchar(255)                                             not null
                    constraint "PK_2a98217d647afc14f8592e4a851"
                        primary key,
                product    varchar(255)                                             not null
                    constraint "FK_b0a8ef871a52c6ba7042ee7e660"
                        references public.product,
                visible    boolean             default true not null,
                ordering   smallint            default '1'::smallint                not null,
                currencies character varying[] default '{THB}'::character varying[] not null
            )
        "#,
    )
    .execute(pg)
    .await
    .expect("Failed to create PG 'provider_config' table");
}

pub async fn seed(pg: &PgPool) {
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
            .push_bind(row.to_string())
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
