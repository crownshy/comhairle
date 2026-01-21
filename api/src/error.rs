use crate::{tools::polis::PolisError, translation_service::TranslationError};
use aide::OperationIo;
use axum::{extract::multipart::MultipartError, http::StatusCode, response::IntoResponse, Json};
use heyform_sdk::HeyFormError;
use ragflow::RagflowError;
use schemars::JsonSchema;
use serde::Serialize;
use serde_json::json;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug, OperationIo)]
#[aide(output)]
pub enum ComhairleError {
    #[error("Database Failed to connect: {0}")]
    DbError(String),

    #[error("Database query error: {0}")]
    DbQueryError(#[from] sea_query::error::Error),

    #[error("Failed to load config: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Polis error: {0}")]
    PolisError(#[from] PolisError),

    #[error("Translation error: {0}")]
    TranslationError(#[from] TranslationError),

    #[error("No translation service configured")]
    NoTranslationServiceConfigured,

    #[error("HeyForm error: {0}")]
    HeyFormError(#[from] HeyFormError),

    #[error("Ragflow error: {0}")]
    RagflowError(#[from] RagflowError),

    #[error("Multipart form parse error: {0}")]
    MultipartParseForm(#[from] MultipartError),

    #[error("Template error: {0}")]
    TemplateError(#[from] minijinja::Error),

    #[error("Serde json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("Username {0} already taken")]
    DuplicateUsername(String),

    #[error("Email {0} already taken")]
    DuplicateEmail(String),

    #[error("Slug {0} already taken")]
    DuplicateSlug(String),

    #[error("Failed to hash password")]
    PasswordHash,

    #[error("The password and email don't match")]
    WrongPassword,

    #[error("The password and password confirmation don't match")]
    PasswordConfirmationMismatch,

    #[error("User Required for this route")]
    UserRequired,

    #[error("Auth Error {0}")]
    AuthJWTError(String),

    #[error("No user with email {0}")]
    NoUserFoundForEmail(String),

    #[error("No user with id {0}")]
    NoUserFoundForId(Uuid),

    #[error("No user found")]
    NoUserFound,

    #[error("{0} not found")]
    ResourceNotFound(String),

    #[error("Failed to create {resource_type}")]
    FailedToCreateResource {
        resource_type: String,
        error: sqlx::Error,
    },

    #[error("Fai;ed to parse order params: {0}")]
    FailedToParseOrderParams(String),

    #[error("User is already participating in workflow: {0}")]
    UserAlreadyParticipatingInWorkflow(String),

    #[error("Update request contained no valid parameters")]
    NoValidUpdates,

    #[error("Failed to create annon user")]
    FailedToCreateAnnonUser,

    #[error("Cant log this type of user in with this flow")]
    WrongUserType,

    #[error("User's email address is already verified")]
    EmailAlreadyVerified,

    #[error("No user logged in")]
    NoLogedInUser,

    #[error("User is not signed up to participate in the conversation")]
    UserIsNotParticipatingInTheConversation,

    #[error("Failed to get a presigned upload url {0}")]
    FailedToGetUploadPresign(String),

    #[error("Failed to get a presigned download url {0}")]
    FailedToGetDownloadPresign(String),

    #[error("Failed to get resource {0}")]
    NoResourceFoundForId(Uuid),

    #[error("Workflow Step has wrong type expected {0}")]
    WorkflowStepHasWrongType(String),

    #[error("Requires Auth User")]
    RequiresAuthUser,

    #[error("Only the owner of the conversation can perform this action")]
    UserIsNotConversationOwner,

    #[error("Failed to create report")]
    FailedToCreateReport(sqlx::Error),

    #[error("Failed to update report")]
    FailedToUpdateReport,

    #[error("Failed to create feedback")]
    FailedToCreateFeedback,

    #[error("Failed to create invite")]
    FailedToCreateInvite(sqlx::Error),

    #[error("Invite does not match logged in user")]
    InviteDoesNotMatchUser,

    #[error("This invite has expired")]
    InviteExpired,

    #[error("Failed to update feedback")]
    FailedToUpdateFeedback,

    #[error("Failed to create impact")]
    FailedToCreateImpact,

    #[error("Failed to update impact")]
    FailedToUpdateImpact(sqlx::Error),

    #[error("Failed to send email")]
    FailedToSendEmail(#[from] lettre::transport::smtp::Error),

    #[error("User id must be a valid uuid")]
    InvalidUserId,

    #[error("User is not authorized to perform this action")]
    UserNotAuthorized,

    #[error("Failed to generate stats for invite {0}")]
    InviteStatsAggregationError(sqlx::Error),

    #[error("Failed to generate stats for Workflow {0}")]
    WorkflowStatsAggregationError(sqlx::Error),

    #[error("WebSocket send error: {0}")]
    WebSocketSendError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("No workflow specified or default workflow found")]
    NoWorkflowFoundForInvite,

    #[error("No chat session was found for this bot on this conversation")]
    NoBotUserSession,

    #[error("No bot_id was found for this conversation")]
    NoConversationBotId,

    #[error("Background worker job failed: {0}")]
    BackgroundJobFailed(String),

    #[error("Failed to queue background worker job")]
    BackgroundJobFailedToQueue,

    #[error("Corrupted data: {0}")]
    CorruptedData(String),

    #[error("Download error: {0}")]
    DownloadError(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Tool config error: {0}")]
    ToolConfigError(String),
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct ComhairleErrorResponse {
    pub err: String,
}

/// Maps different error codes to a response with appropriate
/// status code
impl IntoResponse for ComhairleError {
    fn into_response(self) -> axum::response::Response {
        let status_code = match self {
            ComhairleError::DuplicateUsername(_)
            | ComhairleError::DuplicateEmail(_)
            | ComhairleError::DuplicateSlug(_)
            | ComhairleError::UserAlreadyParticipatingInWorkflow(_) => StatusCode::CONFLICT,
            ComhairleError::ResourceNotFound(_)
            | ComhairleError::NoUserFound
            | ComhairleError::NoUserFoundForEmail(_)
            | ComhairleError::NoUserFoundForId(_) => StatusCode::NOT_FOUND,
            ComhairleError::UserRequired
            | ComhairleError::WrongPassword
            | ComhairleError::RequiresAuthUser
            | ComhairleError::InviteDoesNotMatchUser
            | ComhairleError::NoLogedInUser => StatusCode::UNAUTHORIZED,
            ComhairleError::NoValidUpdates => StatusCode::UNPROCESSABLE_ENTITY,
            ComhairleError::UserNotAuthorized => StatusCode::FORBIDDEN,
            ComhairleError::EmailAlreadyVerified => StatusCode::CONFLICT,
            ComhairleError::PasswordConfirmationMismatch | ComhairleError::BadRequest(_) => {
                StatusCode::BAD_REQUEST
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status_code, Json(json!({"err":self.to_string()}))).into_response()
    }
}
