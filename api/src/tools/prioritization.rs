use std::sync::Arc;

use aide::axum::{
    routing::{delete_with, get_with, post_with, put_with},
    ApiRouter,
};
use async_trait::async_trait;
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::{
        proposal::{self, CreateProposal, LocalizedProposal, Proposal, UpdateProposal},
        proposal_response::{
            self, CreateResponse, ProposalResponse, ProposalResponseFilterOptions,
            ProposalResponseOrderOptions, QuestionResponses,
        },
        translations::TextContentId,
    },
    routes::{
        auth::{RequiredAdminUser, RequiredUser},
        translations::LocaleExtractor,
    },
    schema_helpers::{example_localized_text, example_uuid},
    tools::{ToolConfigSanitize, ToolImpl},
    ComhairleState,
};

#[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq, Clone)]
pub struct PrioritizationToolConfig {
    pub questions: Vec<Question>,
    pub randomize_order: bool,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq, Clone)]
pub struct Question {
    pub id: Uuid,
    pub text: String,
    pub r#type: QuestionType,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
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

#[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq, Clone)]
pub struct SetupQuestion {
    pub text: String,
    pub r#type: QuestionType,
}

impl From<SetupQuestion> for Question {
    fn from(q: SetupQuestion) -> Self {
        Self {
            id: Uuid::new_v4(),
            text: q.text,
            r#type: q.r#type,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub struct PrioritizationToolSetup {
    pub questions: Vec<SetupQuestion>,
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
                        .security_requirement("JWT")
                        .summary("Create proposal")
                        .description(
                            "
Create a new prioritization tool proposal for a given prioritization tool workflow_step
",
                        )
                        .response::<201, Json<ProposalDto>>()
                }),
            )
            .api_route(
                "/prioritization/proposals",
                get_with(list_proposals, |op| {
                    op.id("ListProposals")
                        .tag("Tools")
                        .security_requirement("JWT")
                        .summary("List proposals")
                        .description("List proposals for a given prioritization tool workflow_step")
                        .response::<200, Json<Vec<LocalizedProposalDto>>>()
                }),
            )
            .api_route(
                "/prioritization/proposals/{proposal_id}",
                put_with(update_proposal, |op| {
                    op.id("UpdateProposal")
                        .tag("Tools")
                        .security_requirement("JWT")
                        .summary("Update proposal")
                        .description(
                            "
Update title and/or body of a prioritization tool proposal. Strings are
written to the primary-locale translation of the proposal's TextContent.
",
                        )
                        .response::<200, Json<LocalizedProposalDto>>()
                }),
            )
            .api_route(
                "/prioritization/proposals/{proposal_id}",
                delete_with(delete_proposal, |op| {
                    op.id("DeleteProposal")
                        .tag("Tools")
                        .security_requirement("JWT")
                        .summary("Delete proposal")
                        .description("Delete a prioritization tool proposal")
                        .response::<200, Json<ProposalDto>>()
                }),
            )
            .api_route(
                "/prioritization/proposals/{proposal_id}/responses",
                post_with(create_proposal_response, |op| {
                    op.id("CreateProposalResponse")
                        .tag("Tools")
                        .security_requirement("JWT")
                        .summary("Create proposal response")
                        .description(
                            "
Create a response for prioritization tool proposal
",
                        )
                        .response::<201, Json<ProposalResponseDto>>()
                }),
            )
            .api_route(
                "/prioritization/proposals/{proposal_id}/responses",
                get_with(list_proposal_responses, |op| {
                    op.id("ListProposalResponses")
                        .tag("Tools")
                        .security_requirement("JWT")
                        .summary("List proposal responses")
                        .description("List responses for a prioritization tool proposal")
                        .response::<200, Json<Vec<ProposalResponseDto>>>()
                }),
            )
            .with_state(state.clone())
    }
}

fn prioritization_setup(
    setup_config: &PrioritizationToolSetup,
) -> Result<PrioritizationToolConfig, ComhairleError> {
    Ok(PrioritizationToolConfig {
        questions: setup_config
            .questions
            .clone()
            .into_iter()
            .map(Into::into)
            .collect(),
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

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProposalResponseDto {
    pub id: Uuid,
    pub proposal_id: Uuid,
    pub response: QuestionResponses,
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

impl From<ProposalResponse> for ProposalResponseDto {
    fn from(r: ProposalResponse) -> Self {
        Self {
            id: r.id,
            proposal_id: r.proposal_id,
            response: r.response,
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
    RequiredUser(_user): RequiredUser,
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

#[instrument(err(Debug), skip(state))]
async fn update_proposal(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(_user): RequiredAdminUser,
    LocaleExtractor(locale): LocaleExtractor,
    Path(proposal_id): Path<Uuid>,
    Json(payload): Json<UpdateProposal>,
) -> Result<(StatusCode, Json<LocalizedProposalDto>), ComhairleError> {
    let proposal = proposal::update(&state.db, &proposal_id, &payload, &locale).await?;

    Ok((StatusCode::OK, Json(proposal.into())))
}

#[instrument(err(Debug), skip(state))]
async fn delete_proposal(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Path(proposal_id): Path<Uuid>,
) -> Result<(StatusCode, Json<ProposalDto>), ComhairleError> {
    let proposal = proposal::delete(&state.db, &proposal_id).await?;

    Ok((StatusCode::OK, Json(proposal.into())))
}

#[instrument(err(Debug), skip(state))]
async fn create_proposal_response(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path(proposal_id): Path<Uuid>,
    Json(payload): Json<CreateResponse>,
) -> Result<(StatusCode, Json<ProposalResponseDto>), ComhairleError> {
    let response = proposal_response::create(&state.db, &proposal_id, &user.id, &payload).await?;

    Ok((StatusCode::CREATED, Json(response.into())))
}

#[instrument(err(Debug), skip(state))]
async fn list_proposal_responses(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(_user): RequiredUser,
    Path(proposal_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Vec<ProposalResponseDto>>), ComhairleError> {
    let responses = proposal_response::list(
        &state.db,
        &proposal_id,
        ProposalResponseFilterOptions,
        ProposalResponseOrderOptions,
    )
    .await?
    .into_iter()
    .map(Into::into)
    .collect();

    Ok((StatusCode::OK, Json(responses)))
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use sqlx::PgPool;

    use crate::{
        models::{
            model_test_helpers::{
                get_random_conversation_id, get_random_workflow_id, setup_default_app_and_session,
            },
            proposal_response::Response,
        },
        routes::workflow_steps::dto::WorkflowStepDto,
        test_helpers::prioritization_tool_config,
        tools::ToolConfig,
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

    #[sqlx::test]
    async fn should_create_new_proposal_response_via_api(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let workflow_id = get_random_workflow_id(&app, &mut session).await?;
        let workflow_step = session
            .create_prioritization_workflow_step(&app, &conversation_id, &workflow_id)
            .await?;

        let proposal = proposal::create(
            &pool,
            &workflow_step.id,
            &CreateProposal {
                title: "A new proposal".to_string(),
                body: "Test proposal".to_string(),
            },
            "en",
        )
        .await?;

        let tool_config = match workflow_step.preview_tool_config {
            ToolConfig::Prioritization(config) => config,
            _ => panic!("Incorrect tool_config type"),
        };

        let create_response = CreateResponse {
            question_responses: vec![
                Response {
                    question_id: tool_config.questions.first().unwrap().id,
                    value: -1.0,
                },
                Response {
                    question_id: tool_config.questions[1].id,
                    value: 0.5,
                },
            ],
        };

        let (_, value, _) = session
            .post(
                &app,
                &format!("/tools/prioritization/proposals/{}/responses", proposal.id),
                json!(create_response).to_string().into(),
            )
            .await?;

        let proposal_response: ProposalResponseDto = serde_json::from_value(value)?;

        assert_eq!(
            proposal_response.proposal_id, proposal.id,
            "incorrect proposal_id"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_list_proposal_responses_via_api(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let workflow_id = get_random_workflow_id(&app, &mut session).await?;
        let workflow_step = session
            .create_prioritization_workflow_step(&app, &conversation_id, &workflow_id)
            .await?;

        let proposal_a = proposal::create(
            &pool,
            &workflow_step.id,
            &CreateProposal {
                title: "Proposal A".to_string(),
                body: "Proposal A".to_string(),
            },
            "en",
        )
        .await?;
        let proposal_b = proposal::create(
            &pool,
            &workflow_step.id,
            &CreateProposal {
                title: "Proposal B".to_string(),
                body: "Proposal B".to_string(),
            },
            "en",
        )
        .await?;

        let tool_config = match workflow_step.preview_tool_config {
            ToolConfig::Prioritization(config) => config,
            _ => panic!("Incorrect tool_config type"),
        };

        session.signup_annon(&app).await?;
        let create_response_a_a = CreateResponse {
            question_responses: vec![
                Response {
                    question_id: tool_config.questions.first().unwrap().id,
                    value: -1.0,
                },
                Response {
                    question_id: tool_config.questions[1].id,
                    value: 0.5,
                },
            ],
        };
        let create_response_a_b = CreateResponse {
            question_responses: vec![
                Response {
                    question_id: tool_config.questions.first().unwrap().id,
                    value: 0.5,
                },
                Response {
                    question_id: tool_config.questions[1].id,
                    value: 0.2,
                },
            ],
        };
        session
            .post(
                &app,
                &format!(
                    "/tools/prioritization/proposals/{}/responses",
                    proposal_a.id
                ),
                json!(create_response_a_a).to_string().into(),
            )
            .await?;
        session
            .post(
                &app,
                &format!(
                    "/tools/prioritization/proposals/{}/responses",
                    proposal_a.id
                ),
                json!(create_response_a_b).to_string().into(),
            )
            .await?;

        session.signup_annon(&app).await?;
        let create_response_b_a = CreateResponse {
            question_responses: vec![
                Response {
                    question_id: tool_config.questions.first().unwrap().id,
                    value: -1.0,
                },
                Response {
                    question_id: tool_config.questions[1].id,
                    value: 0.5,
                },
            ],
        };
        let create_response_b_b = CreateResponse {
            question_responses: vec![
                Response {
                    question_id: tool_config.questions.first().unwrap().id,
                    value: 0.5,
                },
                Response {
                    question_id: tool_config.questions[1].id,
                    value: 0.2,
                },
            ],
        };
        session
            .post(
                &app,
                &format!(
                    "/tools/prioritization/proposals/{}/responses",
                    proposal_b.id
                ),
                json!(create_response_b_a).to_string().into(),
            )
            .await?;
        session
            .post(
                &app,
                &format!(
                    "/tools/prioritization/proposals/{}/responses",
                    proposal_b.id
                ),
                json!(create_response_b_b).to_string().into(),
            )
            .await?;

        let (_, value, _) = session
            .get(
                &app,
                &format!(
                    "/tools/prioritization/proposals/{}/responses",
                    proposal_a.id
                ),
            )
            .await?;
        let proposal_a_responses: Vec<ProposalResponseDto> = serde_json::from_value(value)?;

        let (_, value, _) = session
            .get(
                &app,
                &format!(
                    "/tools/prioritization/proposals/{}/responses",
                    proposal_b.id
                ),
            )
            .await?;
        let proposal_b_responses: Vec<ProposalResponseDto> = serde_json::from_value(value)?;

        assert_eq!(proposal_a_responses.len(), 2, "incorrect a total");
        assert_eq!(proposal_b_responses.len(), 2, "incorrect b total");

        Ok(())
    }

    #[sqlx::test]
    async fn should_update_proposal(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let workflow_id = get_random_workflow_id(&app, &mut session).await?;
        let workflow_step = session
            .create_prioritization_workflow_step(&app, &conversation_id, &workflow_id)
            .await?;

        let proposal = proposal::create(
            &pool,
            &workflow_step.id,
            &CreateProposal {
                title: "Old title".to_string(),
                body: "Old body".to_string(),
            },
            "en",
        )
        .await?;

        let (status, value, _) = session
            .put(
                &app,
                &format!("/tools/prioritization/proposals/{}", proposal.id),
                json!({
                    "title": "New title",
                    "body": "New body",
                })
                .to_string()
                .into(),
            )
            .await?;
        assert_eq!(status, StatusCode::OK, "update should succeed");
        let updated: LocalizedProposalDto = serde_json::from_value(value)?;
        assert_eq!(updated.title, "New title", "title not updated");
        assert_eq!(updated.body, "New body", "body not updated");
        assert_eq!(updated.id, proposal.id, "proposal id changed");

        // Partial update: title only.
        let (_, value, _) = session
            .put(
                &app,
                &format!("/tools/prioritization/proposals/{}", proposal.id),
                json!({ "title": "Newer title" }).to_string().into(),
            )
            .await?;
        let updated: LocalizedProposalDto = serde_json::from_value(value)?;
        assert_eq!(updated.title, "Newer title", "title not updated");
        assert_eq!(updated.body, "New body", "body should be unchanged");

        Ok(())
    }

    #[sqlx::test]
    async fn should_delete_proposal(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let workflow_id = get_random_workflow_id(&app, &mut session).await?;
        let workflow_step = session
            .create_prioritization_workflow_step(&app, &conversation_id, &workflow_id)
            .await?;

        let proposal = proposal::create(
            &pool,
            &workflow_step.id,
            &CreateProposal {
                title: "Doomed".to_string(),
                body: "Doomed body".to_string(),
            },
            "en",
        )
        .await?;

        let (status, _, _) = session
            .delete(
                &app,
                &format!("/tools/prioritization/proposals/{}", proposal.id),
            )
            .await?;
        assert_eq!(status, StatusCode::OK, "delete should succeed");

        let (_, value, _) = session
            .get(
                &app,
                &format!(
                    "/tools/prioritization/proposals?workflow_step_id={}",
                    workflow_step.id
                ),
            )
            .await?;
        let remaining: Vec<LocalizedProposalDto> = serde_json::from_value(value)?;
        assert!(remaining.is_empty(), "proposal should be gone");

        Ok(())
    }
}
