use std::sync::Arc;

use crate::bot_service::ComhairleBotService;
use crate::models::translations::{new_translation, TextContentId, TextFormat};
use crate::tools::{self, ToolConfigSanitize};
use crate::ComhairleState;
use chrono::{DateTime, Utc};
use comhairle_macros::{DbJsonBEnum, Translatable};
use partially::Partial;
use schemars::JsonSchema;
use sea_query::PostgresQueryBuilder;
use sea_query::{enum_def, Expr, Order, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use sqlx::{prelude::FromRow, PgPool};
use tracing::warn;
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
    #[partially(transparent)]
    pub tool_config: Option<ToolConfig>,
    pub preview_tool_config: ToolConfig,
    #[partially(omit)]
    pub created_at: DateTime<Utc>,
    #[partially(omit)]
    pub updated_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [WorkflowStepIden; 12] = [
    WorkflowStepIden::Id,
    WorkflowStepIden::Name,
    WorkflowStepIden::WorkflowId,
    WorkflowStepIden::StepOrder,
    WorkflowStepIden::ActivationRule,
    WorkflowStepIden::Description,
    WorkflowStepIden::IsOffline,
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
pub async fn launch(db: &PgPool, workflow_step_id: &Uuid) -> Result<(), ComhairleError> {
    let workflow_step = get_by_id(&db, workflow_step_id).await?;
    let new_live_config = match workflow_step.preview_tool_config {
        ToolConfig::Polis(preview_config) => ToolConfig::Polis(preview_config),
        ToolConfig::Learn(preview_config) => ToolConfig::Learn(preview_config.clone()),
        ToolConfig::HeyForm(preview_config) => {
            ToolConfig::HeyForm(tools::heyform::launch(&preview_config).await?)
        }
        ToolConfig::Stories(preview_config) => ToolConfig::Stories(preview_config.clone()),
        ToolConfig::ElicitationBot(elicitation_bot_tool_config) => {
            ToolConfig::ElicitationBot(elicitation_bot_tool_config.clone())
        }
    };

    update(
        &db,
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

        if let Some(value) = &self.tool_config {
            values.push((WorkflowStepIden::ToolConfig, value.into()))
        };

        if let Some(value) = &self.tool_config {
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
impl LocalisedWorkflowStep {
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
pub async fn get_localised_by_id(
    db: &PgPool,
    id: &Uuid,
    locale: &str,
) -> Result<LocalisedWorkflowStep, ComhairleError> {
    let select_query = Query::select()
        .columns(DEFAULT_COLUMNS.map(|col| (WorkflowStepIden::Table, col)))
        .from(WorkflowStepIden::Table)
        .and_where(Expr::col((WorkflowStepIden::Table, WorkflowStepIden::Id)).eq(id.to_owned()))
        .to_owned();

    let (sql, values) = LocalisedWorkflowStep::query_to_localisation(select_query, locale)
        .build_sqlx(PostgresQueryBuilder);

    let workflow_step = sqlx::query_as_with::<_, LocalisedWorkflowStep, _>(&sql, values)
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

    reset_orders(&mut *transaction, &deleted_step.workflow_id).await?;

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

    if values.len() == 0 {
        return Err(ComhairleError::NoValidUpdates);
    }

    let mut transaction = db.begin().await?;

    // If we are being asked to update the step_order
    // shift the existing number up one to accomodate
    // the new position of the step
    if let Some(target_order) = update.step_order {
        shift_steps_if_in_conflict(&mut *transaction, &workflow_id, target_order, false).await?;
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
        reset_orders(&mut *transaction, &workflow_id).await?
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

pub async fn list_localised(
    db: &PgPool,
    workflow_id: &Uuid,
    locale: &str,
) -> Result<Vec<LocalisedWorkflowStep>, ComhairleError> {
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

    let (sql, values) = LocalisedWorkflowStep::query_to_localisation(query, locale)
        .build_sqlx(PostgresQueryBuilder);

    let workflow_steps = sqlx::query_as_with::<_, LocalisedWorkflowStep, _>(&sql, values)
        .fetch_all(db)
        .await?;

    Ok(workflow_steps)
}

pub async fn setup_tool(
    state: &Arc<ComhairleState>,
    setup: &ToolSetup,
) -> Result<ToolConfig, ComhairleError> {
    let config = match &setup {
        ToolSetup::Polis(polis_tool_setup) => ToolConfig::Polis(
            tools::polis::setup(&polis_tool_setup)
                .await
                .map_err(|err| {
                    warn!("Polis error {err:#?}");
                    err
                })?,
        ),
        ToolSetup::Learn(learn_tool_setup) => {
            ToolConfig::Learn(tools::learn::setup(&learn_tool_setup).await?)
        }
        ToolSetup::HeyForm(hey_form_tool_setup) => {
            ToolConfig::HeyForm(tools::heyform::setup(&hey_form_tool_setup).await?)
        }
        ToolSetup::Stories(stories_tool_setup) => {
            ToolConfig::Stories(tools::stories::setup(&stories_tool_setup).await?)
        }
        ToolSetup::ElicitationBot(elicitation_bot_setup) => ToolConfig::ElicitationBot(
            tools::elicitation_bot::setup(&elicitation_bot_setup, &state.bot_service).await?,
        ),
    };
    Ok(config)
}

pub async fn create(
    state: &Arc<ComhairleState>,
    new_workflow_step: &CreateWorkflowStep,
    workflow_id: Uuid,
    primary_locale: &str,
) -> Result<WorkflowStep, ComhairleError> {
    // Generate Translations
    let name_translation = new_translation(
        &state.db,
        &primary_locale,
        &new_workflow_step.name,
        TextFormat::Plain,
    )
    .await?;

    let description_translation = new_translation(
        &state.db,
        &primary_locale,
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

    let tool_config = setup_tool(state, &new_workflow_step.tool_setup).await?;
    let preview_tool_config = setup_tool(state, &new_workflow_step.tool_setup).await?;

    columns.push(WorkflowStepIden::WorkflowId);
    values.push(workflow_id.into());

    columns.push(WorkflowStepIden::PreviewToolConfig);
    values.push(serde_json::to_value(preview_tool_config).unwrap().into());

    let mut transaction = state.db.begin().await?;

    // Check to see if there is already a
    // workflow set at this order no and if there
    // is make space for the new one

    shift_steps_if_in_conflict(
        &mut *transaction,
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
) -> Result<Option<LocalisedWorkflowStep>, ComhairleError> {
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
        LocalisedWorkflowStep::query_to_localisation(query, "en").build_sqlx(PostgresQueryBuilder);

    let result = sqlx::query_as_with::<_, LocalisedWorkflowStep, _>(&sql, values)
        .fetch_optional(db)
        .await?;

    Ok(result)
}
