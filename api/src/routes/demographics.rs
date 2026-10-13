use std::sync::Arc;

use aide::axum::ApiRouter;
use aide::axum::routing::{delete_with, get_with, post_with, put_with};
use axum::Json;
use axum::extract::{Path, Query, State};
use axum::middleware::from_fn_with_state;
use hyper::StatusCode;
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;
use uuid::Uuid;

use crate::ComhairleState;
use crate::error::ComhairleError;
use crate::middleware::permissions::{PermissionRequirement, authorize as permission_middleware};
use crate::models::conversation;
use crate::models::demographics::forms::{self, FormDraftInput, FormDto};
use crate::models::demographics::questions::{self, QuestionDraftInput, QuestionDto, TagDto};
use crate::models::demographics::responses::{
    self, ConsentInput, ConsentState, ResponseDto, ResponseInput,
};
use crate::models::demographics::steps::{self, PendingQuestion};
use crate::models::permissions::system::Action as SystemAction;
use crate::models::permissions::{SystemResource, can_perform_action, conversation::Action};
use crate::models::user_group;
use crate::models::users::User;
use crate::models::workflow;
use crate::models::workflow_step;
use crate::routes::auth::RequiredUser;
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

/// The organization an admin selected in the demographics UI, sent as `?organization=`.
#[derive(Deserialize, JsonSchema)]
struct OrganizationScope {
    organization: Option<Uuid>,
}

/// The selected organization, if any. Admins may only select organizations they belong to.
async fn selected_organization(
    state: &Arc<ComhairleState>,
    admin: &User,
    scope: &OrganizationScope,
) -> Result<Option<Uuid>, ComhairleError> {
    let Some(org) = scope.organization else {
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
    scope: &OrganizationScope,
) -> Result<Uuid, ComhairleError> {
    selected_organization(state, admin, scope)
        .await?
        .ok_or(ComhairleError::UserNotAuthorized)
}

#[derive(Deserialize, JsonSchema)]
struct CreateTagInput {
    name: String,
}

// Organization question bank

#[instrument(skip(state, admin, scope))]
async fn list_tags(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(scope): Query<OrganizationScope>,
) -> Result<Json<Vec<TagDto>>, ComhairleError> {
    let org = selected_organization(&state, &admin, &scope).await?;
    Ok(Json(questions::list_tags(&state.db, org).await?))
}

#[instrument(skip(state, admin, scope, body))]
async fn create_tag(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(scope): Query<OrganizationScope>,
    Json(body): Json<CreateTagInput>,
) -> Result<(StatusCode, Json<TagDto>), ComhairleError> {
    let org = required_organization(&state, &admin, &scope).await?;
    let tag = questions::create_tag(&state.db, Some(org), &body.name).await?;
    Ok((StatusCode::CREATED, Json(tag)))
}

#[instrument(skip(state, admin, scope, locale))]
async fn list_questions(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(scope): Query<OrganizationScope>,
    LocaleExtractor(locale): LocaleExtractor,
) -> Result<Json<Vec<QuestionDto>>, ComhairleError> {
    let org = selected_organization(&state, &admin, &scope).await?;
    Ok(Json(
        questions::list_questions(&state.db, org, &locale).await?,
    ))
}

#[instrument(skip(state, admin, scope, locale, body))]
async fn create_question(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(scope): Query<OrganizationScope>,
    LocaleExtractor(locale): LocaleExtractor,
    Json(body): Json<QuestionDraftInput>,
) -> Result<(StatusCode, Json<QuestionDto>), ComhairleError> {
    let org = required_organization(&state, &admin, &scope).await?;
    let question = questions::create_question(&state.db, Some(org), &body, &locale).await?;
    Ok((StatusCode::CREATED, Json(question)))
}

#[instrument(skip(state, admin, scope, locale))]
async fn get_question(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(scope): Query<OrganizationScope>,
    LocaleExtractor(locale): LocaleExtractor,
    Path(question_id): Path<Uuid>,
) -> Result<Json<QuestionDto>, ComhairleError> {
    let org = selected_organization(&state, &admin, &scope).await?;
    let question = questions::get_question(&state.db, org, question_id, &locale).await?;
    Ok(Json(question))
}

#[instrument(skip(state, admin, scope, locale))]
async fn start_question_draft(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(scope): Query<OrganizationScope>,
    LocaleExtractor(locale): LocaleExtractor,
    Path(question_id): Path<Uuid>,
) -> Result<Json<QuestionDto>, ComhairleError> {
    let org = required_organization(&state, &admin, &scope).await?;
    let question = questions::start_draft(&state.db, Some(org), question_id, &locale).await?;
    Ok(Json(question))
}

#[instrument(skip(state, admin, scope, locale, body))]
async fn update_question_draft(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(scope): Query<OrganizationScope>,
    LocaleExtractor(locale): LocaleExtractor,
    Path(question_id): Path<Uuid>,
    Json(body): Json<QuestionDraftInput>,
) -> Result<Json<QuestionDto>, ComhairleError> {
    let org = required_organization(&state, &admin, &scope).await?;
    let question =
        questions::update_draft(&state.db, Some(org), question_id, &body, &locale).await?;
    Ok(Json(question))
}

#[instrument(skip(state, admin, scope))]
async fn discard_question_draft(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(scope): Query<OrganizationScope>,
    Path(question_id): Path<Uuid>,
) -> Result<StatusCode, ComhairleError> {
    let org = required_organization(&state, &admin, &scope).await?;
    questions::discard_draft(&state.db, Some(org), question_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[instrument(skip(state, admin, scope, locale))]
async fn publish_question(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(scope): Query<OrganizationScope>,
    LocaleExtractor(locale): LocaleExtractor,
    Path(question_id): Path<Uuid>,
) -> Result<Json<QuestionDto>, ComhairleError> {
    let org = required_organization(&state, &admin, &scope).await?;
    let question = questions::publish_question(&state.db, Some(org), question_id, &locale).await?;
    Ok(Json(question))
}

#[instrument(skip(state, admin, scope, locale))]
async fn archive_question(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(scope): Query<OrganizationScope>,
    LocaleExtractor(locale): LocaleExtractor,
    Path(question_id): Path<Uuid>,
) -> Result<Json<QuestionDto>, ComhairleError> {
    let org = required_organization(&state, &admin, &scope).await?;
    let question = questions::archive_question(&state.db, Some(org), question_id, &locale).await?;
    Ok(Json(question))
}

#[instrument(skip(state, admin, scope, locale))]
async fn list_forms(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(scope): Query<OrganizationScope>,
    LocaleExtractor(locale): LocaleExtractor,
) -> Result<Json<Vec<FormDto>>, ComhairleError> {
    // Forms belong to an organization and there are no system forms, so this is empty without one.
    let forms = match selected_organization(&state, &admin, &scope).await? {
        Some(org) => forms::list_forms(&state.db, org, &locale).await?,
        None => Vec::new(),
    };
    Ok(Json(forms))
}

#[instrument(skip(state, admin, scope, locale, body))]
async fn create_form(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(scope): Query<OrganizationScope>,
    LocaleExtractor(locale): LocaleExtractor,
    Json(body): Json<FormDraftInput>,
) -> Result<(StatusCode, Json<FormDto>), ComhairleError> {
    let org = required_organization(&state, &admin, &scope).await?;
    let form = forms::create_form(&state.db, org, &body, &locale).await?;
    Ok((StatusCode::CREATED, Json(form)))
}

#[instrument(skip(state, admin, scope, locale))]
async fn get_form(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(scope): Query<OrganizationScope>,
    LocaleExtractor(locale): LocaleExtractor,
    Path(form_id): Path<Uuid>,
) -> Result<Json<FormDto>, ComhairleError> {
    let org = selected_organization(&state, &admin, &scope)
        .await?
        .ok_or_else(|| ComhairleError::ResourceNotFound("demographic form".into()))?;
    Ok(Json(
        forms::get_form(&state.db, org, form_id, &locale).await?,
    ))
}

#[instrument(skip(state, admin, scope, locale))]
async fn start_form_draft(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(scope): Query<OrganizationScope>,
    LocaleExtractor(locale): LocaleExtractor,
    Path(form_id): Path<Uuid>,
) -> Result<Json<FormDto>, ComhairleError> {
    let org = required_organization(&state, &admin, &scope).await?;
    Ok(Json(
        forms::start_form_draft(&state.db, org, form_id, &locale).await?,
    ))
}

#[instrument(skip(state, admin, scope, locale, body))]
async fn update_form_draft(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(scope): Query<OrganizationScope>,
    LocaleExtractor(locale): LocaleExtractor,
    Path(form_id): Path<Uuid>,
    Json(body): Json<FormDraftInput>,
) -> Result<Json<FormDto>, ComhairleError> {
    let org = required_organization(&state, &admin, &scope).await?;
    Ok(Json(
        forms::update_form_draft(&state.db, org, form_id, &body, &locale).await?,
    ))
}

#[instrument(skip(state, admin, scope))]
async fn discard_form_draft(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(scope): Query<OrganizationScope>,
    Path(form_id): Path<Uuid>,
) -> Result<StatusCode, ComhairleError> {
    let org = required_organization(&state, &admin, &scope).await?;
    forms::discard_form_draft(&state.db, org, form_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[instrument(skip(state, admin, scope, locale))]
async fn publish_form(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(scope): Query<OrganizationScope>,
    LocaleExtractor(locale): LocaleExtractor,
    Path(form_id): Path<Uuid>,
) -> Result<Json<FormDto>, ComhairleError> {
    let org = required_organization(&state, &admin, &scope).await?;
    Ok(Json(
        forms::publish_form(&state.db, org, form_id, &locale).await?,
    ))
}

#[instrument(skip(state, admin, scope, locale))]
async fn archive_form(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(admin): RequiredUser,
    Query(scope): Query<OrganizationScope>,
    LocaleExtractor(locale): LocaleExtractor,
    Path(form_id): Path<Uuid>,
) -> Result<Json<FormDto>, ComhairleError> {
    let org = required_organization(&state, &admin, &scope).await?;
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

#[instrument(skip(state, user))]
async fn list_my_responses(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
) -> Result<Json<Vec<ResponseDto>>, ComhairleError> {
    Ok(Json(responses::my_responses(&state.db, user.id).await?))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    let requires = |action: SystemAction| {
        from_fn_with_state(
            PermissionRequirement::<SystemResource>::new(action),
            permission_middleware::<SystemResource>,
        )
    };
    ApiRouter::new()
        .api_route(
            "/tags",
            get_with(list_tags, |op| {
                op.id("ListDemographicTags")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<Vec<TagDto>>>()
            })
            .route_layer(requires(SystemAction::DemographicRead)),
        )
        .api_route(
            "/tags",
            post_with(create_tag, |op| {
                op.id("CreateDemographicTag")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<201, Json<TagDto>>()
            })
            .route_layer(requires(SystemAction::DemographicManage)),
        )
        .api_route(
            "/questions",
            get_with(list_questions, |op| {
                op.id("ListDemographicQuestions")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<Vec<QuestionDto>>>()
            })
            .route_layer(requires(SystemAction::DemographicRead)),
        )
        .api_route(
            "/questions",
            post_with(create_question, |op| {
                op.id("CreateDemographicQuestion")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<201, Json<QuestionDto>>()
            })
            .route_layer(requires(SystemAction::DemographicManage)),
        )
        .api_route(
            "/questions/{question_id}",
            get_with(get_question, |op| {
                op.id("GetDemographicQuestion")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<QuestionDto>>()
            })
            .route_layer(requires(SystemAction::DemographicRead)),
        )
        .api_route(
            "/questions/{question_id}/draft",
            post_with(start_question_draft, |op| {
                op.id("StartDemographicQuestionDraft")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<QuestionDto>>()
            })
            .route_layer(requires(SystemAction::DemographicManage)),
        )
        .api_route(
            "/questions/{question_id}/draft",
            put_with(update_question_draft, |op| {
                op.id("UpdateDemographicQuestionDraft")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<QuestionDto>>()
            })
            .route_layer(requires(SystemAction::DemographicManage)),
        )
        .api_route(
            "/questions/{question_id}/draft",
            delete_with(discard_question_draft, |op| {
                op.id("DiscardDemographicQuestionDraft")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response_with::<204, (), _>(|res| res.description("Draft discarded"))
            })
            .route_layer(requires(SystemAction::DemographicManage)),
        )
        .api_route(
            "/questions/{question_id}/publish",
            post_with(publish_question, |op| {
                op.id("PublishDemographicQuestion")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<QuestionDto>>()
            })
            .route_layer(requires(SystemAction::DemographicManage)),
        )
        .api_route(
            "/questions/{question_id}/archive",
            post_with(archive_question, |op| {
                op.id("ArchiveDemographicQuestion")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<QuestionDto>>()
            })
            .route_layer(requires(SystemAction::DemographicManage)),
        )
        .api_route(
            "/forms",
            get_with(list_forms, |op| {
                op.id("ListDemographicForms")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<Vec<FormDto>>>()
            })
            .route_layer(requires(SystemAction::DemographicRead)),
        )
        .api_route(
            "/forms",
            post_with(create_form, |op| {
                op.id("CreateDemographicForm")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<201, Json<FormDto>>()
            })
            .route_layer(requires(SystemAction::DemographicManage)),
        )
        .api_route(
            "/forms/{form_id}",
            get_with(get_form, |op| {
                op.id("GetDemographicForm")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<FormDto>>()
            })
            .route_layer(requires(SystemAction::DemographicRead)),
        )
        .api_route(
            "/forms/{form_id}/draft",
            post_with(start_form_draft, |op| {
                op.id("StartDemographicFormDraft")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<FormDto>>()
            })
            .route_layer(requires(SystemAction::DemographicManage)),
        )
        .api_route(
            "/forms/{form_id}/draft",
            put_with(update_form_draft, |op| {
                op.id("UpdateDemographicFormDraft")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<FormDto>>()
            })
            .route_layer(requires(SystemAction::DemographicManage)),
        )
        .api_route(
            "/forms/{form_id}/draft",
            delete_with(discard_form_draft, |op| {
                op.id("DiscardDemographicFormDraft")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response_with::<204, (), _>(|res| res.description("Draft discarded"))
            })
            .route_layer(requires(SystemAction::DemographicManage)),
        )
        .api_route(
            "/forms/{form_id}/publish",
            post_with(publish_form, |op| {
                op.id("PublishDemographicForm")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<FormDto>>()
            })
            .route_layer(requires(SystemAction::DemographicManage)),
        )
        .api_route(
            "/forms/{form_id}/archive",
            post_with(archive_form, |op| {
                op.id("ArchiveDemographicForm")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<FormDto>>()
            })
            .route_layer(requires(SystemAction::DemographicManage)),
        )
        .api_route(
            "/defaults",
            get_with(list_defaults, |op| {
                op.id("ListDemographicDefaults")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<Vec<QuestionDto>>>()
            })
            .route_layer(requires(SystemAction::DemographicDefaultManage)),
        )
        .api_route(
            "/defaults",
            post_with(create_default, |op| {
                op.id("CreateDemographicDefault")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<201, Json<QuestionDto>>()
            })
            .route_layer(requires(SystemAction::DemographicDefaultManage)),
        )
        .api_route(
            "/defaults/{question_id}/draft",
            put_with(update_default_draft, |op| {
                op.id("UpdateDemographicDefaultDraft")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<QuestionDto>>()
            })
            .route_layer(requires(SystemAction::DemographicDefaultManage)),
        )
        .api_route(
            "/defaults/{question_id}/publish",
            post_with(publish_default, |op| {
                op.id("PublishDemographicDefault")
                    .tag("Demographics")
                    .security_requirement("JWT")
                    .response::<200, Json<QuestionDto>>()
            })
            .route_layer(requires(SystemAction::DemographicDefaultManage)),
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

#[cfg(test)]
mod tests {
    use crate::setup_server;
    use crate::test_helpers::{TEST_PASSWORD, UserSession, test_state};
    use axum::http::StatusCode;
    use serde_json::json;
    use sqlx::PgPool;
    use std::error::Error;
    use std::sync::Arc;
    use uuid::Uuid;

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
                json!({ "text": "Only super admins", "question_type": "any_string" })
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
        let scoped_tags = format!("/demographics/tags?organization={org_id}");
        let scoped_questions = format!("/demographics/questions?organization={org_id}");
        let scoped_forms = format!("/demographics/forms?organization={org_id}");

        let (status, _, _) = member
            .post(
                &app,
                &scoped_tags,
                json!({ "name": "Community" }).to_string().into(),
            )
            .await?;
        assert_eq!(
            status,
            StatusCode::CREATED,
            "members can manage their organization"
        );

        let (status, questions, _) = member.get(&app, &scoped_questions).await?;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            questions.as_array().ok_or("questions should be a list")?.len(),
            DEFAULT_QUESTION_COUNT
        );

        let (status, forms, _) = member.get(&app, &scoped_forms).await?;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(forms, json!([]));

        let mut outsider = UserSession::new(
            "test_admin",
            TEST_PASSWORD,
            "test@crown-shy.com",
        );
        outsider.signup(&app).await?;

        let (status, _, _) = outsider.get(&app, &scoped_questions).await?;
        assert_eq!(
            status,
            StatusCode::FORBIDDEN,
            "non-members cannot read an organization's questions"
        );

        let (status, _, _) = outsider.get(&app, &scoped_forms).await?;
        assert_eq!(status, StatusCode::FORBIDDEN);

        let (status, _, _) = outsider
            .post(
                &app,
                &scoped_tags,
                json!({ "name": "Not allowed" }).to_string().into(),
            )
            .await?;
        assert_eq!(status, StatusCode::FORBIDDEN);

        let (status, questions, _) = outsider.get(&app, "/demographics/questions").await?;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            questions.as_array().ok_or("questions should be a list")?.len(),
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
