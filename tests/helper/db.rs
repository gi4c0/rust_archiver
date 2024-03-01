use lib::{
    configuration::{self, AppConfig, DBConfig},
    db::create_pg_connection,
};
use sqlx::PgPool;

pub async fn create_test_connection() -> PgPool {
    let mut config = configuration::parse_config();

    config = AppConfig {
        pg: DBConfig {
            db_name: "archiver_rust_test".to_string(),
            ..config.pg
        },
        ..config
    };

    let conn = create_pg_connection(&config.pg).await;
    sqlx::migrate!("./migrations")
        .run(&conn)
        .await
        .expect("Failed to run migrations");

    truncate_tables(&conn).await;
    conn
}

async fn truncate_tables(pg: &PgPool) {
    let table_names = vec!["balance", "user"];

    for table in table_names {
        sqlx::query(&format!("TRUNCATE TABLE public.{table} CASCADE;"))
            .execute(pg)
            .await
            .expect("Failed to truncate table");
    }
}

pub async fn drop_schema(pg: &PgPool, schema_name: &str) {
    sqlx::query(&format!(r#"DROP SCHEMA IF EXISTS {schema_name} CASCADE;"#))
        .execute(pg)
        .await
        .expect("Failed to create archive schema");
}

pub async fn create_archive_schema(pg: &PgPool, schema_name: &str) {
    sqlx::query(&format!(r#"CREATE SCHEMA IF NOT EXISTS {schema_name};"#))
        .execute(pg)
        .await
        .expect("Failed to create archive schema");
}
