//! Mirrors free-text answers from each tool into generic `statement` rows, tagged
//! with where they came from, so the theme-extraction pipeline can run over any
//! tool's text without knowing its shape. Re-running a harvest refreshes content
//! on the matching `(workflow_step_id, source_tool, source_sub_id)` row instead of
//! duplicating it.

use std::sync::Arc;

use heyform_sdk::LoginInput;
use heyform_sdk::client::HeyFormClient;
use uuid::Uuid;

use crate::ComhairleState;
use crate::error::ComhairleError;
use crate::models::polis_statement_aux::{self, PolisStatementAuxFilterOptions};
use crate::models::proposal;
use crate::models::proposal_response::{
    self, ProposalResponseFilterOptions, ProposalResponseOrderOptions, ResponseValue,
};
use crate::models::statement::{self, CreateStatement, Statement};
use crate::models::thinking_space_answer::{self, ThinkingSpaceAnswerFilterOptions};
use crate::tools::heyform;

/// Harvests the free-text source data for `workflow_step_id` from `tool` into
/// `statement` rows. `sub_scope` narrows prioritization to a single proposal or
/// section (`"proposal:{id}"` or `"proposal:{id}/section:{id}"`); ignored by the
/// other tools.
pub async fn harvest(
    state: &Arc<ComhairleState>,
    workflow_step_id: Uuid,
    tool: &str,
    sub_scope: Option<&str>,
) -> Result<Vec<Statement>, ComhairleError> {
    match tool {
        "heyform" => harvest_heyform(state, workflow_step_id).await,
        "polis" => harvest_polis(state, workflow_step_id).await,
        "thinking_space" => harvest_thinking_space(state, workflow_step_id).await,
        "prioritization" => harvest_prioritization(state, workflow_step_id, sub_scope).await,
        other => Err(ComhairleError::BadRequest(format!(
            "Unsupported source tool: {other}"
        ))),
    }
}

async fn harvest_heyform(
    state: &Arc<ComhairleState>,
    workflow_step_id: Uuid,
) -> Result<Vec<Statement>, ComhairleError> {
    let config = heyform::get_heyform_config_for_workflow_step(state, workflow_step_id).await?;
    let client = HeyFormClient::new(heyform::heyform_base_url(&config.server_url))?;
    client
        .login(LoginInput {
            email: config.admin_user.clone(),
            password: config.admin_password.clone(),
        })
        .await?;

    let (form, report, submissions) = tokio::try_join!(
        client.get_form(&config.survey_id),
        client.get_form_report(config.survey_id.clone()),
        heyform::fetch_all_submissions(&client, &config.survey_id, "inbox"),
    )?;
    let insights = heyform::build_survey_insights(&form, &report, &submissions);

    let mut statements = Vec::new();
    for question in insights.questions {
        let Some(submissions) = question.submissions else {
            continue;
        };
        for submission in submissions {
            let Some(text) = submission.value.as_str() else {
                continue;
            };
            if text.trim().is_empty() {
                continue;
            }

            let payload = CreateStatement {
                user_id: None,
                content: text.to_string(),
                context: format!("HeyForm question: {}", question.title),
                ai_generated: false,
                source_tool: "heyform".to_string(),
                source_sub_id: Some(format!("{}:{}", question.id, submission.submission_id)),
            };
            statements
                .push(statement::upsert_from_source(&state.db, workflow_step_id, &payload).await?);
        }
    }

    Ok(statements)
}

async fn harvest_polis(
    state: &Arc<ComhairleState>,
    workflow_step_id: Uuid,
) -> Result<Vec<Statement>, ComhairleError> {
    let aux_rows = polis_statement_aux::list(
        &state.db,
        Some(workflow_step_id),
        None,
        PolisStatementAuxFilterOptions::default(),
    )
    .await?;

    let mut statements = Vec::new();
    for aux in aux_rows {
        if aux.statement_text.trim().is_empty() {
            continue;
        }

        let payload = CreateStatement {
            user_id: None,
            content: aux.statement_text.clone(),
            context: "Polis statement".to_string(),
            ai_generated: false,
            source_tool: "polis".to_string(),
            source_sub_id: Some(aux.polis_statement_id.to_string()),
        };
        statements
            .push(statement::upsert_from_source(&state.db, workflow_step_id, &payload).await?);
    }

    Ok(statements)
}

async fn harvest_thinking_space(
    state: &Arc<ComhairleState>,
    workflow_step_id: Uuid,
) -> Result<Vec<Statement>, ComhairleError> {
    let answers = thinking_space_answer::list(
        &state.db,
        &workflow_step_id,
        ThinkingSpaceAnswerFilterOptions::default(),
    )
    .await?;

    let mut statements = Vec::new();
    for answer in answers {
        if answer.answer.trim().is_empty() {
            continue;
        }

        let payload = CreateStatement {
            user_id: Some(answer.user_id),
            content: answer.answer.clone(),
            context: answer.question.clone(),
            ai_generated: false,
            source_tool: "thinking_space".to_string(),
            // The answer row's own id uniquely identifies it; root_question_id is
            // shared across every participant who answered that question.
            source_sub_id: Some(answer.id.to_string()),
        };
        statements
            .push(statement::upsert_from_source(&state.db, workflow_step_id, &payload).await?);
    }

    Ok(statements)
}

async fn harvest_prioritization(
    state: &Arc<ComhairleState>,
    workflow_step_id: Uuid,
    sub_scope: Option<&str>,
) -> Result<Vec<Statement>, ComhairleError> {
    let proposals = proposal::list(&state.db, &workflow_step_id).await?;

    let mut statements = Vec::new();
    for prop in proposals {
        let proposal_scope = format!("proposal:{}", prop.id);
        if let Some(scope) = sub_scope
            && scope != proposal_scope
            && !scope.starts_with(&format!("{proposal_scope}/section:"))
        {
            continue;
        }

        let responses = proposal_response::list(
            &state.db,
            &prop.id,
            ProposalResponseFilterOptions::default(),
            ProposalResponseOrderOptions::default(),
        )
        .await?;

        for response in responses {
            for entry in &response.response.0 {
                let ResponseValue::Text(text) = &entry.value else {
                    continue;
                };
                if text.trim().is_empty() {
                    continue;
                }

                let entry_scope = match entry.section_id {
                    Some(section_id) => format!("{proposal_scope}/section:{section_id}"),
                    None => proposal_scope.clone(),
                };
                if let Some(scope) = sub_scope
                    && scope != entry_scope
                {
                    continue;
                }

                let payload = CreateStatement {
                    user_id: Some(response.user_id),
                    content: text.clone(),
                    context: format!("Prioritization response to question {}", entry.question_id),
                    ai_generated: false,
                    source_tool: "prioritization".to_string(),
                    source_sub_id: Some(format!(
                        "{entry_scope}:{}:{}",
                        response.id, entry.question_id
                    )),
                };
                statements.push(
                    statement::upsert_from_source(&state.db, workflow_step_id, &payload).await?,
                );
            }
        }
    }

    Ok(statements)
}
