use core::fmt;

use chrono::{DateTime, Utc};
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{enum_def, ConditionalStatement, Expr, JoinType, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use tracing::instrument;
use uuid::Uuid;

use crate::error::ComhairleError;

use super::workflow_step::WorkflowStepIden;

/// Defines the type of authentication has been used to create
/// The user
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, sqlx::Type, Clone, JsonSchema)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "snake_case")]
pub enum ProgressStatus {
    #[sqlx(rename = "not_started")]
    NotStarted,
    #[sqlx(rename = "in_progress")]
    InProgress,
    #[sqlx(rename = "done")]
    Done,
}

impl Into<sea_query::Value> for ProgressStatus {
    fn into(self) -> sea_query::Value {
        sea_query::Value::String(Some(Box::new(self.to_string())))
    }
}

impl fmt::Display for ProgressStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            ProgressStatus::NotStarted => "not_started",
            ProgressStatus::InProgress => "in_progress",
            ProgressStatus::Done => "done",
        };
        write!(f, "{}", value)
    }
}

#[derive(Partial, Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "user_progress")]
pub struct UserProgress {
    pub id: Uuid,
    pub user_id: Uuid,
    pub workflow_step_id: Uuid,
    pub status: ProgressStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [(UserProgressIden, UserProgressIden); 6] = [
    (UserProgressIden::Table, UserProgressIden::Id),
    (UserProgressIden::Table, UserProgressIden::UserId),
    (UserProgressIden::Table, UserProgressIden::WorkflowStepId),
    (UserProgressIden::Table, UserProgressIden::Status),
    (UserProgressIden::Table, UserProgressIden::CreatedAt),
    (UserProgressIden::Table, UserProgressIden::UpdatedAt),
];

#[instrument(err(Debug))]
pub async fn create(
    db: &PgPool,
    user_id: &Uuid,
    workflow_step_id: &Uuid,
    status: ProgressStatus,
) -> Result<UserProgress, ComhairleError> {
    let (sql, values) = Query::insert()
        .into_table(UserProgressIden::Table)
        .columns([
            UserProgressIden::UserId,
            UserProgressIden::WorkflowStepId,
            UserProgressIden::Status,
        ])
        .values([
            user_id.to_owned().into(),
            workflow_step_id.to_owned().into(),
            status.to_owned().into(),
        ])
        .unwrap()
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let result = sqlx::query_as_with::<_, UserProgress, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(result)
}
#[instrument(err(Debug))]
pub async fn update(
    db: &PgPool,
    user_id: &Uuid,
    workflow_step_id: &Uuid,
    status: ProgressStatus,
) -> Result<UserProgress, ComhairleError> {
    let (sql, values) = Query::update()
        .table(UserProgressIden::Table)
        .values([(UserProgressIden::Status, status.into())])
        .and_where(Expr::col(UserProgressIden::UserId).eq(user_id.to_owned()))
        .and_where(Expr::col(UserProgressIden::WorkflowStepId).eq(workflow_step_id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let result = sqlx::query_as_with::<_, UserProgress, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(result)
}

#[instrument(err(Debug))]
pub async fn list_for_user_on_workflow(
    db: &PgPool,
    user_id: &Uuid,
    workflow_id: &Uuid,
) -> Result<Vec<UserProgress>, ComhairleError> {
    let (sql, values) = Query::select()
        .from(UserProgressIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(
            Expr::col((UserProgressIden::Table, UserProgressIden::UserId)).eq(user_id.to_owned()),
        )
        .and_where(
            Expr::col((WorkflowStepIden::Table, WorkflowStepIden::WorkflowId))
                .eq(workflow_id.to_owned()),
        )
        .join(
            JoinType::InnerJoin,
            WorkflowStepIden::Table,
            Expr::col((UserProgressIden::Table, UserProgressIden::WorkflowStepId))
                .equals((WorkflowStepIden::Table, WorkflowStepIden::Id)),
        )
        .to_owned()
        .build_sqlx(PostgresQueryBuilder);

    let result = sqlx::query_as_with::<_, UserProgress, _>(&sql, values)
        .fetch_all(db)
        .await;

    match result {
        Ok(result) => Ok(result),
        Err(sqlx::Error::Database(db_err)) => {
            let pg_err = db_err.downcast_ref::<sqlx::postgres::PgDatabaseError>();
            if pg_err.code() == "23505" {
                return Err(ComhairleError::UserAlreadyParticipatingInWorkflow(
                    workflow_id.to_string(),
                ));
            }
            Err(ComhairleError::DatabaseError(sqlx::Error::Database(db_err)))
        }
        Err(e) => Err(ComhairleError::DatabaseError(e)),
    }
}
