use std::{collections::HashMap, sync::Arc};

use aide::{
    OperationIo,
    axum::{
        ApiRouter,
        routing::{get_with, post_with, put_with},
    },
};
use async_trait::async_trait;
use axum::{
    body::Body,
    extract::{Json, Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    ComhairleState,
    bot_service::AgentConversationRequest,
    error::ComhairleError,
    models::{
        bot_service_user_session::{self, BotServiceSessionContext},
        thinking_space_answer::{
            self, AnswerStatus, CreateAnswer, ThinkingSpaceAnswer,
            ThinkingSpaceAnswerFilterOptions, UpdateAnswer,
        },
        thinking_space_follow_up_question::{
            self, CreateFollowUpQuestions, ThinkingSpaceFollowUpQuestion,
            ThinkingSpaceFollowUpQuestionFilterOptions, UpdateFollowUpQuestions,
        },
        thinking_space_summary::{
            self, CreateSummary, ThinkingSpaceSummary, ThinkingSpaceSummaryFilterOptions,
            UpdateSummary,
        },
        workflow_step,
    },
    routes::auth::{RequiredAdminUser, RequiredUser},
    tools::ToolConfig,
};

use super::{ToolConfigSanitize, ToolImpl};

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, PartialEq)]
pub struct ThinkingSpaceQuestion {
    pub id: Uuid,
    pub text: String,
    /// Admin-authored description of *why* this question is being asked — fed to
    /// the AI as `question_intent` so it can generate sharper follow-ups. Never
    /// shown to participants.
    pub intent: String,
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, PartialEq)]
pub struct ThinkingSpaceToolConfig {
    pub topic: String,
    pub root_questions: Vec<ThinkingSpaceQuestion>,
    pub follow_up_rounds_count: u8,
}

impl ToolConfigSanitize for ThinkingSpaceToolConfig {
    fn sanitize(&self) -> Self {
        self.clone()
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, PartialEq)]
pub struct ThinkingSpaceSetupQuestion {
    pub text: String,
    pub intent: String,
}

impl From<ThinkingSpaceSetupQuestion> for ThinkingSpaceQuestion {
    fn from(q: ThinkingSpaceSetupQuestion) -> Self {
        Self {
            id: Uuid::new_v4(),
            text: q.text,
            intent: q.intent,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct ThinkingSpaceToolSetup {
    pub topic: String,
    pub root_questions: Vec<ThinkingSpaceSetupQuestion>,
    pub follow_up_rounds_count: u8,
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct ThinkingSpaceReport;

/// Zero-sized marker type for ThinkingSpace tool implementation
pub struct ThinkingSpaceTool;

#[async_trait]
impl ToolImpl for ThinkingSpaceTool {
    type Config = ThinkingSpaceToolConfig;
    type Setup = ThinkingSpaceToolSetup;
    type Report = ThinkingSpaceReport;

    async fn setup(
        setup: &Self::Setup,
        _state: &Arc<ComhairleState>,
    ) -> Result<Self::Config, ComhairleError> {
        thinking_space_setup(setup).await
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

    async fn delete(
        _config: &Self::Config,
        state: &Arc<ComhairleState>,
        workflow_step_id: &Uuid,
    ) -> Result<(), ComhairleError> {
        let sessions = bot_service_user_session::list_by_workflow_step_id_and_context(
            &state.db,
            workflow_step_id,
            BotServiceSessionContext::ThinkingSpace,
        )
        .await?;

        if sessions.is_empty() {
            return Ok(());
        }

        let bot_service = state.required_bot_service()?;
        let bot_service_config = state
            .config
            .bot_service
            .as_ref()
            .ok_or(ComhairleError::NoBotServiceConfigured)?;

        for session in &sessions {
            bot_service
                .delete_agent_session(
                    &session.bot_service_session_id,
                    &bot_service_config.thinking_space_agent_id,
                )
                .await?;
        }

        let session_ids: Vec<Uuid> = sessions.into_iter().map(|session| session.id).collect();
        bot_service_user_session::delete_by_ids(&state.db, &session_ids).await?;

        Ok(())
    }

    fn routes(state: &Arc<ComhairleState>) -> ApiRouter {
        ApiRouter::new()
            .api_route(
                "/thinking_space",
                post_with(converse, |op| {
                    op.tag("Tools")
                        .summary("Converse with thinking space")
                        .security_requirement("JWT")
                        .description(
                            "
Streamed LLM response.
⚠️ This endpoint returns a streaming response on success.
Generated API clients are NOT suitable for consuming this endpoint.
Use a raw HTTP request and process the response body incrementally.
",
                        )
                }),
            )
            .api_route(
                "/thinking_space/answers",
                post_with(create_thinking_space_answer, |op| {
                    op.id("CreateThinkingSpaceAnswer")
                        .summary("Create thinking space answer")
                        .description("Create an answer for thinking space workflow step question")
                        .security_requirement("JWT")
                        .response::<201, Json<ThinkingSpaceAnswerDto>>()
                }),
            )
            .api_route(
                "/thinking_space/answers",
                get_with(list_thinking_space_answers, |op| {
                    op.id("ListThinkingSpaceAnswers")
                        .summary("List thinking space answers")
                        .description("List answer for thinking space workflow step")
                        .security_requirement("JWT")
                        .response::<200, Json<Vec<ThinkingSpaceAnswerDto>>>()
                }),
            )
            .api_route(
                "/thinking_space/answers/{answer_id}",
                put_with(update_thinking_space_answer, |op| {
                    op.id("UpdateThinkingSpaceAnswer")
                        .summary("Update thinking space answer")
                        .description("Update an answer for thinking space workflow step question")
                        .security_requirement("JWT")
                        .response::<200, Json<ThinkingSpaceAnswerDto>>()
                }),
            )
            .api_route(
                "/thinking_space/summaries/generate",
                post_with(generate_thinking_space_summary, |op| {
                    op.id("GenerateThinkingSpaceSummary")
                        .summary("Generate thinking space summary")
                        .description("Generates a thinking space summary via bot service agent")
                        .security_requirement("JWT")
                        .response::<201, Json<ThinkingSpaceSummaryDto>>()
                }),
            )
            .api_route(
                "/thinking_space/summaries",
                post_with(update_or_create_thinking_space_summary, |op| {
                    op.id("UpdateOrCreateThinkingSpaceSummary")
                        .summary("Update or create thinking space summary")
                        .description("Update a summary if already exists or create a new summary")
                        .security_requirement("JWT")
                        .response::<200, Json<ThinkingSpaceSummaryDto>>()
                        .response::<201, Json<ThinkingSpaceSummaryDto>>()
                }),
            )
            .api_route(
                "/thinking_space/summaries/{summary_id}",
                get_with(get_thinkin_space_summary, |op| {
                    op.id("GetThinkingSpaceSummary")
                        .summary("Get thinking space summary")
                        .description("Get a thinking space summary by id")
                        .security_requirement("JWT")
                        .response::<200, Json<ThinkingSpaceSummaryDto>>()
                }),
            )
            .api_route(
                "/thinking_space/summaries",
                get_with(list_thinking_space_summaries, |op| {
                    op.id("ListThinkingSpaceSummaries")
                        .summary("List thinking space summaries")
                        .description("List thinking space summaries")
                        .security_requirement("JWT")
                        .response::<200, Json<Vec<ThinkingSpaceSummaryDto>>>()
                }),
            )
            .api_route(
                "/thinking_space/follow_ups",
                post_with(create_thinking_space_follow_up_questions, |op| {
                    op.id("CreateThinkingSpaceFollowUpQuestions")
                        .summary("Create thinking space follow up questions")
                        .description("Create thinking space follow up questions")
                        .security_requirement("JWT")
                        .response::<201, Json<ThinkingSpaceFollowUpQuestionDto>>()
                }),
            )
            .api_route(
                "/thinking_space/follow_ups/{follow_up_id}",
                put_with(update_thinking_space_follow_up_questions, |op| {
                    op.id("UpdateThinkingSpaceFollowUpQuestions")
                        .summary("Update thinking space follow up questions")
                        .description("Update thinking space follow up questions")
                        .security_requirement("JWT")
                        .response::<200, Json<ThinkingSpaceFollowUpQuestionDto>>()
                }),
            )
            .api_route(
                "/thinking_space/follow_ups",
                get_with(list_thinking_space_follow_up_questions, |op| {
                    op.id("ListThinkingSpaceFollowUpQuestions")
                        .summary("List thinking space follow up questions")
                        .description("List thinking space follow up questions")
                        .security_requirement("JWT")
                        .response::<200, Json<Vec<ThinkingSpaceFollowUpQuestionDto>>>()
                }),
            )
            .api_route(
                "/thinking_space/insights",
                get_with(get_thinking_space_insights, |op| {
                    op.id("GetThinkingSpaceInsights")
                        .summary("Get thinking space insights data")
                        .description("Get thinking space insights data")
                        .security_requirement("JWT")
                        .response::<200, Json<ThinkingSpaceInsightsResponse>>()
                }),
            )
            .with_state(state.clone())
    }
}

async fn thinking_space_setup(
    config: &ThinkingSpaceToolSetup,
) -> Result<ThinkingSpaceToolConfig, ComhairleError> {
    Ok(ThinkingSpaceToolConfig {
        topic: config.topic.clone(),
        root_questions: config
            .root_questions
            .clone()
            .into_iter()
            .map(Into::into)
            .collect(),
        follow_up_rounds_count: config.follow_up_rounds_count,
    })
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThinkingSpaceAnswerDto {
    pub id: Uuid,
    pub workflow_step_id: Uuid,
    pub root_question_id: Option<Uuid>,
    pub is_follow_up: bool,
    pub question: String,
    pub answer: String,
    pub other_questions: Vec<String>,
    pub status: AnswerStatus,
}

impl From<ThinkingSpaceAnswer> for ThinkingSpaceAnswerDto {
    fn from(a: ThinkingSpaceAnswer) -> Self {
        Self {
            id: a.id,
            workflow_step_id: a.workflow_step_id,
            root_question_id: a.root_question_id,
            is_follow_up: a.is_follow_up,
            answer: a.answer,
            question: a.question,
            other_questions: a.other_questions,
            status: a.status,
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThinkingSpaceSummaryDto {
    pub id: Uuid,
    pub workflow_step_id: Uuid,
    pub user_id: Uuid,
    pub summary: String,
    pub is_ai_generated: bool,
    pub ai_generated_summary: Option<String>,
}

impl From<ThinkingSpaceSummary> for ThinkingSpaceSummaryDto {
    fn from(s: ThinkingSpaceSummary) -> Self {
        Self {
            id: s.id,
            workflow_step_id: s.workflow_step_id,
            user_id: s.user_id,
            summary: s.summary,
            is_ai_generated: s.is_ai_generated,
            ai_generated_summary: s.ai_generated_summary,
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThinkingSpaceFollowUpQuestionDto {
    pub id: Uuid,
    pub workflow_step_id: Uuid,
    pub user_id: Uuid,
    pub root_question_id: Uuid,
    pub follow_up_questions: Vec<String>,
}

impl From<ThinkingSpaceFollowUpQuestion> for ThinkingSpaceFollowUpQuestionDto {
    fn from(f: ThinkingSpaceFollowUpQuestion) -> Self {
        Self {
            id: f.id,
            workflow_step_id: f.workflow_step_id,
            user_id: f.user_id,
            root_question_id: f.root_question_id,
            follow_up_questions: f.follow_up_questions,
        }
    }
}

#[derive(Deserialize, Debug, JsonSchema, Clone, PartialEq)]
pub struct ConversationRequest {
    pub workflow_step_id: Uuid,
    pub history: String,
    pub starting_question: String,
    pub question_intent: String,
}

// Wrapper struct required as a workaround to generate documentation for handlers
// that return `axum::body::Body` for streamed responses.
#[derive(OperationIo)]
pub struct StreamBody(Body);

impl IntoResponse for StreamBody {
    fn into_response(self) -> Response {
        self.0.into_response()
    }
}

#[instrument(err(Debug), skip(state))]
async fn converse(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Json(payload): Json<ConversationRequest>,
) -> Result<StreamBody, ComhairleError> {
    let bot_service = state.required_bot_service()?;
    let bot_service_config = state
        .config
        .bot_service
        .as_ref()
        .ok_or(ComhairleError::NoBotServiceConfigured)?;

    let workflow_step = workflow_step::get_by_id(&state.db, &payload.workflow_step_id).await?;

    // TODO: think more creafully how we handle this in preview mode
    let tool_config = match (workflow_step.tool_config, workflow_step.preview_tool_config) {
        (Some(ToolConfig::ThinkingSpace(config)), _) => config,
        (None, ToolConfig::ThinkingSpace(config)) => config,

        _ => {
            return Err(ComhairleError::ToolConfigError(
                "incorrect config type".to_string(),
            ));
        }
    };

    let session = bot_service_user_session::get_or_create(
        &state,
        BotServiceSessionContext::ThinkingSpace,
        &user.id,
        None,
        Some(&payload.workflow_step_id),
    )
    .await?;

    let params = AgentConversationRequest {
        question: payload.history.clone(),
        topic: Some(tool_config.topic.clone()),
        history: Some(payload.history),
        starting_question: Some(payload.starting_question),
        question_intent: Some(payload.question_intent),
        ..Default::default()
    };
    let stream = bot_service
        .converse_with_agent(
            Some(&session.bot_service_session_id),
            &bot_service_config.thinking_space_agent_id,
            params,
        )
        .await?;

    let body = Body::from_stream(stream);
    Ok(StreamBody(body))
}

#[derive(Deserialize, Debug, JsonSchema, Default)]
pub struct CreateAnswerRequest {
    workflow_step_id: Uuid,
    question: String,
    answer: String,
    other_questions: Option<Vec<String>>,
    root_question_id: Option<Uuid>,
    is_follow_up: Option<bool>,
}

#[instrument(err(Debug), skip(state))]
async fn create_thinking_space_answer(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Json(payload): Json<CreateAnswerRequest>,
) -> Result<(StatusCode, Json<ThinkingSpaceAnswerDto>), ComhairleError> {
    let params = CreateAnswer {
        question: payload.question,
        answer: payload.answer,
        other_questions: payload.other_questions,
        root_question_id: payload.root_question_id,
        is_follow_up: payload.is_follow_up,
    };

    let answer =
        thinking_space_answer::create(&state.db, &payload.workflow_step_id, &user.id, &params)
            .await?;

    Ok((StatusCode::CREATED, Json(answer.into())))
}

#[derive(Deserialize, JsonSchema, Debug)]
struct ListThinkingSpaceAnswersQuery {
    workflow_step_id: Uuid,
}

#[instrument(err(Debug), skip(state))]
async fn list_thinking_space_answers(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Query(filter_options): Query<ThinkingSpaceAnswerFilterOptions>,
    Query(ListThinkingSpaceAnswersQuery { workflow_step_id }): Query<ListThinkingSpaceAnswersQuery>,
) -> Result<(StatusCode, Json<Vec<ThinkingSpaceAnswerDto>>), ComhairleError> {
    let answers = thinking_space_answer::list(&state.db, &workflow_step_id, filter_options)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();

    Ok((StatusCode::OK, Json(answers)))
}

#[instrument(err(Debug), skip(state))]
async fn update_thinking_space_answer(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path(answer_id): Path<Uuid>,
    Json(payload): Json<UpdateAnswer>,
) -> Result<(StatusCode, Json<ThinkingSpaceAnswerDto>), ComhairleError> {
    let answer = thinking_space_answer::update(&state.db, &answer_id, &payload).await?;

    Ok((StatusCode::OK, Json(answer.into())))
}

#[derive(Deserialize, JsonSchema, Debug)]
struct GenerateThinkingSpaceSummary {
    workflow_step_id: Uuid,
}

#[derive(Serialize, JsonSchema, Debug)]
struct ThinkingSpaceSummaryQa {
    question: String,
    answer: String,
}

impl From<ThinkingSpaceAnswer> for ThinkingSpaceSummaryQa {
    fn from(a: ThinkingSpaceAnswer) -> Self {
        Self {
            answer: a.answer,
            question: a.question,
        }
    }
}

#[instrument(err(Debug), skip(state))]
async fn generate_thinking_space_summary(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Json(payload): Json<GenerateThinkingSpaceSummary>,
) -> Result<(StatusCode, Json<ThinkingSpaceSummaryDto>), ComhairleError> {
    let bot_service = state.required_bot_service()?;
    let bot_service_config = state
        .config
        .bot_service
        .as_ref()
        .ok_or(ComhairleError::NoBotServiceConfigured)?;

    let workflow_step = workflow_step::get_by_id(&state.db, &payload.workflow_step_id).await?;

    // TODO: think more creafully how we handle this in preview mode
    let tool_config = match (workflow_step.tool_config, workflow_step.preview_tool_config) {
        (Some(ToolConfig::ThinkingSpace(config)), _) => config,
        (None, ToolConfig::ThinkingSpace(config)) => config,

        _ => {
            return Err(ComhairleError::ToolConfigError(
                "incorrect config type".to_string(),
            ));
        }
    };

    let filter_options = ThinkingSpaceAnswerFilterOptions {
        user_id: Some(user.id),
        ..Default::default()
    };
    let answers: Vec<ThinkingSpaceSummaryQa> =
        thinking_space_answer::list(&state.db, &payload.workflow_step_id, filter_options)
            .await?
            .into_iter()
            .map(Into::into)
            .collect();
    let answers_json = serde_json::json!(answers).to_string();

    let params = AgentConversationRequest {
        question: "".to_string(),
        topic: Some(tool_config.topic.clone()),
        survey_responses: Some(answers_json),
        ..Default::default()
    };

    // Create a dedicated session per request so concurrent summary generations
    // from different users cannot share bot-service session state.
    let (_, agent_session) = bot_service
        .create_agent_session(&bot_service_config.thinking_space_summary_agent_id)
        .await?;

    let stream = bot_service
        .converse_with_agent(
            Some(&agent_session.id),
            &bot_service_config.thinking_space_summary_agent_id,
            params,
        )
        .await?;

    // // Parse SSE stream to get final event with complete summary
    let sse_events = bot_service.parse_sse_stream_to_events(stream).await?;

    let final_event = sse_events
        .iter()
        .rfind(|e| e.event == "node_finished" && e.component_type == Some("Message".to_string()))
        .ok_or_else(|| {
            ComhairleError::CorruptedData("Missing summary from bot service agent".to_string())
        })?;

    let summary_content = final_event
        .clone()
        .content
        .ok_or(ComhairleError::CorruptedData(
            "Missing summary from bot service agent".to_string(),
        ))?;

    let create_summary = CreateSummary {
        summary: summary_content,
        is_ai_generated: Some(true),
    };
    let summary = thinking_space_summary::create(
        &state.db,
        user.id,
        payload.workflow_step_id,
        &create_summary,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(summary.into())))
}

#[derive(Deserialize, JsonSchema, Debug)]
struct ThinkingSpaceQuery {
    workflow_step_id: Uuid,
}

#[instrument(err(Debug), skip(state))]
async fn get_thinkin_space_summary(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Query(ThinkingSpaceQuery {
        workflow_step_id: _,
    }): Query<ThinkingSpaceQuery>,
    Path(summary_id): Path<Uuid>,
) -> Result<(StatusCode, Json<ThinkingSpaceSummaryDto>), ComhairleError> {
    let summary = thinking_space_summary::get_by_id(&state.db, summary_id).await?;

    // Hide other users' summaries behind a 404 rather than 403 so we don't
    // leak that a summary with this id exists.
    if summary.user_id != user.id {
        return Err(ComhairleError::ResourceNotFound(
            "ThinkingSpaceSummary".to_string(),
        ));
    }

    Ok((StatusCode::OK, Json(summary.into())))
}

#[derive(Deserialize, JsonSchema, Debug)]
struct UpdateCreateThinkingSpace {
    workflow_step_id: Uuid,
    summary_id: Option<Uuid>,
    summary: String,
}

#[instrument(err(Debug), skip(state))]
async fn update_or_create_thinking_space_summary(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Json(payload): Json<UpdateCreateThinkingSpace>,
) -> Result<(StatusCode, Json<ThinkingSpaceSummaryDto>), ComhairleError> {
    match payload.summary_id {
        Some(summary_id) => {
            let update_summary = UpdateSummary {
                summary: payload.summary,
            };
            let summary =
                thinking_space_summary::update(&state.db, summary_id, &update_summary).await?;

            Ok((StatusCode::OK, Json(summary.into())))
        }
        None => {
            let create_summary = CreateSummary {
                summary: payload.summary,
                is_ai_generated: Some(false),
            };
            let summary = thinking_space_summary::create(
                &state.db,
                user.id,
                payload.workflow_step_id,
                &create_summary,
            )
            .await?;

            Ok((StatusCode::CREATED, Json(summary.into())))
        }
    }
}

#[instrument(err(Debug), skip(state))]
async fn list_thinking_space_summaries(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Query(mut filter_options): Query<ThinkingSpaceSummaryFilterOptions>,
    Query(ThinkingSpaceQuery { workflow_step_id }): Query<ThinkingSpaceQuery>,
) -> Result<(StatusCode, Json<Vec<ThinkingSpaceSummaryDto>>), ComhairleError> {
    // Scope to the authenticated user regardless of any caller-supplied user_id
    // — summaries are private to the user that authored them.
    filter_options.user_id = Some(user.id);

    let summaries = thinking_space_summary::list(&state.db, &workflow_step_id, filter_options)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();

    Ok((StatusCode::OK, Json(summaries)))
}

#[instrument(err(Debug), skip(state))]
async fn create_thinking_space_follow_up_questions(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Json(payload): Json<CreateFollowUpQuestions>,
) -> Result<(StatusCode, Json<ThinkingSpaceFollowUpQuestionDto>), ComhairleError> {
    let follow_ups =
        thinking_space_follow_up_question::create(&state.db, user.id, &payload).await?;

    Ok((StatusCode::CREATED, Json(follow_ups.into())))
}

#[instrument(err(Debug), skip(state))]
async fn update_thinking_space_follow_up_questions(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path(follow_up_id): Path<Uuid>,
    Json(payload): Json<UpdateFollowUpQuestions>,
) -> Result<(StatusCode, Json<ThinkingSpaceFollowUpQuestionDto>), ComhairleError> {
    let follow_ups =
        thinking_space_follow_up_question::update(&state.db, follow_up_id, &payload).await?;

    Ok((StatusCode::OK, Json(follow_ups.into())))
}

#[instrument(err(Debug), skip(state))]
async fn list_thinking_space_follow_up_questions(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Query(ThinkingSpaceQuery { workflow_step_id }): Query<ThinkingSpaceQuery>,
    Query(filter_options): Query<ThinkingSpaceFollowUpQuestionFilterOptions>,
) -> Result<(StatusCode, Json<Vec<ThinkingSpaceFollowUpQuestionDto>>), ComhairleError> {
    let follow_ups =
        thinking_space_follow_up_question::list(&state.db, &workflow_step_id, filter_options)
            .await?
            .into_iter()
            .map(Into::into)
            .collect();

    Ok((StatusCode::OK, Json(follow_ups)))
}

#[derive(Serialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct AnswersByRoot {
    root: ThinkingSpaceAnswerDto,
    follow_ups: Vec<ThinkingSpaceAnswerDto>,
}

#[derive(Serialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct ThinkingSpaceUserInsights {
    user_id: Uuid,
    summary: ThinkingSpaceSummaryDto,
    answers: Vec<AnswersByRoot>,
}

#[derive(Serialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct ThinkingSpaceInsightsResponse {
    users: Vec<ThinkingSpaceUserInsights>,
}

#[instrument(err(Debug), skip(state))]
async fn get_thinking_space_insights(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    Query(ThinkingSpaceQuery { workflow_step_id }): Query<ThinkingSpaceQuery>,
) -> Result<(StatusCode, Json<ThinkingSpaceInsightsResponse>), ComhairleError> {
    let summaries = thinking_space_summary::list(
        &state.db,
        &workflow_step_id,
        ThinkingSpaceSummaryFilterOptions {
            ..Default::default()
        },
    )
    .await?;
    let answers = thinking_space_answer::list(
        &state.db,
        &workflow_step_id,
        ThinkingSpaceAnswerFilterOptions {
            ..Default::default()
        },
    )
    .await?;

    let mut roots_by_user: HashMap<Uuid, Vec<ThinkingSpaceAnswerDto>> = HashMap::new();
    let mut follow_ups_by_root: HashMap<Uuid, Vec<ThinkingSpaceAnswerDto>> = HashMap::new();
    for answer in answers {
        match answer.root_question_id {
            Some(root_id) => follow_ups_by_root
                .entry(root_id)
                .or_default()
                .push(answer.into()),
            None => roots_by_user
                .entry(answer.user_id)
                .or_default()
                .push(answer.into()),
        }
    }

    let user_insights: Vec<ThinkingSpaceUserInsights> = summaries
        .into_iter()
        .map(|summary| {
            let roots = roots_by_user.remove(&summary.user_id).unwrap_or_default();

            let answers = roots
                .into_iter()
                .map(|root| {
                    let follow_ups = follow_ups_by_root.remove(&root.id).unwrap_or_default();
                    AnswersByRoot { root, follow_ups }
                })
                .collect();

            ThinkingSpaceUserInsights {
                user_id: summary.user_id,
                summary: summary.into(),
                answers,
            }
        })
        .collect();

    Ok((
        StatusCode::OK,
        Json(ThinkingSpaceInsightsResponse {
            users: user_insights,
        }),
    ))
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
        test_helpers::thinking_space_tool_config,
    };

    use super::*;

    use std::error::Error;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_thinking_space_answer_via_api(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
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
                    "description": "A test workflow_step with thinking space",
                    "is_offline": false,
                    "required": false,
                    "tool_setup": thinking_space_tool_config(),
                }),
            )
            .await?;
        let workflow_step: WorkflowStepDto = serde_json::from_value(value)?;

        let (_, value, _) = session
            .post(
                &app,
                "/tools/thinking_space/answers",
                json!({
                    "workflow_step_id": workflow_step.id,
                    "question": "A test question",
                    "answer": "A test answer",
                })
                .to_string()
                .into(),
            )
            .await?;
        let answer_a: ThinkingSpaceAnswerDto = serde_json::from_value(value)?;

        let (_, value, _) = session
            .post(
                &app,
                "/tools/thinking_space/answers",
                json!({
                    "workflow_step_id": workflow_step.id,
                    "question": "A test question",
                    "answer": "A test answer",
                    "is_follow_up": true,
                    "root_question_id": answer_a.id
                })
                .to_string()
                .into(),
            )
            .await?;
        let answer_b: ThinkingSpaceAnswerDto = serde_json::from_value(value)?;

        assert_eq!(
            answer_a.workflow_step_id, workflow_step.id,
            "incorrect workflow_step_id"
        );
        assert_eq!(
            answer_b.root_question_id.unwrap(),
            answer_a.id,
            "incorrect root_question_id"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_list_thinking_space_answers_via_api(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
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
                    "description": "A test workflow_step with thinking space",
                    "is_offline": false,
                    "required": false,
                    "tool_setup": thinking_space_tool_config(),
                }),
            )
            .await?;
        let workflow_step: WorkflowStepDto = serde_json::from_value(value)?;

        session
            .post(
                &app,
                "/tools/thinking_space/answers",
                json!({
                    "workflow_step_id": workflow_step.id,
                    "question": "A test question A",
                    "answer": "A test answer A",
                })
                .to_string()
                .into(),
            )
            .await?;
        session
            .post(
                &app,
                "/tools/thinking_space/answers",
                json!({
                    "workflow_step_id": workflow_step.id,
                    "question": "A test question B",
                    "answer": "A test answer B",
                })
                .to_string()
                .into(),
            )
            .await?;
        session
            .post(
                &app,
                "/tools/thinking_space/answers",
                json!({
                    "workflow_step_id": workflow_step.id,
                    "question": "A test question C",
                    "answer": "A test answer C",
                })
                .to_string()
                .into(),
            )
            .await?;

        let (_, value, _) = session
            .get(
                &app,
                &format!(
                    "/tools/thinking_space/answers?workflow_step_id={}",
                    workflow_step.id
                ),
            )
            .await?;
        let answers: Vec<ThinkingSpaceAnswerDto> = serde_json::from_value(value)?;

        assert_eq!(answers.len(), 3, "incorrect total");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_update_thinking_space_answer_via_api(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
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
                    "description": "A test workflow_step with thinking space",
                    "is_offline": false,
                    "required": false,
                    "tool_setup": thinking_space_tool_config(),
                }),
            )
            .await?;
        let workflow_step: WorkflowStepDto = serde_json::from_value(value)?;

        let (_, value, _) = session
            .post(
                &app,
                "/tools/thinking_space/answers",
                json!({
                    "workflow_step_id": workflow_step.id,
                    "question": "A test question",
                    "answer": "A test answer",
                })
                .to_string()
                .into(),
            )
            .await?;
        let new_answer: ThinkingSpaceAnswerDto = serde_json::from_value(value)?;

        assert_eq!(
            new_answer.status,
            AnswerStatus::Pending,
            "incorrect status before update"
        );
        assert_eq!(
            new_answer.answer,
            "A test answer".to_string(),
            "incorrect answer before update"
        );

        let (_, value, _) = session
            .put(
                &app,
                &format!("/tools/thinking_space/answers/{}", new_answer.id),
                json!({
                    "answer": "An updated answer",
                    "status": "approved"
                })
                .to_string()
                .into(),
            )
            .await?;
        let answer: ThinkingSpaceAnswerDto = serde_json::from_value(value)?;

        assert_eq!(
            answer.status,
            AnswerStatus::Approved,
            "incorrect status after update"
        );
        assert_eq!(
            answer.answer,
            "An updated answer".to_string(),
            "incorrect answer after update"
        );

        Ok(())
    }
}
