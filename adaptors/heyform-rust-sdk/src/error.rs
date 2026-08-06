use reqwest::StatusCode;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, HeyFormError>;

#[derive(Error, Debug)]
pub enum HeyFormError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON serialization/deserialization failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("URL parsing failed: {0}")]
    Url(#[from] url::ParseError),

    #[error("GraphQL error: {0}")]
    GraphQL(String),

    #[error("Authentication failed: {0}")]
    Authentication(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Not found: {0}")]
    NotFound(String),
}

impl Into<StatusCode> for &HeyFormError {
    fn into(self) -> StatusCode {
        match self {
            HeyFormError::Http(err) => {
                if let Some(status) = err.status() {
                    status
                } else {
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            },
            HeyFormError::Json(_)
            | HeyFormError::Url(_)
            | HeyFormError::GraphQL(_)
            | HeyFormError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            HeyFormError::Authentication(_) => StatusCode::UNAUTHORIZED,
            HeyFormError::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }
}

impl HeyFormError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            HeyFormError::Http(err) => {
                if let Some(status) = err.status() {
                    status
                } else {
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            },
            HeyFormError::Json(_)
            | HeyFormError::Url(_)
            | HeyFormError::GraphQL(_)
            | HeyFormError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            HeyFormError::Authentication(_) => StatusCode::UNAUTHORIZED,
            HeyFormError::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }
}
