use apalis::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

use super::error::Result;
use crate::ComhairleState;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WikiPollModerationNotifyJob {
    pub job_id: Uuid,
    pub workflow_step_id: Uuid,
}

pub async fn send_wiki_poll_moderation_notify_email(
    req: WikiPollModerationNotifyJob,
    _state: Data<Arc<ComhairleState>>,
) -> Result<()> {
    info!(
        workflow_step_id = %req.workflow_step_id,
        job_id = %req.job_id,
        "Sending wiki poll notification email to moderators"
    );

    Ok(())
}
