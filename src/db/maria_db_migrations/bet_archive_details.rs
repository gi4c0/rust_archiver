use sqlx::MySqlPool;

pub async fn create_bet_archive_details_table(maria_db_pool: &MySqlPool) {
    sqlx::query(
        r#"
            create table if not exists public.bet_archive_details
            (
                id      varchar(256)  not null
                    primary key,
                details varchar(256)  null,
                replay  varchar(1000) null
            )
                collate = utf8mb4_unicode_ci;
        "#,
    )
    .execute(maria_db_pool)
    .await
    .expect("Failed to create Maria DB table: bet_archive_details");
}
