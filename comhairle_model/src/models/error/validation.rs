//! Failures caused by a malformed or unactionable request payload.

use thiserror::Error;

use super::{DomainError, ErrorKind};

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unprocessable: {0}")]
    Unprocessable(String),

    #[error("Update request contained no valid parameters")]
    NoValidUpdates,

    #[error("Unsupported Content-Type: {0}")]
    UnsupportedContentType(String),
}

impl DomainError for ValidationError {
    fn kind(&self) -> ErrorKind {
        match self {
            ValidationError::BadRequest(_) | ValidationError::UnsupportedContentType(_) => {
                ErrorKind::Invalid
            }
            ValidationError::Unprocessable(_) | ValidationError::NoValidUpdates => {
                ErrorKind::Unprocessable
            }
        }
    }
}
