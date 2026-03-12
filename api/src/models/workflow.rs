use std::sync::Arc;

use chrono::{DateTime, Utc};

#[cfg(test)]
use fake::Dummy;

use partially::Partial;
use schemars::JsonSchema;
use sea_query::{enum_def, Expr, Order, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use tracing::instrument;
use uuid::Uuid;

use crate::{error::ComhairleError, ComhairleState};

use super::{
    user_conversation_preferences,
    user_participation::{self, UserParticipation, UserParticipationIden},
    user_progress::{self, UserProgressIden},
    users::User,
    workflow_step::{self, WorkflowStepIden},
};

#[derive(Partial, Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "workflow")]
#[partially(derive(Deserialize, Debug, JsonSchema, Default))]
pub struct Workflow {
    #[partially(omit)]
    pub id: Uuid,
    #[partially(omit)]
    pub conversation_id: Option<Uuid>,
    pub name: String,
    pub description: String,
    pub is_active: bool,
    pub is_public: bool,
    pub auto_login: bool,
    pub event_id: Option<Uuid>,
    #[partially(omit)]
    pub owner_id: Uuid,
    #[partially(transparent)]
    pub region_id: Option<Uuid>,
    #[partially(omit)]
    pub created_at: DateTime<Utc>,
    #[partially(omit)]
    pub updated_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [WorkflowIden; 12] = [
    WorkflowIden::Id,
    WorkflowIden::ConversationId,
    WorkflowIden::Name,
    WorkflowIden::Description,
    WorkflowIden::IsPublic,
    WorkflowIden::IsActive,
    WorkflowIden::EventId,
    WorkflowIden::CreatedAt,
    WorkflowIden::UpdatedAt,
    WorkflowIden::OwnerId,
    WorkflowIden::RegionId,
    WorkflowIden::AutoLogin,
];

impl PartialWorkflow {
    pub fn to_values(&self) -> Vec<(WorkflowIden, sea_query::SimpleExpr)> {
        let mut values = vec![];
        if let Some(value) = &self.name {
            values.push((WorkflowIden::Name, value.into()))
        };
        if let Some(value) = &self.description {
            values.push((WorkflowIden::Description, value.into()))
        };

        if let Some(value) = self.is_public {
            values.push((WorkflowIden::IsPublic, value.into()))
        };
        if let Some(value) = self.is_active {
            values.push((WorkflowIden::IsActive, value.into()))
        };
        if let Some(value) = self.auto_login {
            values.push((WorkflowIden::AutoLogin, value.into()))
        };
        if let Some(value) = self.region_id {
            values.push((WorkflowIden::RegionId, value.into()))
        };
        values
    }
}

/// Get a conversation by ID
#[instrument(err(Debug))]
pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<Workflow, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(WorkflowIden::Table)
        .and_where(Expr::col(WorkflowIden::Id).eq(id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let conversation = sqlx::query_as_with::<_, Workflow, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("Workflow".into()))?;

    Ok(conversation)
}

pub async fn launch(
    db: &PgPool,
    workflow_id: &Uuid,
    state: &Arc<ComhairleState>,
) -> Result<(), ComhairleError> {
    let steps = workflow_step::list(db, workflow_id).await?;
    for step in steps {
        workflow_step::launch(db, &step.id, state).await?;
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[cfg_attr(test, derive(Dummy))]
pub struct CreateWorkflow {
    pub name: String,
    pub description: String,
    pub is_active: bool,
    pub is_public: bool,
    pub auto_login: bool,
    pub region_id: Option<Uuid>,
}

impl CreateWorkflow {
    pub fn columns(&self) -> Vec<WorkflowIden> {
        let mut columns = vec![
            WorkflowIden::Name,
            WorkflowIden::Description,
            WorkflowIden::IsActive,
            WorkflowIden::IsPublic,
            WorkflowIden::AutoLogin,
        ];

        if self.region_id.is_some() {
            columns.push(WorkflowIden::RegionId);
        }

        columns
    }
    pub fn values(&self) -> Vec<sea_query::SimpleExpr> {
        let mut values = vec![
            self.name.to_owned().into(),
            self.description.to_owned().into(),
            self.is_active.into(),
            self.is_public.into(),
            self.auto_login.into(),
        ];

        if let Some(value) = &self.region_id {
            values.push((*value).into());
        }

        values
    }
}

#[instrument(err(Debug))]
pub async fn register_user(
    db: &PgPool,
    workflow_id: &Uuid,
    user: &User,
) -> Result<UserParticipation, ComhairleError> {
    let workflow = get_by_id(db, workflow_id).await?;
    let user_participation = user_participation::create(db, &user.id, workflow_id).await?;

    let workflow_steps = workflow_step::list(db, workflow_id).await?;

    for step in workflow_steps {
        user_progress::create(
            db,
            &user.id,
            &step.id,
            user_progress::ProgressStatus::NotStarted,
        )
        .await?;
    }

    // Check to see if the user already has preferences for this
    // conversastion
    if let Some(conversation_id) = workflow.conversation_id {
        let user_preferences = user_conversation_preferences::get_by_user_and_conversation(
            db,
            &user.id,
            &conversation_id,
        )
        .await;

        // If they dont, create some
        if user_preferences.is_err() {
            user_conversation_preferences::create_with_defaults(db, &user.id, &conversation_id)
                .await?;
        }
    }

    Ok(user_participation)
}

// TODO ensure this deletes all workflow steps on deletion
#[instrument(err(Debug))]
pub async fn delete(db: &PgPool, id: &Uuid) -> Result<Workflow, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(WorkflowIden::Table)
        .and_where(Expr::col(WorkflowIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let workflow = sqlx::query_as_with::<_, Workflow, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("Conversation".into()))?;

    Ok(workflow)
}

#[instrument(err(Debug))]
pub async fn update(
    db: &PgPool,
    id: Uuid,
    update: &PartialWorkflow,
) -> Result<Workflow, ComhairleError> {
    let values = update.to_values();

    if values.is_empty() {
        return Err(ComhairleError::NoValidUpdates);
    }

    let (sql, values) = Query::update()
        .table(WorkflowIden::Table)
        .values(values)
        .and_where(Expr::col(WorkflowIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let workflow = sqlx::query_as_with::<_, Workflow, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(workflow)
}

/// Retrieves a list of workflows for a conversation or an event.
///
/// # Arguments
///
/// * `db` - Database connection pool
/// * `conversation_id` - Unique identifier of the conversation the workflow may be attached to
/// * `event_id` - Optional unique identifier of the event may be attached to
///
/// If `event_id` is `Some` the constructed query will filter for workflows with
/// the matching `event_id`. Otherwise, the constructed query will filter for workflows
/// with the matching `conversation_id`.
///
/// As events are attached to conversations and event routes are nested under
/// `/conversation`, the `conversation_id` is always passed as an argument, even if
/// it is not used in the final database query.
#[instrument(err(Debug))]
pub async fn list(
    db: &PgPool,
    conversation_id: Uuid,
    event_id: Option<Uuid>,
) -> Result<Vec<Workflow>, ComhairleError> {
    let mut query = Query::select()
        .from(WorkflowIden::Table)
        .columns(DEFAULT_COLUMNS)
        .to_owned();

    if let Some(e_id) = event_id {
        query = query
            .and_where(Expr::col(WorkflowIden::EventId).eq(e_id))
            .to_owned();
    } else {
        query = query
            .and_where(Expr::col(WorkflowIden::ConversationId).eq(conversation_id))
            .to_owned();
    }

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let workflows = sqlx::query_as_with::<_, Workflow, _>(&sql, values)
        .fetch_all(db)
        .await?;

    Ok(workflows)
}

#[derive(Serialize, Deserialize, JsonSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowStats {
    pub total_users: i32,
    pub step_stats: Vec<WorkflowStepStats>,
    pub signup_stats: Vec<DailySignupStats>,
}

#[derive(Serialize, Deserialize, JsonSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowStepStats {
    id: Uuid,
    pub completed: i32,
    pub started: i32,
}

/// Calculate stastistics for the workflow
pub async fn stats(db: &PgPool, workflow_id: Uuid) -> Result<WorkflowStats, ComhairleError> {
    let (sql, values) = Query::select()
        .from(WorkflowStepIden::Table)
        .column((WorkflowStepIden::Table, WorkflowStepIden::Id))
        .expr(Expr::cust(
            "COUNT(CASE WHEN user_progress.status = 'done' THEN 1 END)::INT as completed",
        ))
        .expr(Expr::cust(
            "COUNT(CASE WHEN user_progress.status = 'in_progress' THEN 1 END)::INT as started",
        ))
        .join(
            sea_query::JoinType::LeftJoin,
            UserProgressIden::Table,
            Expr::col((UserProgressIden::Table, UserProgressIden::WorkflowStepId))
                .equals((WorkflowStepIden::Table, WorkflowStepIden::Id)),
        )
        .and_where(
            Expr::col((WorkflowStepIden::Table, WorkflowStepIden::WorkflowId)).eq(workflow_id),
        )
        .group_by_col((WorkflowStepIden::Table, WorkflowStepIden::Id))
        .group_by_col(WorkflowStepIden::StepOrder)
        .order_by(
            (WorkflowStepIden::Table, WorkflowStepIden::StepOrder),
            Order::Asc,
        )
        .to_owned()
        .build_sqlx(PostgresQueryBuilder);

    let step_stats = sqlx::query_as_with::<_, WorkflowStepStats, _>(&sql, values)
        .fetch_all(db)
        .await?;

    let (sql, values) = Query::select()
        .from(UserParticipationIden::Table)
        .expr(Expr::cust("COUNT(*)::INT as count"))
        .and_where(Expr::col(UserParticipationIden::WorkflowId).eq(workflow_id))
        .to_owned()
        .build_sqlx(PostgresQueryBuilder);

    let total_users: i32 = sqlx::query_scalar_with(&sql, values).fetch_one(db).await?;
    let signup_stats = get_workflow_signup_stats(db, workflow_id).await?;

    Ok(WorkflowStats {
        step_stats,
        total_users,
        signup_stats,
    })
}

/// Creates a new workflow record for a given conversation or event.
///
/// # Arguments
///
/// * `db` - Database connection pool
/// * `workflow` - Params for the new workflow to create
/// * `conversation_id` - Optional Unique identifier of the conversation the workflow may be attached to
/// * `event_id` - Optional unique identifier of the event may be attached to
/// * `owner_id` - Unique identifier of the author user of the new workflow
///
/// If `conversation_id` is `Some` the workflow will be created referencing the specified conversation.
///
/// If `event_id` is `Some` the workflow will be created referencing the specified event.
#[instrument(err(Debug))]
pub async fn create(
    db: &PgPool,
    workflow: &CreateWorkflow,
    conversation_id: Option<Uuid>,
    event_id: Option<Uuid>,
    owner_id: Uuid,
) -> Result<Workflow, ComhairleError> {
    let mut columns = workflow.columns();
    let mut values = workflow.values();

    if let Some(c_id) = conversation_id {
        columns.push(WorkflowIden::ConversationId);
        values.push(c_id.into());
    }

    if let Some(e_id) = event_id {
        columns.push(WorkflowIden::EventId);
        values.push(e_id.into());
    }

    columns.push(WorkflowIden::OwnerId);
    values.push(owner_id.into());

    let (sql, values) = Query::insert()
        .into_table(WorkflowIden::Table)
        .columns(columns)
        .values(values)
        .unwrap()
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let workflow = sqlx::query_as_with::<_, Workflow, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(workflow)
}

#[derive(FromRow, Serialize, Deserialize, JsonSchema, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DailySignupStats {
    pub day: DateTime<Utc>,
    pub users: i32,
}

pub async fn get_workflow_signup_stats(
    db: &PgPool,
    workflow_id: Uuid,
) -> Result<Vec<DailySignupStats>, ComhairleError> {
    let result = sqlx::query_as::<_, DailySignupStats>(
        r#"
        SELECT date_trunc('day', created_at) as day,
        count(*)::INT as users
        FROM user_participation 
        where workflow_id = $1
        GROUP BY date_trunc('day', created_at)
        ORDER BY date_trunc('day', created_at) ASC;
    "#,
    )
    .bind(workflow_id)
    .fetch_all(db)
    .await
    .map_err(ComhairleError::WorkflowStatsAggregationError)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::{
        models::model_test_helpers::setup_default_app_and_session,
        routes::{
            conversations::dto::ConversationDto, regions::dto::RegionDto,
            workflows::dto::WorkflowDto,
        },
    };

    use super::*;

    use std::error::Error;

    #[sqlx::test]
    async fn should_create_workflow_with_region_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, convo_res, _) = session.create_random_conversation(&app).await?;
        let conversation: ConversationDto = serde_json::from_value(convo_res)?;
        let (_, region_res, _) = session.create_random_region(&app).await?;
        let region: RegionDto = serde_json::from_value(region_res)?;

        let (_, user, _) = session.current_user(&app).await?;

        let params = CreateWorkflow {
            name: "test_workflow".to_string(),
            description: "a test workflow".to_string(),
            is_active: true,
            is_public: true,
            auto_login: false,
            region_id: Some(region.id),
        };

        let workflow = create(&pool, &params, Some(conversation.id), None, user.id).await?;

        assert_eq!(
            workflow.region_id.unwrap(),
            region.id,
            "incorrect region_id"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_update_workflow_region_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, convo_res, _) = session.create_random_conversation(&app).await?;
        let conversation: ConversationDto = serde_json::from_value(convo_res)?;
        let (_, region_res, _) = session.create_random_region(&app).await?;
        let region: RegionDto = serde_json::from_value(region_res)?;
        let (_, workflow_res, _) = session
            .create_random_workflow(&app, &conversation.id.to_string())
            .await?;
        let workflow: WorkflowDto = serde_json::from_value(workflow_res)?;

        assert!(
            workflow.region_id.is_none(),
            "incorrect region_id before update"
        );

        let workflow = update(
            &pool,
            workflow.id,
            &PartialWorkflow {
                region_id: Some(region.id),
                ..Default::default()
            },
        )
        .await?;

        assert_eq!(
            workflow.region_id.unwrap(),
            region.id,
            "incorrect region_id"
        );

        Ok(())
    }
}
