use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use chrono::{DateTime, Utc};
#[cfg(test)]
use fake::Dummy;
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{
    Alias, Asterisk, Cond, Expr, Func, FunctionCall, JoinType, Keyword, Order,
    PostgresQueryBuilder, Query, SimpleExpr, enum_def,
};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::prelude::FromRow;
use tracing::instrument;
use uuid::Uuid;

use super::event::EventIden;
use super::user_activity::UserActivityIden;
use super::user_participation::{self, UserParticipation, UserParticipationIden};
use super::user_progress::{self, ProgressStatus, UserProgressIden};
use super::users::User;
use super::workflow_step::{self, WorkflowStepIden};
use super::{SqlxResultExt, user_conversation_preferences};
use crate::ComhairleState;
use crate::error::ComhairleError;
use crate::models::invite_response::{self, InviteResponseIden};
use crate::models::invites::InviteIden;

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
#[instrument(err(Debug), skip(db))]
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

#[instrument(err(Debug), skip(state))]
pub async fn launch(state: &Arc<ComhairleState>, workflow_id: &Uuid) -> Result<(), ComhairleError> {
    let steps = workflow_step::list(&state.db, workflow_id).await?;
    for step in steps {
        workflow_step::launch(state, &step.id).await?;
    }

    Ok(())
}

#[instrument(err(Debug), skip(db))]
pub async fn check_user_is_owner(
    db: &PgPool,
    workflow_id: &Uuid,
    user_id: &Uuid,
) -> Result<(), ComhairleError> {
    let workflow = get_by_id(db, workflow_id).await?;
    if let Some(conversation_id) = workflow.conversation_id {
        let conversation = super::conversation::get_by_id(db, &conversation_id).await?;
        if conversation.owner_id != *user_id {
            return Err(ComhairleError::UserNotAuthorized);
        }
    } else {
        return Err(ComhairleError::UserNotAuthorized);
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

#[instrument(err(Debug), skip(db))]
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

    if let Some(conversation_id) = workflow.conversation_id {
        user_conversation_preferences::create_defaults_if_missing(db, &user.id, &conversation_id)
            .await?;
    }

    Ok(user_participation)
}

#[instrument(err(Debug), skip(state))]
pub async fn delete(state: &Arc<ComhairleState>, id: &Uuid) -> Result<Workflow, ComhairleError> {
    let workflow_steps = workflow_step::list(&state.db, id).await?;

    for step in workflow_steps {
        workflow_step::delete(state, &step.id).await?;
    }

    let (sql, values) = Query::delete()
        .from_table(WorkflowIden::Table)
        .and_where(Expr::col(WorkflowIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let workflow = sqlx::query_as_with::<_, Workflow, _>(&sql, values)
        .fetch_one(&state.db)
        .await
        .resolve_db_err("Workflow")?;

    Ok(workflow)
}

#[instrument(err(Debug), skip(db))]
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

/// Lists every workflow belonging to a conversation, including workflows attached only to
/// one of its events (those have no `conversation_id`).
#[instrument(err(Debug), skip(db))]
pub async fn list_for_conversation(
    db: &PgPool,
    conversation_id: Uuid,
) -> Result<Vec<Workflow>, ComhairleError> {
    let event_ids = Query::select()
        .column(EventIden::Id)
        .from(EventIden::Table)
        .and_where(Expr::col(EventIden::ConversationId).eq(conversation_id))
        .to_owned();

    let (sql, values) = Query::select()
        .from(WorkflowIden::Table)
        .columns(DEFAULT_COLUMNS)
        .cond_where(
            Cond::any()
                .add(Expr::col(WorkflowIden::ConversationId).eq(conversation_id))
                .add(Expr::col(WorkflowIden::EventId).in_subquery(event_ids)),
        )
        .build_sqlx(PostgresQueryBuilder);

    let workflows = sqlx::query_as_with::<_, Workflow, _>(&sql, values)
        .fetch_all(db)
        .await?;

    Ok(workflows)
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
#[instrument(err(Debug), skip(db))]
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
    pub active_users: i32,
    pub finished_users: i32,
    pub recruited_users: i32,
    pub total_active_seconds: i64,
    pub step_stats: Vec<WorkflowStepStats>,
    pub signup_stats: Vec<DailySignupStats>,
}

#[derive(Serialize, Deserialize, JsonSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowStepStats {
    id: Uuid,
    pub completed: i32,
    pub in_progress: i32,
    /// Active time for participants with done progress, including return visits.
    pub median_active_seconds: Option<f64>,
    pub max_active_seconds: Option<i64>,
}

#[derive(FromRow)]
struct WorkflowActivity {
    user_id: Uuid,
    workflow_step_id: Uuid,
    active_seconds: i32,
    activity_changed_at: DateTime<Utc>,
    is_active: bool,
    ended_at: Option<DateTime<Utc>>,
    status: Option<ProgressStatus>,
}

/// Calculate statistics from progress and tracked active time, including open visits.
pub async fn stats(db: &PgPool, workflow_id: Uuid) -> Result<WorkflowStats, ComhairleError> {
    let as_integer = |count: FunctionCall| SimpleExpr::from(count).cast_as(Alias::new("integer"));
    let status_count = |status: ProgressStatus| {
        as_integer(Func::count(
            Expr::case(
                Expr::col((UserProgressIden::Table, UserProgressIden::Status)).eq(status),
                1,
            )
            .finally(Keyword::Null),
        ))
    };
    let (sql, values) = Query::select()
        .from(WorkflowStepIden::Table)
        .column((WorkflowStepIden::Table, WorkflowStepIden::Id))
        .expr_as(status_count(ProgressStatus::Done), Alias::new("completed"))
        .expr_as(
            status_count(ProgressStatus::InProgress),
            Alias::new("in_progress"),
        )
        .join(
            JoinType::LeftJoin,
            UserProgressIden::Table,
            Expr::col((UserProgressIden::Table, UserProgressIden::WorkflowStepId))
                .equals((WorkflowStepIden::Table, WorkflowStepIden::Id)),
        )
        .and_where(
            Expr::col((WorkflowStepIden::Table, WorkflowStepIden::WorkflowId)).eq(workflow_id),
        )
        .group_by_col((WorkflowStepIden::Table, WorkflowStepIden::Id))
        .group_by_col((WorkflowStepIden::Table, WorkflowStepIden::StepOrder))
        .order_by(
            (WorkflowStepIden::Table, WorkflowStepIden::StepOrder),
            Order::Asc,
        )
        .build_sqlx(PostgresQueryBuilder);
    let rows: Vec<(Uuid, i32, i32)> = sqlx::query_as_with(&sql, values).fetch_all(db).await?;

    let unfinished_steps = Query::select()
        .expr(Expr::val(1))
        .from(WorkflowStepIden::Table)
        .join(
            JoinType::LeftJoin,
            UserProgressIden::Table,
            Cond::all()
                .add(
                    Expr::col((UserProgressIden::Table, UserProgressIden::WorkflowStepId))
                        .equals((WorkflowStepIden::Table, WorkflowStepIden::Id)),
                )
                .add(
                    Expr::col((UserProgressIden::Table, UserProgressIden::UserId))
                        .equals((UserParticipationIden::Table, UserParticipationIden::UserId)),
                )
                .add(
                    Expr::col((UserProgressIden::Table, UserProgressIden::Status))
                        .eq(ProgressStatus::Done),
                ),
        )
        .and_where(
            Expr::col((WorkflowStepIden::Table, WorkflowStepIden::WorkflowId)).eq(workflow_id),
        )
        .and_where(Expr::col((UserProgressIden::Table, UserProgressIden::Id)).is_null())
        .to_owned();
    let (sql, values) = Query::select()
        .from(UserParticipationIden::Table)
        .expr_as(
            as_integer(Func::count(Expr::col(Asterisk))),
            Alias::new("total"),
        )
        .expr_as(
            as_integer(Func::count(
                Expr::case(Expr::exists(unfinished_steps).not(), 1).finally(Keyword::Null),
            )),
            Alias::new("finished"),
        )
        .and_where(Expr::col(UserParticipationIden::WorkflowId).eq(workflow_id))
        .build_sqlx(PostgresQueryBuilder);
    let (total_users, finished_users): (i32, i32) =
        sqlx::query_as_with(&sql, values).fetch_one(db).await?;
    let finished_users = if rows.is_empty() { 0 } else { finished_users };

    let (sql, values) = Query::select()
        .from(InviteResponseIden::Table)
        .expr(as_integer(Func::count_distinct(Expr::col((
            InviteResponseIden::Table,
            InviteResponseIden::UserId,
        )))))
        .join(
            JoinType::LeftJoin,
            InviteIden::Table,
            Expr::col((InviteIden::Table, InviteIden::Id))
                .equals((InviteResponseIden::Table, InviteResponseIden::InviteId)),
        )
        .join(
            JoinType::LeftJoin,
            WorkflowIden::Table,
            Expr::col((WorkflowIden::Table, WorkflowIden::ConversationId))
                .equals((InviteIden::Table, InviteIden::ConversationId)),
        )
        .and_where(Expr::col((WorkflowIden::Table, WorkflowIden::Id)).eq(workflow_id))
        .and_where(Expr::col((InviteIden::Table, InviteIden::EventId)).is_null())
        .and_where(
            Expr::col((InviteResponseIden::Table, InviteResponseIden::Response))
                .eq(invite_response::Response::Accept),
        )
        .build_sqlx(PostgresQueryBuilder);
    let recruited_users: i32 = sqlx::query_scalar_with(&sql, values).fetch_one(db).await?;

    let (sql, values) = Query::select()
        .from(UserActivityIden::Table)
        .columns([
            (UserActivityIden::Table, UserActivityIden::UserId),
            (UserActivityIden::Table, UserActivityIden::WorkflowStepId),
            (UserActivityIden::Table, UserActivityIden::ActiveSeconds),
            (UserActivityIden::Table, UserActivityIden::ActivityChangedAt),
            (UserActivityIden::Table, UserActivityIden::IsActive),
            (UserActivityIden::Table, UserActivityIden::EndedAt),
        ])
        .column((UserProgressIden::Table, UserProgressIden::Status))
        .join(
            JoinType::InnerJoin,
            WorkflowStepIden::Table,
            Expr::col((UserActivityIden::Table, UserActivityIden::WorkflowStepId))
                .equals((WorkflowStepIden::Table, WorkflowStepIden::Id)),
        )
        .join(
            JoinType::InnerJoin,
            UserParticipationIden::Table,
            Cond::all()
                .add(
                    Expr::col((UserParticipationIden::Table, UserParticipationIden::UserId))
                        .equals((UserActivityIden::Table, UserActivityIden::UserId)),
                )
                .add(
                    Expr::col((
                        UserParticipationIden::Table,
                        UserParticipationIden::WorkflowId,
                    ))
                    .equals((WorkflowStepIden::Table, WorkflowStepIden::WorkflowId)),
                ),
        )
        .join(
            JoinType::LeftJoin,
            UserProgressIden::Table,
            Cond::all()
                .add(
                    Expr::col((UserProgressIden::Table, UserProgressIden::UserId))
                        .equals((UserActivityIden::Table, UserActivityIden::UserId)),
                )
                .add(
                    Expr::col((UserProgressIden::Table, UserProgressIden::WorkflowStepId))
                        .equals((UserActivityIden::Table, UserActivityIden::WorkflowStepId)),
                ),
        )
        .and_where(
            Expr::col((WorkflowStepIden::Table, WorkflowStepIden::WorkflowId)).eq(workflow_id),
        )
        .build_sqlx(PostgresQueryBuilder);
    let activities: Vec<WorkflowActivity> = sqlx::query_as_with(&sql, values).fetch_all(db).await?;
    let now = Utc::now();
    let mut active_users = HashSet::new();
    let mut total_active_seconds = 0;
    let mut completed_times: HashMap<Uuid, Vec<i64>> = HashMap::new();
    for activity in activities {
        let is_active = activity.is_active && activity.ended_at.is_none();
        let current_seconds = if is_active {
            active_users.insert(activity.user_id);
            (now - activity.activity_changed_at).num_seconds().max(0)
        } else {
            0
        };
        let active_seconds = i64::from(activity.active_seconds) + current_seconds;
        total_active_seconds += active_seconds;
        if activity.status == Some(ProgressStatus::Done) {
            completed_times
                .entry(activity.workflow_step_id)
                .or_default()
                .push(active_seconds);
        }
    }
    let step_stats = rows
        .into_iter()
        .map(|(id, completed, in_progress)| {
            let times = completed_times.entry(id).or_default();
            times.sort_unstable();
            let median_active_seconds = if times.is_empty() {
                None
            } else {
                Some((times[(times.len() - 1) / 2] as f64 + times[times.len() / 2] as f64) / 2.0)
            };
            WorkflowStepStats {
                id,
                completed,
                in_progress,
                median_active_seconds,
                max_active_seconds: times.last().copied(),
            }
        })
        .collect();
    let signup_stats = get_workflow_signup_stats(db, workflow_id).await?;

    Ok(WorkflowStats {
        step_stats,
        total_users,
        active_users: i32::try_from(active_users.len()).map_err(|error| {
            ComhairleError::BadRequest(format!("Too many active users: {error}"))
        })?,
        finished_users,
        recruited_users,
        total_active_seconds,
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
#[instrument(err(Debug), skip(db))]
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
        .values(values)?
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
    use std::error::Error;

    use super::*;
    use crate::models::model_test_helpers::setup_default_app_and_session;
    use crate::routes::conversations::dto::ConversationDto;
    use crate::routes::regions::dto::RegionDto;
    use crate::routes::workflows::dto::WorkflowDto;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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
