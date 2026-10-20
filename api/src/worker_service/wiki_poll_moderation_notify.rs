use apalis::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

use super::error::{RecordWorkerError, Result, WorkerServiceError};
use crate::ComhairleState;
use crate::models::conversation::{self, ConversationFilterOptions, ConversationOrderOptions};
use crate::models::job::{self, CreateJob};
use crate::models::pagination::PageOptions;
use crate::models::workflow;
use crate::models::workflow_step;
use crate::tools::ToolConfig;
use crate::wiki_poll_service::ModerationStatus;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WikiPollModerationNotifyRequest;

pub async fn retrieve_and_enqueue_wiki_poll_moderation_notify_jobs(
    _req: WikiPollModerationNotifyRequest,
    state: Data<Arc<ComhairleState>>,
) -> Result<()> {
    info!("Checking for live wiki poll conversations");

    let worker_service = state
        .required_worker_service()
        .map_err(|_| WorkerServiceError::NoWorkerServiceConfigured)?;

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
    .map_err(|e| WorkerServiceError::DbError(e.to_string()))?;

    if live_conversations.total == 0 {
        info!("Currently no live conversations. Exiting.");
        return Ok(());
    }

    for conversation in live_conversations.records {
        let job = job::create(
            &state.db,
            CreateJob {
                ..Default::default()
            },
        )
        .await
        .map_err(|e| WorkerServiceError::DbError(e.to_string()))?;

        worker_service
            .push_send_wiki_poll_moderation_notify_job(SendWikiPollModerationNotifyJob {
                job_id: job.id,
                conversation_id: conversation.id,
            })
            .await
            .map_err(|_| WorkerServiceError::BackgroundJobFailedToQueue)?;
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SendWikiPollModerationNotifyJob {
    job_id: Uuid,
    conversation_id: Uuid,
}

pub async fn send_wiki_poll_moderation_notify_emails(
    req: SendWikiPollModerationNotifyJob,
    state: Data<Arc<ComhairleState>>,
) -> Result<()> {
    info!(
        conversation_id = %req.conversation_id,
        job_id = %req.job_id,
        "Checking for wiki poll moderation comments"
    );

    let workflows = workflow::list_for_conversation(&state.db, req.conversation_id)
        .await
        .map_err(|e| WorkerServiceError::DbError(e.to_string()))
        .ok_or_record_failure(&req.job_id, &state.db)
        .await?;

    let mut wiki_poll_steps = Vec::new();
    for workflow in workflows {
        let workflow_steps = workflow_step::list(&state.db, &workflow.id)
            .await
            .map_err(|e| WorkerServiceError::DbError(e.to_string()))
            .ok_or_record_failure(&req.job_id, &state.db)
            .await?;

        wiki_poll_steps.extend(workflow_steps.into_iter().filter_map(|step| {
            let Some(ToolConfig::Polis(config)) = &step.tool_config else {
                return None;
            };
            let config = config.clone();
            Some((step, config))
        }));
    }

    if wiki_poll_steps.is_empty() {
        let message = "Conversation contains no wiki poll steps, exiting.";
        info!(
            job_id = %req.job_id,
            message
        );
        job::complete(&state.db, req.job_id, message)
            .await
            .map_err(|e| WorkerServiceError::DbError(e.to_string()))?;

        return Ok(());
    }

    let mut steps_with_pending = Vec::new();
    // TODO: add toggle for workflow_step to enable email feature
    for (step, config) in wiki_poll_steps {
        let mut pending_count = 0;
        // TODO: check what `moderation=true` does on polis request
        let comments = state
            .wiki_poll_service
            .get_comments(&config.poll_id)
            .await
            .map_err(|e| WorkerServiceError::WikiPollServiceError(e.to_string()))
            .ok_or_record_failure(&req.job_id, &state.db)
            .await?;

        comments.into_iter().for_each(|comment| {
            let moderation_status = ModerationStatus::try_from(comment.moderation);

            if let Ok(status) = moderation_status
                && status == ModerationStatus::Pending
            {
                pending_count += 1;
            }
        });

        if pending_count > 0 {
            steps_with_pending.push((step, pending_count));
        }
    }

    // TODO:
    // 2. Send emails to mod and owner if so

    Ok(())
}
