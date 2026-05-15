use apalis::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;

use crate::{models::event::list_upcoming_event_participants, ComhairleState};

use super::error::{RecordWorkerError, Result, WorkerServiceError};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EventEmailRequest;

pub async fn schedule_event_emails(
    req: EventEmailRequest,
    state: Data<Arc<ComhairleState>>,
) -> Result<()> {
    info!("Look for upcoming events and schedule emails");

    let users = list_upcoming_event_participants(&state.db).await.unwrap();

    // 1. Query upcoming events
    // 2. Query event_attendances with role participants joined with user to get email address
    // 3. Send emails
    // 4. Update reminder_sent_at timestamp on event table so that reminders are only sent once

    Ok(())
}
