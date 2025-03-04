use sqlx_postgres::PgPool;
use tracing::info;

use crate::error::ComhairleError;

pub async fn setup_db(connection_str: &str) -> Result<PgPool, ComhairleError> {
    let pool = PgPool::connect(connection_str)
        .await
        .map_err(|e| ComhairleError::DbError(e.to_string()))?;

    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), ComhairleError> {
    info!("Running migrations");

    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| ComhairleError::DbError(e.to_string()))
        .expect("Failed to run migrations");

    info!("Finished running migrations");
    Ok(())
}
