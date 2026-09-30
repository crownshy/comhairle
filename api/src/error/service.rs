//! Failures from the service layer: optional services that were not configured,
//! and errors bubbling up from the third-party tools we integrate.
//!
//! This lives in `api` rather than the model layer because every variant names a
//! service that `ComhairleState` owns.

use hyper::StatusCode;
use thiserror::Error;

use crate::bulk_storage_service::error::BulkStorageError;
use crate::models::error::{DomainError, ErrorKind};
use crate::tools::polis::PolisError;
use crate::transcription_service::error::TranscriptionServiceError;
use crate::translation_service::error::TranslationError;
use crate::websockets::error::WebsocketError;
use crate::wiki_poll_service::error::WikiPollServiceError;
use crate::worker_service::error::WorkerServiceError;
use heyform_sdk::HeyFormError;
use ragflow::RagflowError;

#[derive(Error, Debug)]
pub enum ServiceError {
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

    #[error("HeyForm error: {0}")]
    HeyFormError(#[from] HeyFormError),

    #[error("Ragflow error: {0}")]
    RagflowError(#[from] RagflowError),

    #[error("WebSocket send error: {0}")]
    WebSocketSendError(String),

    #[error("WebSocket handler error: {0}")]
    WebSocketHandlerError(Box<WebsocketError>),

    #[error("Background worker job failed: {0}")]
    BackgroundJobFailed(String),

    #[error("Failed to queue background worker job")]
    BackgroundJobFailedToQueue,

    #[error("Redis error: {0}")]
    RedisError(String),

    #[error("Stream chunk error: {0}")]
    StreamChunkError(String),
}

/// Forwards an upstream status through [`ErrorKind::Status`] so the wrapped
/// service keeps control of the response code, exactly as the previous
/// `Into::<StatusCode>::into(err)` arms did.
fn passthrough(status: StatusCode) -> ErrorKind {
    ErrorKind::Status(status.as_u16())
}

impl DomainError for ServiceError {
    fn kind(&self) -> ErrorKind {
        match self {
            ServiceError::PolisError(e) => passthrough(e.into()),
            ServiceError::WikiPollServiceError(e) => passthrough(e.into()),
            ServiceError::TranslationError(e) => passthrough(e.into()),
            ServiceError::BulkStorageError(e) => passthrough(e.into()),
            ServiceError::TranscriptionError(e) => passthrough(e.into()),
            ServiceError::WorkerError(e) => passthrough(e.into()),
            ServiceError::HeyFormError(e) => passthrough(e.into()),
            ServiceError::RagflowError(e) => passthrough(e.into()),
            _ => ErrorKind::Internal,
        }
    }
}
