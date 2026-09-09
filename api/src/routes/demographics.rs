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
use crate::models::demographics::{
    self, ConversationDemographics, ConversationDemographicsFilterOptions,
    CreateConversationDemographics, CreateDemographicsQuestion, CreateDemographicsResponse,
    DemographicsQuestion, DemographicsQuestionsFilterOptions, DemographicsResponse,
    DemographicsResponsesFilterOptions, PartialDemographicsQuestion, PartialDemographicsResponse,
};
use crate::models::pagination::{PageOptions, PaginatedResults};

// ============================================================================
// Conversation-demographics associations - CR.D
// ============================================================================

/// Get all associations between conversations and demographics questions, with optional filters for conversation ID and question slug.
#[instrument(err(Debug), skip(state))]
pub async fn get_conversation_demographics(
    State(state): State<Arc<ComhairleState>>,
    Query(filters): Query<ConversationDemographicsFilterOptions>,
    Query(page_options): Query<PageOptions>,
) -> Result<(StatusCode, Json<PaginatedResults<ConversationDemographics>>), ComhairleError> {
    demographics::get_conversation_demographics(&state.db, filters, page_options)
        .await
        .map(|results| (StatusCode::OK, Json(results)))
}

/// Create a new association between a conversation and a demographics question.
#[instrument(err(Debug), skip(state))]
pub async fn create_conversation_demographics(
    State(state): State<Arc<ComhairleState>>,
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
    Json((conversation_id, question_slug)): Json<(Uuid, String)>,
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

/// Create a new demographics question.
#[instrument(err(Debug), skip(state))]
pub async fn create_demographics_question(
    State(state): State<Arc<ComhairleState>>,
    Json(payload): Json<CreateDemographicsQuestion>,
) -> Result<(StatusCode, Json<DemographicsQuestion>), ComhairleError> {
    demographics::create_demographics_question(&state.db, payload)
        .await
        .map(|result| (StatusCode::CREATED, Json(result)))
}

/// Update a demographics question.
#[instrument(err(Debug), skip(state))]
pub async fn update_demographics_question(
    State(state): State<Arc<ComhairleState>>,
    Path(question_slug): Path<String>,
    Json(payload): Json<PartialDemographicsQuestion>,
) -> Result<(StatusCode, Json<DemographicsQuestion>), ComhairleError> {
    demographics::update_demographics_question(&state.db, question_slug, payload)
        .await
        .map(|result| (StatusCode::OK, Json(result)))
}

/// Delete a demographics question.
#[instrument(err(Debug), skip(state))]
pub async fn delete_demographics_question(
    State(state): State<Arc<ComhairleState>>,
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
#[instrument(err(Debug), skip(state))]
pub async fn get_demographics_responses(
    State(state): State<Arc<ComhairleState>>,
    Query(filters): Query<DemographicsResponsesFilterOptions>,
    Query(page_options): Query<PageOptions>,
) -> Result<(StatusCode, Json<PaginatedResults<DemographicsResponse>>), ComhairleError> {
    demographics::get_demographics_responses(&state.db, filters, page_options)
        .await
        .map(|result| (StatusCode::OK, Json(result)))
}

/// Add a new response for a specific demographics question and user.
#[instrument(err(Debug), skip(state))]
pub async fn create_demographics_response(
    State(state): State<Arc<ComhairleState>>,
    Json(payload): Json<CreateDemographicsResponse>,
) -> Result<(StatusCode, Json<DemographicsResponse>), ComhairleError> {
    demographics::create_demographics_response(&state.db, payload)
        .await
        .map(|result| (StatusCode::CREATED, Json(result)))
}

/// Update a response for a specific demographics question and user.
#[instrument(err(Debug), skip(state))]
pub async fn update_demographics_response(
    State(state): State<Arc<ComhairleState>>,
    Path((question_slug, user_id)): Path<(String, Uuid)>,
    Json(payload): Json<PartialDemographicsResponse>,
) -> Result<(StatusCode, Json<DemographicsResponse>), ComhairleError> {
    demographics::update_demographics_response(&state.db, question_slug, user_id, payload)
        .await
        .map(|result| (StatusCode::OK, Json(result)))
}

/// Delete a response for a specific demographics question and user.
#[instrument(err(Debug), skip(state))]
pub async fn delete_demographics_response(
    State(state): State<Arc<ComhairleState>>,
    Path((question_slug, user_id)): Path<(String, Uuid)>,
) -> Result<(StatusCode, Json<Option<DemographicsResponse>>), ComhairleError> {
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
                    .response::<200, Json<PaginatedResults<ConversationDemographics>>>()
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
