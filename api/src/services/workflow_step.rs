//! Workflow step orchestration.
//!
//! These functions coordinate a workflow step's lifecycle across the database and
//! the tool services, so they need [`ComhairleState`]. The persistence half of a
//! workflow step (rows, queries, `get_by_id`, `update`, `list`) stays in
//! [`crate::models::workflow_step`]; this module is the layer above it.

use std::sync::Arc;

use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use tracing::{instrument, warn};
use uuid::Uuid;

use crate::models::SqlxResultExt;
use crate::models::translations::{TextFormat, new_translation};
use crate::models::user_progress;
use crate::models::workflow_step::{
    CreateWorkflowStep, DEFAULT_COLUMNS, PartialWorkflowStep, WorkflowStep, WorkflowStepIden,
    get_by_id, reset_orders, shift_steps_if_in_conflict, update,
};
use crate::tools::{ToolConfig, ToolConfigExt, ToolSetup, ToolSetupExt};
use crate::{ComhairleState, error::ComhairleError};

/// Create the live version of this workflow step
#[instrument(err(Debug), skip(state))]
pub async fn launch(
    state: &Arc<ComhairleState>,
    workflow_step_id: &Uuid,
) -> Result<(), ComhairleError> {
    let workflow_step = get_by_id(&state.db, workflow_step_id).await?;
    // Use the new trait method for cloning the tool
    let new_live_config = workflow_step.preview_tool_config.clone_tool(state).await?;

    update(
        &state.db,
        workflow_step_id,
        &workflow_step.workflow_id,
        &PartialWorkflowStep {
            tool_config: Some(new_live_config.clone()),
            ..Default::default()
        },
    )
    .await?;

    // When a Polis poll goes live, seed the aux statement table from the new
    // live poll so moderation/theming has rows to work with immediately.
    if let ToolConfig::Polis(config) = &new_live_config {
        crate::tools::polis::sync_statement_aux_inner(state, workflow_step_id, config).await?;
    }

    Ok(())
}

/// Delete a workflow_step by ID, returning the deleted step
/// If the workflow step is live, returns an error and does not delete the step
#[instrument(err(Debug), skip(state))]
pub async fn delete(
    state: &Arc<ComhairleState>,
    id: &Uuid,
) -> Result<WorkflowStep, ComhairleError> {
    let workflow_step = get_by_id(&state.db, id).await?;

    if let Some(tool_config) = workflow_step.tool_config.as_ref() {
        tool_config.delete(state, id).await?;
    }

    if workflow_step
        .tool_config
        .as_ref()
        .map(|tool_config| tool_config != &workflow_step.preview_tool_config)
        .unwrap_or(true)
    {
        workflow_step.preview_tool_config.delete(state, id).await?;
    }

    let mut transaction = state.db.begin().await?;

    // Delete and return the workflow_step
    let (delete_sql, delete_values) = Query::delete()
        .from_table(WorkflowStepIden::Table)
        .and_where(Expr::col(WorkflowStepIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let deleted_step = sqlx::query_as_with::<_, WorkflowStep, _>(&delete_sql, delete_values)
        .fetch_one(&mut *transaction)
        .await
        .resolve_db_err("Workflow Step")?;

    reset_orders(&mut transaction, &deleted_step.workflow_id).await?;

    transaction.commit().await?;
    Ok(deleted_step)
}

pub async fn setup_tool(
    setup: &ToolSetup,
    state: &Arc<ComhairleState>,
    locale: &str,
) -> Result<ToolConfig, ComhairleError> {
    // Use the new trait method for setup
    setup.setup(state, locale).await.map_err(|err| {
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
    is_live: bool,
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

    let preview_tool_config =
        setup_tool(&new_workflow_step.tool_setup, state, primary_locale).await?;

    columns.push(WorkflowStepIden::WorkflowId);
    values.push(workflow_id.into());

    columns.push(WorkflowStepIden::PreviewToolConfig);
    values.push(serde_json::to_value(&preview_tool_config)?.into());

    if is_live {
        columns.push(WorkflowStepIden::ToolConfig);
        values.push(serde_json::to_value(&preview_tool_config)?.into());
    }

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

    // Backfill user_progress for users already registered on this workflow,
    // so adding a step after registration doesn't leave them without rows
    // and unable to advance.
    user_progress::create_for_workflow_participants(
        &mut transaction,
        &workflow_step_result.id,
        &workflow_id,
    )
    .await?;

    transaction.commit().await?;

    Ok(workflow_step_result)
}
