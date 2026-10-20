use apalis::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

use super::error::{RecordWorkerError, Result, WorkerServiceError};
use crate::ComhairleState;
use crate::models::conversation::{self, ConversationFilterOptions, ConversationOrderOptions};
use crate::models::pagination::PageOptions;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WikiPollModerationNotifyJob {
    pub job_id: Uuid,
    pub workflow_step_id: Uuid,
}

pub async fn send_wiki_poll_moderation_notify_email(
    req: WikiPollModerationNotifyJob,
    state: Data<Arc<ComhairleState>>,
) -> Result<()> {
    info!(
        workflow_step_id = %req.workflow_step_id,
        job_id = %req.job_id,
        "Sending wiki poll notification email to moderators"
    );

    // TODO: check the default pagination
    let live_conversations = conversation::list(
        &state.db,
        PageOptions::default(),
        ConversationOrderOptions {
            created_at: None,
            title: None,
        },
        ConversationFilterOptions {
            is_live: Some(true),
            ..Default::default()
        },
        None,
    )
    .await
    .map_err(|e| WorkerServiceError::DbError(e.to_string()))
    .ok_or_record_failure(&req.job_id, &state.db)
    .await?;

    if live_conversations.total == 0 {
        info!("Currently no live conversations. Exiting.");
        return Ok(());
    }

    Ok(())
}
