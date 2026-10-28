//! Shared gate for public report-data endpoints (ADR-0061).
//!
//! Each embeddable report section block gets its own unauthenticated endpoint that returns only
//! the aggregated, moderated fields that section block draws. Every one of them must go through
//! [`serve_public_report_data`], so a step's data is only served for section blocks the published
//! report embeds, and only as [`anonymous::PublicReportData`].

pub mod anonymous;
pub mod embeds;

use std::sync::Arc;

use anonymous::PublicReportData;
use embeds::{EmbeddedSectionBlock, embedded_section_blocks};
use sqlx::PgPool;
use uuid::Uuid;

use crate::ComhairleState;
use crate::error::ComhairleError;
use crate::models::permissions::conversation::Action;
use crate::models::permissions::{self};
use crate::models::users::User;
use crate::models::{conversation, event, report, workflow, workflow_step};
use crate::tools::ToolConfig;

/// Serve one section block's public data. `item` says which one the block shows, for blocks
/// like a HeyForm question, and must match the embed's. `for_tool` narrows the step's config to the endpoint's
/// own tool, for example `|config| match config { ToolConfig::Polis(polis) => Some(polis), _ =>
/// None }`, and `build` turns that config into the response.
///
/// Returns `ResourceNotFound` when the step doesn't exist, its conversation's report isn't
/// published, the report doesn't embed this section block for this step, or the step runs a
/// different tool, so an anonymous caller can't tell these apart.
pub async fn serve_public_report_data<Config, Data: PublicReportData>(
    db: &PgPool,
    workflow_step_id: &Uuid,
    section_block: &str,
    item: Option<&str>,
    for_tool: impl FnOnce(ToolConfig) -> Option<Config>,
    build: impl AsyncFnOnce(Config) -> Result<Data, ComhairleError>,
) -> Result<Data, ComhairleError> {
    let config = public_tool_config(db, workflow_step_id, section_block, item, for_tool).await?;
    build(config).await
}

/// Serve a section block's data to a facilitator before it is public, so the report editor and
/// preview show what readers will get. Skips the published and embedded checks, and needs
/// permission to update the step's conversation instead.
pub async fn serve_report_data_preview<Config, Data: PublicReportData>(
    state: &Arc<ComhairleState>,
    user: &User,
    workflow_step_id: &Uuid,
    for_tool: impl FnOnce(ToolConfig) -> Option<Config>,
    build: impl AsyncFnOnce(Config) -> Result<Data, ComhairleError>,
) -> Result<Data, ComhairleError> {
    let step = check_can_update_step(state, user, workflow_step_id).await?;
    let config =
        for_tool(step.tool_config.unwrap_or(step.preview_tool_config)).ok_or_else(|| {
            ComhairleError::ResourceNotFound(format!(
                "Report data for workflow step {workflow_step_id}"
            ))
        })?;
    build(config).await
}

/// The step, when the user can update its conversation. For report editing tools that only
/// facilitators use, such as listing what a step could embed.
pub async fn check_can_update_step(
    state: &Arc<ComhairleState>,
    user: &User,
    workflow_step_id: &Uuid,
) -> Result<workflow_step::WorkflowStep, ComhairleError> {
    let step = workflow_step::get_by_id(&state.db, workflow_step_id).await?;
    let conversation_id = conversation_of_workflow(&state.db, &step.workflow_id).await?;
    let conversation = conversation::get_by_id(&state.db, &conversation_id).await?;
    let can_update = permissions::can_perform_action(
        state,
        &conversation.id,
        Action::Update,
        &user.id,
        Some(&conversation.owner_id),
    )
    .await?;
    if !can_update {
        return Err(ComhairleError::UserNotAuthorized);
    }
    Ok(step)
}

async fn public_tool_config<Config>(
    db: &PgPool,
    workflow_step_id: &Uuid,
    section_block: &str,
    item: Option<&str>,
    for_tool: impl FnOnce(ToolConfig) -> Option<Config>,
) -> Result<Config, ComhairleError> {
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

    let requested = EmbeddedSectionBlock {
        workflow_step_id: *workflow_step_id,
        section_block: section_block.to_owned(),
        item: item.map(str::to_owned),
    };
    if !embedded_section_blocks(db, &report)
        .await?
        .contains(&requested)
    {
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

    use serde_json::json;

    use super::anonymous::{Count, public_report_data};
    use super::*;
    use crate::models::model_test_helpers::{
        get_random_conversation_id, setup_default_app_and_session,
    };
    use crate::models::report::PartialReport;
    use crate::models::translations::{
        UpdateTextTranslation, get_text_translation_by_content_and_locale, update_text_translation,
    };
    use crate::routes::workflow_steps::dto::WorkflowStepDto;
    use crate::routes::workflows::dto::WorkflowDto;
    use crate::tools::learn::LearnToolConfig;
    use crate::tools::polis::PolisToolConfig;

    const EMBEDDED: &str = "polis-key-stats";
    const NOT_EMBEDDED: &str = "polis-opinion-groups";

    public_report_data! {
        struct ExampleSectionBlock {
            participants: Count,
        }
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_only_serve_section_blocks_the_published_report_embeds(
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
            public_tool_config(&pool, &step.id, EMBEDDED, None, learn).await,
            "step was served with no report",
        );

        report::create_for_conversation(&pool, conversation_id, "en").await?;
        embed_in_summary(&pool, conversation_id, &step.id, EMBEDDED).await?;
        assert_not_found(
            public_tool_config(&pool, &step.id, EMBEDDED, None, learn).await,
            "step was served from an unpublished report",
        );

        set_published(&pool, conversation_id, true).await?;
        let config = public_tool_config(&pool, &step.id, EMBEDDED, None, learn).await?;
        assert_eq!(
            Some(ToolConfig::Learn(config)),
            step.tool_config.or(Some(step.preview_tool_config)),
            "wrong tool config served"
        );
        let served = serve_public_report_data(&pool, &step.id, EMBEDDED, None, learn, async |_| {
            Ok(ExampleSectionBlock {
                participants: Count(3),
            })
        })
        .await?;
        assert_eq!(served.participants, Count(3), "built data not returned");

        assert_not_found(
            public_tool_config(&pool, &step.id, NOT_EMBEDDED, None, learn).await,
            "a section block the report doesn't embed was served",
        );
        assert_not_found(
            public_tool_config(&pool, &step.id, EMBEDDED, Some("question-1"), learn).await,
            "an item the report doesn't embed was served",
        );
        assert_not_found(
            public_tool_config(&pool, &step.id, EMBEDDED, None, polis).await,
            "step was served to another tool's endpoint",
        );

        set_published(&pool, conversation_id, false).await?;
        assert_not_found(
            public_tool_config(&pool, &step.id, EMBEDDED, None, learn).await,
            "step was still served after the report was unpublished",
        );

        Ok(())
    }

    async fn embed_in_summary(
        pool: &PgPool,
        conversation_id: Uuid,
        workflow_step_id: &Uuid,
        section_block: &str,
    ) -> Result<(), ComhairleError> {
        let report = report::get_for_conversation(pool, conversation_id).await?;
        let summary =
            get_text_translation_by_content_and_locale(pool, &report.summary, "en").await?;
        let document = json!({
            "type": "doc",
            "content": [{
                "type": "reportComponentEmbed",
                "attrs": { "toolStepId": workflow_step_id, "componentType": section_block }
            }]
        });
        update_text_translation(
            pool,
            &summary.id,
            &UpdateTextTranslation {
                content: Some(document.to_string()),
                ..Default::default()
            },
        )
        .await?;
        Ok(())
    }

    async fn set_published(
        pool: &PgPool,
        conversation_id: Uuid,
        is_public: bool,
    ) -> Result<(), ComhairleError> {
        report::update(
            pool,
            conversation_id,
            PartialReport {
                is_public: Some(is_public),
                conversation_id: None,
                body: None,
                section_configs: None,
            },
        )
        .await?;
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
