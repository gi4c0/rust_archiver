use sqlx::MySqlPool;

pub mod bet_archive_details;
pub mod bet_table;
pub mod user_card_table;

pub async fn create_maria_db_tables(maria_db_pool: &MySqlPool) {
    tokio::join!(
        bet_archive_details::create_bet_archive_details_table(maria_db_pool),
        bet_table::create_bet_table(maria_db_pool),
        user_card_table::create_maria_db_user_card_table(maria_db_pool),
    );
}
