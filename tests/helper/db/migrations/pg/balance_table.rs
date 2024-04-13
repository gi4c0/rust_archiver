use sqlx::PgPool;

pub async fn create_balance_table(pg: &PgPool) {
    let sql = include_str!("../../../../../migrations/20240412202916_balance.sql");

    sqlx::query(sql)
        .execute(pg)
        .await
        .expect("Failed to create PG 'bet_status' table");
}
