use crate::{
    bulk_storage_service::error::BulkStorageError, tools::polis::PolisError,
    transcription_service::error::TranscriptionServiceError,
    translation_service::error::TranslationError, websockets::error::WebsocketError,
    wiki_poll_service::error::WikiPollServiceError, worker_service::error::WorkerServiceError,
};
use aide::OperationIo;
use axum::{
    Json,
    extract::{multipart::MultipartError, rejection::PathRejection},
    http::StatusCode,
    response::IntoResponse,
};
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

    #[error("Wiki poll service error: {0}")]
    WikiPollServiceError(#[from] WikiPollServiceError),

    #[error("Translation error: {0}")]
    TranslationError(#[from] TranslationError),

    #[error("Bulk storage error: {0}")]
    BulkStorageError(#[from] BulkStorageError),

    #[error("Transcription error: {0}")]
    TranscriptionError(#[from] TranscriptionServiceError),

    #[error("Worker error: {0}")]
    WorkerError(#[from] WorkerServiceError),

    #[error("Email builder error: {0}")]
    EmailBuilderError(#[from] lettre::error::Error),

    #[error("Email address error: {0}")]
    EmailAddressError(#[from] lettre::address::AddressError),

    #[error("Email content type error: {0}")]
    EmailContentTypeError(#[from] lettre::message::header::ContentTypeErr),

    #[error("No translation service configured")]
    NoTranslationServiceConfigured,

    #[error("No bot service configured")]
    NoBotServiceConfigured,

    #[error("No bulk storage service configured")]
    NoBulkStorageServiceConfigured,

    #[error("No video service configured")]
    NoVideoServiceConfigured,

    #[error("No transcription service configured")]
    NoTranscriptionServiceConfigured,

    #[error("No worker service configured")]
    NoWorkerServiceConfigured,

    #[error("No categorization service configured")]
    NoCategorizationServiceConfigured,

    #[error("HeyForm error: {0}")]
    HeyFormError(#[from] HeyFormError),

    #[error("Ragflow error: {0}")]
    RagflowError(#[from] RagflowError),

    #[error("Multipart form parse error: {0}")]
    MultipartParseForm(#[from] MultipartError),

    #[error("Path rejection: {0}")]
    PathRejection(#[from] PathRejection),

    #[error("Template error: {0}")]
    TemplateError(#[from] minijinja::Error),

    #[error("Serde json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("CSS inliner error: {0}")]
    CssInlinerError(#[from] css_inline::error::InlineError),

    #[error("Username {0} already taken")]
    DuplicateUsername(String),

    #[error("Email {0} already taken")]
    DuplicateEmail(String),

    #[error("Slug {0} already taken")]
    DuplicateSlug(String),

    #[error("A recording named {0} already exists for this event")]
    DuplicateRecordingName(String),

    #[error("Failed to hash password")]
    PasswordHash,

    #[error("The password and email don't match")]
    WrongPassword,

    #[error("The password and password confirmation don't match")]
    PasswordConfirmationMismatch,

    #[error("Password does not meet security requirements: {0}")]
    WeakPassword(String),

    #[error("User required for this route")]
    UserRequired,

    #[error("Auth Error {0}")]
    AuthJWTError(String),

    #[error("Auth Error {0}")]
    AuthWebhookSignatureError(String),

    #[error("Locale Error {0}")]
    LocaleError(String),

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

    #[error("An invite response has already been created for this invite by this user")]
    InviteResponseAlreadyCreated,

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

    #[error("Invalid api key")]
    InvalidApiKey,

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

    #[error("Failed to create invite response")]
    FailedToCreateInviteResponse(sqlx::Error),

    #[error("Invite does not match logged in user")]
    InviteDoesNotMatchUser,

    #[error("This invite has expired")]
    InviteExpired,

    #[error("This invite is has an invalid type")]
    InvalidInviteType,

    #[error("This invite has an invalid resource: {0}")]
    InvalidInviteResource(String),

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

    #[error("WebSocket handler error: {0}")]
    WebSocketHandlerError(Box<WebsocketError>),

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

    #[error("Preview tool and live tool config dont match type")]
    ToolConfigMismatch,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Tool config error: {0}")]
    ToolConfigError(String),

    #[error("Event at max capacity")]
    EventAtCapacity,

    #[error("Event has past")]
    EventHasPast,

    #[error("User is already registered for event: {0}")]
    UserAlreadyRegisteredForEvent(String),

    #[error("Conversation already live")]
    ConversationAlreadyLive,

    #[error("Event missing video_meeting_id")]
    NoVideoMeetingId,

    #[error("Missing email template schema")]
    MissingEmailTemplateSchema(String),

    #[error("Missing email template")]
    MissingEmailTemplate(String),

    #[error("CSV error: {0}")]
    CsvError(#[from] csv::Error),

    #[error("UTF-8 conversion error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error("Unsupported Content-Type: {0}")]
    UnsupportedContentType(String),

    #[error("Redis error: {0}")]
    RedisError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),
    #[error("Role '{0}' is already granted on this resource")]
    RoleAlreadyGranted(String),

    #[error("Role '{0}' is not granted on this resource")]
    RoleNotFound(String),

    #[error("Cannot revoke the last system admin role")]
    CannotRevokeLastAdmin,
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
            | ComhairleError::ConversationAlreadyLive
            | ComhairleError::EmailAlreadyVerified
            | ComhairleError::EventAtCapacity
            | ComhairleError::InviteResponseAlreadyCreated
            | ComhairleError::DuplicateSlug(_)
            | ComhairleError::DuplicateRecordingName(_)
            | ComhairleError::UserAlreadyRegisteredForEvent(_)
            | ComhairleError::UserAlreadyParticipatingInWorkflow(_)
            | ComhairleError::Conflict(_)
            | ComhairleError::RoleAlreadyGranted(_) => StatusCode::CONFLICT,
            ComhairleError::RoleNotFound(_) => StatusCode::NOT_FOUND,
            ComhairleError::CannotRevokeLastAdmin => StatusCode::FORBIDDEN,
            ComhairleError::ResourceNotFound(_)
            | ComhairleError::NoUserFound
            | ComhairleError::NoUserFoundForEmail(_)
            | ComhairleError::NoUserFoundForId(_) => StatusCode::NOT_FOUND,
            ComhairleError::UserRequired
            | ComhairleError::WrongPassword
            | ComhairleError::InvalidApiKey
            | ComhairleError::RequiresAuthUser
            | ComhairleError::InviteDoesNotMatchUser
            | ComhairleError::UserIsNotConversationOwner
            | ComhairleError::NoLogedInUser => StatusCode::UNAUTHORIZED,
            ComhairleError::NoValidUpdates | ComhairleError::EventHasPast => {
                StatusCode::UNPROCESSABLE_ENTITY
            }
            ComhairleError::UserNotAuthorized | ComhairleError::AuthWebhookSignatureError(_) => {
                StatusCode::FORBIDDEN
            }
            ComhairleError::PasswordConfirmationMismatch
            | ComhairleError::WeakPassword(_)
            | ComhairleError::UnsupportedContentType(_)
            | ComhairleError::BadRequest(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status_code, Json(json!({"err":self.to_string()}))).into_response()
    }
}
