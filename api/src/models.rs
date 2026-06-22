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
pub mod polis_statement_aux;
pub mod proposal;
pub mod proposal_response;
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

pub trait SqlxResultExt<T> {
    fn not_found_as(self, resource: &str) -> Result<T, ComhairleError>;
}

impl<T> SqlxResultExt<T> for Result<T, sqlx::Error> {
    fn not_found_as(self, resource: &str) -> Result<T, ComhairleError> {
        self.map_err(|e| match e {
            sqlx::Error::RowNotFound => ComhairleError::ResourceNotFound(resource.into()),
            other => ComhairleError::DatabaseError(other),
        })
    }
}
