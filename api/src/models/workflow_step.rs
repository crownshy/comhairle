use std::sync::Arc;

use crate::models::translations::{new_translation, TextContentId, TextFormat};
use crate::models::users::UserIden;
use crate::tools::ToolConfigSanitize;
use crate::ComhairleState;
use chrono::{DateTime, Utc};
use comhairle_macros::{DbJsonBEnum, Translatable};
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{enum_def, Expr, Order, Query};
use sea_query::{JoinType, PostgresQueryBuilder};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use sqlx::{prelude::FromRow, PgPool};
use tracing::{instrument, warn};
use uuid::Uuid;

use crate::error::ComhairleError;
use crate::models::user_progress::{ProgressStatus, UserProgressIden};
use crate::tools::{ToolConfig, ToolSetup};

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
    #[partially(omit)]
    pub created_at: DateTime<Utc>,
    #[partially(omit)]
    pub updated_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [WorkflowStepIden; 13] = [
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
    WorkflowStepIden::CreatedAt,
    WorkflowStepIden::UpdatedAt,
];

/// Will renormalize the step orders as part of a wider transaction
/// So for example [ 3, 4 , 5, 30] will become [1,2,3,4]
async fn reset_orders(pool: &mut PgConnection, workflow_id: &Uuid) -> Result<(), ComhairleError> {
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

/// Create the live version of this workflow step
pub async fn launch(
    db: &PgPool,
    workflow_step_id: &Uuid,
    state: &Arc<ComhairleState>,
) -> Result<(), ComhairleError> {
    let workflow_step = get_by_id(db, workflow_step_id).await?;
    // Use the new trait method for cloning the tool
    let new_live_config = workflow_step.preview_tool_config.clone_tool(state).await?;

    update(
        db,
        workflow_step_id,
        &workflow_step.workflow_id,
        &PartialWorkflowStep {
            tool_config: Some(new_live_config),
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

/// Shift if
async fn shift_steps_if_in_conflict(
    transaction: &mut PgConnection,
    workflow_id: &Uuid,
    target_step_order: i32,
    shift_up: bool,
) -> Result<(), ComhairleError> {
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
        values
    }
}
impl LocalizedWorkflowStep {
    pub fn sanatize(&mut self) {
        self.preview_tool_config = self.preview_tool_config.sanatize();
        self.tool_config = self.tool_config.clone().map(|s| s.sanatize());
    }
}
impl WorkflowStep {
    pub fn sanatize(&mut self) {
        self.preview_tool_config = self.preview_tool_config.sanatize();
        self.tool_config = self.tool_config.clone().map(|s| s.sanatize());
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
pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<WorkflowStep, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(WorkflowStepIden::Table)
        .and_where(Expr::col(WorkflowStepIden::Id).eq(id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let workflow_step = sqlx::query_as_with::<_, WorkflowStep, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("WorkflowStep".into()))?;

    Ok(workflow_step)
}

/// Get a workflow_step by ID (localized)
#[instrument(err(Debug))]
pub async fn get_localised_by_id(
    db: &PgPool,
    id: &Uuid,
    locale: &str,
) -> Result<LocalizedWorkflowStep, ComhairleError> {
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
        .map_err(|_| ComhairleError::ResourceNotFound("WorkflowStep".into()))?;

    Ok(workflow_step)
}

pub async fn delete(db: &PgPool, id: &Uuid) -> Result<WorkflowStep, ComhairleError> {
    let mut transaction = db.begin().await?;

    // Delete and return the workflow_step
    let (delete_sql, delete_values) = Query::delete()
        .from_table(WorkflowStepIden::Table)
        .and_where(Expr::col(WorkflowStepIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let deleted_step = sqlx::query_as_with::<_, WorkflowStep, _>(&delete_sql, delete_values)
        .fetch_one(&mut *transaction)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("workflow_step".into()))?;

    reset_orders(&mut transaction, &deleted_step.workflow_id).await?;

    transaction.commit().await?;
    Ok(deleted_step)
}

pub async fn update(
    db: &PgPool,
    workflow_step_id: &Uuid,
    workflow_id: &Uuid,
    update: &PartialWorkflowStep,
) -> Result<WorkflowStep, ComhairleError> {
    let values = update.to_values();

    if values.is_empty() {
        return Err(ComhairleError::NoValidUpdates);
    }

    let mut transaction = db.begin().await?;

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

pub async fn list(db: &PgPool, workflow_id: &Uuid) -> Result<Vec<WorkflowStep>, ComhairleError> {
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

#[instrument(err(Debug))]
pub async fn list_localized(
    db: &PgPool,
    workflow_id: &Uuid,
    locale: &str,
) -> Result<Vec<LocalizedWorkflowStep>, ComhairleError> {
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
    step: LocalizedWorkflowStep,
    status: ProgressStatus,
}

#[instrument(err(Debug))]
pub async fn list_localized_with_progress(
    db: &PgPool,
    workflow_id: &Uuid,
    locale: &str,
    user_id: &Uuid,
) -> Result<Vec<LocalizedWorkflowStepWithProgress>, ComhairleError> {
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

pub async fn list_with_translations(
    db: &PgPool,
    workflow_id: &Uuid,
    locale: &str,
) -> Result<Vec<WorkflowStepWithTranslations>, ComhairleError> {
    let workflow_steps = list(db, workflow_id).await?;
    let mut steps_with_translations = Vec::new();
    for step in workflow_steps {
        let step_with_trans = WorkflowStepWithTranslations::from_original(db, step, locale).await?;
        steps_with_translations.push(step_with_trans);
    }
    Ok(steps_with_translations)
}

pub async fn setup_tool(
    setup: &ToolSetup,
    state: &Arc<ComhairleState>,
) -> Result<ToolConfig, ComhairleError> {
    // Use the new trait method for setup
    setup.setup(state).await.map_err(|err| {
        warn!("Tool setup error {err:#?}");
        err
    })
}

#[instrument(err(Debug), skip(state))]
pub async fn create(
    state: &Arc<ComhairleState>,
    new_workflow_step: &CreateWorkflowStep,
    workflow_id: Uuid,
    primary_locale: &str,
) -> Result<WorkflowStep, ComhairleError> {
    // Generate Translations
    let name_translation = new_translation(
        &state.db,
        primary_locale,
        &new_workflow_step.name,
        TextFormat::Plain,
    )
    .await?;

    let description_translation = new_translation(
        &state.db,
        primary_locale,
        &new_workflow_step.description,
        TextFormat::Rich,
    )
    .await?;

    let mut columns = new_workflow_step.columns();
    let mut values = new_workflow_step.values();

    columns.push(WorkflowStepIden::Name);
    values.push(name_translation.id.into());

    columns.push(WorkflowStepIden::Description);
    values.push(description_translation.id.into());

    let preview_tool_config = setup_tool(&new_workflow_step.tool_setup, state).await?;

    columns.push(WorkflowStepIden::WorkflowId);
    values.push(workflow_id.into());

    columns.push(WorkflowStepIden::PreviewToolConfig);
    values.push(serde_json::to_value(preview_tool_config).unwrap().into());

    let mut transaction = state.db.begin().await?;

    // Check to see if there is already a
    // workflow set at this order no and if there
    // is make space for the new one

    shift_steps_if_in_conflict(
        &mut transaction,
        &workflow_id,
        new_workflow_step.step_order,
        true,
    )
    .await?;

    // Query to then insert the workflow step in the gap
    let (sql, values) = Query::insert()
        .into_table(WorkflowStepIden::Table)
        .columns(columns)
        .values(values)
        .unwrap()
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let workflow_step_result = sqlx::query_as_with::<_, WorkflowStep, _>(&sql, values)
        .fetch_one(&mut *transaction)
        .await?;

    transaction.commit().await?;

    Ok(workflow_step_result)
}

pub async fn get_current_active_step_for_user(
    db: &PgPool,
    user_id: &Uuid,
    workflow_id: &Uuid,
) -> Result<Option<WorkflowStep>, ComhairleError> {
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

pub async fn get_current_active_step_for_user_localised(
    db: &PgPool,
    user_id: &Uuid,
    workflow_id: &Uuid,
) -> Result<Option<LocalizedWorkflowStep>, ComhairleError> {
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

#[cfg(test)]
mod tests {
    use crate::{
        models::{
            self,
            model_test_helpers::{get_random_conversation_id, setup_default_app_and_session},
            users::User,
            workflow,
        },
        routes::{
            auth::SignupRequest, workflow_steps::dto::WorkflowStepDto, workflows::dto::WorkflowDto,
        },
    };

    use super::*;
    use std::error::Error;

    #[sqlx::test]
    async fn should_update_can_revisit_field(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let (_, workflow_res, _) = session
            .create_random_workflow(&app, &conversation_id.to_string())
            .await?;
        let workflow: WorkflowDto = serde_json::from_value(workflow_res)?;
        let steps_res = session
            .create_random_workflow_steps(
                &app,
                &conversation_id.to_string(),
                &workflow.id.to_string(),
                1,
            )
            .await?;
        let step: WorkflowStepDto = serde_json::from_value(steps_res.first().unwrap().to_owned())?;

        assert!(!step.can_revisit, "incorrect can_revisit before update");

        let step = update(
            &pool,
            &step.id,
            &workflow.id,
            &PartialWorkflowStep {
                can_revisit: Some(true),
                ..Default::default()
            },
        )
        .await?;

        assert!(step.can_revisit, "incorrect can_revisit after update");

        Ok(())
    }

    #[sqlx::test]
    async fn should_list_localized_steps(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let (_, workflow_res, _) = session
            .create_random_workflow(&app, &conversation_id.to_string())
            .await?;
        let workflow: WorkflowDto = serde_json::from_value(workflow_res)?;
        let _ = session
            .create_random_workflow_steps(
                &app,
                &conversation_id.to_string(),
                &workflow.id.to_string(),
                5,
            )
            .await?;

        let steps = list_localized(&pool, &workflow.id, "en").await?;

        assert_eq!(steps.len(), 5, "incorrect number of steps");

        Ok(())
    }

    #[sqlx::test]
    async fn should_list_localized_steps_with_user_progress(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let (_, workflow_res, _) = session
            .create_random_workflow(&app, &conversation_id.to_string())
            .await?;
        let workflow: WorkflowDto = serde_json::from_value(workflow_res)?;
        let steps = session
            .create_random_workflow_steps(
                &app,
                &conversation_id.to_string(),
                &workflow.id.to_string(),
                5,
            )
            .await?;
        let first_step: WorkflowStepDto = serde_json::from_value(steps.first().unwrap().clone())?;
        let second_step: WorkflowStepDto = serde_json::from_value(steps.get(1).unwrap().clone())?;
        let user = models::users::create_user(
            &SignupRequest {
                username: "test_user".to_string(),
                password: "test_pw".to_string(),
                avatar_url: None,
                email: "test_email".to_string(),
            },
            &pool,
        )
        .await?;
        workflow::register_user(&pool, &workflow.id, &user).await?;

        models::user_progress::update(&pool, &user.id, &first_step.id, ProgressStatus::Done)
            .await?;
        models::user_progress::update(&pool, &user.id, &second_step.id, ProgressStatus::InProgress)
            .await?;

        let steps = list_localized_with_progress(&pool, &workflow.id, "en", &user.id).await?;
        assert_eq!(steps[0].status, ProgressStatus::Done, "incorrect first step progess status");
        assert_eq!(steps[1].status, ProgressStatus::InProgress, "incorrect second step progess status");
        assert_eq!(steps[2].status, ProgressStatus::NotStarted, "incorrect third step progess status");
        assert_eq!(steps[3].status, ProgressStatus::NotStarted, "incorrect fourth step progess status");
        assert_eq!(steps[4].status, ProgressStatus::NotStarted, "incorrect fifth step progess status");

        Ok(())
    }
}
