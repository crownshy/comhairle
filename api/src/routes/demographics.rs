use std::sync::Arc;

use aide::axum::ApiRouter;
use aide::axum::routing::{get_with, post_with, put_with};
use axum::Json;
use axum::extract::{Path, State};
use hyper::StatusCode;
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;
use uuid::Uuid;

use crate::ComhairleState;
use crate::error::ComhairleError;
use crate::models::conversation;
use crate::models::demographics::forms::{self, FormDraftInput, FormDto};
use crate::models::demographics::questions::{self, QuestionDraftInput, QuestionDto, TagDto};
use crate::models::demographics::responses::{
    self, ConsentInput, ConsentState, ResponseDto, ResponseInput,
};
use crate::models::demographics::steps::{self, PendingQuestion};
use crate::models::permissions::{can_perform_action, conversation::Action};
use crate::models::users::User;
use crate::models::workflow;
use crate::models::workflow_step;
use crate::routes::auth::{RequiredAdminUser, RequiredSuperAdminUser, RequiredUser};
use crate::routes::translations::LocaleExtractor;

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

/// The organization whose question bank an admin is managing.
fn admin_organization(user: &User) -> Result<Uuid, ComhairleError> {
    user.organization_id
        .ok_or(ComhairleError::UserNotAuthorized)
}

#[derive(Deserialize, JsonSchema)]
struct CreateTagInput {
    name: String,
}

// Organization question bank

#[instrument(skip(state, admin))]
async fn list_tags(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(admin): RequiredAdminUser,
) -> Result<Json<Vec<TagDto>>, ComhairleError> {
    let org = admin_organization(&admin)?;
    Ok(Json(questions::list_tags(&state.db, Some(org)).await?))
}

#[instrument(skip(state, admin, body))]
async fn create_tag(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(admin): RequiredAdminUser,
    Json(body): Json<CreateTagInput>,
) -> Result<(StatusCode, Json<TagDto>), ComhairleError> {
    let org = admin_organization(&admin)?;
    let tag = questions::create_tag(&state.db, Some(org), &body.name).await?;
    Ok((StatusCode::CREATED, Json(tag)))
}

#[instrument(skip(state, admin, locale))]
async fn list_questions(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(admin): RequiredAdminUser,
    LocaleExtractor(locale): LocaleExtractor,
) -> Result<Json<Vec<QuestionDto>>, ComhairleError> {
    let org = admin_organization(&admin)?;
    Ok(Json(
        questions::list_questions(&state.db, Some(org), &locale).await?,
    ))
}

#[instrument(skip(state, admin, locale, body))]
async fn create_question(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(admin): RequiredAdminUser,
    LocaleExtractor(locale): LocaleExtractor,
    Json(body): Json<QuestionDraftInput>,
) -> Result<(StatusCode, Json<QuestionDto>), ComhairleError> {
    let org = admin_organization(&admin)?;
    let question = questions::create_question(&state.db, Some(org), &body, &locale).await?;
    Ok((StatusCode::CREATED, Json(question)))
}

#[instrument(skip(state, admin, locale))]
async fn get_question(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(admin): RequiredAdminUser,
    LocaleExtractor(locale): LocaleExtractor,
    Path(question_id): Path<Uuid>,
) -> Result<Json<QuestionDto>, ComhairleError> {
    let org = admin_organization(&admin)?;
    let question = questions::get_question(&state.db, Some(org), question_id, &locale).await?;
    Ok(Json(question))
}

#[instrument(skip(state, admin, locale))]
async fn start_question_draft(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(admin): RequiredAdminUser,
    LocaleExtractor(locale): LocaleExtractor,
    Path(question_id): Path<Uuid>,
) -> Result<Json<QuestionDto>, ComhairleError> {
    let org = admin_organization(&admin)?;
    let question = questions::start_draft(&state.db, Some(org), question_id, &locale).await?;
    Ok(Json(question))
}

#[instrument(skip(state, admin, locale, body))]
async fn update_question_draft(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(admin): RequiredAdminUser,
    LocaleExtractor(locale): LocaleExtractor,
    Path(question_id): Path<Uuid>,
    Json(body): Json<QuestionDraftInput>,
) -> Result<Json<QuestionDto>, ComhairleError> {
    let org = admin_organization(&admin)?;
    let question =
        questions::update_draft(&state.db, Some(org), question_id, &body, &locale).await?;
    Ok(Json(question))
}

#[instrument(skip(state, admin))]
async fn discard_question_draft(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(admin): RequiredAdminUser,
    Path(question_id): Path<Uuid>,
) -> Result<StatusCode, ComhairleError> {
    let org = admin_organization(&admin)?;
    questions::discard_draft(&state.db, Some(org), question_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[instrument(skip(state, admin, locale))]
async fn publish_question(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(admin): RequiredAdminUser,
    LocaleExtractor(locale): LocaleExtractor,
    Path(question_id): Path<Uuid>,
) -> Result<Json<QuestionDto>, ComhairleError> {
    let org = admin_organization(&admin)?;
    let question = questions::publish_question(&state.db, Some(org), question_id, &locale).await?;
    Ok(Json(question))
}

#[instrument(skip(state, admin, locale))]
async fn archive_question(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(admin): RequiredAdminUser,
    LocaleExtractor(locale): LocaleExtractor,
    Path(question_id): Path<Uuid>,
) -> Result<Json<QuestionDto>, ComhairleError> {
    let org = admin_organization(&admin)?;
    let question = questions::archive_question(&state.db, Some(org), question_id, &locale).await?;
    Ok(Json(question))
}

#[instrument(skip(state, admin, locale))]
async fn list_forms(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(admin): RequiredAdminUser,
    LocaleExtractor(locale): LocaleExtractor,
) -> Result<Json<Vec<FormDto>>, ComhairleError> {
    let org = admin_organization(&admin)?;
    Ok(Json(forms::list_forms(&state.db, org, &locale).await?))
}

#[instrument(skip(state, admin, locale, body))]
async fn create_form(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(admin): RequiredAdminUser,
    LocaleExtractor(locale): LocaleExtractor,
    Json(body): Json<FormDraftInput>,
) -> Result<(StatusCode, Json<FormDto>), ComhairleError> {
    let org = admin_organization(&admin)?;
    let form = forms::create_form(&state.db, org, &body, &locale).await?;
    Ok((StatusCode::CREATED, Json(form)))
}

#[instrument(skip(state, admin, locale))]
async fn get_form(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(admin): RequiredAdminUser,
    LocaleExtractor(locale): LocaleExtractor,
    Path(form_id): Path<Uuid>,
) -> Result<Json<FormDto>, ComhairleError> {
    let org = admin_organization(&admin)?;
    Ok(Json(
        forms::get_form(&state.db, org, form_id, &locale).await?,
    ))
}

#[instrument(skip(state, admin, locale))]
async fn start_form_draft(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(admin): RequiredAdminUser,
    LocaleExtractor(locale): LocaleExtractor,
    Path(form_id): Path<Uuid>,
) -> Result<Json<FormDto>, ComhairleError> {
    let org = admin_organization(&admin)?;
    Ok(Json(
        forms::start_form_draft(&state.db, org, form_id, &locale).await?,
    ))
}

#[instrument(skip(state, admin, locale, body))]
async fn update_form_draft(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(admin): RequiredAdminUser,
    LocaleExtractor(locale): LocaleExtractor,
    Path(form_id): Path<Uuid>,
    Json(body): Json<FormDraftInput>,
) -> Result<Json<FormDto>, ComhairleError> {
    let org = admin_organization(&admin)?;
    Ok(Json(
        forms::update_form_draft(&state.db, org, form_id, &body, &locale).await?,
    ))
}

#[instrument(skip(state, admin))]
async fn discard_form_draft(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(admin): RequiredAdminUser,
    Path(form_id): Path<Uuid>,
) -> Result<StatusCode, ComhairleError> {
    let org = admin_organization(&admin)?;
    forms::discard_form_draft(&state.db, org, form_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[instrument(skip(state, admin, locale))]
async fn publish_form(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(admin): RequiredAdminUser,
    LocaleExtractor(locale): LocaleExtractor,
    Path(form_id): Path<Uuid>,
) -> Result<Json<FormDto>, ComhairleError> {
    let org = admin_organization(&admin)?;
    Ok(Json(
        forms::publish_form(&state.db, org, form_id, &locale).await?,
    ))
}

#[instrument(skip(state, admin, locale))]
async fn archive_form(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(admin): RequiredAdminUser,
    LocaleExtractor(locale): LocaleExtractor,
    Path(form_id): Path<Uuid>,
) -> Result<Json<FormDto>, ComhairleError> {
    let org = admin_organization(&admin)?;
    Ok(Json(
        forms::archive_form(&state.db, org, form_id, &locale).await?,
    ))
}

// System defaults (super admin)

#[instrument(skip(state, _admin, locale))]
async fn list_defaults(
    State(state): State<Arc<ComhairleState>>,
    _admin: RequiredSuperAdminUser,
    LocaleExtractor(locale): LocaleExtractor,
) -> Result<Json<Vec<QuestionDto>>, ComhairleError> {
    Ok(Json(
        questions::list_questions(&state.db, None, &locale).await?,
    ))
}

#[instrument(skip(state, _admin, locale, body))]
async fn create_default(
    State(state): State<Arc<ComhairleState>>,
    _admin: RequiredSuperAdminUser,
    LocaleExtractor(locale): LocaleExtractor,
    Json(body): Json<QuestionDraftInput>,
) -> Result<(StatusCode, Json<QuestionDto>), ComhairleError> {
    let question = questions::create_question(&state.db, None, &body, &locale).await?;
    Ok((StatusCode::CREATED, Json(question)))
}

#[instrument(skip(state, _admin, locale, body))]
async fn update_default_draft(
    State(state): State<Arc<ComhairleState>>,
    _admin: RequiredSuperAdminUser,
    LocaleExtractor(locale): LocaleExtractor,
    Path(question_id): Path<Uuid>,
    Json(body): Json<QuestionDraftInput>,
) -> Result<Json<QuestionDto>, ComhairleError> {
    let question = questions::update_draft(&state.db, None, question_id, &body, &locale).await?;
    Ok(Json(question))
}

#[instrument(skip(state, _admin, locale))]
async fn publish_default(
    State(state): State<Arc<ComhairleState>>,
    _admin: RequiredSuperAdminUser,
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

#[instrument(skip(state, user))]
async fn list_my_responses(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
) -> Result<Json<Vec<ResponseDto>>, ComhairleError> {
    Ok(Json(responses::my_responses(&state.db, user.id).await?))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/tags",
            get_with(list_tags, |op| {
                op.id("ListDemographicTags")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<Vec<TagDto>>>()
            })
            .post_with(create_tag, |op| {
                op.id("CreateDemographicTag")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<201, Json<TagDto>>()
            }),
        )
        .api_route(
            "/questions",
            get_with(list_questions, |op| {
                op.id("ListDemographicQuestions")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<Vec<QuestionDto>>>()
            })
            .post_with(create_question, |op| {
                op.id("CreateDemographicQuestion")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<201, Json<QuestionDto>>()
            }),
        )
        .api_route(
            "/questions/{question_id}",
            get_with(get_question, |op| {
                op.id("GetDemographicQuestion")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<QuestionDto>>()
            }),
        )
        .api_route(
            "/questions/{question_id}/draft",
            post_with(start_question_draft, |op| {
                op.id("StartDemographicQuestionDraft")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<QuestionDto>>()
            })
            .put_with(update_question_draft, |op| {
                op.id("UpdateDemographicQuestionDraft")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<QuestionDto>>()
            })
            .delete_with(discard_question_draft, |op| {
                op.id("DiscardDemographicQuestionDraft")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response_with::<204, (), _>(|res| res.description("Draft discarded"))
            }),
        )
        .api_route(
            "/questions/{question_id}/publish",
            post_with(publish_question, |op| {
                op.id("PublishDemographicQuestion")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<QuestionDto>>()
            }),
        )
        .api_route(
            "/questions/{question_id}/archive",
            post_with(archive_question, |op| {
                op.id("ArchiveDemographicQuestion")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<QuestionDto>>()
            }),
        )
        .api_route(
            "/forms",
            get_with(list_forms, |op| {
                op.id("ListDemographicForms")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<Vec<FormDto>>>()
            })
            .post_with(create_form, |op| {
                op.id("CreateDemographicForm")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<201, Json<FormDto>>()
            }),
        )
        .api_route(
            "/forms/{form_id}",
            get_with(get_form, |op| {
                op.id("GetDemographicForm")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<FormDto>>()
            }),
        )
        .api_route(
            "/forms/{form_id}/draft",
            post_with(start_form_draft, |op| {
                op.id("StartDemographicFormDraft")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<FormDto>>()
            })
            .put_with(update_form_draft, |op| {
                op.id("UpdateDemographicFormDraft")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<FormDto>>()
            })
            .delete_with(discard_form_draft, |op| {
                op.id("DiscardDemographicFormDraft")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response_with::<204, (), _>(|res| res.description("Draft discarded"))
            }),
        )
        .api_route(
            "/forms/{form_id}/publish",
            post_with(publish_form, |op| {
                op.id("PublishDemographicForm")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<FormDto>>()
            }),
        )
        .api_route(
            "/forms/{form_id}/archive",
            post_with(archive_form, |op| {
                op.id("ArchiveDemographicForm")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<FormDto>>()
            }),
        )
        .api_route(
            "/defaults",
            get_with(list_defaults, |op| {
                op.id("ListDemographicDefaults")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<Vec<QuestionDto>>>()
            })
            .post_with(create_default, |op| {
                op.id("CreateDemographicDefault")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<201, Json<QuestionDto>>()
            }),
        )
        .api_route(
            "/defaults/{question_id}/draft",
            put_with(update_default_draft, |op| {
                op.id("UpdateDemographicDefaultDraft")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<QuestionDto>>()
            }),
        )
        .api_route(
            "/defaults/{question_id}/publish",
            post_with(publish_default, |op| {
                op.id("PublishDemographicDefault")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<QuestionDto>>()
            }),
        )
        .api_route(
            "/steps/{step_id}/pending",
            get_with(pending_step_questions, |op| {
                op.id("GetPendingDemographicQuestions")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<Vec<PendingQuestion>>>()
            }),
        )
        .api_route(
            "/question-versions/{version_id}/consent",
            put_with(set_question_consent, |op| {
                op.id("SetDemographicConsent")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<ConsentState>>()
            }),
        )
        .api_route(
            "/question-versions/{version_id}/response",
            put_with(put_my_response, |op| {
                op.id("PutDemographicResponse")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<ResponseDto>>()
            }),
        )
        .api_route(
            "/me/responses",
            get_with(list_my_responses, |op| {
                op.id("ListMyDemographicResponses")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<Vec<ResponseDto>>>()
            }),
        )
        .api_route(
            "/me/responses/{version_id}",
            put_with(put_my_response, |op| {
                op.id("PutMyDemographicResponse")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<ResponseDto>>()
            }),
        )
        .with_state(state.clone())
}
