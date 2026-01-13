use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{enum_def, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use uuid::Uuid;

use crate::error::ComhairleError;

#[derive(Serialize, Deserialize, FromRow, Clone, JsonSchema)]
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

const DEFAULT_COLUMNS: [JobIden; 7] = [
    JobIden::Id,
    JobIden::CreatedAt,
    JobIden::FinishedAt,
    JobIden::Error,
    JobIden::CompletionMessage,
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
        let mut values = vec!["running".to_string().into()];
        if let Some(value) = &self.step {
            values.push(value.into());
        }
        if let Some(value) = &self.progress {
            values.push((*value).into());
        }
        values
    }
}

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

pub async fn delete(db: &PgPool, id: &Uuid) -> Result<Job, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(JobIden::Table)
        .and_where(Expr::col(JobIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let job = sqlx::query_as_with::<_, Job, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("Job".into()))?;

    Ok(job)
}

pub async fn get_id_id(db: &PgPool, id: &Uuid) -> Result<Job, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(JobIden::Table)
        .and_where(Expr::col(JobIden::Id).eq(id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let job = sqlx::query_as_with::<_, Job, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("Job".into()))?;

    Ok(job)
}
