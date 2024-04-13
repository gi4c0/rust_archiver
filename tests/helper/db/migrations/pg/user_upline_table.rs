use sqlx::PgPool;

pub async fn create_table(pg: &PgPool) {
    let sql = include_str!("../../../../../migrations/20240412202806_user_upline.sql");

    sqlx::query(sql)
        .execute(pg)
        .await
        .expect("Failed to create 'user_upline' table");
}
