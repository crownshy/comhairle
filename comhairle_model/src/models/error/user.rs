//! Failures about user records: lookup, uniqueness, and account kind.

use thiserror::Error;
use uuid::Uuid;

use super::{DataError, DomainError, ErrorKind};

#[derive(Error, Debug)]
pub enum UserError {
    #[error("No user with email {0}")]
    NoUserFoundForEmail(String),

    #[error("No user with id {0}")]
    NoUserFoundForId(Uuid),

    #[error("No user found")]
    NoUserFound,

    #[error("Email {0} already taken")]
    DuplicateEmail(String),

    #[error("Guest code {0} already taken")]
    DuplicateGuestCode(String),

    #[error("User's email address is already verified")]
    EmailAlreadyVerified,

    #[error("Cant log this type of user in with this flow")]
    WrongUserType,

    #[error("User id must be a valid uuid")]
    InvalidUserId,

    #[error("Failed to create guest user")]
    FailedToCreateGuestUser,

    /// A persistence failure hit while serving a user request.
    #[error(transparent)]
    Data(#[from] DataError),
}

impl DomainError for UserError {
    fn kind(&self) -> ErrorKind {
        match self {
            UserError::Data(err) => err.kind(),
            UserError::NoUserFoundForEmail(_)
            | UserError::NoUserFoundForId(_)
            | UserError::NoUserFound => ErrorKind::NotFound,
            UserError::DuplicateEmail(_)
            | UserError::DuplicateGuestCode(_)
            | UserError::EmailAlreadyVerified => ErrorKind::Conflict,
            // `WrongUserType`, `InvalidUserId` and `FailedToCreateGuestUser` have
            // always fallen through to a 500; preserved rather than corrected here.
            UserError::WrongUserType
            | UserError::InvalidUserId
            | UserError::FailedToCreateGuestUser => ErrorKind::Internal,
        }
    }
}
