use std::sync::Arc;

use aide::axum::ApiRouter;
use aide::axum::routing::{delete_with, get_with, post_with, put_with};
use async_trait::async_trait;
use axum::Json;
use axum::extract::{Path, Query, State};
use axum::middleware::from_fn_with_state;
use hyper::StatusCode;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use super::{ToolConfigSanitize, ToolImpl};
use crate::ComhairleState;
use crate::error::ComhairleError;
use crate::middleware::permissions::{PermissionRequirement, authorize as permission_middleware};
use crate::models::demographics::responses::{self, ConsentState};
use crate::models::demographics::types::{
    ConsentInput, DemographicFormDraftInput, DemographicFormDto, DemographicFormLatestDto,
    DemographicTag, MyDemographicAnswer, PendingQuestion, QuestionDraftInput, QuestionDto,
    QuestionLatestDto, ResponseDto, ResponseInput,
};
use crate::models::demographics::{forms, questions, steps};
use crate::models::permissions::conversation::Action;
use crate::models::permissions::system::Action as SystemAction;
use crate::models::permissions::{SystemResource, can_perform_action};
use crate::models::users::User;
use crate::models::{conversation, user_group, workflow, workflow_step};
use crate::routes::auth::RequiredUser;
use crate::routes::translations::LocaleExtractor;

/// Which version of a demographic question or form a step asks.
#[derive(Clone, Copy, Deserialize, Serialize, Debug, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum VersionPolicy {
    /// The latest published version until the conversation launches, then the version that was
    /// latest at launch.
    Latest,
    /// A specific published version that never changes.
    Pinned { version_id: Uuid },
}

/// Overrides one question of a form for this step. Questions without an override use the
/// form's own settings.
#[derive(Clone, Copy, Deserialize, Serialize, Debug, JsonSchema, PartialEq, Eq)]
pub struct QuestionOverride {
    pub question_id: Uuid,
    pub enabled: bool,
    pub required: bool,
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, PartialEq, Eq)]
pub struct DemographicQuestionToolConfig {
    pub question_id: Uuid,
    pub version: VersionPolicy,
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, PartialEq, Eq)]
pub struct DemographicFormToolConfig {
    pub form_id: Uuid,
    pub version: VersionPolicy,
    #[serde(default)]
    pub questions: Vec<QuestionOverride>,
}

impl ToolConfigSanitize for DemographicQuestionToolConfig {
    fn sanitize(&self) -> Self {
        self.clone()
    }
}

impl ToolConfigSanitize for DemographicFormToolConfig {
    fn sanitize(&self) -> Self {
        self.clone()
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct DemographicReport;

pub type DemographicQuestionToolSetup = DemographicQuestionToolConfig;
pub type DemographicFormToolSetup = DemographicFormToolConfig;

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct DemographicQuestionToolExIm;

pub struct DemographicQuestionTool;

#[async_trait]
impl ToolImpl for DemographicQuestionTool {
    type Config = DemographicQuestionToolConfig;
    type Setup = DemographicQuestionToolSetup;
    type Report = DemographicReport;
    type ExportImport = DemographicQuestionToolExIm;

    async fn setup(
        setup: &Self::Setup,
        _state: &Arc<ComhairleState>,
        _locale: &str,
    ) -> Result<Self::Config, ComhairleError> {
        Ok(setup.clone())
    }

    async fn clone_tool(
        config: &Self::Config,
        state: &Arc<ComhairleState>,
    ) -> Result<Self::Config, ComhairleError> {
        steps::freeze_question_config(&state.db, config.clone()).await
    }

    fn sanitize(config: Self::Config) -> Self::Config {
        config.sanitize()
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
        let requires = |action: SystemAction| {
            from_fn_with_state(
                PermissionRequirement::<SystemResource>::new(action),
                permission_middleware::<SystemResource>,
            )
        };

        ApiRouter::new()
            .api_route(
                "/demographics/tags",
                get_with(list_tags, |op| {
                    op.id("ListDemographicTags")
                        .tag("Demographics")
                        .security_requirement("JWT")
                        .response::<200, Json<Vec<DemographicTag>>>()
                })
                .route_layer(requires(SystemAction::DemographicRead)),
            )
            .api_route(
                "/demographics/tags",
                post_with(create_tag, |op| {
                    op.id("CreateDemographicTag")
                        .tag("Demographics")
                        .security_requirement("JWT")
                        .response::<201, Json<DemographicTag>>()
                })
                .route_layer(requires(SystemAction::DemographicManage)),
            )
            .api_route(
                "/demographics/questions",
                get_with(list_questions, |op| {
                    op.id("ListDemographicQuestions")
                        .tag("Demographics")
                        .security_requirement("JWT")
                        .response::<200, Json<Vec<QuestionDto>>>()
                })
                .route_layer(requires(SystemAction::DemographicRead)),
            )
            .api_route(
                "/demographics/questions/latest",
                get_with(list_questions_latest, |op| {
                    op.id("ListDemographicQuestionsLatest")
                        .tag("Demographics")
                        .security_requirement("JWT")
                        .response::<200, Json<Vec<QuestionLatestDto>>>()
                })
                .route_layer(requires(SystemAction::DemographicRead)),
            )
            .api_route(
                "/demographics/questions",
                post_with(create_question, |op| {
                    op.id("CreateDemographicQuestion")
                        .tag("Demographics")
                        .security_requirement("JWT")
                        .response::<201, Json<QuestionDto>>()
                })
                .route_layer(requires(SystemAction::DemographicManage)),
            )
            .api_route(
                "/demographics/questions/{question_id}",
                get_with(get_question, |op| {
                    op.id("GetDemographicQuestion")
                        .tag("Demographics")
                        .security_requirement("JWT")
                        .response::<200, Json<QuestionDto>>()
                })
                .route_layer(requires(SystemAction::DemographicRead)),
            )
            .api_route(
                "/demographics/questions/{question_id}/latest",
                get_with(get_question_latest, |op| {
                    op.id("GetDemographicsQuestionLatest")
                        .tag("Demographics")
                        .security_requirement("JWT")
                        .response::<200, Json<QuestionLatestDto>>()
                })
                .route_layer(requires(SystemAction::DemographicRead)),
            )
            .api_route(
                "/demographics/questions/{question_id}/draft",
                post_with(start_question_draft, |op| {
                    op.id("StartDemographicQuestionDraft")
                        .tag("Demographics")
                        .security_requirement("JWT")
                        .response::<200, Json<QuestionDto>>()
                })
                .route_layer(requires(SystemAction::DemographicManage)),
            )
            .api_route(
                "/demographics/questions/{question_id}/draft",
                put_with(update_question_draft, |op| {
                    op.id("UpdateDemographicQuestionDraft")
                        .tag("Demographics")
                        .security_requirement("JWT")
                        .response::<200, Json<QuestionDto>>()
                })
                .route_layer(requires(SystemAction::DemographicManage)),
            )
            .api_route(
                "/demographics/questions/{question_id}/draft",
                delete_with(discard_question_draft, |op| {
                    op.id("DiscardDemographicQuestionDraft")
                        .tag("Demographics")
                        .security_requirement("JWT")
                        .response_with::<204, (), _>(|res| res.description("Draft discarded"))
                })
                .route_layer(requires(SystemAction::DemographicManage)),
            )
            .api_route(
                "/demographics/questions/{question_id}/publish",
                post_with(publish_question, |op| {
                    op.id("PublishDemographicQuestion")
                        .tag("Demographics")
                        .security_requirement("JWT")
                        .response::<200, Json<QuestionDto>>()
                })
                .route_layer(requires(SystemAction::DemographicManage)),
            )
            .api_route(
                "/demographics/questions/{question_id}/archive",
                post_with(archive_question, |op| {
                    op.id("ArchiveDemographicQuestion")
                        .tag("Demographics")
                        .security_requirement("JWT")
                        .response::<200, Json<QuestionDto>>()
                })
                .route_layer(requires(SystemAction::DemographicManage)),
            )
            .api_route(
                "/demographics/defaults",
                get_with(list_defaults, |op| {
                    op.id("ListDemographicDefaults")
                        .tag("Demographics")
                        .security_requirement("JWT")
                        .response::<200, Json<Vec<QuestionDto>>>()
                })
                .route_layer(requires(SystemAction::DemographicDefaultManage)),
            )
            .api_route(
                "/demographics/defaults",
                post_with(create_default, |op| {
                    op.id("CreateDemographicDefault")
                        .tag("Demographics")
                        .security_requirement("JWT")
                        .response::<201, Json<QuestionDto>>()
                })
                .route_layer(requires(SystemAction::DemographicDefaultManage)),
            )
            .api_route(
                "/demographics/defaults/{question_id}/draft",
                put_with(update_default_draft, |op| {
                    op.id("UpdateDemographicDefaultDraft")
                        .tag("Demographics")
                        .security_requirement("JWT")
                        .response::<200, Json<QuestionDto>>()
                })
                .route_layer(requires(SystemAction::DemographicDefaultManage)),
            )
            .api_route(
                "/demographics/defaults/{question_id}/publish",
                post_with(publish_default, |op| {
                    op.id("PublishDemographicDefault")
                        .tag("Demographics")
                        .security_requirement("JWT")
                        .response::<200, Json<QuestionDto>>()
                })
                .route_layer(requires(SystemAction::DemographicDefaultManage)),
            )
            .api_route(
                "/demographics/question-versions/{version_id}/consent",
                put_with(set_question_consent, |op| {
                    op.id("SetDemographicConsent")
                        .tag("Demographics")
                        .security_requirement("JWT")
                        .response::<200, Json<ConsentState>>()
                }),
            )
            .api_route(
                "/demographics/question-versions/{version_id}/response",
                put_with(put_my_response, |op| {
                    op.id("PutDemographicResponse")
                        .tag("Demographics")
                        .security_requirement("JWT")
                        .response::<200, Json<ResponseDto>>()
                }),
            )
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct DemographicFormToolExIm;

pub struct DemographicFormTool;

#[async_trait]
impl ToolImpl for DemographicFormTool {
    type Config = DemographicFormToolConfig;
    type Setup = DemographicFormToolSetup;
    type Report = DemographicReport;
    type ExportImport = DemographicFormToolExIm;

    async fn setup(
        setup: &Self::Setup,
        _state: &Arc<ComhairleState>,
        _locale: &str,
    ) -> Result<Self::Config, ComhairleError> {
        Ok(setup.clone())
    }

    async fn clone_tool(
        config: &Self::Config,
        state: &Arc<ComhairleState>,
    ) -> Result<Self::Config, ComhairleError> {
        steps::freeze_form_config(&state.db, config.clone()).await
    }

    fn sanitize(config: Self::Config) -> Self::Config {
        config.sanitize()
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
        let requires = |action: SystemAction| {
            from_fn_with_state(
                PermissionRequirement::<SystemResource>::new(action),
                permission_middleware::<SystemResource>,
            )
        };
        ApiRouter::new()
            .api_route(
                "/demographics/forms",
                get_with(list_forms, |op| {
                    op.id("ListDemographicForms")
                        .tag("Demographics")
                        .security_requirement("JWT")
                        .response::<200, Json<Vec<DemographicFormDto>>>()
                })
                .route_layer(requires(SystemAction::DemographicRead)),
            )
            .api_route(
                "/demographics/forms/latest",
                get_with(list_forms_latest, |op| {
                    op.id("ListDemographicFormsLatest")
                        .tag("Demographics")
                        .security_requirement("JWT")
                        .response::<200, Json<Vec<DemographicFormLatestDto>>>()
                })
                .route_layer(requires(SystemAction::DemographicRead)),
            )
            .api_route(
                "/demographics/forms",
                post_with(create_form, |op| {
                    op.id("CreateDemographicForm")
                        .tag("Demographics")
                        .security_requirement("JWT")
                        .response::<201, Json<DemographicFormDto>>()
                })
                .route_layer(requires(SystemAction::DemographicManage)),
            )
            .api_route(
                "/demographics/forms/{form_id}",
                get_with(get_form, |op| {
                    op.id("GetDemographicForm")
                        .tag("Demographics")
                        .security_requirement("JWT")
                        .response::<200, Json<DemographicFormDto>>()
                })
                .route_layer(requires(SystemAction::DemographicRead)),
            )
            .api_route(
                "/demographics/forms/{form_id}/latest",
                get_with(get_form_latest, |op| {
                    op.id("GetDemographicFormLatest")
                        .tag("Demographics")
                        .security_requirement("JWT")
                        .response::<200, Json<DemographicFormLatestDto>>()
                })
                .route_layer(requires(SystemAction::DemographicRead)),
            )
            .api_route(
                "/demographics/forms/{form_id}/draft",
                post_with(start_form_draft, |op| {
                    op.id("StartDemographicFormDraft")
                        .tag("Demographics")
                        .security_requirement("JWT")
                        .response::<200, Json<DemographicFormDto>>()
                })
                .route_layer(requires(SystemAction::DemographicManage)),
            )
            .api_route(
                "/demographics/forms/{form_id}/draft",
                put_with(update_form_draft, |op| {
                    op.id("UpdateDemographicFormDraft")
                        .tag("Demographics")
                        .security_requirement("JWT")
                        .response::<200, Json<DemographicFormDto>>()
                })
                .route_layer(requires(SystemAction::DemographicManage)),
            )
            .api_route(
                "/demographics/forms/{form_id}/draft",
                delete_with(discard_form_draft, |op| {
                    op.id("DiscardDemographicFormDraft")
                        .tag("Demographics")
                        .security_requirement("JWT")
                        .response_with::<204, (), _>(|res| res.description("Draft discarded"))
                })
                .route_layer(requires(SystemAction::DemographicManage)),
            )
            .api_route(
                "/demographics/forms/{form_id}/publish",
                post_with(publish_form, |op| {
                    op.id("PublishDemographicForm")
                        .tag("Demographics")
                        .security_requirement("JWT")
                        .response::<200, Json<DemographicFormDto>>()
                })
                .route_layer(requires(SystemAction::DemographicManage)),
            )
            .api_route(
                "/demographics/forms/{form_id}/archive",
                post_with(archive_form, |op| {
                    op.id("ArchiveDemographicForm")
                        .tag("Demographics")
                        .security_requirement("JWT")
                        .response::<200, Json<DemographicFormDto>>()
                })
                .route_layer(requires(SystemAction::DemographicManage)),
            )
            .api_route(
                "/demographics/steps/{step_id}/pending",
                get_with(pending_step_questions, |op| {
                    op.id("GetPendingDemographicQuestions")
                        .tag("Demographics")
                        .security_requirement("JWT")
                        .response::<200, Json<Vec<PendingQuestion>>>()
                }),
            )
            .api_route(
                "/demographics/me/answers",
                get_with(list_my_answers, |op| {
                    op.id("ListMyDemographicAnswers")
                        .tag("Demographics")
                        .security_requirement("JWT")
                        .response::<200, Json<Vec<MyDemographicAnswer>>>()
                }),
            )
            .api_route(
                "/demographics/me/answers/{response_id}/anonymise",
                post_with(anonymise_my_answer, |op| {
                    op.id("AnonymiseMyDemographicAnswer")
                        .tag("Demographics")
                        .security_requirement("JWT")
                        .response_with::<204, (), _>(|res| res.description("Answer anonymised"))
                }),
            )
    }
}

/// Whether `user` is allowed to view `conversation_id`: admins, the owner, anyone
/// with `ConversationRead`, or anyone at all once the conversation is live.
async fn can_view_conversation(
    state: &Arc<ComhairleState>,
    user: Option<&User>,
    conversation_id: Uuid,
) -> Result<bool, ComhairleError> {
    let conversation = conversation::get_by_id(&state.db, &conversation_id).await?;

    if conversation.is_live {
        return Ok(true);
    }

    let Some(user) = user else {
        return Ok(false);
    };

    if user.id == conversation.owner_id {
        return Ok(true);
    }

    can_perform_action(
        state,
        &conversation.id,
        Action::Read,
        &user.id,
        Some(&conversation.owner_id),
    )
    .await
}

/// The organization an admin selected in the demographics UI, sent as `?organization=`.
#[derive(Deserialize, JsonSchema)]
struct DefaultParams {
    organization_id: Option<Uuid>,
}

/// The organization an admin selected in the demographics UI, sent as `?organization=`.
/// Whether or not to include archived resources in the response, sent as `?include_archived=`.
#[derive(Deserialize, JsonSchema)]
struct ListQueryParams {
    organization_id: Option<Uuid>,
    include_archived: Option<bool>,
}

/// The selected organization, if any. Admins may only select organizations they belong to.
async fn selected_organization(
    state: &Arc<ComhairleState>,
    admin: &User,
    organization_id: Option<Uuid>,
) -> Result<Option<Uuid>, ComhairleError> {
    let Some(org) = organization_id else {
        return Ok(None);
    };
    let is_member = admin.organization_id == Some(org)
        || user_group::is_organization_member(&state.db, org, admin.id).await?;
    if is_member {
        Ok(Some(org))
    } else {
        Err(ComhairleError::UserNotAuthorized)
    }
}

/// The selected organization, required for writes and for org-owned resources.
async fn required_organization(
    state: &Arc<ComhairleState>,
    admin: &User,
    organization_id: Option<Uuid>,
) -> Result<Uuid, ComhairleError> {
    selected_organization(state, admin, organization_id)
        .await?
        .ok_or(ComhairleError::UserNotAuthorized)
}

#[derive(Deserialize, JsonSchema)]
struct CreateTagInput {
    name: String,
}

// Organization question bank

#[instrument(skip(state, admin, params))]
async fn list_tags(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(params): Query<DefaultParams>,
) -> Result<Json<Vec<DemographicTag>>, ComhairleError> {
    let org = selected_organization(&state, &admin, params.organization_id).await?;
    Ok(Json(questions::list_tags(&state.db, org).await?))
}

#[instrument(skip(state, admin, params, body))]
async fn create_tag(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(params): Query<DefaultParams>,
    Json(body): Json<CreateTagInput>,
) -> Result<(StatusCode, Json<DemographicTag>), ComhairleError> {
    let org = required_organization(&state, &admin, params.organization_id).await?;
    let tag = questions::create_tag(&state.db, Some(org), &body.name).await?;
    Ok((StatusCode::CREATED, Json(tag)))
}

#[instrument(skip(state, admin, params, locale))]
async fn list_questions(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(params): Query<DefaultParams>,
    LocaleExtractor(locale): LocaleExtractor,
) -> Result<Json<Vec<QuestionDto>>, ComhairleError> {
    let org = selected_organization(&state, &admin, params.organization_id).await?;
    Ok(Json(
        questions::list_questions(&state.db, org, &locale).await?,
    ))
}

#[instrument(skip(state, admin, params, locale))]
async fn list_questions_latest(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(params): Query<ListQueryParams>,
    LocaleExtractor(locale): LocaleExtractor,
) -> Result<Json<Vec<QuestionLatestDto>>, ComhairleError> {
    let org = selected_organization(&state, &admin, params.organization_id).await?;
    let include_archived = params.include_archived.unwrap_or(false);
    let questions = questions::list_questions(&state.db, org, &locale).await?;

    let latest_questions = questions
        .iter()
        .filter_map(|question| {
            if !include_archived && question.question.archived_at.is_some() {
                return None;
            };

            let Some(question_latest) =
                question.clone().draft.or(question.latest_published.clone())
            else {
                return None;
            };

            Some(QuestionLatestDto {
                draft_version: question
                    .draft
                    .as_ref()
                    .map(|d| d.version.version_number)
                    .unwrap_or(0),
                published_version: question
                    .latest_published
                    .as_ref()
                    .map(|lp| lp.version.version_number)
                    .unwrap_or(0),
                tags: question.tags.clone(),
                meta: question.question.clone(),
                question: question_latest,
            })
        })
        .collect();

    Ok(Json(latest_questions))
}

#[instrument(skip(state, admin, params, locale, body))]
async fn create_question(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(params): Query<DefaultParams>,
    LocaleExtractor(locale): LocaleExtractor,
    Json(body): Json<QuestionDraftInput>,
) -> Result<(StatusCode, Json<QuestionDto>), ComhairleError> {
    let org = required_organization(&state, &admin, params.organization_id).await?;
    let question = questions::create_question(&state.db, Some(org), &body, &locale).await?;
    Ok((StatusCode::CREATED, Json(question)))
}

#[instrument(skip(state, admin, params, locale))]
async fn get_question(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(params): Query<DefaultParams>,
    LocaleExtractor(locale): LocaleExtractor,
    Path(question_id): Path<Uuid>,
) -> Result<Json<QuestionDto>, ComhairleError> {
    let org = selected_organization(&state, &admin, params.organization_id).await?;
    let question = questions::get_question(&state.db, org, question_id, &locale).await?;
    Ok(Json(question))
}

#[instrument(skip(state, admin, params, locale))]
async fn get_question_latest(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(params): Query<DefaultParams>,
    LocaleExtractor(locale): LocaleExtractor,
    Path(question_id): Path<Uuid>,
) -> Result<Json<QuestionLatestDto>, ComhairleError> {
    let org = selected_organization(&state, &admin, params.organization_id).await?;
    let question = questions::get_question(&state.db, org, question_id, &locale).await?;

    let question_latest = QuestionLatestDto {
        draft_version: question
            .draft
            .as_ref()
            .map(|d| d.version.version_number)
            .unwrap_or(0),
        published_version: question
            .latest_published
            .as_ref()
            .map(|lp| lp.version.version_number)
            .unwrap_or(0),
        tags: question.tags.clone(),
        meta: question.question.clone(),
        question: question
            .clone()
            .draft
            .or(question.latest_published.clone())
            .ok_or(ComhairleError::ResourceNotFound("question".into()))?,
    };

    Ok(Json(question_latest))
}

#[instrument(skip(state, admin, params, locale))]
async fn start_question_draft(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(params): Query<DefaultParams>,
    LocaleExtractor(locale): LocaleExtractor,
    Path(question_id): Path<Uuid>,
) -> Result<Json<QuestionDto>, ComhairleError> {
    let org = required_organization(&state, &admin, params.organization_id).await?;
    let question = questions::start_draft(&state.db, Some(org), question_id, &locale).await?;
    Ok(Json(question))
}

#[instrument(skip(state, admin, params, locale, body))]
async fn update_question_draft(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(params): Query<DefaultParams>,
    LocaleExtractor(locale): LocaleExtractor,
    Path(question_id): Path<Uuid>,
    Json(body): Json<QuestionDraftInput>,
) -> Result<Json<QuestionDto>, ComhairleError> {
    let org = required_organization(&state, &admin, params.organization_id).await?;
    let question =
        questions::update_draft(&state.db, Some(org), question_id, &body, &locale).await?;
    Ok(Json(question))
}

#[instrument(skip(state, admin, params))]
async fn discard_question_draft(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(params): Query<DefaultParams>,
    Path(question_id): Path<Uuid>,
) -> Result<StatusCode, ComhairleError> {
    let org = required_organization(&state, &admin, params.organization_id).await?;
    questions::discard_draft(&state.db, Some(org), question_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[instrument(skip(state, admin, params, locale))]
async fn publish_question(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(params): Query<DefaultParams>,
    LocaleExtractor(locale): LocaleExtractor,
    Path(question_id): Path<Uuid>,
) -> Result<Json<QuestionDto>, ComhairleError> {
    let org = required_organization(&state, &admin, params.organization_id).await?;
    let question = questions::publish_question(&state.db, Some(org), question_id, &locale).await?;
    Ok(Json(question))
}

#[instrument(skip(state, admin, params, locale))]
async fn archive_question(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(params): Query<DefaultParams>,
    LocaleExtractor(locale): LocaleExtractor,
    Path(question_id): Path<Uuid>,
) -> Result<Json<QuestionDto>, ComhairleError> {
    let org = required_organization(&state, &admin, params.organization_id).await?;
    let question = questions::archive_question(&state.db, Some(org), question_id, &locale).await?;
    Ok(Json(question))
}

#[instrument(skip(state, admin, params, locale))]
async fn list_forms(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(params): Query<DefaultParams>,
    LocaleExtractor(locale): LocaleExtractor,
) -> Result<Json<Vec<DemographicFormDto>>, ComhairleError> {
    // Forms belong to an organization and there are no system forms, so this is empty without one.
    let forms = match selected_organization(&state, &admin, params.organization_id).await? {
        Some(org) => forms::list_forms(&state.db, org, &locale).await?,
        None => Vec::new(),
    };
    Ok(Json(forms))
}

#[instrument(skip(state, admin, params, locale))]
async fn list_forms_latest(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(params): Query<ListQueryParams>,
    LocaleExtractor(locale): LocaleExtractor,
) -> Result<Json<Vec<DemographicFormLatestDto>>, ComhairleError> {
    // Forms belong to an organization and there are no system forms, so this is empty without one.
    let forms = match selected_organization(&state, &admin, params.organization_id).await? {
        Some(org) => forms::list_forms(&state.db, org, &locale).await?,
        None => Vec::new(),
    };

    let include_archived = params.include_archived.unwrap_or(false);

    let forms_latest = forms
        .iter()
        .filter_map(|form| {
            if !include_archived && form.form.archived_at.is_some() {
                return None;
            };

            let Some(form_latest) = form.clone().draft.or(form.latest_published.clone()) else {
                return None;
            };

            Some(DemographicFormLatestDto {
                draft_version: form
                    .draft
                    .as_ref()
                    .map(|f| f.version.version_number)
                    .unwrap_or(0),
                published_version: form
                    .latest_published
                    .as_ref()
                    .map(|f| f.version.version_number)
                    .unwrap_or(0),
                meta: form.form.clone(),
                form: form_latest,
            })
        })
        .collect();

    Ok(Json(forms_latest))
}

#[instrument(skip(state, admin, params, locale, body))]
async fn create_form(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(params): Query<DefaultParams>,
    LocaleExtractor(locale): LocaleExtractor,
    Json(body): Json<DemographicFormDraftInput>,
) -> Result<(StatusCode, Json<DemographicFormDto>), ComhairleError> {
    let org = required_organization(&state, &admin, params.organization_id).await?;
    let form = forms::create_form(&state.db, org, &body, &locale).await?;
    Ok((StatusCode::CREATED, Json(form)))
}

#[instrument(skip(state, admin, params, locale))]
async fn get_form(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(params): Query<DefaultParams>,
    LocaleExtractor(locale): LocaleExtractor,
    Path(form_id): Path<Uuid>,
) -> Result<Json<DemographicFormDto>, ComhairleError> {
    let org = selected_organization(&state, &admin, params.organization_id)
        .await?
        .ok_or_else(|| ComhairleError::ResourceNotFound("demographic form".into()))?;
    Ok(Json(
        forms::get_form(&state.db, org, form_id, &locale).await?,
    ))
}

#[instrument(skip(state, admin, params, locale))]
async fn get_form_latest(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(params): Query<DefaultParams>,
    LocaleExtractor(locale): LocaleExtractor,
    Path(form_id): Path<Uuid>,
) -> Result<Json<DemographicFormLatestDto>, ComhairleError> {
    let org = selected_organization(&state, &admin, params.organization_id)
        .await?
        .ok_or_else(|| ComhairleError::ResourceNotFound("demographic form".into()))?;

    let form = forms::get_form(&state.db, org, form_id, &locale).await?;

    let form_latest = DemographicFormLatestDto {
        draft_version: form
            .draft
            .as_ref()
            .map(|d| d.version.version_number)
            .unwrap_or(0),
        published_version: form
            .latest_published
            .as_ref()
            .map(|lp| lp.version.version_number)
            .unwrap_or(0),
        meta: form.form.clone(),
        form: form
            .clone()
            .draft
            .or(form.latest_published.clone())
            .ok_or(ComhairleError::ResourceNotFound("demographic form".into()))?,
    };

    Ok(Json(form_latest))
}

#[instrument(skip(state, admin, params, locale))]
async fn start_form_draft(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(params): Query<DefaultParams>,
    LocaleExtractor(locale): LocaleExtractor,
    Path(form_id): Path<Uuid>,
) -> Result<Json<DemographicFormDto>, ComhairleError> {
    let org = required_organization(&state, &admin, params.organization_id).await?;
    Ok(Json(
        forms::start_form_draft(&state.db, org, form_id, &locale).await?,
    ))
}

#[instrument(skip(state, admin, params, locale, body))]
async fn update_form_draft(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(params): Query<DefaultParams>,
    LocaleExtractor(locale): LocaleExtractor,
    Path(form_id): Path<Uuid>,
    Json(body): Json<DemographicFormDraftInput>,
) -> Result<Json<DemographicFormDto>, ComhairleError> {
    let org = required_organization(&state, &admin, params.organization_id).await?;
    Ok(Json(
        forms::update_form_draft(&state.db, org, form_id, &body, &locale).await?,
    ))
}

#[instrument(skip(state, admin, params))]
async fn discard_form_draft(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(params): Query<DefaultParams>,
    Path(form_id): Path<Uuid>,
) -> Result<StatusCode, ComhairleError> {
    let org = required_organization(&state, &admin, params.organization_id).await?;
    forms::discard_form_draft(&state.db, org, form_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[instrument(skip(state, admin, params, locale))]
async fn publish_form(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(params): Query<DefaultParams>,
    LocaleExtractor(locale): LocaleExtractor,
    Path(form_id): Path<Uuid>,
) -> Result<Json<DemographicFormDto>, ComhairleError> {
    let org = required_organization(&state, &admin, params.organization_id).await?;
    Ok(Json(
        forms::publish_form(&state.db, org, form_id, &locale).await?,
    ))
}

#[instrument(skip(state, admin, params, locale))]
async fn archive_form(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(params): Query<DefaultParams>,
    LocaleExtractor(locale): LocaleExtractor,
    Path(form_id): Path<Uuid>,
) -> Result<Json<DemographicFormDto>, ComhairleError> {
    let org = required_organization(&state, &admin, params.organization_id).await?;
    Ok(Json(
        forms::archive_form(&state.db, org, form_id, &locale).await?,
    ))
}

// System defaults (super admin)

#[instrument(skip(state, locale))]
async fn list_defaults(
    State(state): State<Arc<ComhairleState>>,
    LocaleExtractor(locale): LocaleExtractor,
) -> Result<Json<Vec<QuestionDto>>, ComhairleError> {
    Ok(Json(
        questions::list_questions(&state.db, None, &locale).await?,
    ))
}

#[instrument(skip(state, locale, body))]
async fn create_default(
    State(state): State<Arc<ComhairleState>>,
    LocaleExtractor(locale): LocaleExtractor,
    Json(body): Json<QuestionDraftInput>,
) -> Result<(StatusCode, Json<QuestionDto>), ComhairleError> {
    let question = questions::create_question(&state.db, None, &body, &locale).await?;
    Ok((StatusCode::CREATED, Json(question)))
}

#[instrument(skip(state, locale, body))]
async fn update_default_draft(
    State(state): State<Arc<ComhairleState>>,
    LocaleExtractor(locale): LocaleExtractor,
    Path(question_id): Path<Uuid>,
    Json(body): Json<QuestionDraftInput>,
) -> Result<Json<QuestionDto>, ComhairleError> {
    let question = questions::update_draft(&state.db, None, question_id, &body, &locale).await?;
    Ok(Json(question))
}

#[instrument(skip(state, locale))]
async fn publish_default(
    State(state): State<Arc<ComhairleState>>,
    LocaleExtractor(locale): LocaleExtractor,
    Path(question_id): Path<Uuid>,
) -> Result<Json<QuestionDto>, ComhairleError> {
    let question = questions::publish_question(&state.db, None, question_id, &locale).await?;
    Ok(Json(question))
}

// Participant runtime

#[instrument(skip(state, user, locale))]
async fn pending_step_questions(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    LocaleExtractor(locale): LocaleExtractor,
    Path(step_id): Path<Uuid>,
) -> Result<Json<Vec<PendingQuestion>>, ComhairleError> {
    let step = workflow_step::get_by_id(&state.db, &step_id).await?;
    let workflow = workflow::get_by_id(&state.db, &step.workflow_id).await?;
    let conversation_id = workflow.conversation_id.ok_or_else(|| {
        ComhairleError::ResourceNotFound("workflow is not attached to a conversation".into())
    })?;

    if !can_view_conversation(&state, Some(&user), conversation_id).await? {
        return Err(ComhairleError::UserNotAuthorized);
    }

    let pending = steps::pending_questions(&state.db, user.id, &step, &locale).await?;
    Ok(Json(pending))
}

#[instrument(skip(state, user, locale, body))]
async fn set_question_consent(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    LocaleExtractor(locale): LocaleExtractor,
    Path(version_id): Path<Uuid>,
    Json(body): Json<ConsentInput>,
) -> Result<Json<ConsentState>, ComhairleError> {
    let consent =
        responses::set_consent(&state.db, user.id, version_id, body.granted, &locale).await?;
    Ok(Json(consent))
}

#[instrument(skip(state, user, locale, body))]
async fn put_my_response(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    LocaleExtractor(locale): LocaleExtractor,
    Path(version_id): Path<Uuid>,
    Json(body): Json<ResponseInput>,
) -> Result<Json<ResponseDto>, ComhairleError> {
    let response = responses::put_response(&state.db, user.id, version_id, &body, &locale).await?;
    Ok(Json(response))
}

#[instrument(skip(state, user, locale))]
async fn list_my_answers(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    LocaleExtractor(locale): LocaleExtractor,
) -> Result<Json<Vec<MyDemographicAnswer>>, ComhairleError> {
    Ok(Json(
        responses::my_answers(&state.db, user.id, &locale).await?,
    ))
}

/// Detaches one of the caller's answers from their account. Answers are never deleted.
#[instrument(skip(state, user))]
async fn anonymise_my_answer(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path(response_id): Path<Uuid>,
) -> Result<StatusCode, ComhairleError> {
    responses::anonymise_response(&state.db, user.id, response_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::sync::Arc;

    use axum::http::StatusCode;
    use serde_json::json;
    use sqlx::PgPool;
    use uuid::Uuid;

    use crate::setup_server;
    use crate::test_helpers::{TEST_PASSWORD, UserSession, test_state};

    const DEFAULT_QUESTION_COUNT: usize = 5;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_show_system_questions_without_organization_but_block_changes(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin = UserSession::new_admin();
        admin.signup(&app).await?;

        let (status, questions, _) = admin.get(&app, "/demographics/questions").await?;
        assert_eq!(
            status,
            StatusCode::OK,
            "admins outside an organization can list questions"
        );
        let questions = questions.as_array().ok_or("questions should be a list")?;
        assert_eq!(questions.len(), DEFAULT_QUESTION_COUNT);
        assert!(questions.iter().all(|q| q["organization_id"].is_null()));

        let default_id = questions[0]["id"]
            .as_str()
            .ok_or("question id")?
            .to_string();
        let (status, _, _) = admin
            .get(&app, &format!("/demographics/questions/{default_id}"))
            .await?;
        assert_eq!(status, StatusCode::OK);

        let (status, forms, _) = admin.get(&app, "/demographics/forms").await?;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(forms, json!([]));

        let (status, tags, _) = admin.get(&app, "/demographics/tags").await?;
        assert_eq!(status, StatusCode::OK);
        assert!(!tags.as_array().ok_or("tags should be a list")?.is_empty());

        let (status, _, _) = admin
            .post(
                &app,
                "/demographics/tags",
                json!({ "name": "Not allowed" }).to_string().into(),
            )
            .await?;
        assert_eq!(
            status,
            StatusCode::FORBIDDEN,
            "creating a tag needs an organization"
        );

        let (status, _, _) = admin
            .get(&app, &format!("/demographics/forms/{}", Uuid::new_v4()))
            .await?;
        assert_eq!(status, StatusCode::NOT_FOUND);

        let (status, _, _) = admin
            .post(
                &app,
                "/demographics/defaults",
                json!({ "text": "Only super admins", "question_type": "text" })
                    .to_string()
                    .into(),
            )
            .await?;
        assert_eq!(
            status,
            StatusCode::FORBIDDEN,
            "only super admins manage system defaults"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_only_manage_organizations_the_admin_belongs_to(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut member = UserSession::new_admin();
        member.signup(&app).await?;
        let (status, organization, _) = member.create_random_organization(&app).await?;
        assert_eq!(status, StatusCode::CREATED);
        let org_id = organization["id"]
            .as_str()
            .ok_or("organization id")?
            .to_string();
        let paramsd_tags = format!("/demographics/tags?organization={org_id}");
        let paramsd_questions = format!("/demographics/questions?organization={org_id}");
        let paramsd_forms = format!("/demographics/forms?organization={org_id}");

        let (status, _, _) = member
            .post(
                &app,
                &paramsd_tags,
                json!({ "name": "Community" }).to_string().into(),
            )
            .await?;
        assert_eq!(
            status,
            StatusCode::CREATED,
            "members can manage their organization"
        );

        let (status, questions, _) = member.get(&app, &paramsd_questions).await?;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            questions
                .as_array()
                .ok_or("questions should be a list")?
                .len(),
            DEFAULT_QUESTION_COUNT
        );

        let (status, forms, _) = member.get(&app, &paramsd_forms).await?;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(forms, json!([]));

        let mut outsider = UserSession::new("test_admin", TEST_PASSWORD, "test@crown-shy.com");
        outsider.signup(&app).await?;

        let (status, _, _) = outsider.get(&app, &paramsd_questions).await?;
        assert_eq!(
            status,
            StatusCode::FORBIDDEN,
            "non-members cannot read an organization's questions"
        );

        let (status, _, _) = outsider.get(&app, &paramsd_forms).await?;
        assert_eq!(status, StatusCode::FORBIDDEN);

        let (status, _, _) = outsider
            .post(
                &app,
                &paramsd_tags,
                json!({ "name": "Not allowed" }).to_string().into(),
            )
            .await?;
        assert_eq!(status, StatusCode::FORBIDDEN);

        let (status, questions, _) = outsider.get(&app, "/demographics/questions").await?;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            questions
                .as_array()
                .ok_or("questions should be a list")?
                .len(),
            DEFAULT_QUESTION_COUNT,
            "system defaults stay readable without a selected organization"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_refuse_demographics_to_users_without_admin_permission(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut participant = UserSession::new(
            "demo_participant",
            TEST_PASSWORD,
            "participant@crown-shy.com",
        );
        participant.signup(&app).await?;

        let (status, _, _) = participant.get(&app, "/demographics/questions").await?;
        assert_eq!(status, StatusCode::FORBIDDEN);

        Ok(())
    }
}
