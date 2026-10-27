use hyper::StatusCode;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum TranslationError {
    #[error("Translation Failed")]
    TranslationFailed(String),
}
pub type Result<T> = std::result::Result<T, TranslationError>;

impl From<&TranslationError> for StatusCode {
    fn from(_val: &TranslationError) -> Self {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
