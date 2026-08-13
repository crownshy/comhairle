use hyper::StatusCode;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum TranslationError {
    #[error("Translation Failed")]
    TranslationFailed(String),
}
pub type Result<T> = std::result::Result<T, TranslationError>;

impl Into<StatusCode> for &TranslationError {
    fn into(self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
