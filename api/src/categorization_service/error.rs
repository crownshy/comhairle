use thiserror::Error;

#[derive(Error, Debug)]
pub enum CategorizationServiceError {
    #[error("External service failure: {0}")]
    ExternalServiceFailure(String),

    #[error("Http error: {0}")]
    HttpError(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, CategorizationServiceError>;
