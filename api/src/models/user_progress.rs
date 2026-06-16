use core::fmt;

use chrono::{DateTime, Utc};
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{enum_def, Expr, JoinType, PostgresQueryBuilder, Query, SimpleExpr};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgConnection, PgPool};
use tracing::instrument;
use uuid::Uuid;

use crate::error::ComhairleError;

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

/// Create a NotStarted user_progress row for every user already registered
/// on a workflow, for the given workflow_step. Used when a step is added
/// after users have registered, so they aren't stranded without a progress
/// row for the new step. Takes a `&mut PgConnection` so it can be run
/// inside a wider transaction (e.g. with the step insert) or standalone
/// against a connection acquired from a pool.
#[instrument(err(Debug))]
pub async fn create_for_workflow_participants(
    db: &mut PgConnection,
    workflow_step_id: &Uuid,
    workflow_id: &Uuid,
) -> Result<(), ComhairleError> {
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

#[instrument(err(Debug))]
pub async fn update(
    db: &PgPool,
    user_id: &Uuid,
    workflow_step_id: &Uuid,
    update_progress: &UpdateUserProgress,
) -> Result<UserProgress, ComhairleError> {
    let values = update_progress.to_values();

    if values.is_empty() {
        return Err(ComhairleError::NoValidUpdates);
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

#[cfg(test)]
mod tests {
    use crate::{
        models::{
            model_test_helpers::{get_random_conversation_id, setup_default_app_and_session},
            users,
        },
        routes::{workflow_steps::dto::WorkflowStepDto, workflows::dto::WorkflowDto},
    };

    use super::*;

    use std::error::Error;

    #[sqlx::test]
    async fn should_create_user_progress(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let (_, value, _) = session
            .create_random_workflow(&app, &conversation_id.to_string())
            .await?;
        let workflow: WorkflowDto = serde_json::from_value(value)?;

        let values = session
            .create_random_workflow_steps(
                &app,
                &conversation_id.to_string(),
                &workflow.id.to_string(),
                2,
            )
            .await?;
        let steps: Vec<WorkflowStepDto> = values
            .into_iter()
            .map(|v| serde_json::from_value(v).unwrap())
            .collect();

        let user = users::create_annon_user(&pool).await?;

        let user_progress = create(
            &pool,
            &user.id,
            &steps.first().unwrap().id,
            ProgressStatus::InProgress,
        )
        .await?;

        assert_eq!(user_progress.user_id, user.id, "user_ids don't match");

        Ok(())
    }

    #[sqlx::test]
    async fn should_update_user_progress(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let (_, value, _) = session
            .create_random_workflow(&app, &conversation_id.to_string())
            .await?;
        let workflow: WorkflowDto = serde_json::from_value(value)?;

        let values = session
            .create_random_workflow_steps(
                &app,
                &conversation_id.to_string(),
                &workflow.id.to_string(),
                2,
            )
            .await?;
        let steps: Vec<WorkflowStepDto> = values
            .into_iter()
            .map(|v| serde_json::from_value(v).unwrap())
            .collect();

        let user = users::create_annon_user(&pool).await?;

        let user_progress = create(
            &pool,
            &user.id,
            &steps.first().unwrap().id,
            ProgressStatus::NotStarted,
        )
        .await?;

        assert_eq!(
            user_progress.status,
            ProgressStatus::NotStarted,
            "incorrect status before update"
        );
        assert!(
            user_progress.permission_to_share_with_organizers,
            "incorrect permission before update"
        );

        let update_params = UpdateUserProgress {
            status: Some(ProgressStatus::Done),
            permission_to_share_with_organizers: Some(false),
            ..Default::default()
        };

        let user_progress =
            update(&pool, &user.id, &steps.first().unwrap().id, &update_params).await?;

        assert_eq!(
            user_progress.status,
            ProgressStatus::Done,
            "incorrect status after update"
        );
        assert!(
            !user_progress.permission_to_share_with_organizers,
            "incorrect permission after update"
        );

        Ok(())
    }
}
