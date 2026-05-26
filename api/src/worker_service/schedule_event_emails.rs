use apalis::prelude::*;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, sync::Arc};
use tracing::info;
use uuid::Uuid;

use crate::{
    models::{
        event::{self, list_upcoming_event_participants, PartialEvent, UpcomingEventParticipant},
        job::{self, CreateJob, UpdateJob},
        otp, users,
    },
    routes::auth::{generate_jwt, OtpClaims},
    ComhairleState,
};

use super::error::{RecordWorkerError, Result, WorkerServiceError};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SendEventReminderJob {
    pub job_id: Uuid,
    pub participant: UpcomingEventParticipant,
}

pub async fn send_event_reminder(
    req: SendEventReminderJob,
    state: Data<Arc<ComhairleState>>,
) -> Result<()> {
    info!(
        user_id = %req.participant.user_id,
        job_id = %req.job_id,
        "Sending event reminder to attendee"
    );

    let user = users::get_user_by_id(&req.participant.user_id, &state.db)
        .await
        .map_err(|e| WorkerServiceError::DbError(e.to_string()))
        .ok_or_record_failure(&req.job_id, &state.db)
        .await?;

    let event = event::get_localized_by_id(
        &state.db,
        &req.participant.event_id,
        &req.participant.primary_locale,
    )
    .await
    .map_err(|e| WorkerServiceError::DbError(e.to_string()))
    .ok_or_record_failure(&req.job_id, &state.db)
    .await?;

    let event_link = format!(
        "/conversations/{}/events/{}/live",
        req.participant.conversation_id, req.participant.event_id
    );

    let expiry = Utc::now() + Duration::hours(24);
    let otp = otp::create(
        &state.db,
        &req.participant.user_id,
        Some(event_link),
        Some(expiry),
    )
    .await
    .map_err(|e| WorkerServiceError::DbError(e.to_string()))
    .ok_or_record_failure(&req.job_id, &state.db)
    .await?;

    let email = req
        .participant
        .user_email
        .ok_or(WorkerServiceError::WrongUserType)
        .ok_or_record_failure(&req.job_id, &state.db)
        .await?;

    let claims = OtpClaims {
        email: email.clone(),
        otp: otp.code.clone(),
    };
    let otp_token = generate_jwt()
        .user(&user)
        .secret(&state.config.jwt_secret)
        .custom_claims(claims)
        .duration(chrono::Duration::minutes(10))
        .call();

    let encoded_redirect_url = urlencoding::encode(&otp.redirect_url);
    let otp_link = format!(
        "{}/auth/login-otp/{}?backTo={}",
        state.config.domain, otp_token, encoded_redirect_url
    );

    state
        .mailer
        .send_event_reminder(email, &event, &None, otp_link)
        .map_err(|e| WorkerServiceError::MailerError(e.to_string()))
        .ok_or_record_failure(&req.job_id, &state.db)
        .await?;

    let update_job = UpdateJob {
        status: Some("completed".to_string()),
        finished_at: Some(Utc::now()),
        completion_message: Some("Send event reminder job completed successfully".to_string()),
        ..Default::default()
    };

    let _ = job::update(&state.db, &req.job_id, update_job)
        .await
        .map_err(|e| WorkerServiceError::DbError(e.to_string()))?;

    info!(
        user_id = %req.participant.user_id,
        job_id = %req.job_id,
        "Reminder successfully sent to event attendee"
    );

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EventEmailRequest;

pub async fn schedule_event_emails(
    _req: EventEmailRequest,
    state: Data<Arc<ComhairleState>>,
) -> Result<()> {
    info!("Checking for upcoming events to schedule email reminders for");

    let worker_service = state
        .required_worker_service()
        .map_err(|_| WorkerServiceError::NoWorkerServiceConfigured)?;

    let participants = list_upcoming_event_participants(&state.db)
        .await
        .map_err(|e| WorkerServiceError::DbError(e.to_string()))?;

    if participants.is_empty() {
        info!("Currently no upcoming events. Exiting.");
        return Ok(());
    }

    for participant in &participants {
        // Enqueue separate jobs for each participant email so that single failure doesn't
        // result in emails being resent to each recipient
        let job = job::create(
            &state.db,
            CreateJob {
                ..Default::default()
            },
        )
        .await
        .map_err(|e| WorkerServiceError::DbError(e.to_string()))?;

        worker_service
            .push_event_reminder_job(SendEventReminderJob {
                job_id: job.id,
                participant: participant.to_owned(),
            })
            .await
            .map_err(|_| WorkerServiceError::BackgroundJobFailedToQueue)?;
    }

    let update_event = PartialEvent {
        reminder_sent_at: Some(Utc::now()),
        ..Default::default()
    };

    let event_ids: Vec<Uuid> = participants
        .iter()
        .map(|p| p.event_id)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    // TODO: this optimistically updates each event.reminder_sent_at. Refactor to store
    // the job ids for each participant job in a hashmap, then check the status
    // of each job and mark each event.reminder_sent_at once all jobs are completed or re-enqueue.
    for event_id in &event_ids {
        event::update(&state.db, event_id, &update_event)
            .await
            .map_err(|e| WorkerServiceError::DbError(e.to_string()))?;
    }

    info!(
        event_ids = format!("{event_ids:#?}"),
        "Successfully scheduled event reminders to attendees"
    );

    Ok(())
}
