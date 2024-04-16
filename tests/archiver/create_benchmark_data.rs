use dotenvy::dotenv;
use time::{Duration, OffsetDateTime};

use crate::helper::{
    db::{create_maria_db_test_connection, create_pg_test_connection},
    test_data::prepare_data,
};

// This is test to generate test data for benchmark
#[ignore]
#[tokio::test]
async fn create_benchmark_data() {
    dotenv().unwrap();
    env_logger::init();

    let pg_pool = create_pg_test_connection().await;
    let maria_db_pool = create_maria_db_test_connection().await;
    let start_date = OffsetDateTime::now_utc().date() - Duration::days(2);

    prepare_data(&pg_pool, &maria_db_pool, start_date, Duration::seconds(2)).await;
}
