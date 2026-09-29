//! Failures about reports, feedback, and report impacts.

use thiserror::Error;

use super::{DataError, DomainError, ErrorKind};

#[derive(Error, Debug)]
pub enum ReportError {
    #[error("Failed to create report")]
    FailedToCreateReport(sqlx::Error),

    #[error("Failed to update report")]
    FailedToUpdateReport,

    #[error("Failed to create feedback")]
    FailedToCreateFeedback,

    #[error("Failed to update feedback")]
    FailedToUpdateFeedback,

    #[error("Failed to create impact")]
    FailedToCreateImpact,

    #[error("Failed to update impact")]
    FailedToUpdateImpact(sqlx::Error),

    /// A persistence failure hit while serving a report request.
    #[error(transparent)]
    Data(#[from] DataError),
}

impl DomainError for ReportError {
    fn kind(&self) -> ErrorKind {
        match self {
            ReportError::Data(err) => err.kind(),
            // The rest are writes that failed for a reason the caller cannot act
            // on, so they all surface as 500s, matching current behaviour.
            _ => ErrorKind::Internal,
        }
    }
}
