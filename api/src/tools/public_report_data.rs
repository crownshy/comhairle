//! Shared guard for public report-data endpoints (ADR-0061).
//!
//! Each embeddable report section block gets its own unauthenticated endpoint that returns only
//! the aggregated, moderated fields that section block draws. Every one of them must go through
//! [`public_report_tool_config`] so a step's data is only ever served once its conversation's
//! report is published.

use sqlx::PgPool;
use uuid::Uuid;

use crate::error::ComhairleError;
use crate::models::{event, report, workflow, workflow_step};
use crate::tools::ToolConfig;

/// Load the tool config behind a public report-data request. `for_tool` narrows the config to the
/// endpoint's own tool, for example `|config| match config { ToolConfig::Polis(polis) =>
/// Some(polis), _ => None }`.
///
/// Returns `ResourceNotFound` when the step doesn't exist, its conversation has no published
/// report, or it runs a different tool, so an anonymous caller can't tell these apart.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "first caller is the Polis public endpoints (#1253)"
    )
)]
pub async fn public_report_tool_config<T>(
    db: &PgPool,
    workflow_step_id: &Uuid,
    for_tool: impl FnOnce(ToolConfig) -> Option<T>,
) -> Result<T, ComhairleError> {
    let not_found = || {
        ComhairleError::ResourceNotFound(format!(
            "Report data for workflow step {workflow_step_id}"
        ))
    };

    // Database failures pass through so an outage doesn't look like an unpublished report.
    let hide_which_is_missing = |err: ComhairleError| match err {
        ComhairleError::ResourceNotFound(_) => not_found(),
        other => other,
    };

    let step = workflow_step::get_by_id(db, workflow_step_id)
        .await
        .map_err(hide_which_is_missing)?;
    let conversation_id = conversation_of_workflow(db, &step.workflow_id)
        .await
        .map_err(hide_which_is_missing)?;
    let report = report::get_for_conversation(db, conversation_id)
        .await
        .map_err(hide_which_is_missing)?;

    if !report.is_public {
        return Err(not_found());
    }

    for_tool(step.tool_config.unwrap_or(step.preview_tool_config)).ok_or_else(not_found)
}

/// Event workflows have no `conversation_id` of their own, only through their event.
async fn conversation_of_workflow(db: &PgPool, workflow_id: &Uuid) -> Result<Uuid, ComhairleError> {
    let workflow = workflow::get_by_id(db, workflow_id).await?;
    if let Some(conversation_id) = workflow.conversation_id {
        return Ok(conversation_id);
    }
    let event_id = workflow
        .event_id
        .ok_or_else(|| ComhairleError::ResourceNotFound("Workflow conversation".into()))?;
    Ok(event::get_by_id(db, &event_id).await?.conversation_id)
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;
    use crate::models::model_test_helpers::{
        get_random_conversation_id, setup_default_app_and_session,
    };
    use crate::models::report::PartialReport;
    use crate::routes::workflow_steps::dto::WorkflowStepDto;
    use crate::routes::workflows::dto::WorkflowDto;
    use crate::tools::learn::LearnToolConfig;
    use crate::tools::polis::PolisToolConfig;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_only_serve_steps_once_the_report_is_published(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
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

        assert_not_found(
            public_report_tool_config(&pool, &step.id, learn).await,
            "step was served with no report",
        );

        report::create_for_conversation(&pool, conversation_id, "en").await?;
        assert_not_found(
            public_report_tool_config(&pool, &step.id, learn).await,
            "step was served from an unpublished report",
        );

        report::update(
            &pool,
            conversation_id,
            PartialReport {
                is_public: Some(true),
                conversation_id: None,
                body: None,
                section_configs: None,
            },
        )
        .await?;

        let config = public_report_tool_config(&pool, &step.id, learn).await?;
        assert_eq!(
            Some(ToolConfig::Learn(config)),
            step.tool_config.or(Some(step.preview_tool_config)),
            "wrong tool config served"
        );

        assert_not_found(
            public_report_tool_config(&pool, &step.id, polis).await,
            "step was served to another tool's endpoint",
        );

        report::update(
            &pool,
            conversation_id,
            PartialReport {
                is_public: Some(false),
                conversation_id: None,
                body: None,
                section_configs: None,
            },
        )
        .await?;
        assert_not_found(
            public_report_tool_config(&pool, &step.id, learn).await,
            "step was still served after the report was unpublished",
        );

        Ok(())
    }

    fn assert_not_found<T>(result: Result<T, ComhairleError>, message: &str) {
        assert!(
            matches!(result, Err(ComhairleError::ResourceNotFound(_))),
            "{message}"
        );
    }

    fn learn(config: ToolConfig) -> Option<LearnToolConfig> {
        match config {
            ToolConfig::Learn(learn) => Some(learn),
            _ => None,
        }
    }

    fn polis(config: ToolConfig) -> Option<PolisToolConfig> {
        match config {
            ToolConfig::Polis(polis) => Some(polis),
            _ => None,
        }
    }
}
