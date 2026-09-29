//! Domain errors for the model layer.
//!
//! Rather than one flat error enum spanning the whole application, each bounded
//! context owns a narrow error type describing only the ways *that* context can
//! fail. A model function returns the narrowest type it can; functions that span
//! contexts widen to [`ModelError`], and only the HTTP boundary widens further to
//! `ComhairleError`.
//!
//! None of these types know about HTTP. They report an abstract [`ErrorKind`] and
//! `api`'s `IntoResponse` impl is the single place that turns a kind into a status
//! code. That is what lets this module move into the model crate without dragging
//! axum along with it.

pub mod auth;
pub mod conversation;
pub mod data;
pub mod event;
pub mod invite;
pub mod permission;
pub mod report;
pub mod user;
pub mod validation;
pub mod workflow;

pub use auth::AuthError;
pub use conversation::ConversationError;
pub use data::DataError;
pub use event::EventError;
pub use invite::InviteError;
pub use permission::PermissionError;
pub use report::ReportError;
pub use user::UserError;
pub use validation::ValidationError;
pub use workflow::WorkflowError;

use thiserror::Error;

/// Transport-agnostic classification of a failure.
///
/// Domain errors report one of these; mapping a kind onto an HTTP status code is
/// the web layer's job, so the model layer never depends on `axum` or `http`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// The requested entity does not exist.
    NotFound,
    /// The request is well formed but conflicts with current state.
    Conflict,
    /// The caller is not authenticated, or their credentials are no longer valid.
    Unauthorized,
    /// The caller is authenticated but not permitted to do this.
    Forbidden,
    /// The request is malformed.
    Invalid,
    /// The request is well formed but cannot be acted on in the current state.
    Unprocessable,
    /// Something went wrong that is not the caller's fault.
    Internal,
    /// Pass an upstream HTTP status through unchanged.
    ///
    /// Only used by errors wrapping a third-party service that already answered
    /// with a meaningful status (see `PolisError`, which forwards whatever Polis
    /// returned). Carried as a `u16` so this enum stays free of `http` types.
    Status(u16),
}

/// Implemented by every domain error so the web layer can classify it without
/// matching on variants it should not know about.
pub trait DomainError: std::error::Error {
    fn kind(&self) -> ErrorKind;
}

/// Union of every model-layer domain error.
///
/// Model functions that span more than one bounded context return this; ones that
/// stay inside a single context should return that context's error directly so the
/// caller can see what can actually go wrong.
#[derive(Error, Debug)]
pub enum ModelError {
    #[error(transparent)]
    Data(#[from] DataError),

    #[error(transparent)]
    Validation(#[from] ValidationError),

    #[error(transparent)]
    User(#[from] UserError),

    #[error(transparent)]
    Auth(#[from] AuthError),

    #[error(transparent)]
    Permission(#[from] PermissionError),

    #[error(transparent)]
    Workflow(#[from] WorkflowError),

    #[error(transparent)]
    Invite(#[from] InviteError),

    #[error(transparent)]
    Event(#[from] EventError),

    #[error(transparent)]
    Conversation(#[from] ConversationError),

    #[error(transparent)]
    Report(#[from] ReportError),
}

/// Every business domain wraps [`DataError`] transparently, because nearly every
/// model function touches the database. `?` only applies one `From`, so without
/// these a `sqlx::Error` could not reach a domain error directly and every query
/// would need an explicit `.map_err`.
macro_rules! forward_db_errors {
    ($($domain:ty),* $(,)?) => {
        $(
            impl From<sqlx::Error> for $domain {
                fn from(err: sqlx::Error) -> Self {
                    Self::Data(DataError::DatabaseError(err))
                }
            }

            impl From<serde_json::Error> for $domain {
                fn from(err: serde_json::Error) -> Self {
                    Self::Data(DataError::SerdeJsonError(err))
                }
            }

            impl From<sea_query::error::Error> for $domain {
                fn from(err: sea_query::error::Error) -> Self {
                    Self::Data(DataError::DbQueryError(err))
                }
            }
        )*
    };
}

forward_db_errors!(
    UserError,
    AuthError,
    PermissionError,
    WorkflowError,
    InviteError,
    EventError,
    ConversationError,
    ReportError,
);

impl From<serde_json::Error> for ModelError {
    fn from(err: serde_json::Error) -> Self {
        ModelError::Data(DataError::SerdeJsonError(err))
    }
}

impl From<sqlx::Error> for ModelError {
    fn from(err: sqlx::Error) -> Self {
        ModelError::Data(DataError::DatabaseError(err))
    }
}

impl From<sea_query::error::Error> for ModelError {
    fn from(err: sea_query::error::Error) -> Self {
        ModelError::Data(DataError::DbQueryError(err))
    }
}

impl DomainError for ModelError {
    fn kind(&self) -> ErrorKind {
        match self {
            ModelError::Data(e) => e.kind(),
            ModelError::Validation(e) => e.kind(),
            ModelError::User(e) => e.kind(),
            ModelError::Auth(e) => e.kind(),
            ModelError::Permission(e) => e.kind(),
            ModelError::Workflow(e) => e.kind(),
            ModelError::Invite(e) => e.kind(),
            ModelError::Event(e) => e.kind(),
            ModelError::Conversation(e) => e.kind(),
            ModelError::Report(e) => e.kind(),
        }
    }
}
