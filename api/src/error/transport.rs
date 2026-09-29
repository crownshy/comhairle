//! Failures at the HTTP and serialization boundary: request extraction,
//! encoding, process configuration, and object-store presigning.

use axum::extract::{multipart::MultipartError, rejection::PathRejection};
use thiserror::Error;

use crate::models::error::{DomainError, ErrorKind};

#[derive(Error, Debug)]
pub enum TransportError {
    #[error("Multipart form parse error: {0}")]
    MultipartParseForm(#[from] MultipartError),

    #[error("Path rejection: {0}")]
    PathRejection(#[from] PathRejection),

    #[error("Fai;ed to parse order params: {0}")]
    FailedToParseOrderParams(String),

    #[error("Serde json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    #[error("CSV error: {0}")]
    CsvError(#[from] csv::Error),

    #[error("UTF-8 conversion error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to load config: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("Locale Error {0}")]
    LocaleError(String),

    #[error("Failed to get a presigned upload url {0}")]
    FailedToGetUploadPresign(String),

    #[error("Failed to get a presigned download url {0}")]
    FailedToGetDownloadPresign(String),

    #[error("Download error: {0}")]
    DownloadError(String),
}

impl DomainError for TransportError {
    fn kind(&self) -> ErrorKind {
        // None of these were classified in the previous `IntoResponse` match, so
        // they all fell through to a 500. Preserved deliberately.
        ErrorKind::Internal
    }
}
