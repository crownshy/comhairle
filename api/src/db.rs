use sqlx_postgres::PgPool;
use tracing::info;

use crate::error::ComhairleError;
use crate::models::error::DataError;

pub async fn setup_db(connection_str: &str) -> Result<PgPool, ComhairleError> {
    let pool = PgPool::connect(connection_str)
        .await
        .map_err(|e| DataError::DbError(e.to_string()))?;

    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), ComhairleError> {
    info!("Running migrations");

    comhairle_model::SQLX_MIGRATOR
        .run(pool)
        .await
        .map_err(|e| DataError::DbError(e.to_string()))
        .expect("Failed to run migrations");

    info!("Finished running migrations");
    Ok(())
}
