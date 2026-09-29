//! Sending a scheduled email.
//!
//! Dispatching needs the mailer off [`ComhairleState`], so it lives here rather
//! than as a method on [`EmailTemplate`], which stays a plain data enum in
//! [`crate::models::scheduled_email`].

use std::sync::Arc;

use crate::models::scheduled_email::EmailTemplate;
use crate::{ComhairleState, error::ComhairleError};

pub async fn send(
    template: &EmailTemplate,
    email: &str,
    state: &Arc<ComhairleState>,
) -> Result<(), ComhairleError> {
    match template {
        EmailTemplate::EventReminder {
            event_id,
            recipient_id,
            owner_id,
            locale,
        } => {
            state
                .mailer
                .send_event_reminder(state, email, *event_id, *recipient_id, *owner_id, locale)
                .await
        }
    }
}
