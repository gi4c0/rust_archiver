use sqlx::PgPool;

use crate::{
    enums::provider::{GameProvider, Lottery},
    helpers::query_helper::get_bet_table_name,
};

use super::create_index;

pub async fn create_lottery_bet_table(pg: &PgPool) {
    let table_name = get_bet_table_name(GameProvider::Lottery(Lottery::StockBritish));

    sqlx::query(&format!(
        r#"
            create table if not exists public.{table_name}
            (
                id                         uuid default uuid_generate_v4() not null
                constraint "PK_id_{table_name}"
                primary key,
                provider_bet_id            varchar(255)                    not null
                constraint "UQ_provider_bet_id_{table_name}"
                unique,
                provider_game_vendor_id    varchar(255)                    not null,
                provider_game_vendor_label varchar(255)                    not null,
                transaction_ids            character varying[]             not null,
                creation_date              timestamp with time zone        not null,
                last_status_change         timestamp with time zone        not null,
                stake                      bigint                          not null,
                valid_amount               bigint,
                wl                         bigint,
                username                   varchar(100)                    not null,
                ip                         varchar(100),
                pt_by_position             bigint[]                        not null,
                commission_percent         bigint[]                        not null,
                commission_amount          bigint[]                        not null,
                funds_delta                bigint[]                        not null,
                details                    text,
                replay                     varchar(1000),
                transactions               text[]                          not null,
                kind                       varchar(100)                    not null,
                user_id                    uuid                            not null
                    constraint "FK_user_{table_name}"
                    references public."user",
                    status                     varchar(255)                    not null
                        constraint "FK_status_{table_name}"
                        references public.bet_status,
                        currency                   varchar(10)                     not null
                            constraint "FK_currency_{table_name}"
                            references public.currency
                            );
            "#
    ))
    .execute(pg)
    .await
    .expect(&format!("Failed to create {table_name} table"));

    for column in ["username", "currency", "kind", "user_id", "status"] {
        create_index(pg, column, &table_name).await;
    }
}
