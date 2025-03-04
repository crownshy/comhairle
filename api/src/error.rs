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

    #[error("Failed to hash password")]
    PasswordHash,

    #[error("User Required for this route")]
    UserRequired,

    #[error("Auth Error {0}")]
    AuthJWTError(String),
}

impl IntoResponse for ComhairleError {
    fn into_response(self) -> axum::response::Response {
        let status_code = match self {
            ComhairleError::DuplicateUsername(_) | ComhairleError::DuplicateEmail(_) => {
                StatusCode::CONFLICT
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status_code, Json(json!({"err":self.to_string()}))).into_response()
    }
}
