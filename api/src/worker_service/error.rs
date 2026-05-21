use chrono::Utc;
use sqlx::PgPool;
use thiserror::Error;
use uuid::Uuid;

use crate::models::job::{self, UpdateJob};

#[derive(Error, Debug)]
pub enum WorkerServiceError {
    #[error("No worker service configured")]
    NoWorkerServiceConfigured,

    #[error("No transcription service configured")]
    NoTranscriptionServiceConfigured,

    #[error("No bulk storage service configured")]
    NoBulkStorageServiceConfigured,

    #[error("No bot service configured")]
    NoBotServiceConfigured,

    #[error("No categorization service configured")]
    NoCategorizationServiceError,

    #[error("Serde json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("DbError: {0}")]
    DbError(String),

    #[error("Mailer error: {0}")]
    MailerError(String),

    #[error("Wrong user type")]
    WrongUserType,

    #[error("Background job failed to queue")]
    BackgroundJobFailedToQueue,

    #[error("Transcription service error: {0}")]
    TranscriptionServiceError(String),

    #[error("Bulk storage service error: {0}")]
    BulkStorageServiceError(String),

    #[error("Categorization service error: {0}")]
    CategorizationServiceError(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("External service failure: {0}")]
    ExternalServiceFailure(String),

    #[error("Job failure: {0}")]
    JobFailure(String),
}

pub type Result<T> = std::result::Result<T, WorkerServiceError>;

/// Extension trait for [`worker_service::error::Result`] that records job failures
/// to the database _before_ propagating the error.
///
/// # Example
///
/// ```rust,ignore
/// let response = failable_operation()
///     .await
///     .ok_or_record_failure(&req.job_id, &state.db)
///     .await?;
/// ```
///
#[allow(async_fn_in_trait)]
pub trait RecordWorkerError<T> {
    /// Records a job failure in the database and propagates the error.
    ///
    /// If `self` is [`Ok`], the value is returned unchanged and no database
    /// update is performed. If `self` [`Err`], the job record identified by
    /// `job_id` is updated with:
    ///
    /// - `status` set to `"failed"`
    /// - `error` set to the error message
    /// - `finished_at` set to the current time
    ///
    /// # Arguments
    ///
    /// - `job_id` - the ID of the job record to update
    /// - `db` - a reference to the database connection pool
    ///
    /// # Errors
    ///
    /// Returns the original error converted into [`WorkerServiceError`] if
    /// `self` is [`Err`] or [`WorkerService::DbError`] if updating the job
    /// record itself fails.
    async fn ok_or_record_failure(self, job_id: &Uuid, db: &PgPool) -> Result<T>;
}

impl<T> RecordWorkerError<T> for Result<T> {
    async fn ok_or_record_failure(self, job_id: &Uuid, db: &PgPool) -> Result<T> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => {
                let update_job = UpdateJob {
                    status: Some("failed".to_string()),
                    error: Some(e.to_string()),
                    finished_at: Some(Utc::now()),
                    ..Default::default()
                };

                let _ = job::update(db, job_id, update_job)
                    .await
                    .map_err(|e| WorkerServiceError::DbError(e.to_string()))?;

                Err(e)
            }
        }
    }
}
