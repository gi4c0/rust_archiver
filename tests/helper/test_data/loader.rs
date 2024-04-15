use lib::{
    archiver::bets::loader::Bet,
    consts::SCHEMA,
    enums::{provider::GameProvider, PositionEnum},
    helpers::query_helper::get_bet_table_name,
    types::Upline,
};
use sqlx::{Execute, PgPool, Postgres, QueryBuilder};

pub async fn insert_bets(pg_pool: &PgPool, mut bets: Vec<Bet>, provider: GameProvider) {
    let schema = &*SCHEMA;
    let table = get_bet_table_name(provider);

    let mut chunk = 100;

    if chunk > bets.len() {
        chunk = bets.len();
    }

    loop {
        let mut lottery_kind = "";

        if let GameProvider::Lottery(_) = provider {
            lottery_kind = ", kind";
        }

        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(format!(
            r#"
                INSERT INTO {schema}.{table} (
                    id,
                    creation_date,
                    last_status_change,
                    stake,
                    valid_amount,
                    wl,
                    user_id,
                    username,
                    ip,
                    status,
                    currency,
                    pt_by_position,
                    commission_percent,
                    commission_amount,
                    funds_delta,
                    details,
                    replay,
                    transaction_ids,
                    transactions,
                    provider_bet_id,
                    provider_game_vendor_id,
                    provider_game_vendor_label
                    {lottery_kind}
                )
            "#
        ));

        query_builder.push_values(bets.drain(0..chunk), |mut b, bet| {
            b.push_bind(bet.id)
                .push_bind(bet.creation_date)
                .push_bind(bet.last_status_change)
                .push_bind(bet.stake)
                .push_bind(bet.valid_amount)
                .push_bind(bet.wl)
                .push_bind(bet.user_id)
                .push_bind(bet.username)
                .push_bind(bet.ip)
                .push_bind(bet.status.to_string())
                .push_bind(bet.currency)
                .push_bind(bet.pt_by_position)
                .push_bind(bet.commission_percent)
                .push_bind(bet.commission_amount)
                .push_bind(bet.funds_delta)
                .push_bind(bet.details)
                .push_bind(bet.replay)
                .push_bind(bet.transaction_ids)
                .push_bind(bet.transactions)
                .push_bind(bet.provider_bet_id)
                .push_bind(bet.provider_game_vendor_id)
                .push_bind(bet.provider_game_vendor_label);

            if let GameProvider::Lottery(p) = provider {
                b.push_bind(p.to_string());
            }
        });

        let mut query = query_builder.build();

        sqlx::query_with(
            query.sql(),
            query
                .take_arguments()
                .expect("Failed to take arguments for insert bets"),
        )
        .execute(pg_pool)
        .await
        .expect("Failed to insert bets");

        if bets.len() == 0 {
            break;
        }

        if chunk > bets.len() {
            chunk = bets.len();
        }
    }
}

pub async fn save_uplines(pg: &PgPool, player_upline: Vec<Upline>) {
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        r#"
            INSERT INTO public.user_upline (
                user_id,
                upline_ids
            )
        "#,
    );

    query_builder.push_values(player_upline.into_iter(), |mut b, row| {
        b.push_bind(row[PositionEnum::Player as usize])
            .push_bind(row);
    });

    let mut query = query_builder.build();

    sqlx::query_with(query.sql(), query.take_arguments().unwrap())
        .execute(pg)
        .await
        .expect("Failed to insert players' upline");
}
