use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{Expr, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow, query_as_with};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::{
        SqlxResultExt,
        pagination::{Order, PageOptions, PaginatedResults},
    },
};

#[derive(Serialize, Deserialize, FromRow, Clone, JsonSchema, Debug)]
#[enum_def(table_name = "job")]
pub struct Job {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub error: Option<String>,
    pub completion_message: Option<String>,
    pub step: Option<String>,
    pub progress: Option<f64>,
    pub status: Option<String>,
}

const DEFAULT_COLUMNS: [JobIden; 8] = [
    JobIden::Id,
    JobIden::CreatedAt,
    JobIden::FinishedAt,
    JobIden::Error,
    JobIden::CompletionMessage,
    JobIden::Progress,
    JobIden::Step,
    JobIden::Status,
];

#[derive(Serialize, Deserialize, JsonSchema, Debug, Default)]
pub struct CreateJob {
    pub step: Option<String>,
    pub progress: Option<f64>,
}

impl CreateJob {
    /// Returns the database columns that will be inserted for this content.
    ///
    /// # Returns
    ///
    /// A vector of JobIden enum values representing the database columns.
    pub fn columns(&self) -> Vec<JobIden> {
        let mut columns = vec![JobIden::Status];
        if self.step.is_some() {
            columns.push(JobIden::Step);
        }
        if self.progress.is_some() {
            columns.push(JobIden::Progress);
        }
        columns
    }

    /// Returns that values to be inserted into the database columns.
    ///
    /// # Returns
    ///
    /// A vector of sea_query::SimpleExpr values correspoinding to the columns.
    pub fn values(&self) -> Vec<sea_query::SimpleExpr> {
        let mut values = vec!["pending".to_string().into()];
        if let Some(value) = &self.step {
            values.push(value.into());
        }
        if let Some(value) = &self.progress {
            values.push((*value).into());
        }
        values
    }
}

#[instrument(err(Debug), skip(db))]
pub async fn create(db: &PgPool, create_job: CreateJob) -> Result<Job, ComhairleError> {
    let columns = create_job.columns();
    let values = create_job.values();

    let (sql, values) = Query::insert()
        .into_table(JobIden::Table)
        .columns(columns)
        .values(values)?
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let job = sqlx::query_as_with::<_, Job, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(job)
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Default)]
pub struct UpdateJob {
    pub finished_at: Option<DateTime<Utc>>,
    pub error: Option<String>,
    pub completion_message: Option<String>,
    pub step: Option<String>,
    pub progress: Option<f64>,
    pub status: Option<String>,
}

impl UpdateJob {
    /// Converts the update struct to database columns-value pairs.
    ///
    /// Only fields that are Some(..) will be included in the update.
    ///
    /// # Returns
    /// A vector of tuples containing the column identifier and the new value.
    pub fn to_values(&self) -> Vec<(JobIden, sea_query::SimpleExpr)> {
        let mut values = vec![];
        if let Some(value) = &self.finished_at {
            values.push((JobIden::FinishedAt, (*value).into()));
        }
        if let Some(value) = &self.error {
            values.push((JobIden::Error, value.into()));
        }
        if let Some(value) = &self.completion_message {
            values.push((JobIden::CompletionMessage, value.into()));
        }
        if let Some(value) = &self.step {
            values.push((JobIden::Step, value.into()));
        }
        if let Some(value) = &self.progress {
            values.push((JobIden::Progress, (*value).into()));
        }
        if let Some(value) = &self.status {
            values.push((JobIden::Status, value.into()));
        }
        values
    }
}

#[instrument(err(Debug), skip(db))]
pub async fn update(db: &PgPool, id: &Uuid, update_job: UpdateJob) -> Result<Job, ComhairleError> {
    let values = update_job.to_values();

    if values.is_empty() {
        return Err(ComhairleError::NoValidUpdates);
    }

    let (sql, values) = Query::update()
        .table(JobIden::Table)
        .values(values)
        .and_where(Expr::col(JobIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let job = sqlx::query_as_with::<_, Job, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(job)
}

/// Convenience method to mark job as completed with custom completion_message
#[instrument(err(Debug), skip(db))]
pub async fn complete(db: &PgPool, id: Uuid, message: &str) -> Result<Job, ComhairleError> {
    let (sql, values) = Query::update()
        .table(JobIden::Table)
        .values([
            (JobIden::Status, "completed".into()),
            (JobIden::FinishedAt, Utc::now().into()),
            (JobIden::CompletionMessage, message.into()),
        ])
        .and_where(Expr::col(JobIden::Id).eq(id))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let job = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(job)
}

#[instrument(err(Debug), skip(db))]
pub async fn delete(db: &PgPool, id: &Uuid) -> Result<Job, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(JobIden::Table)
        .and_where(Expr::col(JobIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let job = sqlx::query_as_with::<_, Job, _>(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Job")?;

    Ok(job)
}

#[instrument(err(Debug), skip(db))]
pub async fn get_id_id(db: &PgPool, id: &Uuid) -> Result<Job, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(JobIden::Table)
        .and_where(Expr::col(JobIden::Id).eq(id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let job = sqlx::query_as_with::<_, Job, _>(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Job")?;

    Ok(job)
}

/// Ordering options for listing jobs.
///
/// Each field represents an optional ordering to apply to the query.
/// If a field is `Some`, an `ORDER BY` clause is added for the
/// corresponding column.
#[derive(Deserialize, Debug, Default, JsonSchema)]
pub struct JobOrderOptions {
    created_at: Option<Order>,
    finished_at: Option<Order>,
}

impl JobOrderOptions {
    /// Applies any configured ordering options to a `SELECT` statement.
    ///
    /// For each field that is `Some`, an `ORDER BY` clause is appended
    /// to the query. Fields that are `None` are ignored.
    ///
    /// The modified query is returned.
    pub fn apply(&self, mut query: sea_query::SelectStatement) -> sea_query::SelectStatement {
        if let Some(order) = &self.created_at {
            query = query.order_by(JobIden::CreatedAt, order.into()).to_owned()
        }
        if let Some(order) = &self.finished_at {
            query = query.order_by(JobIden::FinishedAt, order.into()).to_owned()
        }
        query
    }
}

/// Filtering options for listing jobs.
///
/// Each field represents an optional filter condition.
/// If a field is `Some`, a `WHERE` clause is added to the query
/// matching the corresponding column.
#[derive(Deserialize, Debug, JsonSchema, Default)]
pub struct JobFilterOptions {
    status: Option<String>,
    step: Option<String>,
    progress: Option<String>,
    completion_message: Option<String>,
}

impl JobFilterOptions {
    /// Applies any configured filters to a `SELECT` statement.
    ///
    /// For each field that is `Some`, an `AND` condition is added
    /// to the query. Fields that are `None` are ignored.
    ///
    /// The modified query is returned.
    pub fn apply(&self, mut query: sea_query::SelectStatement) -> sea_query::SelectStatement {
        if let Some(value) = &self.status {
            query = query
                .and_where(Expr::col(JobIden::Status).eq(value))
                .to_owned();
        }
        if let Some(value) = &self.step {
            query = query
                .and_where(Expr::col(JobIden::Step).eq(value))
                .to_owned();
        }
        if let Some(value) = &self.progress {
            query = query
                .and_where(Expr::col(JobIden::Progress).eq(value))
                .to_owned();
        }
        if let Some(value) = &self.completion_message {
            query = query
                .and_where(Expr::col(JobIden::CompletionMessage).eq(value))
                .to_owned();
        }
        query
    }
}

#[instrument(err(Debug), skip(db))]
pub async fn list(
    db: &PgPool,
    page_options: PageOptions,
    order_options: JobOrderOptions,
    filter_options: JobFilterOptions,
) -> Result<PaginatedResults<Job>, ComhairleError> {
    let query = Query::select()
        .from(JobIden::Table)
        .columns(DEFAULT_COLUMNS)
        .to_owned();

    let query = filter_options.apply(query);
    let query = order_options.apply(query);

    let jobs = page_options.fetch_paginated_results(db, query).await?;

    Ok(jobs)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error::Error;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_job(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let job = create(
            &pool,
            CreateJob {
                ..Default::default()
            },
        )
        .await?;

        assert_eq!(
            job.status,
            Some("pending".to_string()),
            "incorrect default status"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_mark_job_as_completed(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let job = create(
            &pool,
            CreateJob {
                ..Default::default()
            },
        )
        .await?;

        assert_eq!(
            job.status,
            Some("pending".to_string()),
            "incorrect status before update"
        );
        assert!(
            job.finished_at.is_none(),
            "incorrect finished_at before update"
        );
        assert!(
            job.completion_message.is_none(),
            "incorrect completion_message before update"
        );

        let job = complete(&pool, job.id, "Completed successfully in a test").await?;

        assert_eq!(
            job.status,
            Some("completed".to_string()),
            "incorrect status after update"
        );
        assert!(
            job.finished_at.is_some(),
            "incorrect finished_at after update"
        );
        assert_eq!(
            job.completion_message,
            Some("Completed successfully in a test".to_string()),
            "incorrect completion_message after update"
        );

        Ok(())
    }
}
