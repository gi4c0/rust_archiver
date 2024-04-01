use sqlx::MySqlPool;

pub async fn create_bet_table(maria_db_pool: &MySqlPool) {
    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS public.bet
            (
                id                         VARCHAR(256)  NOT NULL,
                provider_bet_id            VARCHAR(255)  NOT NULL,
                transaction_ids            VARCHAR(256)  NOT NULL,
                provider_game_vendor_id    VARCHAR(255)  NULL,
                provider_game_vendor_label VARCHAR(255)  NULL,
                creation_date              TIMESTAMP     NOT NULL,
                last_status_change         TIMESTAMP     NOT NULL,
                stake                      BIGINT        NOT NULL,
                valid_amount               BIGINT        NULL,
                wl                         BIGINT        NULL,
                user_id                    VARCHAR(256)  NOT NULL,
                username                   VARCHAR(100)  NOT NULL,
                ip                         VARCHAR(100)  NULL,
                status                     VARCHAR(255)  NOT NULL,
                currency                   VARCHAR(10)   NOT NULL,
                transactions               VARCHAR(256)  NOT NULL,
                pt_by_position_0           BIGINT        NOT NULL,
                pt_by_position_1           BIGINT        NOT NULL,
                pt_by_position_2           BIGINT        NOT NULL,
                pt_by_position_3           BIGINT        NOT NULL,
                pt_by_position_4           BIGINT        NOT NULL,
                pt_by_position_5           BIGINT        NOT NULL,
                pt_by_position_6           BIGINT        NOT NULL,
                commission_percent_0       BIGINT        NOT NULL,
                commission_percent_1       BIGINT        NOT NULL,
                commission_percent_2       BIGINT        NOT NULL,
                commission_percent_3       BIGINT        NOT NULL,
                commission_percent_4       BIGINT        NOT NULL,
                commission_percent_5       BIGINT        NOT NULL,
                commission_percent_6       BIGINT        NOT NULL,
                commission_amount_0        BIGINT        NOT NULL,
                commission_amount_1        BIGINT        NOT NULL,
                commission_amount_2        BIGINT        NOT NULL,
                commission_amount_3        BIGINT        NOT NULL,
                commission_amount_4        BIGINT        NOT NULL,
                commission_amount_5        BIGINT        NOT NULL,
                commission_amount_6        BIGINT        NOT NULL,
                funds_delta_0              BIGINT        NOT NULL,
                funds_delta_1              BIGINT        NOT NULL,
                funds_delta_2              BIGINT        NOT NULL,
                funds_delta_3              BIGINT        NOT NULL,
                funds_delta_4              BIGINT        NOT NULL,
                funds_delta_5              BIGINT        NOT NULL,
                funds_delta_6              BIGINT        NOT NULL,
                details                    VARCHAR(256)  NULL,
                replay                     VARCHAR(1000) NULL,
                provider                   VARCHAR(100)  NULL
            );
        "#,
    )
    .execute(maria_db_pool)
    .await
    .expect("Failed to create Maria DB table: bet");
}
