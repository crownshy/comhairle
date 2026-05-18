use std::sync::Arc;

use aide::axum::{
    routing::{get_with, post_with},
    ApiRouter,
};
use async_trait::async_trait;
use axum::{
    extract::{Json, Query, State},
    http::StatusCode,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::{
        proposal::{self, CreateProposal, LocalizedProposal, Proposal},
        translations::TextContentId,
    },
    routes::{auth::RequiredAdminUser, translations::LocaleExtractor},
    schema_helpers::{example_localized_text, example_uuid},
    tools::{ToolConfigSanitize, ToolImpl},
    ComhairleState,
};

#[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq, Clone)]
pub struct PrioritizationToolConfig {
    questions: Vec<Question>,
    randomize_order: bool,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq, Clone)]
pub struct Question {
    pub text: String,
    pub r#type: QuestionType,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq, Clone)]
pub enum QuestionType {
    Text(String),
    LikertScale { categories: Vec<Category> },
    Continuous { label: String, sub_steps: i32 },
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq, Clone)]
pub struct Category {
    value: f64,
    label: String,
}

impl ToolConfigSanitize for PrioritizationToolConfig {
    fn sanitize(&self) -> Self {
        Self {
            questions: self.questions.clone(),
            randomize_order: self.randomize_order,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub struct PrioritizationToolSetup {
    pub questions: Vec<Question>,
    pub randomize_order: bool,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub struct PrioritizationReport;

pub struct PrioritizationTool;

#[async_trait]
impl ToolImpl for PrioritizationTool {
    type Config = PrioritizationToolConfig;
    type Setup = PrioritizationToolSetup;
    type Report = PrioritizationReport;

    async fn setup(
        setup: &Self::Setup,
        _state: &Arc<ComhairleState>,
    ) -> Result<Self::Config, ComhairleError> {
        prioritization_setup(setup)
    }

    async fn clone_tool(
        config: &Self::Config,
        _state: &Arc<ComhairleState>,
    ) -> Result<Self::Config, ComhairleError> {
        Ok(config.clone())
    }

    fn sanitize(config: Self::Config) -> Self::Config {
        config.sanitize()
    }

    fn routes(state: &Arc<ComhairleState>) -> ApiRouter {
        ApiRouter::new()
            .api_route(
                "/prioritization/proposals",
                post_with(create_proposal, |op| {
                    op.id("CreateProposal")
                        .tag("Tools")
                        .summary("Create proposal")
                        .security_requirement("JWT")
                        .description(
                            "
Create a new prioritization tool proposal for a given prioritization tool workflow_step
",
                        )
                }),
            )
            .api_route(
                "/prioritization/proposals",
                get_with(list_proposals, |op| {
                    op.id("ListProposals")
                        .tag("Tools")
                        .summary("List proposals")
                        .description("List proposals for a given prioritization tool workflow_step")
                }),
            )
            .with_state(state.clone())
    }
}

fn prioritization_setup(
    setup_config: &PrioritizationToolSetup,
) -> Result<PrioritizationToolConfig, ComhairleError> {
    Ok(PrioritizationToolConfig {
        questions: setup_config.questions.clone(),
        randomize_order: setup_config.randomize_order,
    })
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProposalDto {
    #[schemars(example = "example_uuid")]
    pub id: Uuid,
    #[schemars(example = "example_uuid")]
    pub workflow_step_id: Uuid,
    #[schemars(example = "example_uuid")]
    pub title: TextContentId,
    #[schemars(example = "example_uuid")]
    pub body: TextContentId,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocalizedProposalDto {
    #[schemars(example = "example_uuid")]
    pub id: Uuid,
    #[schemars(example = "example_uuid")]
    pub workflow_step_id: Uuid,
    #[schemars(example = "example_localized_text")]
    pub title: String,
    #[schemars(example = "example_localized_text")]
    pub body: String,
}

impl From<Proposal> for ProposalDto {
    fn from(p: Proposal) -> Self {
        Self {
            id: p.id,
            workflow_step_id: p.workflow_step_id,
            title: p.title,
            body: p.body,
        }
    }
}

impl From<LocalizedProposal> for LocalizedProposalDto {
    fn from(p: LocalizedProposal) -> Self {
        Self {
            id: p.id,
            workflow_step_id: p.workflow_step_id,
            title: p.title,
            body: p.body,
        }
    }
}

#[derive(Deserialize, JsonSchema, Debug)]
struct CreateProposalRequest {
    workflow_step_id: Uuid,
    title: String,
    body: String,
}

#[instrument(err(Debug), skip(state))]
async fn create_proposal(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(_user): RequiredAdminUser,
    LocaleExtractor(locale): LocaleExtractor,
    Json(payload): Json<CreateProposalRequest>,
) -> Result<(StatusCode, Json<ProposalDto>), ComhairleError> {
    let params = CreateProposal {
        title: payload.title,
        body: payload.body,
    };
    let proposal = proposal::create(&state.db, &payload.workflow_step_id, &params, &locale).await?;

    Ok((StatusCode::CREATED, Json(proposal.into())))
}

#[derive(Deserialize, JsonSchema, Debug)]
struct ListProposalsQuery {
    workflow_step_id: Uuid,
}

#[instrument(err(Debug), skip(state))]
async fn list_proposals(
    State(state): State<Arc<ComhairleState>>,
    LocaleExtractor(locale): LocaleExtractor,
    Query(ListProposalsQuery { workflow_step_id }): Query<ListProposalsQuery>,
) -> Result<(StatusCode, Json<Vec<LocalizedProposalDto>>), ComhairleError> {
    let proposals = proposal::list(&state.db, &workflow_step_id, &locale)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();

    Ok((StatusCode::OK, Json(proposals)))
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use sqlx::PgPool;

    use crate::{
        models::model_test_helpers::{
            get_random_conversation_id, get_random_workflow_id, setup_default_app_and_session,
        },
        routes::workflow_steps::dto::WorkflowStepDto,
        test_helpers::prioritization_tool_config,
    };

    use super::*;

    use std::error::Error;

    #[sqlx::test]
    async fn should_create_proposal(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;

        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let workflow_id = get_random_workflow_id(&app, &mut session).await?;
        let (_, value, _) = session
            .create_workflow_step(
                &app,
                &conversation_id.to_string(),
                &workflow_id.to_string(),
                json!({
                    "name": "test_workflow_step",
                    "step_order": 1,
                    "activation_rule": "manual",
                    "description": "A test workflow_step with prioritization",
                    "is_offline": false,
                    "required": false,
                    "tool_setup": prioritization_tool_config(),
                }),
            )
            .await?;
        let workflow_step: WorkflowStepDto = serde_json::from_value(value)?;

        let (_, value, _) = session
            .post(
                &app,
                "/tools/prioritization/proposals",
                json!({
                    "workflow_step_id": workflow_step.id,
                    "title": "A new proposal",
                    "body": "Something to propose"
                })
                .to_string()
                .into(),
            )
            .await?;
        let proposal: ProposalDto = serde_json::from_value(value)?;

        assert_eq!(
            proposal.workflow_step_id, workflow_step.id,
            "incorrect workflow_step_id"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_list_proposals(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;

        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let workflow_id = get_random_workflow_id(&app, &mut session).await?;
        let (_, value, _) = session
            .create_workflow_step(
                &app,
                &conversation_id.to_string(),
                &workflow_id.to_string(),
                json!({
                    "name": "test_workflow_step",
                    "step_order": 1,
                    "activation_rule": "manual",
                    "description": "A test workflow_step with prioritization",
                    "is_offline": false,
                    "required": false,
                    "tool_setup": prioritization_tool_config(),
                }),
            )
            .await?;
        let workflow_step: WorkflowStepDto = serde_json::from_value(value)?;

        session
            .post(
                &app,
                "/tools/prioritization/proposals",
                json!({
                    "workflow_step_id": workflow_step.id,
                    "title": "New proposal A",
                    "body": "Something to propose"
                })
                .to_string()
                .into(),
            )
            .await?;
        session
            .post(
                &app,
                "/tools/prioritization/proposals",
                json!({
                    "workflow_step_id": workflow_step.id,
                    "title": "New proposal B",
                    "body": "Something to propose"
                })
                .to_string()
                .into(),
            )
            .await?;
        session
            .post(
                &app,
                "/tools/prioritization/proposals",
                json!({
                    "workflow_step_id": workflow_step.id,
                    "title": "New proposal C",
                    "body": "Something to propose"
                })
                .to_string()
                .into(),
            )
            .await?;

        let (_, value, _) = session
            .get(
                &app,
                &format!(
                    "/tools/prioritization/proposals?workflow_step_id={}",
                    workflow_step.id
                ),
            )
            .await?;
        let results: Vec<LocalizedProposalDto> = serde_json::from_value(value)?;

        assert_eq!(results.len(), 3, "incorrect number of proposals");
        assert!(
            results.iter().any(|p| p.title == "New proposal B"),
            "missing proposal title"
        );

        Ok(())
    }
}
