//! Authentication failures: credentials, tokens, sessions, and API keys.

use thiserror::Error;

use super::{DataError, DomainError, ErrorKind};
use crate::models::refresh_token::RefreshFailure;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Failed to hash password")]
    PasswordHash,

    #[error("The password and email don't match")]
    WrongPassword,

    #[error("The password and password confirmation don't match")]
    PasswordConfirmationMismatch,

    #[error("Password does not meet security requirements: {0}")]
    WeakPassword(String),

    #[error("User required for this route")]
    UserRequired,

    #[error("Auth Error {0}")]
    AuthJWTError(String),

    #[error("Auth Error {0}")]
    AuthWebhookSignatureError(String),

    #[error("No user logged in")]
    NoLoggedInUser,

    #[error("Requires Auth User")]
    RequiresAuthUser,

    #[error("Invalid api key")]
    InvalidApiKey,

    #[error("Session refresh failure: {0}")]
    SessionRefreshFailure(RefreshFailure),

    /// A persistence failure hit while serving an auth request.
    #[error(transparent)]
    Data(#[from] DataError),
}

impl DomainError for AuthError {
    fn kind(&self) -> ErrorKind {
        match self {
            AuthError::Data(err) => err.kind(),
            AuthError::WrongPassword
            | AuthError::UserRequired
            | AuthError::NoLoggedInUser
            | AuthError::RequiresAuthUser
            | AuthError::InvalidApiKey
            | AuthError::SessionRefreshFailure(_) => ErrorKind::Unauthorized,
            // Signature mismatch on an inbound webhook is a 403, not a 401: there is
            // no credential for the caller to correct.
            AuthError::AuthWebhookSignatureError(_) => ErrorKind::Forbidden,
            AuthError::PasswordConfirmationMismatch | AuthError::WeakPassword(_) => {
                ErrorKind::Invalid
            }
            AuthError::PasswordHash | AuthError::AuthJWTError(_) => ErrorKind::Internal,
        }
    }
}
