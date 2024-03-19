use anyhow::{Context, Result};
use sqlx::PgPool;
use time::OffsetDateTime;

pub async fn log_error(pg: &PgPool, err: anyhow::Error) -> Result<()> {
    sqlx::query!(
        r#"
            INSERT INTO public.system_log (
                description,
                date,
                kind
            ) VALUES ($1, $2, $3)
        "#,
        err.to_string(),
        OffsetDateTime::now_utc(),
        ErrorKind::Error as i32,
    )
    .execute(pg)
    .await
    .with_context(|| format!("Failed to log error: '{err}'. Error: {}", err))?;

    Ok(())
}

#[repr(i32)]
pub enum ErrorKind {
    Warning = 100,
    Error = 200,
    Critical = 300,
}
