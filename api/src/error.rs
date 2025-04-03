use aide::OperationIo;
use axum::{http::StatusCode, response::IntoResponse, Json};
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

    #[error("Failed to load config: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

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

    #[error("Failed to create {0}")]
    FailedToCreateResource(String),

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

    #[error("No user logged in")]
    NoLogedInUser,
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
            | ComhairleError::NoLogedInUser => StatusCode::UNAUTHORIZED,
            ComhairleError::NoValidUpdates => StatusCode::UNPROCESSABLE_ENTITY,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status_code, Json(json!({"err":self.to_string()}))).into_response()
    }
}
