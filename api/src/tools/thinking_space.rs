use std::collections::HashMap;
use std::sync::Arc;

use aide::axum::ApiRouter;
use aide::axum::routing::{get_with, post_with, put_with};
use async_trait::async_trait;
use axum::extract::{Json, Path, Query, State};
use axum::http::StatusCode;
use comhairle_macros::TranslatableJson;
use schemars::JsonSchema;
use sensemakar::thinking_space::{FollowUpQuestionGenerator, SummaryGenerator};
use sensemakar_types::thinking_space::{Answer, Question, QuestionChain, QuestionWithAnswer};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::instrument;
use uuid::Uuid;

use super::{ToolConfig, ToolConfigSanitize, ToolImpl};
use crate::models::thinking_space_answer::{
    self, AnswerStatus, CreateAnswer, ThinkingSpaceAnswer, ThinkingSpaceAnswerFilterOptions,
    UpdateAnswer,
};
use crate::models::thinking_space_follow_up_question::{
    self, CreateFollowUpQuestions, ThinkingSpaceFollowUpQuestion,
    ThinkingSpaceFollowUpQuestionFilterOptions, UpdateFollowUpQuestions,
};
use crate::models::thinking_space_summary::{
    self, CreateSummary, ThinkingSpaceSummary, ThinkingSpaceSummaryFilterOptions, UpdateSummary,
};
use crate::models::translations::{
    BuildTextTranslation, TextContentId, TextFormat, localize_translations,
};
use crate::models::workflow_step;
use crate::routes::auth::{RequiredAdminUser, RequiredUser};
use crate::routes::translations::LocaleExtractor;
use crate::{ComhairleError, ComhairleState, sensemakar_service};

/// Mirrors MAX_SUMMARY_WORDS in
/// ui/packages/comhairle/src/lib/tools/thinking_space/wordLimit.ts, which caps
/// how many words a participant can type when editing a summary — generated
/// drafts are asked to stay under the same budget so they don't immediately
/// land over that limit.
const MAX_SUMMARY_WORDS: usize = 500;

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, PartialEq, TranslatableJson)]
pub struct ThinkingSpaceQuestion {
    pub id: Uuid,
    #[translatable]
    pub text: TextContentId,
    /// Admin-authored description of *why* this question is being asked — fed to
    /// the AI as `question_intent` so it can generate sharper follow-ups. Never
    /// shown to participants.
    #[translatable]
    pub intent: TextContentId,
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, PartialEq, TranslatableJson)]
pub struct ThinkingSpaceToolConfig {
    #[translatable]
    pub topic: TextContentId,
    #[translatable]
    pub root_questions: Vec<ThinkingSpaceQuestion>,
    pub follow_up_rounds_count: u8,
}

impl ToolConfigSanitize for ThinkingSpaceToolConfig {
    fn sanitize(&self) -> Self {
        self.clone()
    }
}

// =================
// Setup structs
// =================

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, PartialEq)]
pub struct ThinkingSpaceSetupQuestion {
    pub text: String,
    pub intent: String,
}

impl ThinkingSpaceSetupQuestion {
    async fn build_with_translations(
        self,
        db: &PgPool,
        locale: &str,
    ) -> Result<ThinkingSpaceQuestion, ComhairleError> {
        Ok(ThinkingSpaceQuestion {
            id: Uuid::new_v4(),
            text: self
                .text
                .build_text_translation(db, locale, TextFormat::Plain)
                .await?,
            intent: self
                .intent
                .build_text_translation(db, locale, TextFormat::Plain)
                .await?,
        })
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct ThinkingSpaceToolSetup {
    pub topic: String,
    pub root_questions: Vec<ThinkingSpaceSetupQuestion>,
    pub follow_up_rounds_count: u8,
}

impl ThinkingSpaceToolSetup {
    async fn build_with_translations(
        self,
        db: &PgPool,
        locale: &str,
    ) -> Result<ThinkingSpaceToolConfig, ComhairleError> {
        Ok(ThinkingSpaceToolConfig {
            topic: self
                .topic
                .build_text_translation(db, locale, TextFormat::Plain)
                .await?,
            root_questions: {
                let mut built = Vec::with_capacity(self.root_questions.len());
                for question in self.root_questions {
                    built.push(question.build_with_translations(db, locale).await?);
                }
                built
            },
            follow_up_rounds_count: self.follow_up_rounds_count,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub struct ThinkingSpaceExIm;

#[derive(PartialEq, Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct ThinkingSpaceReport;

/// Zero-sized marker type for ThinkingSpace tool implementation
pub struct ThinkingSpaceTool;

#[async_trait]
impl ToolImpl for ThinkingSpaceTool {
    type Config = ThinkingSpaceToolConfig;
    type Setup = ThinkingSpaceToolSetup;
    type Report = ThinkingSpaceReport;
    type ExportImport = ThinkingSpaceExIm;

    async fn setup(
        setup: &Self::Setup,
        state: &Arc<ComhairleState>,
        locale: &str,
    ) -> Result<Self::Config, ComhairleError> {
        thinking_space_setup(&state.db, setup, locale).await
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
        _state: &Arc<ComhairleState>,
        _workflow_step_id: &Uuid,
    ) -> Result<(), ComhairleError> {
        Ok(())
    }

    async fn export_tool(_config: &Self::Config) -> Result<Self::ExportImport, ComhairleError> {
        Err(ComhairleError::NotImplemented(
            "Not implemented for this tool".to_string(),
        ))
    }

    async fn import_tool(
        _config: &Self::ExportImport,
        _state: &Arc<ComhairleState>,
    ) -> Result<Self::Config, ComhairleError> {
        Err(ComhairleError::NotImplemented(
            "Not implemented for this tool".to_string(),
        ))
    }

    fn routes() -> ApiRouter<Arc<ComhairleState>> {
        ApiRouter::new()
            .api_route(
                "/thinking_space",
                post_with(converse, |op| {
                    op.id("ConverseThinkingSpace")
                        .tag("Tools")
                        .summary("Converse with thinking space")
                        .description(
                            "Generate candidate follow-up questions for a thinking space root answer",
                        )
                        .security_requirement("JWT")
                        .response::<200, Json<ConverseResponseDto>>()
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
    }
}

async fn thinking_space_setup(
    db: &PgPool,
    config: &ThinkingSpaceToolSetup,
    locale: &str,
) -> Result<ThinkingSpaceToolConfig, ComhairleError> {
    config.clone().build_with_translations(db, locale).await
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
    pub areas_of_uncertainty: Option<String>,
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
            areas_of_uncertainty: s.areas_of_uncertainty,
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

async fn get_localized_topic(
    db: &PgPool,
    tool_config: &ThinkingSpaceToolConfig,
    locale: &str,
) -> Result<String, ComhairleError> {
    let translation_map = localize_translations(db, &[tool_config.topic], locale).await?;
    translation_map
        .get(&tool_config.topic)
        .ok_or_else(|| {
            ComhairleError::CorruptedData("Missing topic from thinking space config".to_string())
        })
        .cloned()
}

#[derive(Deserialize, Debug, JsonSchema, Clone, PartialEq)]
pub struct ConversationRequest {
    pub workflow_step_id: Uuid,
    pub root_question_id: Uuid,
    pub question_intent: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConverseResponseDto {
    pub questions: Vec<String>,
}

/// Builds the structured Q&A chain sensemakar needs from this user's stored
/// answers: the root answer identified by `root_question_id`, plus whatever
/// follow-ups have already been answered under it, oldest first.
fn build_question_chain(
    answers: &[ThinkingSpaceAnswer],
    root_question_id: Uuid,
    asking_reasion: String,
) -> Result<QuestionChain, ComhairleError> {
    let root = answers
        .iter()
        .find(|a| a.id == root_question_id)
        .ok_or_else(|| ComhairleError::ResourceNotFound("ThinkingSpaceAnswer".to_string()))?;

    let mut follow_ups: Vec<&ThinkingSpaceAnswer> = answers
        .iter()
        .filter(|a| a.root_question_id == Some(root_question_id))
        .collect();
    follow_ups.sort_by_key(|a| a.created_at);

    Ok(QuestionChain {
        base_interaction: QuestionWithAnswer {
            question: Question {
                text: root.question.clone(),
                asking_reasion,
                ai_generated: false,
            },
            answer: Answer {
                text: root.answer.clone(),
            },
        },
        follow_ups: follow_ups
            .into_iter()
            .map(|a| QuestionWithAnswer {
                question: Question {
                    text: a.question.clone(),
                    asking_reasion: String::new(),
                    ai_generated: true,
                },
                answer: Answer {
                    text: a.answer.clone(),
                },
            })
            .collect(),
    })
}

#[instrument(err(Debug), skip(state))]
async fn converse(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    LocaleExtractor(locale): LocaleExtractor,
    Json(payload): Json<ConversationRequest>,
) -> Result<(StatusCode, Json<ConverseResponseDto>), ComhairleError> {
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

    let topic = get_localized_topic(&state.db, &tool_config, &locale).await?;

    let answers = thinking_space_answer::list(
        &state.db,
        &payload.workflow_step_id,
        ThinkingSpaceAnswerFilterOptions {
            user_id: Some(user.id),
            ..Default::default()
        },
    )
    .await?;

    let question_chain =
        build_question_chain(&answers, payload.root_question_id, payload.question_intent)?;

    let model_cfg = state
        .config
        .sensemaker_model
        .as_ref()
        .ok_or(ComhairleError::NoSensemakerModelConfigured)?;
    let model_config = sensemakar_service::build_model_config(model_cfg)
        .map_err(ComhairleError::SensemakarConfigError)?;
    let model = sensemakar::model_config::build_model(&model_config);

    let generator = FollowUpQuestionGenerator {
        context: None,
        additional_instructions: None,
        reading_age_target: None,
        topic,
    };
    let questions = generator.run_with_model(question_chain, model).await?;

    Ok((
        StatusCode::OK,
        Json(ConverseResponseDto {
            questions: questions.into_iter().map(|q| q.text).collect(),
        }),
    ))
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

#[instrument(err(Debug), skip(state))]
async fn generate_thinking_space_summary(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    LocaleExtractor(locale): LocaleExtractor,
    Json(payload): Json<GenerateThinkingSpaceSummary>,
) -> Result<(StatusCode, Json<ThinkingSpaceSummaryDto>), ComhairleError> {
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

    let topic = get_localized_topic(&state.db, &tool_config, &locale).await?;

    let filter_options = ThinkingSpaceAnswerFilterOptions {
        user_id: Some(user.id),
        ..Default::default()
    };
    let answers =
        thinking_space_answer::list(&state.db, &payload.workflow_step_id, filter_options).await?;

    let root_ids: Vec<Uuid> = answers
        .iter()
        .filter(|a| !a.is_follow_up)
        .map(|a| a.id)
        .collect();

    if root_ids.is_empty() {
        return Err(ComhairleError::CorruptedData(
            "No thinking space answers to summarize".to_string(),
        ));
    }

    let model_cfg = state
        .config
        .sensemaker_model
        .as_ref()
        .ok_or(ComhairleError::NoSensemakerModelConfigured)?;
    let model_config = sensemakar_service::build_model_config(model_cfg)
        .map_err(ComhairleError::SensemakarConfigError)?;
    let model = sensemakar::model_config::build_model(&model_config);

    // A step can have multiple root questions, each answered independently.
    // sensemakar summarizes one base-question-plus-follow-ups chain at a
    // time, so each root gets its own call and the resulting narratives are
    // joined into one overall summary. The per-root budget keeps that joined
    // total near the participant-facing edit limit (see MAX_SUMMARY_WORDS in
    // ui/packages/comhairle/src/lib/tools/thinking_space/wordLimit.ts).
    let per_root_word_limit = MAX_SUMMARY_WORDS / root_ids.len().max(1);
    let word_limit_instruction = Some(format!(
        "Write the summary in no more than {per_root_word_limit} words."
    ));

    let mut summaries = Vec::with_capacity(root_ids.len());
    let mut uncertainties = Vec::with_capacity(root_ids.len());
    for root_id in root_ids {
        let question_chain = build_question_chain(&answers, root_id, String::new())?;

        let generator = SummaryGenerator {
            context: None,
            additional_instructions: word_limit_instruction.clone(),
            reading_age_target: None,
            topic: topic.clone(),
        };
        let result = generator
            .run_with_model(question_chain, model.clone())
            .await?;

        summaries.push(result.summary);
        uncertainties.push(result.areas_of_uncertainty);
    }

    let create_summary = CreateSummary {
        summary: summaries.join("\n\n"),
        is_ai_generated: Some(true),
        areas_of_uncertainty: Some(uncertainties.join("\n\n")),
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
                areas_of_uncertainty: None,
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

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct AnswersByRoot {
    root: ThinkingSpaceAnswerDto,
    follow_ups: Vec<ThinkingSpaceAnswerDto>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct ThinkingSpaceUserInsights {
    user_id: Uuid,
    summary: ThinkingSpaceSummaryDto,
    answers: Vec<AnswersByRoot>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
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
            is_shared_with_organizer: Some(true),
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

    // A participant who revisits the step creates a new summary round, and each
    // round is generated from all of their answers, so the newest one stands in
    // for the user.
    let mut latest_summary_by_user: HashMap<Uuid, ThinkingSpaceSummary> = HashMap::new();
    for summary in summaries {
        let is_newer = latest_summary_by_user
            .get(&summary.user_id)
            .is_none_or(|current| summary.created_at > current.created_at);

        if is_newer {
            latest_summary_by_user.insert(summary.user_id, summary);
        }
    }
    let mut latest_summaries: Vec<ThinkingSpaceSummary> =
        latest_summary_by_user.into_values().collect();
    latest_summaries.sort_by_key(|summary| summary.created_at);

    let user_insights: Vec<ThinkingSpaceUserInsights> = latest_summaries
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
    use std::error::Error;

    use serde_json::json;
    use sqlx::PgPool;

    use super::*;
    use crate::models::model_test_helpers::{
        get_random_conversation_id, get_random_workflow_id, setup_default_app_and_session,
    };
    use crate::models::user_progress::{self, ProgressStatus};
    use crate::routes::workflow_steps::dto::WorkflowStepDto;
    use crate::test_helpers::thinking_space_tool_config;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_thinking_space_answer_via_api(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;

        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let workflow_id = get_random_workflow_id(&app, &mut session, &conversation_id).await?;
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
        let workflow_id = get_random_workflow_id(&app, &mut session, &conversation_id).await?;
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
        let workflow_id = get_random_workflow_id(&app, &mut session, &conversation_id).await?;
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

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_return_one_insight_per_user_with_latest_summary(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;

        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let workflow_id = get_random_workflow_id(&app, &mut session, &conversation_id).await?;
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
                    "question": "A root question",
                    "answer": "A root answer",
                })
                .to_string()
                .into(),
            )
            .await?;
        let root: ThinkingSpaceAnswerDto = serde_json::from_value(value)?;
        session
            .post(
                &app,
                "/tools/thinking_space/answers",
                json!({
                    "workflow_step_id": workflow_step.id,
                    "question": "A follow up question",
                    "answer": "A follow up answer",
                    "is_follow_up": true,
                    "root_question_id": root.id
                })
                .to_string()
                .into(),
            )
            .await?;

        let mut summaries = Vec::new();
        for text in ["First round summary", "Second round summary"] {
            let (_, value, _) = session
                .post(
                    &app,
                    "/tools/thinking_space/summaries",
                    json!({
                        "workflow_step_id": workflow_step.id,
                        "summary": text,
                    })
                    .to_string()
                    .into(),
                )
                .await?;
            let summary: ThinkingSpaceSummaryDto = serde_json::from_value(value)?;
            summaries.push(summary);
        }
        let user_id = summaries[0].user_id;
        user_progress::create(&pool, &user_id, &workflow_step.id, ProgressStatus::Done).await?;

        let (_, value, _) = session
            .get(
                &app,
                &format!(
                    "/tools/thinking_space/insights?workflow_step_id={}",
                    workflow_step.id
                ),
            )
            .await?;
        let insights: ThinkingSpaceInsightsResponse = serde_json::from_value(value)?;

        assert_eq!(insights.users.len(), 1, "expected one insight per user");
        let user_insights = &insights.users[0];
        assert_eq!(user_insights.user_id, user_id, "incorrect user_id");
        assert_eq!(
            user_insights.summary.id, summaries[1].id,
            "expected the latest summary"
        );
        assert_eq!(user_insights.answers.len(), 1, "incorrect root answers");
        assert_eq!(user_insights.answers[0].root.id, root.id, "incorrect root");
        assert_eq!(
            user_insights.answers[0].follow_ups.len(),
            1,
            "incorrect follow ups"
        );

        Ok(())
    }
}
