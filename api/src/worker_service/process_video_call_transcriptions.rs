use apalis::prelude::*;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::job::{self, UpdateJob},
    ComhairleState,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TranscribeRecording {
    pub event_id: Uuid,
    pub job_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UploadTranscription {
    pub transcription_id: Uuid,
    pub job_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RunSenseMaking {
    pub transcription_id: Uuid,
    pub job_id: Uuid,
}

pub async fn transcribe_recording(
    req: TranscribeRecording,
    _ctx: Data<()>,
) -> Result<GoTo<UploadTranscription>, ComhairleError> {
    info!(
        event_id = %req.event_id,
        job_id = %req.job_id,
        "Starting transcription processing job"
    );

    info!("Read video call recording from file system and begin transcription");

    Ok::<_, _>(GoTo::Next(UploadTranscription {
        transcription_id: req.event_id,
        job_id: req.job_id,
    }))
}

pub async fn upload_transcription(
    req: UploadTranscription,
    _ctx: Data<()>,
) -> Result<GoTo<RunSenseMaking>, ComhairleError> {
    info!(
        transcription_id = %req.transcription_id,
        job_id = %req.job_id,
        "Upload transcription via bulk upload service"
    );

    Ok::<_, _>(GoTo::Next(RunSenseMaking {
        transcription_id: req.transcription_id,
        job_id: req.job_id,
    }))
}

pub async fn run_sense_making(
    req: RunSenseMaking,
    state: Data<Arc<ComhairleState>>,
) -> Result<GoTo<&'static str>, ComhairleError> {
    info!(
        transcription_id = %req.transcription_id,
        job_id = %req.job_id,
        "Run video transcription through sense making service"
    );

    let update_job = UpdateJob {
        status: Some("completed".to_string()),
        finished_at: Some(Utc::now()),
        completion_message: Some("Recording successully transcribed".to_string()),
        ..Default::default()
    };
    let _ = job::update(&state.db, &req.job_id, update_job).await?;

    Ok::<_, _>(GoTo::Done(
        "Completed video call transcription job successfully",
    ))
}
