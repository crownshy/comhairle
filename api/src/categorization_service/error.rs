use thiserror::Error;

#[derive(Error, Debug)]
pub enum CategorizationServiceError {}

pub type Result<T> = std::result::Result<T, CategorizationServiceError>;
