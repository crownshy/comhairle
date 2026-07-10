use sqlx::error::ErrorKind;

use crate::error::ComhairleError;

pub mod api_key;
pub mod audio_recording;
pub mod bot_service_user_session;
pub mod conversation;
pub mod conversation_email_notification_recipients;
pub mod email_template_config;
pub mod event;
pub mod event_attendance;
pub mod feedback;
pub mod invite_response;
pub mod invites;
pub mod job;
pub mod media;
pub mod notification;
pub mod notification_delivery;
pub mod organization;
pub mod otp;
pub mod pagination;
pub mod permissions;
pub mod polis_statement_aux;
pub mod proposal;
pub mod proposal_response;
pub mod recruitment_target;
pub mod region;
pub mod report;
pub mod report_impact;
pub mod resource;
pub mod scheduled_email;
pub mod thinking_space_answer;
pub mod thinking_space_follow_up_question;
pub mod thinking_space_summary;
pub mod translations;
pub mod user_conversation_preferences;
pub mod user_participation;
pub mod user_profile;
pub mod user_progress;
pub mod users;
pub mod workflow;
pub mod workflow_step;

#[cfg(test)]
pub mod model_test_helpers;

/// Extension trait for converting `sqlx` query results into domain-level
/// [`ComhairleError`]s.
///
/// This centralizes the mapping from low-level database errors to the
/// HTTP-facing error variants used throughout the API, so call sites don't
/// need to pattern-match on `sqlx::Error` themselves.
pub trait SqlxResultExt<T> {
    /// Resolves a `sqlx` query result into a [`ComhairleError`], classifying
    /// the underlying database error where possible.
    ///
    /// - `sqlx::Error::RowNotFound` is mapped to
    ///   [`ComhairleError::ResourceNotFound`] (HTTP 404), using `resource` as a
    ///   human-readable description of what was being looked up (e.g.
    ///   `"User"`, `"Workflow Step"`).
    /// - Foreign key and unique constraint violations are mapped to
    ///   [`ComhairleError::Conflict`] (HTTP 409), since the request is
    ///   well-formed but conflicts with the current state of the database
    ///   (e.g. referencing a row that doesn't exist, or duplicating a
    ///   unique value).
    /// - All other database errors fall through to
    ///   [`ComhairleError::DatabaseError`], preserving the original
    ///   `sqlx::Error` for logging/debugging.
    ///
    /// # Arguments
    ///
    /// * `resource` - A short, human-readable name for the resource being
    ///   queried. Used in the `ResourceNotFound` and `Conflict` error
    ///   messages.
    ///
    /// # Examples
    ///
    /// ```
    /// let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
    ///     .fetch_one(&pool)
    ///     .await
    ///     .resolve_db_err("user")?;
    /// ```
    fn resolve_db_err(self, resource: &str) -> Result<T, ComhairleError>;
}

impl<T> SqlxResultExt<T> for Result<T, sqlx::Error> {
    fn resolve_db_err(self, resource: &str) -> Result<T, ComhairleError> {
        self.map_err(|e| match e {
            sqlx::Error::RowNotFound => ComhairleError::ResourceNotFound(resource.into()),
            sqlx::Error::Database(ref db_err) => match db_err.kind() {
                ErrorKind::ForeignKeyViolation | ErrorKind::UniqueViolation => {
                    ComhairleError::Conflict(format!(
                        "{resource} conflicts with an existing record"
                    ))
                }
                _ => ComhairleError::DatabaseError(e),
            },
            _ => ComhairleError::DatabaseError(e),
        })
    }
}
