use thiserror::Error;

pub type Result<T> = std::result::Result<T, HeyFormError>;

#[derive(Error, Debug)]
pub enum HeyFormError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("JSON serialization/deserialization failed: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("URL parsing failed: {0}")]
    Url(#[from] url::ParseError),
    
    #[error("GraphQL error: {0}")]
    GraphQL(String),
    
    #[error("Authentication failed: {0}")]
    Authentication(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
}