use thiserror::Error;
#[derive(Error, Debug)]
pub enum TranslationError {
    #[error("Translation Failed")]
    TranslationFailed(String),
}
pub type Result<T> = std::result::Result<T, TranslationError>;
