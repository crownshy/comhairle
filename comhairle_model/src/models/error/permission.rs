//! Authorization failures: role grants and resource-level access checks.

use thiserror::Error;

use super::{DataError, DomainError, ErrorKind};

#[derive(Error, Debug)]
pub enum PermissionError {
    #[error("User is not authorized to perform this action")]
    UserNotAuthorized,

    #[error("Only the owner of the conversation can perform this action")]
    UserIsNotConversationOwner,

    #[error("Role '{0}' is already granted on this resource")]
    RoleAlreadyGranted(String),

    #[error("Role '{0}' is not granted on this resource")]
    RoleNotFound(String),

    #[error("Cannot revoke the last system admin role")]
    CannotRevokeLastSuperAdmin,

    /// The participant has already finished and the conversation does not allow revisits
    /// afterwards. Distinct from `UserNotAuthorized` so the frontend can send
    /// them to the thank-you page rather than surfacing a generic permission error.
    #[error("Participant has already finished this conversation")]
    ParticipantSealed,

    /// A persistence failure hit while serving a permission request.
    #[error(transparent)]
    Data(#[from] DataError),
}

impl DomainError for PermissionError {
    fn kind(&self) -> ErrorKind {
        match self {
            PermissionError::Data(err) => err.kind(),
            PermissionError::UserNotAuthorized
            | PermissionError::UserIsNotConversationOwner
            | PermissionError::CannotRevokeLastSuperAdmin
            | PermissionError::ParticipantSealed => ErrorKind::Forbidden,
            PermissionError::RoleAlreadyGranted(_) => ErrorKind::Conflict,
            PermissionError::RoleNotFound(_) => ErrorKind::NotFound,
        }
    }
}
