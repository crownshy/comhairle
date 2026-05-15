use apalis::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;

use crate::ComhairleState;

use super::error::{RecordWorkerError, Result, WorkerServiceError};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EventEmailRequest;

pub async fn schedule_event_emails(
    req: EventEmailRequest,
    data: Data<Arc<ComhairleState>>,
) -> Result<()> {
    info!("Look for upcoming events and schedule emails");

    Ok(())
}
