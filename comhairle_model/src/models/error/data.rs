//! Persistence-level failures shared by every model module.

use thiserror::Error;
use uuid::Uuid;

use super::{DomainError, ErrorKind};

/// Failures that come from talking to the database, independent of which domain
/// was being read or written.
#[derive(Error, Debug)]
pub enum DataError {
    #[error("Database Failed to connect: {0}")]
    DbError(String),

    #[error("Database query error: {0}")]
    DbQueryError(#[from] sea_query::error::Error),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("{0} not found")]
    ResourceNotFound(String),

    #[error("Failed to get resource {0}")]
    NoResourceFoundForId(Uuid),

    #[error("Failed to create {resource_type}")]
    FailedToCreateResource {
        resource_type: String,
        error: sqlx::Error,
    },

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Corrupted data: {0}")]
    CorruptedData(String),

    /// A JSONB column failed to round-trip. Stored as a `Data` failure rather than
    /// a transport one: the bad value came out of (or is going into) the database,
    /// not off the wire.
    #[error("Serde json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
}

impl DomainError for DataError {
    fn kind(&self) -> ErrorKind {
        match self {
            DataError::ResourceNotFound(_) => ErrorKind::NotFound,
            DataError::Conflict(_) => ErrorKind::Conflict,
            // `NoResourceFoundForId` reads like a 404 but has always fallen through
            // to a 500. Kept as-is so this refactor does not change any response
            // codes; see the note in `api/src/error.rs` about revisiting it.
            DataError::SerdeJsonError(_)
            | DataError::NoResourceFoundForId(_)
            | DataError::DbError(_)
            | DataError::DbQueryError(_)
            | DataError::DatabaseError(_)
            | DataError::FailedToCreateResource { .. }
            | DataError::CorruptedData(_) => ErrorKind::Internal,
        }
    }
}
