//! Failures about invites and invite responses.

use thiserror::Error;

use super::{DataError, DomainError, ErrorKind};

#[derive(Error, Debug)]
pub enum InviteError {
    #[error("Invite does not match logged in user")]
    InviteDoesNotMatchUser,

    #[error("This invite has expired")]
    InviteExpired,

    #[error("This invite is has an invalid type")]
    InvalidInviteType,

    #[error("This invite has an invalid resource: {0}")]
    InvalidInviteResource(String),

    #[error("An invite response has already been created for this invite by this user")]
    InviteResponseAlreadyCreated,

    #[error("No workflow specified or default workflow found")]
    NoWorkflowFoundForInvite,

    #[error("Failed to create invite")]
    FailedToCreateInvite(sqlx::Error),

    #[error("Failed to create invite response")]
    FailedToCreateInviteResponse(sqlx::Error),

    #[error("Failed to generate stats for invite {0}")]
    InviteStatsAggregationError(sqlx::Error),

    /// A persistence failure hit while serving an invite request.
    #[error(transparent)]
    Data(#[from] DataError),
}

impl DomainError for InviteError {
    fn kind(&self) -> ErrorKind {
        match self {
            InviteError::Data(err) => err.kind(),
            // A mismatched invite is treated as an auth failure, not a 403: the
            // caller may be able to fix it by signing in as the invited user.
            InviteError::InviteDoesNotMatchUser => ErrorKind::Unauthorized,
            InviteError::InviteResponseAlreadyCreated => ErrorKind::Conflict,
            InviteError::InviteExpired
            | InviteError::InvalidInviteType
            | InviteError::InvalidInviteResource(_)
            | InviteError::NoWorkflowFoundForInvite
            | InviteError::FailedToCreateInvite(_)
            | InviteError::FailedToCreateInviteResponse(_)
            | InviteError::InviteStatsAggregationError(_) => ErrorKind::Internal,
        }
    }
}
