use crate::bulk_storage_service::error::BulkStorageError;
use crate::tools::polis::PolisError;
use crate::transcription_service::error::TranscriptionServiceError;
use crate::translation_service::error::TranslationError;
use crate::wiki_poll_service::error::WikiPollServiceError;
use crate::worker_service::error::WorkerServiceError;

pub mod email;
pub mod service;
pub mod transport;

pub use email::EmailError;
pub use service::ServiceError;
pub use transport::TransportError;

use crate::models::error::{DomainError, ErrorKind, ModelError};

use aide::OperationIo;
use axum::{
    Json,
    extract::{multipart::MultipartError, rejection::PathRejection},
    http::StatusCode,
    response::IntoResponse,
};
use heyform_sdk::HeyFormError;
use ragflow::RagflowError;
use schemars::JsonSchema;
use serde::Serialize;
use serde_json::json;
use thiserror::Error;

/// Translates a domain error's transport-agnostic [`ErrorKind`] into the HTTP
/// status code we answer with. This is the only place in the codebase that makes
/// that decision, which is what keeps the model layer free of `axum`.
fn status_for(kind: ErrorKind) -> StatusCode {
    match kind {
        ErrorKind::NotFound => StatusCode::NOT_FOUND,
        ErrorKind::Conflict => StatusCode::CONFLICT,
        ErrorKind::Unauthorized => StatusCode::UNAUTHORIZED,
        ErrorKind::Forbidden => StatusCode::FORBIDDEN,
        ErrorKind::Invalid => StatusCode::BAD_REQUEST,
        ErrorKind::Unprocessable => StatusCode::UNPROCESSABLE_ENTITY,
        ErrorKind::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        ErrorKind::Status(raw) => {
            StatusCode::from_u16(raw).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// The error type crossing the HTTP boundary.
///
/// This is a pure wrapper: one variant per bounded context, no failure modes of
/// its own. `Data` through `Report` wrap the model layer's own error types from
/// [`crate::models::error`]; `Service`, `Email` and `Transport` wrap the api-side
/// ones defined alongside this module.
///
/// Construct the domain error, not this. `?` promotes it, and every domain reports
/// its own [`ErrorKind`], so the status mapping in `status_for` stays the single
/// place a failure turns into an HTTP code.
#[derive(Error, Debug, OperationIo)]
#[aide(output)]
pub enum ComhairleError {
    #[error(transparent)]
    Data(crate::models::error::DataError),

    #[error(transparent)]
    Validation(crate::models::error::ValidationError),

    #[error(transparent)]
    User(crate::models::error::UserError),

    #[error(transparent)]
    Auth(crate::models::error::AuthError),

    #[error(transparent)]
    Permission(crate::models::error::PermissionError),

    #[error(transparent)]
    Workflow(crate::models::error::WorkflowError),

    #[error(transparent)]
    Invite(crate::models::error::InviteError),

    #[error(transparent)]
    Event(crate::models::error::EventError),

    #[error(transparent)]
    Conversation(crate::models::error::ConversationError),

    #[error(transparent)]
    Report(crate::models::error::ReportError),

    #[error(transparent)]
    Service(#[from] ServiceError),

    #[error(transparent)]
    Email(#[from] EmailError),

    #[error(transparent)]
    Transport(#[from] TransportError),
}

/// `?` only applies a single `From`, so a domain error reached via [`ModelError`]
/// would otherwise need two hops. These impls collapse that, and `From<ModelError>`
/// below fans the union back out to the same variants.
macro_rules! promote_model_domain {
    ($($variant:ident => $domain:ty),* $(,)?) => {
        $(
            impl From<$domain> for ComhairleError {
                fn from(err: $domain) -> Self {
                    ComhairleError::$variant(err)
                }
            }
        )*
    };
}

promote_model_domain!(
    Data => crate::models::error::DataError,
    Validation => crate::models::error::ValidationError,
    User => crate::models::error::UserError,
    Auth => crate::models::error::AuthError,
    Permission => crate::models::error::PermissionError,
    Workflow => crate::models::error::WorkflowError,
    Invite => crate::models::error::InviteError,
    Event => crate::models::error::EventError,
    Conversation => crate::models::error::ConversationError,
    Report => crate::models::error::ReportError,
);

/// Third-party error types reach the boundary through whichever domain owns them,
/// so a bare `?` still works on a `reqwest`, `lettre`, `serde_json` (etc.) result
/// inside a handler returning [`ComhairleError`].
///
/// Each domain enum already declares `#[from]` for these, but `?` only applies one
/// `From`, so the two-hop path needs collapsing here.
macro_rules! promote_source {
    ($($variant:ident($domain:ident) => $source:ty),* $(,)?) => {
        $(
            impl From<$source> for ComhairleError {
                fn from(err: $source) -> Self {
                    ComhairleError::$variant($domain::from(err))
                }
            }
        )*
    };
}

promote_source!(
    // Services and the tools we integrate.
    Service(ServiceError) => PolisError,
    Service(ServiceError) => WikiPollServiceError,
    Service(ServiceError) => TranslationError,
    Service(ServiceError) => BulkStorageError,
    Service(ServiceError) => TranscriptionServiceError,
    Service(ServiceError) => WorkerServiceError,
    Service(ServiceError) => HeyFormError,
    Service(ServiceError) => RagflowError,
    // Composing and sending mail.
    Email(EmailError) => lettre::error::Error,
    Email(EmailError) => lettre::address::AddressError,
    Email(EmailError) => lettre::message::header::ContentTypeErr,
    Email(EmailError) => lettre::transport::smtp::Error,
    Email(EmailError) => minijinja::Error,
    Email(EmailError) => css_inline::error::InlineError,
    // Request extraction, encoding, process config.
    Transport(TransportError) => MultipartError,
    Transport(TransportError) => PathRejection,
    Transport(TransportError) => serde_json::Error,
    Transport(TransportError) => std::io::Error,
    Transport(TransportError) => csv::Error,
    Transport(TransportError) => std::string::FromUtf8Error,
    Transport(TransportError) => config::ConfigError,
);

/// Raw database errors reach the HTTP boundary as `Data`, so `?` still works on a
/// bare `sqlx` or `sea-query` result inside a handler returning [`ComhairleError`].
impl From<sqlx::Error> for ComhairleError {
    fn from(err: sqlx::Error) -> Self {
        ComhairleError::Data(crate::models::error::DataError::DatabaseError(err))
    }
}

impl From<sea_query::error::Error> for ComhairleError {
    fn from(err: sea_query::error::Error) -> Self {
        ComhairleError::Data(crate::models::error::DataError::DbQueryError(err))
    }
}

impl From<ModelError> for ComhairleError {
    fn from(err: ModelError) -> Self {
        match err {
            ModelError::Data(e) => e.into(),
            ModelError::Validation(e) => e.into(),
            ModelError::User(e) => e.into(),
            ModelError::Auth(e) => e.into(),
            ModelError::Permission(e) => e.into(),
            ModelError::Workflow(e) => e.into(),
            ModelError::Invite(e) => e.into(),
            ModelError::Event(e) => e.into(),
            ModelError::Conversation(e) => e.into(),
            ModelError::Report(e) => e.into(),
        }
    }
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct ComhairleErrorResponse {
    pub err: String,
}

/// Maps different error codes to a response with appropriate
/// status code
impl IntoResponse for ComhairleError {
    fn into_response(self) -> axum::response::Response {
        // Exhaustive by design: every variant is a domain error that classifies
        // itself via `kind()`. Adding a domain without a status mapping is a
        // compile error rather than a silent 500.
        let status_code = match self {
            ComhairleError::Data(ref err) => status_for(err.kind()),
            ComhairleError::Validation(ref err) => status_for(err.kind()),
            ComhairleError::User(ref err) => status_for(err.kind()),
            ComhairleError::Auth(ref err) => status_for(err.kind()),
            ComhairleError::Permission(ref err) => status_for(err.kind()),
            ComhairleError::Workflow(ref err) => status_for(err.kind()),
            ComhairleError::Invite(ref err) => status_for(err.kind()),
            ComhairleError::Event(ref err) => status_for(err.kind()),
            ComhairleError::Conversation(ref err) => status_for(err.kind()),
            ComhairleError::Report(ref err) => status_for(err.kind()),
            ComhairleError::Service(ref err) => status_for(err.kind()),
            ComhairleError::Email(ref err) => status_for(err.kind()),
            ComhairleError::Transport(ref err) => status_for(err.kind()),
        };

        (status_code, Json(json!({"err":self.to_string()}))).into_response()
    }
}
