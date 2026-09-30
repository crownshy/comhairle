//! Failures from composing and sending email.

use thiserror::Error;

use crate::models::error::{DomainError, ErrorKind};

#[derive(Error, Debug)]
pub enum EmailError {
    #[error("Email builder error: {0}")]
    EmailBuilderError(#[from] lettre::error::Error),

    #[error("Email address error: {0}")]
    EmailAddressError(#[from] lettre::address::AddressError),

    #[error("Email content type error: {0}")]
    EmailContentTypeError(#[from] lettre::message::header::ContentTypeErr),

    #[error("Failed to send email")]
    FailedToSendEmail(#[from] lettre::transport::smtp::Error),

    #[error("Template error: {0}")]
    TemplateError(#[from] minijinja::Error),

    #[error("Missing email template schema")]
    MissingEmailTemplateSchema(String),

    #[error("Missing email template")]
    MissingEmailTemplate(String),

    #[error("CSS inliner error: {0}")]
    CssInlinerError(#[from] css_inline::error::InlineError),
}

impl DomainError for EmailError {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Internal
    }
}
