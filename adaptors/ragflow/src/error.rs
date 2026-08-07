use reqwest::StatusCode;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, RagflowError>;

#[derive(Error, Debug)]
pub enum RagflowError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Server returned non-200: {status} - {body}")]
    Api { status: StatusCode, body: String },

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Not found: {0}")]
    NotFound(String),
}

impl Into<StatusCode> for &RagflowError {
    fn into(self) -> StatusCode {
        match self {
            RagflowError::Http(err) => {
                if let Some(status) = err.status() {
                    status
                } else {
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            }
            RagflowError::Api { status, .. } => *status,
            RagflowError::Serde(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RagflowError::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }
}
