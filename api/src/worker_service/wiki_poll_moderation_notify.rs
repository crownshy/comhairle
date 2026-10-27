use std::sync::Arc;

use apalis::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{info, instrument, warn};
use uuid::Uuid;

use super::error::{RecordWorkerError, Result, WorkerServiceError};
use crate::ComhairleState;
use crate::models::conversation::{
    self, ConversationFilterOptions, ConversationOrderOptions, LocalizedConversation,
};
use crate::models::job::{self, CreateJob};
use crate::models::pagination::PageOptions;
use crate::models::workflow_step::LocalizedWorkflowStep;
use crate::models::{permissions, users, workflow, workflow_step};
use crate::tools::ToolConfig;
use crate::tools::polis::PolisToolConfig;
use crate::wiki_poll_service::ModerationStatus;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WikiPollModerationNotifyRequest;

#[instrument(err(Debug), skip_all)]
pub async fn retrieve_and_enqueue_wiki_poll_moderation_notify_jobs(
    _req: WikiPollModerationNotifyRequest,
    state: Data<Arc<ComhairleState>>,
) -> Result<()> {
    info!("Checking for live wiki poll conversations");

    let worker_service = state
        .required_worker_service()
        .map_err(|_| WorkerServiceError::NoWorkerServiceConfigured)?;

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
                conversation,
            })
            .await
            .map_err(|_| WorkerServiceError::BackgroundJobFailedToQueue)?;
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendWikiPollModerationNotifyJob {
    job_id: Uuid,
    conversation: LocalizedConversation,
}

#[instrument(
    err(Debug),
    skip_all,
    fields(job_id = %req.job_id, conversation_id = %req.conversation.id)
)]
pub async fn send_wiki_poll_moderation_notify_emails(
    req: SendWikiPollModerationNotifyJob,
    state: Data<Arc<ComhairleState>>,
) -> Result<()> {
    info!(
        conversation_id = %req.conversation.id,
        job_id = %req.job_id,
        "Checking for wiki poll moderation comments"
    );

    let wiki_poll_steps = collect_wiki_poll_steps(
        &state.db,
        req.conversation.id,
        &req.conversation.primary_locale,
    )
    .await
    .ok_or_record_failure(&req.job_id, &state.db)
    .await?;

    if wiki_poll_steps.is_empty() {
        log_and_complete(
            req.job_id,
            &state.db,
            "Conversation contains no wiki poll steps, exiting.",
        )
        .await?;

        return Ok(());
    }

    let mut steps_with_pending = Vec::new();
    for (step, config) in wiki_poll_steps {
        if !config.notify_moderators {
            continue;
        }

        let mut pending_count = 0;
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
            let moderation_url = format!(
                "{}/admin/conversations/{}/moderate/step/{}",
                state.config.domain, req.conversation.id, step.id
            );
            steps_with_pending.push((step, pending_count, moderation_url));
        }
    }

    if steps_with_pending.is_empty() {
        log_and_complete(
            req.job_id,
            &state.db,
            "No wiki poll steps with pending comments, exiting.",
        )
        .await?;

        return Ok(());
    }

    let emails = collect_moderator_emails(&state.db, &req.conversation)
        .await
        .ok_or_record_failure(&req.job_id, &state.db)
        .await?;

    if emails.is_empty() {
        log_and_complete(
            req.job_id,
            &state.db,
            "No moderators for conversation, exiting.",
        )
        .await?;

        return Ok(());
    }

    for email in emails {
        // Log error and continue if email send fails to prevent retries re-sending
        // emails to successful users
        if let Err(e) = state.mailer.send_moderation_notification_email(
            &email,
            &req.conversation.title,
            &steps_with_pending,
        ) {
            warn!("Failed to send moderation notification email to {email}: {e}");
        }
    }

    log_and_complete(
        req.job_id,
        &state.db,
        "Completed job sending moderation notification emails, exiting.",
    )
    .await?;

    Ok(())
}

async fn log_and_complete(job_id: Uuid, db: &PgPool, message: &str) -> Result<()> {
    info!(job_id = %job_id, message);

    job::complete(db, job_id, message)
        .await
        .map_err(|e| WorkerServiceError::DbError(e.to_string()))?;

    Ok(())
}

async fn collect_wiki_poll_steps(
    db: &PgPool,
    conversation_id: Uuid,
    locale: &str,
) -> Result<Vec<(LocalizedWorkflowStep, PolisToolConfig)>> {
    let workflows = workflow::list_for_conversation(db, conversation_id)
        .await
        .map_err(|e| WorkerServiceError::DbError(e.to_string()))?;

    let mut wiki_poll_steps = Vec::new();
    for workflow in workflows {
        let workflow_steps = workflow_step::list_localized(db, &workflow.id, locale)
            .await
            .map_err(|e| WorkerServiceError::DbError(e.to_string()))?;

        wiki_poll_steps.extend(workflow_steps.into_iter().filter_map(|step| {
            let Some(ToolConfig::Polis(config)) = &step.tool_config else {
                return None;
            };
            let config = config.clone();
            Some((step, config))
        }));
    }

    Ok(wiki_poll_steps)
}

async fn collect_moderator_emails(
    db: &PgPool,
    conversation: &LocalizedConversation,
) -> Result<Vec<String>> {
    let mut emails = Vec::new();
    let owner = users::get_user_by_id(&conversation.owner_id, db)
        .await
        .map_err(|e| WorkerServiceError::DbError(e.to_string()))?;

    if let Some(email) = owner.email {
        emails.push(email);
    }

    let moderators = permissions::list_users_with_permission(
        db,
        "conversation",
        conversation.id,
        Some("moderator"),
    )
    .await
    .map_err(|e| WorkerServiceError::DbError(e.to_string()))?;

    for user in moderators {
        if let Some(email) = user.email {
            emails.push(email)
        }
    }

    Ok(emails)
}
