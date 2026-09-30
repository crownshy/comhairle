use crate::models::error::ModelError;
use core::fmt;

use chrono::{DateTime, Utc};
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{Expr, JoinType, PostgresQueryBuilder, Query, SimpleExpr, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgConnection, PgPool, prelude::FromRow};
use tracing::instrument;
use uuid::Uuid;

use crate::models::error::DataError;
use crate::models::error::ValidationError;
use crate::models::error::WorkflowError;

use super::user_participation::UserParticipationIden;
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

impl From<ProgressStatus> for sea_query::Value {
    fn from(val: ProgressStatus) -> Self {
        sea_query::Value::String(Some(Box::new(val.to_string())))
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
    pub permission_to_share_with_organizers: bool,
    pub permission_to_share_with_other_participants: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [(UserProgressIden, UserProgressIden); 8] = [
    (UserProgressIden::Table, UserProgressIden::Id),
    (UserProgressIden::Table, UserProgressIden::UserId),
    (UserProgressIden::Table, UserProgressIden::WorkflowStepId),
    (UserProgressIden::Table, UserProgressIden::Status),
    (
        UserProgressIden::Table,
        UserProgressIden::PermissionToShareWithOrganizers,
    ),
    (
        UserProgressIden::Table,
        UserProgressIden::PermissionToShareWithOtherParticipants,
    ),
    (UserProgressIden::Table, UserProgressIden::CreatedAt),
    (UserProgressIden::Table, UserProgressIden::UpdatedAt),
];

#[instrument(err(Debug), skip(db))]
pub async fn create(
    db: &PgPool,
    user_id: &Uuid,
    workflow_step_id: &Uuid,
    status: ProgressStatus,
) -> Result<UserProgress, ModelError> {
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

/// Create a NotStarted user_progress row for every user already registered
/// on a workflow, for the given workflow_step. Used when a step is added
/// after users have registered, so they aren't stranded without a progress
/// row for the new step. Takes a `&mut PgConnection` so it can be run
/// inside a wider transaction (e.g. with the step insert) or standalone
/// against a connection acquired from a pool.
#[instrument(err(Debug), skip(db))]
pub async fn create_for_workflow_participants(
    db: &mut PgConnection,
    workflow_step_id: &Uuid,
    workflow_id: &Uuid,
) -> Result<(), ModelError> {
    let select = Query::select()
        .column(UserParticipationIden::UserId)
        .expr(Expr::val(*workflow_step_id))
        .expr(Expr::val(ProgressStatus::NotStarted))
        .from(UserParticipationIden::Table)
        .and_where(Expr::col(UserParticipationIden::WorkflowId).eq(*workflow_id))
        .to_owned();

    let (sql, values) = Query::insert()
        .into_table(UserProgressIden::Table)
        .columns([
            UserProgressIden::UserId,
            UserProgressIden::WorkflowStepId,
            UserProgressIden::Status,
        ])
        .select_from(select)?
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_with(&sql, values).execute(db).await?;

    Ok(())
}

#[derive(Deserialize, Debug, JsonSchema, Default)]
pub struct UpdateUserProgress {
    pub status: Option<ProgressStatus>,
    pub permission_to_share_with_organizers: Option<bool>,
    pub permission_to_share_with_other_participants: Option<bool>,
}

impl UpdateUserProgress {
    /// Would applying this payload to `current` leave it unchanged? Used by the sealed gate
    /// to let a duplicate write through as a no-op instead of erroring. An absent field is
    /// not an update, so it can never make the payload a change.
    pub fn is_noop_for(&self, current: &UserProgress) -> bool {
        self.status.as_ref().is_none_or(|s| *s == current.status)
            && self
                .permission_to_share_with_organizers
                .is_none_or(|v| v == current.permission_to_share_with_organizers)
            && self
                .permission_to_share_with_other_participants
                .is_none_or(|v| v == current.permission_to_share_with_other_participants)
    }

    fn to_values(&self) -> Vec<(UserProgressIden, SimpleExpr)> {
        let mut values = vec![];
        if let Some(value) = &self.status {
            values.push((UserProgressIden::Status, value.clone().into()));
        }
        if let Some(value) = &self.permission_to_share_with_organizers {
            values.push((
                UserProgressIden::PermissionToShareWithOrganizers,
                (*value).into(),
            ));
        }
        if let Some(value) = &self.permission_to_share_with_other_participants {
            values.push((
                UserProgressIden::PermissionToShareWithOtherParticipants,
                (*value).into(),
            ));
        }

        values
    }
}

#[instrument(err(Debug), skip(db))]
pub async fn update(
    db: &PgPool,
    user_id: &Uuid,
    workflow_step_id: &Uuid,
    update_progress: &UpdateUserProgress,
) -> Result<UserProgress, ModelError> {
    let values = update_progress.to_values();

    if values.is_empty() {
        return Err(ValidationError::NoValidUpdates.into());
    }

    let (sql, values) = Query::update()
        .table(UserProgressIden::Table)
        .values(values)
        .and_where(Expr::col(UserProgressIden::UserId).eq(user_id.to_owned()))
        .and_where(Expr::col(UserProgressIden::WorkflowStepId).eq(workflow_step_id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let result = sqlx::query_as_with::<_, UserProgress, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(result)
}

/// A single participant's progress row for one step, if they have one at all. A step added
/// after they registered leaves them with no row, hence the `Option`.
#[instrument(err(Debug), skip(db))]
pub async fn get(
    db: &PgPool,
    user_id: &Uuid,
    workflow_step_id: &Uuid,
) -> Result<Option<UserProgress>, ModelError> {
    let (sql, values) = Query::select()
        .from(UserProgressIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(
            Expr::col((UserProgressIden::Table, UserProgressIden::UserId)).eq(user_id.to_owned()),
        )
        .and_where(
            Expr::col((UserProgressIden::Table, UserProgressIden::WorkflowStepId))
                .eq(workflow_step_id.to_owned()),
        )
        .build_sqlx(PostgresQueryBuilder);

    let result = sqlx::query_as_with::<_, UserProgress, _>(&sql, values)
        .fetch_optional(db)
        .await?;

    Ok(result)
}

#[instrument(err(Debug), skip(db))]
pub async fn list_for_user_on_workflow(
    db: &PgPool,
    user_id: &Uuid,
    workflow_id: &Uuid,
) -> Result<Vec<UserProgress>, ModelError> {
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
                return Err(WorkflowError::UserAlreadyParticipatingInWorkflow(
                    workflow_id.to_string(),
                )
                .into());
            }
            Err(DataError::DatabaseError(sqlx::Error::Database(db_err)).into())
        }
        Err(e) => Err(DataError::DatabaseError(e).into()),
    }
}

/// Has this participant finished, i.e. does every step in the workflow have a `done`
/// progress row for them?
///
/// A workflow with no steps is never finished - "you completed all zero of them" is not a
/// meaningful thing to seal on.
#[instrument(err(Debug), skip(db))]
pub async fn has_finished(
    db: &PgPool,
    user_id: &Uuid,
    workflow_id: &Uuid,
) -> Result<bool, ModelError> {
    // Deliberately the same predicate `/next` uses to decide whether to send a participant to
    // the thank-you page (`get_current_active_step_for_user` returning nothing). Sealing on a
    // separately-written rule would let the two drift, and then a participant could be shown
    // the thank-you page while not counting as finished, or the reverse.
    //
    // That helper left-joins progress, so a step with no progress row at all (one added after
    // the participant registered) correctly reads as unfinished rather than as absent.
    if super::workflow_step::count_for_workflow(db, workflow_id).await? == 0 {
        return Ok(false);
    }

    let next_step =
        super::workflow_step::get_current_active_step_for_user(db, user_id, workflow_id).await?;

    Ok(next_step.is_none())
}

/// Is this participant sealed: have they finished a conversation whose
/// `allow_revisit_after_finishing` is off?
///
/// This is the single definition of sealed in the system. It backs both the flag shipped to
/// the frontend and the write gates on participant contribution routes, so the two can never
/// disagree. Sealed is derived on every call rather than stored; see ADR-0016 for why, and
/// for the consequence that adding a step to a live workflow un-seals everyone who had
/// already finished.
///
/// Event workflows (no `conversation_id`) are never sealed: the setting lives on the
/// conversation and there is nothing to read.
#[instrument(err(Debug), skip(db))]
pub async fn is_sealed(
    db: &PgPool,
    user_id: &Uuid,
    workflow_id: &Uuid,
) -> Result<bool, ModelError> {
    let workflow = super::workflow::get_by_id(db, workflow_id).await?;
    let Some(conversation_id) = workflow.conversation_id else {
        return Ok(false);
    };

    let conversation = super::conversation::get_by_id(db, &conversation_id).await?;
    if conversation.allow_revisit_after_finishing {
        return Ok(false);
    }

    has_finished(db, user_id, workflow_id).await
}
