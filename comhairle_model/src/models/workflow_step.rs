use crate::models::error::ModelError;
use crate::models::error::ValidationError;
use crate::models::tools::ToolConfigSanitize;
use crate::models::translations::TextContentId;
use crate::models::users::UserIden;
use chrono::{DateTime, Utc};
use comhairle_macros::{DbJsonBEnum, Translatable};
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{Expr, Order, Query, enum_def};
use sea_query::{JoinType, PostgresQueryBuilder};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use sqlx::{PgPool, prelude::FromRow};
use tracing::instrument;
use uuid::Uuid;

use crate::models::tools::{ToolConfig, ToolSetup};
use crate::models::{
    SqlxResultExt, moderation_policy,
    user_progress::{ProgressStatus, UserProgressIden},
};

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, DbJsonBEnum, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ActivationRule {
    Manual,
}

#[derive(Partial, Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema, Translatable)]
#[enum_def(table_name = "workflow_step")]
#[partially(derive(Deserialize, Debug, JsonSchema, Default))]
pub struct WorkflowStep {
    #[partially(omit)]
    pub id: Uuid,
    #[partially(omit)]
    pub workflow_id: Uuid,
    pub name: TextContentId,
    pub step_order: i32,
    pub activation_rule: ActivationRule,
    pub description: TextContentId,
    pub is_offline: bool,
    pub required: bool,
    pub can_revisit: bool,
    #[partially(transparent)]
    pub tool_config: Option<ToolConfig>,
    pub preview_tool_config: ToolConfig,
    pub request_user_share_permission: bool,
    #[partially(omit)]
    pub created_at: DateTime<Utc>,
    #[partially(omit)]
    pub updated_at: DateTime<Utc>,
}

/// Helper trait to simplify working with workflow_step tool_configs across
/// methods and endpoints that return differing types of workflow_steps.
pub trait WithToolConfig {
    fn tool_config(&self) -> Option<&ToolConfig>;
    fn preview_tool_config(&self) -> &ToolConfig;
}

impl WithToolConfig for WorkflowStep {
    fn tool_config(&self) -> Option<&ToolConfig> {
        self.tool_config.as_ref()
    }
    fn preview_tool_config(&self) -> &ToolConfig {
        &self.preview_tool_config
    }
}

impl WithToolConfig for LocalizedWorkflowStep {
    fn tool_config(&self) -> Option<&ToolConfig> {
        self.tool_config.as_ref()
    }
    fn preview_tool_config(&self) -> &ToolConfig {
        &self.preview_tool_config
    }
}

impl WithToolConfig for WorkflowStepWithTranslations {
    fn tool_config(&self) -> Option<&ToolConfig> {
        self.tool_config.as_ref()
    }
    fn preview_tool_config(&self) -> &ToolConfig {
        &self.preview_tool_config
    }
}

impl WithToolConfig for LocalizedWorkflowStepWithProgress {
    fn tool_config(&self) -> Option<&ToolConfig> {
        self.step.tool_config.as_ref()
    }
    fn preview_tool_config(&self) -> &ToolConfig {
        &self.step.preview_tool_config
    }
}

pub const DEFAULT_COLUMNS: [WorkflowStepIden; 14] = [
    WorkflowStepIden::Id,
    WorkflowStepIden::Name,
    WorkflowStepIden::WorkflowId,
    WorkflowStepIden::StepOrder,
    WorkflowStepIden::ActivationRule,
    WorkflowStepIden::Description,
    WorkflowStepIden::IsOffline,
    WorkflowStepIden::CanRevisit,
    WorkflowStepIden::ToolConfig,
    WorkflowStepIden::PreviewToolConfig,
    WorkflowStepIden::Required,
    WorkflowStepIden::RequestUserSharePermission,
    WorkflowStepIden::CreatedAt,
    WorkflowStepIden::UpdatedAt,
];

/// Will renormalize the step orders as part of a wider transaction
/// So for example [ 3, 4 , 5, 30] will become [1,2,3,4]
#[instrument(err(Debug), skip(pool))]
pub async fn reset_orders(pool: &mut PgConnection, workflow_id: &Uuid) -> Result<(), ModelError> {
    sqlx::query(
        "
            UPDATE workflow_step  SET step_order = new_step_order FROM (
                SELECT id, row_number()  OVER (PARTITION BY workflow_id order by step_order) as new_step_order
                from workflow_step 
                where workflow_id= $1 
            ) as ranked
            where workflow_step.id =ranked.id and workflow_id = $1 
        ",
    )
    .bind(workflow_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Shift if
#[instrument(err(Debug), skip(transaction))]
pub async fn shift_steps_if_in_conflict(
    transaction: &mut PgConnection,
    workflow_id: &Uuid,
    target_step_order: i32,
    shift_up: bool,
) -> Result<(), ModelError> {
    let (existing_step_sql, existing_step_values) = Query::select()
        .expr(Expr::cust("COUNT(*)::INT as count"))
        .from(WorkflowStepIden::Table)
        .and_where(Expr::col(WorkflowStepIden::WorkflowId).eq(*workflow_id))
        .and_where(Expr::col(WorkflowStepIden::StepOrder).eq(target_step_order))
        .build_sqlx(PostgresQueryBuilder);

    let count: i32 = sqlx::query_scalar_with(&existing_step_sql, existing_step_values)
        .fetch_one(&mut *transaction)
        .await?;

    if count == 1 {
        // Shift all workflow steps on this workflow
        // that have less than or equal order to this one
        // by -1

        // This is required so we can update the
        // order numbers all at once. Otherwise
        // the constraint gets hit and we error
        sqlx::query("SET CONSTRAINTS ALL DEFERRED")
            .execute(&mut *transaction)
            .await?;

        let order_select = match shift_up {
            true => Expr::col(WorkflowStepIden::StepOrder).gte(target_step_order),
            false => Expr::col(WorkflowStepIden::StepOrder).lte(target_step_order),
        };

        let shift_value = match shift_up {
            true => Expr::col(WorkflowStepIden::StepOrder).add(1),
            false => Expr::col(WorkflowStepIden::StepOrder).sub(1),
        };

        let (shift_sql, shift_values) = Query::update()
            .table(WorkflowStepIden::Table)
            .value(WorkflowStepIden::StepOrder, shift_value)
            .and_where(Expr::col(WorkflowStepIden::WorkflowId).eq(*workflow_id))
            .and_where(order_select)
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&shift_sql, shift_values)
            .execute(&mut *transaction)
            .await?;
    }
    Ok(())
}

impl PartialWorkflowStep {
    pub fn to_values(&self) -> Vec<(WorkflowStepIden, sea_query::SimpleExpr)> {
        let mut values = vec![];
        if let Some(value) = &self.name {
            values.push((WorkflowStepIden::Name, value.into()))
        };
        if let Some(value) = &self.description {
            values.push((WorkflowStepIden::Description, value.into()))
        };
        if let Some(value) = &self.activation_rule {
            values.push((WorkflowStepIden::ActivationRule, value.into()))
        };
        if let Some(value) = &self.can_revisit {
            values.push((WorkflowStepIden::CanRevisit, (*value).into()))
        };
        if let Some(value) = &self.tool_config {
            values.push((WorkflowStepIden::ToolConfig, value.into()))
        };
        if let Some(value) = &self.preview_tool_config {
            values.push((WorkflowStepIden::PreviewToolConfig, value.into()))
        };
        if let Some(value) = self.step_order {
            values.push((WorkflowStepIden::StepOrder, value.into()))
        };
        if let Some(value) = self.is_offline {
            values.push((WorkflowStepIden::IsOffline, value.into()))
        };
        if let Some(value) = self.required {
            values.push((WorkflowStepIden::Required, value.into()))
        };
        if let Some(value) = self.request_user_share_permission {
            values.push((WorkflowStepIden::RequestUserSharePermission, value.into()))
        };
        values
    }
}
impl LocalizedWorkflowStep {
    pub fn sanatize(&mut self) {
        self.preview_tool_config = self.preview_tool_config.sanitize();
        self.tool_config = self.tool_config.clone().map(|s| s.sanitize());
    }
}
impl WorkflowStep {
    pub fn sanatize(&mut self) {
        self.preview_tool_config = self.preview_tool_config.sanitize();
        self.tool_config = self.tool_config.clone().map(|s| s.sanitize());
    }
}

#[derive(Partial, Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct CreateWorkflowStep {
    pub name: String,
    pub step_order: i32,
    pub activation_rule: ActivationRule,
    pub description: String,
    pub is_offline: bool,
    pub tool_setup: ToolSetup,
    pub required: bool,
}

impl CreateWorkflowStep {
    pub fn columns(&self) -> Vec<WorkflowStepIden> {
        vec![
            WorkflowStepIden::StepOrder,
            WorkflowStepIden::ActivationRule,
            WorkflowStepIden::IsOffline,
            WorkflowStepIden::Required,
        ]
    }

    pub fn values(&self) -> Vec<sea_query::SimpleExpr> {
        vec![
            self.step_order.into(),
            serde_json::to_value(self.activation_rule.clone())
                .unwrap()
                .into(),
            self.is_offline.into(),
            self.required.into(),
        ]
    }
}

/// Get a workflow_step by ID (original struct, not localized)
#[instrument(err(Debug), skip(db))]
pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<WorkflowStep, ModelError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(WorkflowStepIden::Table)
        .and_where(Expr::col(WorkflowStepIden::Id).eq(id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let workflow_step = sqlx::query_as_with::<_, WorkflowStep, _>(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Workflow Step")?;

    Ok(workflow_step)
}

/// Get a workflow_step by ID (localized)
#[instrument(err(Debug), skip(db))]
pub async fn get_localised_by_id(
    db: &PgPool,
    id: &Uuid,
    locale: &str,
) -> Result<LocalizedWorkflowStep, ModelError> {
    let select_query = Query::select()
        .columns(DEFAULT_COLUMNS.map(|col| (WorkflowStepIden::Table, col)))
        .from(WorkflowStepIden::Table)
        .and_where(Expr::col((WorkflowStepIden::Table, WorkflowStepIden::Id)).eq(id.to_owned()))
        .to_owned();

    let (sql, values) = LocalizedWorkflowStep::query_to_localisation(select_query, locale)
        .build_sqlx(PostgresQueryBuilder);

    let workflow_step = sqlx::query_as_with::<_, LocalizedWorkflowStep, _>(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Workflow Step")?;

    Ok(workflow_step)
}

#[instrument(err(Debug), skip(db))]
pub async fn update(
    db: &PgPool,
    workflow_step_id: &Uuid,
    workflow_id: &Uuid,
    update: &PartialWorkflowStep,
) -> Result<WorkflowStep, ModelError> {
    let values = update.to_values();

    if values.is_empty() {
        return Err(ValidationError::NoValidUpdates.into());
    }

    let mut transaction = db.begin().await?;

    // Held until the step is saved, so a policy delete can't land between check and write.
    let policy_ids = [
        update.tool_config.as_ref(),
        update.preview_tool_config.as_ref(),
    ]
    .into_iter()
    .flatten()
    .filter_map(ToolConfig::moderation_policy_id);
    for policy_id in policy_ids {
        moderation_policy::lock_for_step(&mut transaction, *workflow_step_id, policy_id).await?;
    }

    // If we are being asked to update the step_order
    // shift the existing number up one to accomodate
    // the new position of the step
    if let Some(target_order) = update.step_order {
        shift_steps_if_in_conflict(&mut transaction, workflow_id, target_order, false).await?;
    }

    // Check to see if there is already a
    // workflow set at this order no

    let (sql, values) = Query::update()
        .table(WorkflowStepIden::Table)
        .values(values)
        .and_where(Expr::col(WorkflowStepIden::Id).eq(workflow_step_id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let workflow = sqlx::query_as_with::<_, WorkflowStep, _>(&sql, values)
        .fetch_one(&mut *transaction)
        .await?;

    // Reset the orders to plug the gap if needed

    if update.step_order.is_some() {
        reset_orders(&mut transaction, workflow_id).await?
    }

    transaction.commit().await?;
    Ok(workflow)
}

#[instrument(err(Debug), skip(db))]
pub async fn list(db: &PgPool, workflow_id: &Uuid) -> Result<Vec<WorkflowStep>, ModelError> {
    let query = Query::select()
        .from(WorkflowStepIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(Expr::col(WorkflowStepIden::WorkflowId).eq(*workflow_id))
        .order_by(WorkflowStepIden::StepOrder, Order::Asc)
        .to_owned();

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let workflow_steps = sqlx::query_as_with::<_, WorkflowStep, _>(&sql, values)
        .fetch_all(db)
        .await?;

    Ok(workflow_steps)
}

#[instrument(err(Debug), skip(db))]
pub async fn list_localized(
    db: &PgPool,
    workflow_id: &Uuid,
    locale: &str,
) -> Result<Vec<LocalizedWorkflowStep>, ModelError> {
    let query = Query::select()
        .from(WorkflowStepIden::Table)
        .columns(DEFAULT_COLUMNS.map(|col| (WorkflowStepIden::Table, col)))
        .and_where(
            Expr::col((WorkflowStepIden::Table, WorkflowStepIden::WorkflowId)).eq(*workflow_id),
        )
        .order_by(
            (WorkflowStepIden::Table, WorkflowStepIden::StepOrder),
            Order::Asc,
        )
        .to_owned();

    let (sql, values) = LocalizedWorkflowStep::query_to_localisation(query, locale)
        .build_sqlx(PostgresQueryBuilder);

    let workflow_steps = sqlx::query_as_with::<_, LocalizedWorkflowStep, _>(&sql, values)
        .fetch_all(db)
        .await?;

    Ok(workflow_steps)
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, FromRow)]
pub struct LocalizedWorkflowStepWithProgress {
    #[sqlx(flatten)]
    #[serde(flatten)]
    pub step: LocalizedWorkflowStep,
    pub status: ProgressStatus,
}

#[instrument(err(Debug), skip(db))]
pub async fn list_localized_with_progress(
    db: &PgPool,
    workflow_id: &Uuid,
    locale: &str,
    user_id: &Uuid,
) -> Result<Vec<LocalizedWorkflowStepWithProgress>, ModelError> {
    let query = Query::select()
        .from(WorkflowStepIden::Table)
        .columns(DEFAULT_COLUMNS.map(|col| (WorkflowStepIden::Table, col)))
        .column((UserProgressIden::Table, UserProgressIden::Status))
        .join(
            JoinType::InnerJoin,
            UserProgressIden::Table,
            Expr::col((WorkflowStepIden::Table, WorkflowStepIden::Id))
                .equals((UserProgressIden::Table, UserProgressIden::WorkflowStepId)),
        )
        .join(
            JoinType::InnerJoin,
            UserIden::Table,
            Expr::col((UserIden::Table, UserIden::Id))
                .equals((UserProgressIden::Table, UserProgressIden::UserId)),
        )
        .and_where(
            Expr::col((WorkflowStepIden::Table, WorkflowStepIden::WorkflowId)).eq(*workflow_id),
        )
        .and_where(Expr::col((UserIden::Table, UserIden::Id)).eq(*user_id))
        .order_by(
            (WorkflowStepIden::Table, WorkflowStepIden::StepOrder),
            Order::Asc,
        )
        .to_owned();

    let (sql, values) = LocalizedWorkflowStep::query_to_localisation(query, locale)
        .build_sqlx(PostgresQueryBuilder);

    let workflow_steps = sqlx::query_as_with(&sql, values).fetch_all(db).await?;

    Ok(workflow_steps)
}

#[instrument(err(Debug), skip(db))]
pub async fn list_with_translations(
    db: &PgPool,
    workflow_id: &Uuid,
    locale: &str,
) -> Result<Vec<WorkflowStepWithTranslations>, ModelError> {
    let workflow_steps = list(db, workflow_id).await?;
    let mut steps_with_translations = Vec::new();
    for step in workflow_steps {
        let step_with_trans = WorkflowStepWithTranslations::from_original(db, step, locale).await?;
        steps_with_translations.push(step_with_trans);
    }
    Ok(steps_with_translations)
}

/// How many steps does this workflow have?
///
/// Used by the seal to tell "finished every step" apart from "there are no steps", which
/// otherwise look identical to `get_current_active_step_for_user`.
#[instrument(err(Debug), skip(db))]
pub async fn count_for_workflow(db: &PgPool, workflow_id: &Uuid) -> Result<i64, ModelError> {
    let (sql, values) = Query::select()
        .expr(Expr::col((WorkflowStepIden::Table, WorkflowStepIden::Id)).count())
        .from(WorkflowStepIden::Table)
        .and_where(
            Expr::col((WorkflowStepIden::Table, WorkflowStepIden::WorkflowId)).eq(*workflow_id),
        )
        .build_sqlx(PostgresQueryBuilder);

    let (count,): (i64,) = sqlx::query_as_with(&sql, values).fetch_one(db).await?;

    Ok(count)
}

#[instrument(err(Debug), skip(db))]
pub async fn get_current_active_step_for_user(
    db: &PgPool,
    user_id: &Uuid,
    workflow_id: &Uuid,
) -> Result<Option<WorkflowStep>, ModelError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS.map(|col| (WorkflowStepIden::Table, col)))
        .from(WorkflowStepIden::Table)
        .left_join(
            UserProgressIden::Table,
            Expr::col((WorkflowStepIden::Table, WorkflowStepIden::Id))
                .equals((UserProgressIden::Table, UserProgressIden::WorkflowStepId))
                .and(Expr::col((UserProgressIden::Table, UserProgressIden::UserId)).eq(*user_id)),
        )
        .and_where(
            Expr::col((WorkflowStepIden::Table, WorkflowStepIden::WorkflowId)).eq(*workflow_id),
        )
        .and_where(
            Expr::col((UserProgressIden::Table, UserProgressIden::Status))
                .ne(ProgressStatus::Done)
                .or(Expr::col((UserProgressIden::Table, UserProgressIden::Status)).is_null()),
        )
        .order_by(
            (WorkflowStepIden::Table, WorkflowStepIden::StepOrder),
            sea_query::Order::Asc,
        )
        .limit(1)
        .build_sqlx(PostgresQueryBuilder);

    let result = sqlx::query_as_with::<_, WorkflowStep, _>(&sql, values)
        .fetch_optional(db)
        .await?;

    Ok(result)
}

#[instrument(err(Debug), skip(db))]
pub async fn get_current_active_step_for_user_localised(
    db: &PgPool,
    user_id: &Uuid,
    workflow_id: &Uuid,
) -> Result<Option<LocalizedWorkflowStep>, ModelError> {
    let query = Query::select()
        .columns(DEFAULT_COLUMNS.map(|col| (WorkflowStepIden::Table, col)))
        .from(WorkflowStepIden::Table)
        .left_join(
            UserProgressIden::Table,
            Expr::col((WorkflowStepIden::Table, WorkflowStepIden::Id))
                .equals((UserProgressIden::Table, UserProgressIden::WorkflowStepId))
                .and(Expr::col((UserProgressIden::Table, UserProgressIden::UserId)).eq(*user_id)),
        )
        .and_where(
            Expr::col((WorkflowStepIden::Table, WorkflowStepIden::WorkflowId)).eq(*workflow_id),
        )
        .and_where(
            Expr::col((UserProgressIden::Table, UserProgressIden::Status))
                .ne(ProgressStatus::Done)
                .or(Expr::col((UserProgressIden::Table, UserProgressIden::Status)).is_null()),
        )
        .order_by(
            (WorkflowStepIden::Table, WorkflowStepIden::StepOrder),
            sea_query::Order::Asc,
        )
        .limit(1)
        .to_owned();

    let (sql, values) =
        LocalizedWorkflowStep::query_to_localisation(query, "en").build_sqlx(PostgresQueryBuilder);

    let result = sqlx::query_as_with::<_, LocalizedWorkflowStep, _>(&sql, values)
        .fetch_optional(db)
        .await?;

    Ok(result)
}
