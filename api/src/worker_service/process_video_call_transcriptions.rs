use apalis::prelude::*;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

use crate::{
    models::job::{self, UpdateJob},
    ComhairleState,
};

use super::error::{RecordWorkerError, Result, WorkerServiceError};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TranscribeRecording {
    pub event_id: Uuid,
    pub room_id: Option<String>,
    pub job_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerateReport {
    pub transcription_id: Uuid,
    pub job_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UploadReport {
    pub transcription_id: Uuid,
    pub job_id: Uuid,
}

pub async fn transcribe_recording(
    req: TranscribeRecording,
    state: Data<Arc<ComhairleState>>,
) -> Result<GoTo<GenerateReport>> {
    let transcription_service = state
        .required_transcription_service()
        .map_err(|_| WorkerServiceError::NoTranscriptionServiceConfigured)
        .ok_or_record_failure(&req.job_id, &state.db)
        .await?;

    info!(
        event_id = %req.event_id,
        job_id = %req.job_id,
        "Starting transcription sensemaking pipeline"
    );

    let recording_location = format!(
        "events/{}{}",
        req.event_id,
        req.room_id
            .as_deref()
            .map_or(String::new(), |id| format!("/rooms/{id}"))
    );

    let _result = transcription_service
        .transcribe_from_bulk_store(
            "comhairle-media",
            &recording_location,
            &state.bulk_storage_service,
        )
        .await
        .map_err(|e| WorkerServiceError::TranscriptionServiceError(e.to_string()))
        .ok_or_record_failure(&req.job_id, &state.db)
        .await?;

    Ok::<_, _>(GoTo::Next(GenerateReport {
        transcription_id: req.event_id,
        job_id: req.job_id,
    }))
}

pub async fn generate_report_from_sensemaking(
    req: GenerateReport,
    _state: Data<Arc<ComhairleState>>,
) -> Result<GoTo<UploadReport>> {
    info!(
        transcription_id = %req.transcription_id,
        job_id = %req.job_id,
        "Run audio transcription through sense making service and generate report"
    );

    Ok::<_, _>(GoTo::Next(UploadReport {
        transcription_id: req.transcription_id,
        job_id: req.job_id,
    }))
}

pub async fn upload_report(
    req: UploadReport,
    state: Data<Arc<ComhairleState>>,
) -> Result<GoTo<&'static str>> {
    info!(
        transcription_id = %req.transcription_id,
        job_id = %req.job_id,
        "Upload report via bulk storage service"
    );

    let update_job = UpdateJob {
        status: Some("completed".to_string()),
        finished_at: Some(Utc::now()),
        completion_message: Some(
            "Transcription sensemaking pipeline completed successfully".to_string(),
        ),
        ..Default::default()
    };

    let _ = job::update(&state.db, &req.job_id, update_job)
        .await
        .map_err(|e| WorkerServiceError::DbError(e.to_string()))?;

    Ok::<_, _>(GoTo::Done(
        "Transcription sensemaking pipeline completed successfully",
    ))
}
