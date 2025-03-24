use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
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

    #[error("User Required for this route")]
    UserRequired,

    #[error("Auth Error {0}")]
    AuthJWTError(String),

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
            ComhairleError::ResourceNotFound(_) => StatusCode::NOT_FOUND,
            ComhairleError::UserRequired => StatusCode::UNAUTHORIZED,
            ComhairleError::NoValidUpdates => StatusCode::UNPROCESSABLE_ENTITY,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status_code, Json(json!({"err":self.to_string()}))).into_response()
    }
}
