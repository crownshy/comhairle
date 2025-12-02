use reqwest::StatusCode;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, RagflowError>;

#[derive(Error, Debug)]
pub enum RagflowError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Server returned non-200: {status} - {body}")]
    Api { status: StatusCode, body: String },

    #[error("Not found: {0}")]
    NotFound(String),
}
