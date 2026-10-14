use sqlx_postgres::PgPool;
use tracing::info;

use crate::error::ComhairleError;

pub trait DbSource: Send + Sync + std::fmt::Debug {
    type Conn: std::ops::DerefMut<Target = sqlx::PgConnection> + Send;
    type Tx: DbTransaction;

    fn acquire(self) -> impl Future<Output = Result<Self::Conn, sqlx::Error>> + Send;

    fn begin(self) -> impl Future<Output = Result<Self::Tx, sqlx::Error>> + Send;
}

/// A transaction started through [`DbSource::begin`].
pub trait DbTransaction:
    std::ops::DerefMut<Target = sqlx::PgConnection> + Send + Sync + std::fmt::Debug
{
    fn commit(self) -> impl Future<Output = Result<(), sqlx::Error>> + Send;

    fn rollback(self) -> impl Future<Output = Result<(), sqlx::Error>> + Send;
}

impl<'t> DbTransaction for sqlx::Transaction<'t, sqlx::Postgres> {
    async fn commit(self) -> Result<(), sqlx::Error> {
        sqlx::Transaction::commit(self).await
    }

    async fn rollback(self) -> Result<(), sqlx::Error> {
        sqlx::Transaction::rollback(self).await
    }
}

impl DbSource for &sqlx::PgPool {
    type Conn = sqlx::pool::PoolConnection<sqlx::Postgres>;
    type Tx = sqlx::Transaction<'static, sqlx::Postgres>;

    async fn acquire(self) -> Result<Self::Conn, sqlx::Error> {
        sqlx::PgPool::acquire(self).await
    }

    async fn begin(self) -> Result<Self::Tx, sqlx::Error> {
        sqlx::PgPool::begin(self).await
    }
}

impl<'b> DbSource for &'b mut sqlx::PgConnection {
    type Conn = &'b mut sqlx::PgConnection;
    type Tx = sqlx::Transaction<'b, sqlx::Postgres>;

    async fn acquire(self) -> Result<Self::Conn, sqlx::Error> {
        Ok(self)
    }

    async fn begin(self) -> Result<Self::Tx, sqlx::Error> {
        sqlx::Connection::begin(self).await
    }
}

impl<'b, T: DbTransaction> DbSource for &'b mut T {
    type Conn = &'b mut sqlx::PgConnection;
    type Tx = sqlx::Transaction<'b, sqlx::Postgres>;

    async fn acquire(self) -> Result<Self::Conn, sqlx::Error> {
        Ok(&mut **self)
    }

    async fn begin(self) -> Result<Self::Tx, sqlx::Error> {
        sqlx::Connection::begin(&mut **self).await
    }
}

impl<'b> DbSource for &'b mut sqlx::pool::PoolConnection<sqlx::Postgres> {
    type Conn = &'b mut sqlx::PgConnection;
    type Tx = sqlx::Transaction<'b, sqlx::Postgres>;

    async fn acquire(self) -> Result<Self::Conn, sqlx::Error> {
        Ok(&mut **self)
    }

    async fn begin(self) -> Result<Self::Tx, sqlx::Error> {
        sqlx::Connection::begin(&mut **self).await
    }
}

pub async fn setup_db(connection_str: &str) -> Result<PgPool, ComhairleError> {
    let pool = PgPool::connect(connection_str)
        .await
        .map_err(|e| ComhairleError::DbError(e.to_string()))?;

    Ok(pool)
}

pub async fn run_migrations(pool: impl crate::DbSource) -> Result<(), ComhairleError> {
    let mut conn = pool.acquire().await?;
    info!("Running migrations");

    sqlx::migrate!("./migrations")
        .run(&mut *conn)
        .await
        .expect("Failed to run migrations");

    info!("Finished running migrations");
    Ok(())
}
