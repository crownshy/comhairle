//! Shared guard for public report-data endpoints (ADR-0061).
//!
//! Each embeddable report section block gets its own unauthenticated endpoint that returns only the
//! aggregated, moderated fields that section block draws. Every one of them must go through
//! [`public_report_tool_config`] so a step's data is only ever served after an admin has opted
//! it in with `report_data_public`.

use sqlx::PgPool;
use uuid::Uuid;

use crate::error::ComhairleError;
use crate::models::workflow_step;
use crate::tools::ToolConfig;

/// Load the tool config behind a public report-data request. `for_tool` narrows the config to the
/// endpoint's own tool, for example `|config| match config { ToolConfig::Polis(polis) =>
/// Some(polis), _ => None }`.
///
/// Returns `ResourceNotFound` when the step doesn't exist, hasn't opted in, or runs a different
/// tool, so an anonymous caller can't tell these apart. Falls back to the preview config before
/// launch, so a facilitator can build the report ahead of time.
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

    let step = workflow_step::get_by_id(db, workflow_step_id).await?;

    if !step.report_data_public {
        return Err(not_found());
    }

    for_tool(step.tool_config.unwrap_or(step.preview_tool_config)).ok_or_else(not_found)
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;
    use crate::models::model_test_helpers::{
        get_random_conversation_id, setup_default_app_and_session,
    };
    use crate::models::workflow_step::{self, PartialWorkflowStep};
    use crate::routes::workflow_steps::dto::WorkflowStepDto;
    use crate::routes::workflows::dto::WorkflowDto;
    use crate::tools::learn::LearnToolConfig;
    use crate::tools::polis::PolisToolConfig;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_only_serve_steps_that_opted_in(pool: PgPool) -> Result<(), Box<dyn Error>> {
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

        assert!(!step.report_data_public, "steps must default to private");
        assert!(
            matches!(
                public_report_tool_config(&pool, &step.id, learn).await,
                Err(ComhairleError::ResourceNotFound(_))
            ),
            "private step was served"
        );

        workflow_step::update(
            &pool,
            &step.id,
            &workflow.id,
            &PartialWorkflowStep {
                report_data_public: Some(true),
                ..Default::default()
            },
        )
        .await?;

        let config = public_report_tool_config(&pool, &step.id, learn).await?;
        assert_eq!(
            ToolConfig::Learn(config),
            step.preview_tool_config,
            "wrong tool config served"
        );

        assert!(
            matches!(
                public_report_tool_config(&pool, &step.id, polis).await,
                Err(ComhairleError::ResourceNotFound(_))
            ),
            "step was served to another tool's endpoint"
        );

        Ok(())
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
