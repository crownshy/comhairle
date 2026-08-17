use std::sync::Arc;

use aide::axum::{
    ApiRouter,
    routing::{delete_with, get_with, post_with},
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
    ComhairleState,
    error::ComhairleError,
    models::{
        proposal::{self, CreateProposal, LocalizedProposal, Proposal, ProposalWithTranslations},
        proposal_response::{
            self, CreateResponse, ProposalResponse, ProposalResponseFilterOptions,
            ProposalResponseOrderOptions, QuestionResponses, ResponseValue,
        },
        proposal_section::{
            self, LocalizedProposalSection, ProposalSection, ProposalSectionWithTranslations,
        },
        translations::TextContentId,
        user_progress, workflow_step,
    },
    routes::{
        auth::{RequiredAdminUser, RequiredUser, is_user_admin},
        translations::LocaleExtractor,
    },
    schema_helpers::{example_localized_text, example_uuid},
    tools::{ToolConfig, ToolConfigSanitize, ToolImpl},
};

#[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq, Clone)]
pub struct PrioritizationToolConfig {
    /// Questions asked once about the proposal as a whole.
    pub questions: Vec<Question>,
    /// Questions asked about each section individually. The same set is used
    /// for every section; participants answer them once per section.
    #[serde(default)]
    pub section_questions: Vec<Question>,
    pub randomize_order: bool,
    pub alignment_question_id: Option<Uuid>,
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
    Text,
    LikertScale {
        categories: Vec<Category>,
    },
    Continuous {
        #[serde(default = "default_sub_steps")]
        sub_steps: i32,
        #[serde(default)]
        min_value: f64,
        #[serde(default = "default_max_value")]
        max_value: f64,
        #[serde(default)]
        min_label: String,
        #[serde(default)]
        max_label: String,
    },
}

fn default_sub_steps() -> i32 {
    10
}

fn default_max_value() -> f64 {
    10.0
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
            section_questions: self.section_questions.clone(),
            randomize_order: self.randomize_order,
            alignment_question_id: self.alignment_question_id,
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
    #[serde(default)]
    pub section_questions: Vec<SetupQuestion>,
    pub randomize_order: bool,
    pub alignment_question_id: Option<Uuid>,
}

#[derive(PartialEq, Serialize, Deserialize, Debug, JsonSchema, Clone)]
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
                        .description(
                            "List proposals for a given prioritization tool workflow_step. \
                             Admin callers may pass `withTranslations=true` to receive raw \
                             TextContentId references plus full translation data so the \
                             admin UI can drive the standard TranslatableField component.",
                        )
                        .response::<200, Json<ProposalsListResponse>>()
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
                "/prioritization/proposals/{proposal_id}/sections",
                post_with(create_proposal_section, |op| {
                    op.id("CreateProposalSection")
                        .tag("Tools")
                        .security_requirement("JWT")
                        .summary("Create proposal section")
                        .description("Append a section to a prioritization tool proposal")
                        .response::<201, Json<ProposalSectionDto>>()
                }),
            )
            .api_route(
                "/prioritization/proposals/{proposal_id}/sections/{section_id}",
                delete_with(delete_proposal_section, |op| {
                    op.id("DeleteProposalSection")
                        .tag("Tools")
                        .security_requirement("JWT")
                        .summary("Delete proposal section")
                        .description("Delete a section from a prioritization tool proposal")
                        .response::<200, Json<ProposalSectionDto>>()
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
            .api_route(
                "/prioritization/insights",
                get_with(get_prioritization_insights, |op| {
                    op.id("GetPrioritizationInsights")
                        .tag("Tools")
                        .security_requirement("JWT")
                        .summary("Get prioritization insights")
                        .description("Insights reporting data for prioritization tool step")
                        .response::<200, Json<PrioritizationInsightsResponse>>()
                }),
            )
            .with_state(state.clone())
    }
}

fn prioritization_setup(
    setup_config: &PrioritizationToolSetup,
) -> Result<PrioritizationToolConfig, ComhairleError> {
    let questions: Vec<Question> = setup_config
        .questions
        .clone()
        .into_iter()
        .map(Into::into)
        .collect();

    let alignment_question_id = if questions.is_empty() {
        None
    } else {
        Some(questions[0].id)
    };

    Ok(PrioritizationToolConfig {
        questions,
        section_questions: setup_config
            .section_questions
            .clone()
            .into_iter()
            .map(Into::into)
            .collect(),
        randomize_order: setup_config.randomize_order,
        alignment_question_id,
    })
}

/// Raw (TextContentId) section reference returned alongside a freshly created proposal.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProposalSectionDto {
    #[schemars(example = "example_uuid")]
    pub id: Uuid,
    pub position: i32,
    #[schemars(example = "example_uuid")]
    pub body: TextContentId,
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
    pub sections: Vec<ProposalSectionDto>,
}

/// Response variants for `ListProposals`. Admins may opt-in to the
/// `WithTranslations` variant via `?withTranslations=true` to drive the
/// `TranslatableField` editor; everyone else receives the localized variant.
#[derive(Serialize, JsonSchema, Debug)]
#[serde(untagged)]
pub enum ProposalsListResponse {
    WithTranslations(Vec<ProposalWithTranslationsDto>),
    Localized(Vec<LocalizedProposalDto>),
}

/// Locale-resolved section shown to participants.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocalizedProposalSectionDto {
    #[schemars(example = "example_uuid")]
    pub id: Uuid,
    pub position: i32,
    #[schemars(example = "example_localized_text")]
    pub body: String,
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
    pub sections: Vec<LocalizedProposalSectionDto>,
}

/// Section plus its full translation metadata, for the admin editor.
#[derive(Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SectionWithTranslationsDto {
    #[schemars(example = "example_uuid")]
    pub id: Uuid,
    pub position: i32,
    #[schemars(example = "example_localized_text")]
    pub body: String,
    pub body_translations: proposal_section::Translation,
}

/// Proposal plus its title/section translation metadata, for the admin editor.
#[derive(Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProposalWithTranslationsDto {
    #[schemars(example = "example_uuid")]
    pub id: Uuid,
    #[schemars(example = "example_uuid")]
    pub workflow_step_id: Uuid,
    #[schemars(example = "example_localized_text")]
    pub title: String,
    pub title_translations: proposal::Translation,
    pub sections: Vec<SectionWithTranslationsDto>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProposalResponseDto {
    pub id: Uuid,
    pub proposal_id: Uuid,
    pub user_id: Uuid,
    pub response: QuestionResponses,
}

impl From<ProposalSection> for ProposalSectionDto {
    fn from(s: ProposalSection) -> Self {
        Self {
            id: s.id,
            position: s.position,
            body: s.body,
        }
    }
}

impl ProposalDto {
    fn from_parts(p: Proposal, sections: Vec<ProposalSection>) -> Self {
        Self {
            id: p.id,
            workflow_step_id: p.workflow_step_id,
            title: p.title,
            sections: sections.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<LocalizedProposalSection> for LocalizedProposalSectionDto {
    fn from(s: LocalizedProposalSection) -> Self {
        Self {
            id: s.id,
            position: s.position,
            body: s.body,
        }
    }
}

impl LocalizedProposalDto {
    fn from_parts(p: LocalizedProposal, sections: Vec<LocalizedProposalSection>) -> Self {
        Self {
            id: p.id,
            workflow_step_id: p.workflow_step_id,
            title: p.title,
            sections: sections.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<ProposalSectionWithTranslations> for SectionWithTranslationsDto {
    fn from(s: ProposalSectionWithTranslations) -> Self {
        Self {
            id: s.id,
            position: s.position,
            body: s.body,
            body_translations: s.translations.body,
        }
    }
}

impl ProposalWithTranslationsDto {
    fn from_parts(
        p: ProposalWithTranslations,
        sections: Vec<ProposalSectionWithTranslations>,
    ) -> Self {
        Self {
            id: p.id,
            workflow_step_id: p.workflow_step_id,
            title: p.title,
            title_translations: p.translations.title,
            sections: sections.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<ProposalResponse> for ProposalResponseDto {
    fn from(r: ProposalResponse) -> Self {
        Self {
            id: r.id,
            proposal_id: r.proposal_id,
            user_id: r.user_id,
            response: r.response,
        }
    }
}

#[derive(Deserialize, JsonSchema, Debug)]
struct CreateProposalRequest {
    workflow_step_id: Uuid,
    title: String,
    /// Ordered list of section bodies (rich text).
    #[serde(default)]
    sections: Vec<String>,
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
        sections: payload.sections,
    };
    let (proposal, sections) =
        proposal::create(&state.db, &payload.workflow_step_id, &params, &locale).await?;

    Ok((
        StatusCode::CREATED,
        Json(ProposalDto::from_parts(proposal, sections)),
    ))
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
struct ListProposalsQuery {
    workflow_step_id: Uuid,
    #[serde(default)]
    with_translations: bool,
}

#[instrument(err(Debug), skip(state))]
async fn list_proposals(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    LocaleExtractor(locale): LocaleExtractor,
    Query(ListProposalsQuery {
        workflow_step_id,
        with_translations,
    }): Query<ListProposalsQuery>,
) -> Result<(StatusCode, Json<ProposalsListResponse>), ComhairleError> {
    if with_translations && is_user_admin(&state, &user).await {
        let proposals =
            proposal::list_with_translations(&state.db, &workflow_step_id, &locale).await?;
        let mut out = Vec::with_capacity(proposals.len());
        for proposal in proposals {
            let sections =
                proposal_section::list_with_translations(&state.db, &proposal.id, &locale).await?;
            out.push(ProposalWithTranslationsDto::from_parts(proposal, sections));
        }
        return Ok((
            StatusCode::OK,
            Json(ProposalsListResponse::WithTranslations(out)),
        ));
    }

    let proposals = proposal::list_localized(&state.db, &workflow_step_id, &locale).await?;
    let mut out = Vec::with_capacity(proposals.len());
    for proposal in proposals {
        let sections = proposal_section::list_localized(&state.db, &proposal.id, &locale).await?;
        out.push(LocalizedProposalDto::from_parts(proposal, sections));
    }

    Ok((StatusCode::OK, Json(ProposalsListResponse::Localized(out))))
}

#[instrument(err(Debug), skip(state))]
async fn delete_proposal(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Path(proposal_id): Path<Uuid>,
) -> Result<(StatusCode, Json<ProposalDto>), ComhairleError> {
    // Fetch sections before delete so the response echoes what was removed
    // (the DB cascade deletes them along with the proposal).
    let sections = proposal_section::list(&state.db, &proposal_id).await?;
    let proposal = proposal::delete(&state.db, &proposal_id).await?;
    Ok((
        StatusCode::OK,
        Json(ProposalDto::from_parts(proposal, sections)),
    ))
}

#[derive(Deserialize, JsonSchema, Debug)]
struct CreateSectionRequest {
    /// Section body (rich text).
    body: String,
    /// Optional explicit position; appended to the end when omitted.
    #[serde(default)]
    position: Option<i32>,
}

#[instrument(err(Debug), skip(state))]
async fn create_proposal_section(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(_user): RequiredAdminUser,
    LocaleExtractor(locale): LocaleExtractor,
    Path(proposal_id): Path<Uuid>,
    Json(payload): Json<CreateSectionRequest>,
) -> Result<(StatusCode, Json<ProposalSectionDto>), ComhairleError> {
    let position = match payload.position {
        Some(position) => position,
        None => proposal_section::next_position(&state.db, &proposal_id).await?,
    };
    let section =
        proposal_section::create(&state.db, &proposal_id, position, &payload.body, &locale).await?;
    Ok((StatusCode::CREATED, Json(section.into())))
}

#[instrument(err(Debug), skip(state))]
async fn delete_proposal_section(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Path((_proposal_id, section_id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<ProposalSectionDto>), ComhairleError> {
    let section = proposal_section::delete(&state.db, &section_id).await?;
    Ok((StatusCode::OK, Json(section.into())))
}

/// Record a participant's response to a proposal.
///
/// Refused for a sealed participant: once they have finished a conversation that does not
/// allow revisits afterwards, their contributions are closed. This is the backend half
/// of the seal, and the only one of the Waves tools whose participant writes reach us at all
/// (surveys and Polis post directly to their own servers). See ADR-0016.
#[instrument(err(Debug), skip(state))]
async fn create_proposal_response(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path(proposal_id): Path<Uuid>,
    Json(payload): Json<CreateResponse>,
) -> Result<(StatusCode, Json<ProposalResponseDto>), ComhairleError> {
    let proposal = proposal::get_by_id(&state.db, &proposal_id).await?;
    let step = workflow_step::get_by_id(&state.db, &proposal.workflow_step_id).await?;

    if user_progress::is_sealed(&state.db, &user.id, &step.workflow_id).await? {
        return Err(ComhairleError::ParticipantSealed);
    }

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

#[derive(Deserialize, JsonSchema, Debug)]
struct GetInsightsQuery {
    workflow_step_id: Uuid,
}

#[derive(Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
struct RankedProposal {
    alignment_rating: f64,
    #[serde(flatten)]
    proposal: LocalizedProposalDto,
    responses: Vec<ProposalResponseDto>,
}

#[derive(Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
struct PrioritizationInsightsResponse {
    ranked_proposals: Vec<RankedProposal>,
}

#[instrument(err(Debug), skip(state))]
async fn get_prioritization_insights(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    Query(GetInsightsQuery { workflow_step_id }): Query<GetInsightsQuery>,
    LocaleExtractor(locale): LocaleExtractor,
) -> Result<(StatusCode, Json<PrioritizationInsightsResponse>), ComhairleError> {
    let workflow_step = workflow_step::get_by_id(&state.db, &workflow_step_id).await?;
    let tool_config = match workflow_step.tool_config {
        Some(ToolConfig::Prioritization(config)) => config,
        None => return Err(ComhairleError::ConversationNotLive),
        _ => {
            return Err(ComhairleError::WorkflowStepHasWrongType(
                "Prioritization type required".to_string(),
            ));
        }
    };

    let alignment_question_id = if let Some(q_id) = tool_config.alignment_question_id {
        q_id
    } else {
        return Err(ComhairleError::Unprocessable(
            "Cannot generate insights. Missing alignment question".to_string(),
        ));
    };

    let proposals = proposal::list_localized(&state.db, &workflow_step_id, &locale).await?;
    // let mut proposal_dtos = Vec::with_capacity(proposals.len());
    let mut ranked_proposals = Vec::with_capacity(proposals.len());

    for proposal in proposals {
        let sections = proposal_section::list_localized(&state.db, &proposal.id, &locale).await?;
        let proposal_responses = proposal_response::list(
            &state.db,
            &proposal.id,
            ProposalResponseFilterOptions,
            ProposalResponseOrderOptions,
        )
        .await?;

        let proposal_dto = LocalizedProposalDto::from_parts(proposal, sections);

        let mut alignment_score: f64 = 0.0;
        for response in &proposal_responses {
            let Some(alignment_question_response) = response.response.0.iter().find_map(|r| {
                // Discard responses that don't match alignment question
                if r.question_id != alignment_question_id {
                    return None;
                }

                // Discard text values
                match r.value {
                    ResponseValue::Number(v) => Some(v),
                    ResponseValue::Text(_) => None,
                }
            }) else {
                continue;
            };

            alignment_score += alignment_question_response
        }

        ranked_proposals.push(RankedProposal {
            alignment_rating: alignment_score,
            proposal: proposal_dto,
            responses: proposal_responses.into_iter().map(Into::into).collect(),
        })
    }

    ranked_proposals.sort_by(|a, b| b.alignment_rating.total_cmp(&a.alignment_rating));

    Ok((
        StatusCode::OK,
        Json(PrioritizationInsightsResponse { ranked_proposals }),
    ))
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

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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
                    "sections": ["Something to propose"]
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

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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
                    "sections": ["Something to propose"]
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
                    "sections": ["Something to propose"]
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
                    "sections": ["Something to propose"]
                })
                .to_string()
                .into(),
            )
            .await?;

        let (_, value, _) = session
            .get(
                &app,
                &format!(
                    "/tools/prioritization/proposals?workflowStepId={}",
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

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_new_proposal_response_via_api(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let workflow_id = get_random_workflow_id(&app, &mut session).await?;
        let workflow_step = session
            .create_prioritization_workflow_step(&app, &conversation_id, &workflow_id)
            .await?;

        let (proposal, _) = proposal::create(
            &pool,
            &workflow_step.id,
            &CreateProposal {
                title: "A new proposal".to_string(),
                sections: vec!["Test proposal".to_string()],
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
                    value: (-1.0_f64).into(),
                    section_id: None,
                },
                Response {
                    question_id: tool_config.questions[1].id,
                    value: 0.5_f64.into(),
                    section_id: None,
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

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_list_proposal_responses_via_api(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let workflow_id = get_random_workflow_id(&app, &mut session).await?;
        let workflow_step = session
            .create_prioritization_workflow_step(&app, &conversation_id, &workflow_id)
            .await?;

        let (proposal_a, _) = proposal::create(
            &pool,
            &workflow_step.id,
            &CreateProposal {
                title: "Proposal A".to_string(),
                sections: vec!["Proposal A".to_string()],
            },
            "en",
        )
        .await?;
        let (proposal_b, _) = proposal::create(
            &pool,
            &workflow_step.id,
            &CreateProposal {
                title: "Proposal B".to_string(),
                sections: vec!["Proposal B".to_string()],
            },
            "en",
        )
        .await?;

        let tool_config = match workflow_step.preview_tool_config {
            ToolConfig::Prioritization(config) => config,
            _ => panic!("Incorrect tool_config type"),
        };

        let create_response_a = CreateResponse {
            question_responses: vec![
                Response {
                    question_id: tool_config.questions.first().unwrap().id,
                    value: (-1.0_f64).into(),
                    section_id: None,
                },
                Response {
                    question_id: tool_config.questions[1].id,
                    value: 0.5_f64.into(),
                    section_id: None,
                },
            ],
        };
        let create_response_b = CreateResponse {
            question_responses: vec![
                Response {
                    question_id: tool_config.questions.first().unwrap().id,
                    value: 0.5_f64.into(),
                    section_id: None,
                },
                Response {
                    question_id: tool_config.questions[1].id,
                    value: 0.2_f64.into(),
                    section_id: None,
                },
            ],
        };

        // Two distinct users each respond to both proposals, so each proposal
        // ends up with two responses. Same-user re-posts now upsert (single row
        // per (proposal, user)) so duplicate users would collapse.
        session.signup_annon(&app).await?;
        session
            .post(
                &app,
                &format!(
                    "/tools/prioritization/proposals/{}/responses",
                    proposal_a.id
                ),
                json!(create_response_a).to_string().into(),
            )
            .await?;
        session
            .post(
                &app,
                &format!(
                    "/tools/prioritization/proposals/{}/responses",
                    proposal_b.id
                ),
                json!(create_response_b).to_string().into(),
            )
            .await?;

        session.signup_annon(&app).await?;
        session
            .post(
                &app,
                &format!(
                    "/tools/prioritization/proposals/{}/responses",
                    proposal_a.id
                ),
                json!(create_response_a).to_string().into(),
            )
            .await?;
        session
            .post(
                &app,
                &format!(
                    "/tools/prioritization/proposals/{}/responses",
                    proposal_b.id
                ),
                json!(create_response_b).to_string().into(),
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
}
