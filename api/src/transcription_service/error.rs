use hyper::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TranscriptionServiceError {
    #[error("Current transciber does not support batch operations")]
    BatchProcessingUnsupported,

    #[error("Current transciber does not support batch operations")]
    StreamingProcessingUnsupported,

    #[error("Transcription failed: {0}")]
    TranscriptionFailure(String),

    #[error("Streaming transcription failed: {0}")]
    StreamingTranscriptionFailure(String),

    #[error("Batch transcription failed: {0}")]
    BatchTranscriptionFailure(String),
}
pub type Result<T> = std::result::Result<T, TranscriptionServiceError>;

impl Into<StatusCode> for &TranscriptionServiceError {
    fn into(self) -> StatusCode {
        match self {
            TranscriptionServiceError::BatchProcessingUnsupported
            | TranscriptionServiceError::StreamingProcessingUnsupported => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
