use std::sync::Arc;

use aide::axum::ApiRouter;
use aide::axum::routing::{delete_with, get_with, post_with, put_with};
use axum::Json;
use axum::extract::{Path, Query, State};
use hyper::StatusCode;
use tracing::instrument;
use uuid::Uuid;

use crate::ComhairleState;
use crate::error::ComhairleError;
use crate::models::conversation;
use crate::models::demographics::{
    self, ConversationDemographics, ConversationDemographicsFilterOptions,
    CreateConversationDemographics, CreateDemographicsQuestion, CreateDemographicsResponse,
    DemographicsQuestion, DemographicsQuestionsFilterOptions, DemographicsResponse,
    DemographicsResponsesFilterOptions, PartialDemographicsQuestion, PartialDemographicsResponse,
};
use crate::models::pagination::{PageOptions, PaginatedResults};
use crate::models::permissions::{Action, can_perform_resource_action};
use crate::models::users::User;
use crate::routes::auth::{OptionalUser, RequiredAdminUser, RequiredUser, is_user_admin};

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

    can_perform_resource_action(
        state,
        &conversation.id,
        Action::ConversationRead,
        &user.id,
        user.organization_id.as_ref(),
        Some(&conversation.owner_id),
    )
    .await
}

// ============================================================================
// Conversation-demographics associations - CR.D
// ============================================================================

/// Get all associations between conversations and demographics questions, with optional filters for conversation ID and question slug.
/// A `conversation_id` filter is required for non-admins.
#[instrument(err(Debug), skip(state))]
pub async fn get_conversation_demographics(
    State(state): State<Arc<ComhairleState>>,
    OptionalUser(user): OptionalUser,
    Query(filters): Query<ConversationDemographicsFilterOptions>,
    Query(page_options): Query<PageOptions>,
) -> Result<(StatusCode, Json<PaginatedResults<ConversationDemographics>>), ComhairleError> {
    let is_admin = match &user {
        Some(user) => is_user_admin(&state, user).await,
        None => false,
    };

    if !is_admin {
        let conversation_id = filters
            .conversation_id
            .ok_or(ComhairleError::UserNotAuthorized)?;

        if !can_view_conversation(&state, user.as_ref(), conversation_id).await? {
            return Err(ComhairleError::UserNotAuthorized);
        }
    }

    demographics::get_conversation_demographics(&state.db, filters, page_options)
        .await
        .map(|results| (StatusCode::OK, Json(results)))
}

/// Create a new association between a conversation and a demographics question.
#[instrument(err(Debug), skip(state))]
pub async fn create_conversation_demographics(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(payload): Json<CreateConversationDemographics>,
) -> Result<(StatusCode, Json<ConversationDemographics>), ComhairleError> {
    demographics::create_conversation_demographics(&state.db, payload)
        .await
        .map(|result| (StatusCode::CREATED, Json(result)))
}

/// Remove an association between a conversation and a demographics question.
#[instrument(err(Debug), skip(state))]
pub async fn delete_conversation_demographics(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Path((conversation_id, question_slug)): Path<(Uuid, String)>,
) -> Result<(StatusCode, Json<Option<ConversationDemographics>>), ComhairleError> {
    demographics::delete_conversation_demographics(&state.db, conversation_id, question_slug)
        .await
        .map(|result| (StatusCode::OK, Json(result)))
}

// ============================================================================
// Demographics questions - CRUD
// ============================================================================

/// Get a demographics question by its slug.
#[instrument(err(Debug), skip(state))]
pub async fn get_demographics_questions(
    State(state): State<Arc<ComhairleState>>,
    Query(filters): Query<DemographicsQuestionsFilterOptions>,
    Query(page_options): Query<PageOptions>,
) -> Result<(StatusCode, Json<PaginatedResults<DemographicsQuestion>>), ComhairleError> {
    demographics::get_demographics_questions(&state.db, filters, page_options)
        .await
        .map(|result| (StatusCode::OK, Json(result)))
}

/// Create a new demographics question. Admin-only.
#[instrument(err(Debug), skip(state))]
pub async fn create_demographics_question(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(payload): Json<CreateDemographicsQuestion>,
) -> Result<(StatusCode, Json<DemographicsQuestion>), ComhairleError> {
    demographics::create_demographics_question(&state.db, payload)
        .await
        .map(|result| (StatusCode::CREATED, Json(result)))
}

/// Update a demographics question. Admin-only.
#[instrument(err(Debug), skip(state))]
pub async fn update_demographics_question(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Path(question_slug): Path<String>,
    Json(payload): Json<PartialDemographicsQuestion>,
) -> Result<(StatusCode, Json<DemographicsQuestion>), ComhairleError> {
    demographics::update_demographics_question(&state.db, question_slug, payload)
        .await
        .map(|result| (StatusCode::OK, Json(result)))
}

/// Delete a demographics question. Admin-only.
#[instrument(err(Debug), skip(state))]
pub async fn delete_demographics_question(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Path(question_slug): Path<String>,
) -> Result<(StatusCode, Json<Option<DemographicsQuestion>>), ComhairleError> {
    demographics::delete_demographics_question(&state.db, question_slug)
        .await
        .map(|result| (StatusCode::OK, Json(result)))
}

// ============================================================================
// Demographics responses - CRUD
// ============================================================================

/// Get all responses for a specific demographics question.
///
/// Non-admins get their `user_id` filter forced to themselves and cannot query
/// for other users.
#[instrument(err(Debug), skip(state))]
pub async fn get_demographics_responses(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Query(mut filters): Query<DemographicsResponsesFilterOptions>,
    Query(page_options): Query<PageOptions>,
) -> Result<(StatusCode, Json<PaginatedResults<DemographicsResponse>>), ComhairleError> {
    if !is_user_admin(&state, &user).await {
        if filters.user_id.is_some_and(|user_id| user_id != user.id) {
            return Err(ComhairleError::UserNotAuthorized);
        }
        filters.user_id = Some(user.id);
    }

    demographics::get_demographics_responses(&state.db, filters, page_options)
        .await
        .map(|result| (StatusCode::OK, Json(result)))
}

/// Add a new response for a specific demographics question and user.
#[instrument(err(Debug), skip(state))]
pub async fn create_demographics_response(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Json(payload): Json<CreateDemographicsResponse>,
) -> Result<(StatusCode, Json<DemographicsResponse>), ComhairleError> {
    if payload.user_id != user.id && !is_user_admin(&state, &user).await {
        return Err(ComhairleError::UserNotAuthorized);
    }

    demographics::create_demographics_response(&state.db, payload)
        .await
        .map(|result| (StatusCode::CREATED, Json(result)))
}

/// Update a response for a specific demographics question and user.
#[instrument(err(Debug), skip(state))]
pub async fn update_demographics_response(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path((question_slug, user_id)): Path<(String, Uuid)>,
    Json(payload): Json<PartialDemographicsResponse>,
) -> Result<(StatusCode, Json<DemographicsResponse>), ComhairleError> {
    if user_id != user.id && !is_user_admin(&state, &user).await {
        return Err(ComhairleError::UserNotAuthorized);
    }

    demographics::update_demographics_response(&state.db, question_slug, user_id, payload)
        .await
        .map(|result| (StatusCode::OK, Json(result)))
}

/// Delete a response for a specific demographics question and user.
#[instrument(err(Debug), skip(state))]
pub async fn delete_demographics_response(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path((question_slug, user_id)): Path<(String, Uuid)>,
) -> Result<(StatusCode, Json<Option<DemographicsResponse>>), ComhairleError> {
    if user_id != user.id && !is_user_admin(&state, &user).await {
        return Err(ComhairleError::UserNotAuthorized);
    }

    demographics::delete_demographics_response(&state.db, question_slug, user_id)
        .await
        .map(|result| (StatusCode::OK, Json(result)))
}

// ============================================================================
// Routes for demographics
// ============================================================================

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
    .nest_api_service(
        "/conversations_questions",
        ApiRouter::new()
            .api_route("/", get_with(get_conversation_demographics, |op| {
                op.id("GetConversationDemographics")
                    .tag("Demographics")
                    .summary("Get conversation demographics")
                    .description("Retrieve demographics responses for a specific conversation and question")
                    .security_requirement("JWT")
                    .response::<200, Json<PaginatedResults<ConversationDemographics>>>()
            }))
            .api_route("/", post_with(create_conversation_demographics, |op| {
                op.id("CreateConversationDemographics")
                    .tag("Demographics")
                    .summary("Create a conversation demographics response")
                    .description("Create a new demographics response for a specific conversation and question")
                    .security_requirement("JWT")
                    .response::<201, Json<ConversationDemographics>>()
            }))
            .api_route("/{conversation_id}/{question_slug}/", delete_with(delete_conversation_demographics, |op| {
                op.id("DeleteConversationDemographicsByQuestion")
                    .tag("Demographics")
                    .summary("Delete conversation demographics by question")
                    .description("Delete demographics responses for a specific conversation and question")
                    .security_requirement("JWT")
                    .response::<200, Json<Option<ConversationDemographics>>>()
            }))
            .with_state(state.clone())
    )
    .nest_api_service(
        "/questions",
        ApiRouter::new()
            .api_route("/", get_with(get_demographics_questions, |op| {
                op.id("GetDemographicsQuestions")
                    .tag("Demographics")
                    .summary("List of demographics questions")
                    .description("Paginated list of demographics questions with optional filtering and ordering")
                    .security_requirement("JWT")
                    .response::<200, Json<PaginatedResults<DemographicsQuestion>>>()
            }))
            .api_route("/", post_with(create_demographics_question, |op| {
                op.id("CreateDemographicsQuestion")
                    .tag("Demographics")
                    .summary("Create a demographics question")
                    .description("Create a new demographics question")
                    .security_requirement("JWT")
                    .response::<201, Json<DemographicsQuestion>>()
            }))
            .api_route("/{question_slug}", put_with(update_demographics_question, |op| {
                op.id("UpdateDemographicsQuestion")
                    .tag("Demographics")
                    .summary("Update a demographics question")
                    .description("Update a specific demographics question")
                    .security_requirement("JWT")
                    .response::<200, Json<DemographicsQuestion>>()
            }))
            .api_route("/{question_slug}", delete_with(delete_demographics_question, |op| {
                op.id("DeleteDemographicsQuestion")
                    .tag("Demographics")
                    .summary("Delete a demographics question")
                    .description("Delete a specific demographics question")
                    .security_requirement("JWT")
                    .response::<200, Json<Option<DemographicsQuestion>>>()
            }))
            .with_state(state.clone())
        )
        .nest_api_service(
            "/responses",
            ApiRouter::new()
                .api_route("/", get_with(get_demographics_responses, |op| {
                    op.id("GetDemographicsResponses")
                        .tag("Demographics")
                        .summary("List of demographics responses")
                        .description("Paginated list of demographics responses with optional filtering and ordering")
                        .security_requirement("JWT")
                        .response::<200, Json<PaginatedResults<DemographicsResponse>>>()
                }))
                .api_route("/", post_with(create_demographics_response, |op| {
                    op.id("CreateDemographicsResponse")
                        .tag("Demographics")
                        .summary("Create a demographics response")
                        .description("Create a new response for a specific demographics question and user")
                        .security_requirement("JWT")
                        .response::<201, Json<DemographicsResponse>>()
                }))
                .api_route("/{question_slug}/{user_id}", put_with(update_demographics_response, |op| {
                    op.id("UpdateDemographicsResponse")
                        .tag("Demographics")
                        .summary("Update a demographics response")
                        .description("Update a response for a specific demographics question and user")
                        .security_requirement("JWT")
                        .response::<200, Json<DemographicsResponse>>()
                }))
                .api_route("/{question_slug}/{user_id}", delete_with(delete_demographics_response, |op| {
                    op.id("DeleteDemographicsResponse")
                        .tag("Demographics")
                        .summary("Delete a demographics response")
                        .description("Delete a response for a specific demographics question and user")
                        .security_requirement("JWT")
                        .response::<200, Json<Option<DemographicsResponse>>>()
                }))
                .with_state(state.clone())
        )
        .with_state(state.clone())
}
