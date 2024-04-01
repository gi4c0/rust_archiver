use sqlx::MySqlPool;

pub async fn create_maria_db_user_card_table(maria_db_pool: &MySqlPool) {
    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS public.user_card
            (
                id        VARCHAR(256) NOT NULL
                    PRIMARY KEY,
                username  VARCHAR(100) NOT NULL,
                parent_id VARCHAR(256) NULL,
                position  TINYINT      NOT NULL
            )
                collate = utf8mb4_unicode_ci;
        "#,
    )
    .execute(maria_db_pool)
    .await
    .expect("Failed to create Maria DB table: user_card");
}
