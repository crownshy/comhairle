//! Shared guard for public report-data endpoints (ADR-0046).
//!
//! Each embeddable report widget gets its own unauthenticated endpoint that returns only the
//! aggregated, moderated fields that widget draws. Every one of them must go through
//! [`public_report_tool_config`] so a step's data is only ever served after an admin has opted
//! it in with `report_data_public`.

use sqlx::PgPool;
use uuid::Uuid;

use crate::error::ComhairleError;
use crate::models::workflow_step;
use crate::tools::ToolConfig;

/// Load the tool config behind a public report-data request.
///
/// Returns `ResourceNotFound` when the step hasn't opted in, the same as for a step that doesn't
/// exist, so an anonymous caller can't tell the two apart. Falls back to the preview config
/// before launch, so a facilitator can build the report ahead of time.
pub async fn public_report_tool_config(
    db: &PgPool,
    workflow_step_id: &Uuid,
) -> Result<ToolConfig, ComhairleError> {
    let step = workflow_step::get_by_id(db, workflow_step_id).await?;

    if !step.report_data_public {
        return Err(ComhairleError::ResourceNotFound(format!(
            "Report data for workflow step {workflow_step_id}"
        )));
    }

    Ok(step.tool_config.unwrap_or(step.preview_tool_config))
}

#[cfg(test)]
mod tests {
    use crate::{
        models::{
            model_test_helpers::{get_random_conversation_id, setup_default_app_and_session},
            workflow_step::{self, PartialWorkflowStep},
        },
        routes::{workflow_steps::dto::WorkflowStepDto, workflows::dto::WorkflowDto},
    };

    use super::*;
    use std::error::Error;

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
                public_report_tool_config(&pool, &step.id).await,
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

        let config = public_report_tool_config(&pool, &step.id).await?;
        assert_eq!(config, step.preview_tool_config, "wrong tool config served");

        Ok(())
    }
}
