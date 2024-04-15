use sqlx::PgPool;

pub async fn create_user_table(pg: &PgPool) {
    let sql = include_str!("../../../../../migrations/20240412106555_user.sql");

    sqlx::query(sql)
        .execute(pg)
        .await
        .expect("Failed to create PG 'user' table");

    sqlx::query(
        r#"create index if not exists user_username_idx on public."user" (username varchar_pattern_ops);"#,
    )
    .execute(pg)
    .await
    .expect("Failed to create PG 'user' table");

    sqlx::query(
        r#"create index if not exists user_username_idx
        on public."user" (username varchar_pattern_ops);"#,
    )
    .execute(pg)
    .await
    .expect("Failed to create PG 'user' table");
}
