use aide::OperationIo;
use thiserror::Error;

use crate::tools::polis::PolisError;

#[derive(Error, Debug, OperationIo)]
#[aide(output)]
pub enum WikiPollServiceError {
    #[error("Polis error: {0}")]
    PolisError(#[from] PolisError),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
}
