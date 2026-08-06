use aide::OperationIo;
use hyper::StatusCode;
use thiserror::Error;

use crate::tools::polis::PolisError;

#[derive(Error, Debug, OperationIo)]
#[aide(output)]
pub enum WikiPollServiceError {
    #[error("Polis error: {0}")]
    PolisError(#[from] PolisError),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Moderation status integer not recognized")]
    UnknownModerationStatus,
}

impl Into<StatusCode> for &WikiPollServiceError {
    fn into(self) -> StatusCode {
        match self {
            WikiPollServiceError::PolisError(polis_error) => Into::<StatusCode>::into(polis_error),
            WikiPollServiceError::Http(err) => {
                if let Some(status) = err.status() {
                    status
                } else {
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            }
            WikiPollServiceError::UnknownModerationStatus => StatusCode::BAD_REQUEST,
        }
    }
}
