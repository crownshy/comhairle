use apalis::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

use crate::{
    mailer::build_invite_attachment,
    models::{
        job::{self, CreateJob},
        scheduled_email::{list_upcoming_scheduled_emails, ScheduledEmail},
    },
    ComhairleState,
};

use super::error::{RecordWorkerError, Result, WorkerServiceError};

#[derive(Serialize, Deserialize, Debug)]
pub struct SendScheduledEmailJob {
    pub job_id: Uuid,
    pub scheduled_email: ScheduledEmail,
}

pub async fn send_scheduled_email(
    req: SendScheduledEmailJob,
    state: Data<Arc<ComhairleState>>,
) -> Result<()> {
    info!(
        scheduled_email_id = %req.scheduled_email.id,
        job_id = %req.job_id,
        "Sending scheduled email to recipient"
    );

    let attachment = req
        .scheduled_email
        .email_config
        .attachment
        .as_deref()
        .and_then(|body| build_invite_attachment(body.to_string()).ok());

    state
        .mailer
        .send_email(
            &req.scheduled_email.user_email,
            &req.scheduled_email.email_config.subject,
            &req.scheduled_email.email_config.template.to_string(),
            req.scheduled_email
                .clone()
                .email_config
                .template
                .to_mailer_context(),
            attachment,
        )
        .map_err(|e| WorkerServiceError::MailerError(e.to_string()))
        .ok_or_record_failure(&req.job_id, &state.db)
        .await?;

    // Mark status as sent
    req.scheduled_email
        .sent(&state.db)
        .await
        .map_err(|e| WorkerServiceError::DbError(e.to_string()))?;

    job::complete(
        &state.db,
        req.job_id,
        "Send event reminder job completed successfully",
    )
    .await
    .map_err(|e| WorkerServiceError::DbError(e.to_string()))?;

    info!(
        scheduled_email_id = %req.scheduled_email.id,
        job_id = %req.job_id,
        "Scheduled email successfully sent to recipient"
    );

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ScheduledEmailsRequest;

pub async fn retrieve_and_enqueue_scheduled_emails(
    _req: ScheduledEmailsRequest,
    state: Data<Arc<ComhairleState>>,
) -> Result<()> {
    info!("Checking for upcoming scheduled_emails");

    let worker_service = state
        .required_worker_service()
        .map_err(|_| WorkerServiceError::NoWorkerServiceConfigured)?;

    // TODO: is 10 minutes sufficient
    let scheduled_emails = list_upcoming_scheduled_emails(&state.db, chrono::Duration::minutes(10))
        .await
        .map_err(|e| WorkerServiceError::DbError(e.to_string()))?;

    if scheduled_emails.is_empty() {
        info!("Currently no upcoming scheduled emails. Exiting.");
        return Ok(());
    }

    for scheduled_email in scheduled_emails {
        let job = job::create(
            &state.db,
            CreateJob {
                ..Default::default()
            },
        )
        .await
        .map_err(|e| WorkerServiceError::DbError(e.to_string()))?;

        worker_service
            .push_send_scheduled_email_job(SendScheduledEmailJob {
                job_id: job.id,
                scheduled_email,
            })
            .await
            .map_err(|_| WorkerServiceError::BackgroundJobFailedToQueue)?;
    }

    Ok(())
}
