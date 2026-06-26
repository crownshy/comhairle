use apalis::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

use crate::{
    categorization_service::Comment,
    models::{audio_recording::AudioFormat, job},
    transcription_service::Transcription,
    ComhairleState,
};

use super::error::{RecordWorkerError, Result, WorkerServiceError};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TranscribeRecording {
    pub event_id: Uuid,
    pub conversation_id: Uuid,
    /// The room (audio_recording) being processed.
    pub room_id: Uuid,
    pub job_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerateReport {
    pub transcription_key: String,
    pub event_id: Uuid,
    pub conversation_id: Uuid,
    /// The room (audio_recording) being processed.
    pub room_id: Uuid,
    pub job_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UploadReport {
    pub report_job_id: String,
    pub conversation_id: Uuid,
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
    let bulk_storage_service = state
        .required_bulk_storage_service()
        .map_err(|_| WorkerServiceError::NoBulkStorageServiceConfigured)
        .ok_or_record_failure(&req.job_id, &state.db)
        .await?;
    let bulk_storage_config = state
        .config
        .bulk_storage_service
        .as_ref()
        .ok_or(WorkerServiceError::NoBulkStorageServiceConfigured)
        .ok_or_record_failure(&req.job_id, &state.db)
        .await?;

    info!(
        event_id = %req.event_id,
        job_id = %req.job_id,
        "Starting transcription sensemaking pipeline"
    );

    let recording_location = format!("events/{}/rooms/{}", req.event_id, req.room_id);

    // Look up the room to find the uploaded file format. Default to WAV if the
    // room record can't be read, to preserve the historical behaviour.
    let audio_format = crate::models::audio_recording::get_by_id(&state.db, &req.room_id)
        .await
        .map(|r| r.file_extension)
        .unwrap_or(AudioFormat::Wav);

    let result = transcription_service
        .transcribe_from_bulk_store(
            &bulk_storage_config.store_name,
            &recording_location,
            audio_format,
            bulk_storage_service,
        )
        .await
        .map_err(|e| WorkerServiceError::TranscriptionServiceError(e.to_string()))
        .ok_or_record_failure(&req.job_id, &state.db)
        .await?;

    info!(
        event_id = %req.event_id,
        job_id = %req.job_id,
        completed_at = result.completed_at.to_string(),
        "Transcription from bulk storage completed successfully"
    );

    Ok::<_, _>(GoTo::Next(GenerateReport {
        transcription_key: format!("{recording_location}/transcript.json"),
        event_id: req.event_id,
        conversation_id: req.conversation_id,
        room_id: req.room_id,
        job_id: req.job_id,
    }))
}

pub async fn generate_sensemaking_report(
    req: GenerateReport,
    state: Data<Arc<ComhairleState>>,
) -> Result<GoTo<&'static str>> {
    let categorization_service = state
        .required_categorization_service()
        .map_err(|_| WorkerServiceError::NoCategorizationServiceError)
        .ok_or_record_failure(&req.job_id, &state.db)
        .await?;
    let webhook_secret = &state
        .config
        .categorization_service
        .as_ref()
        .ok_or(WorkerServiceError::NoCategorizationServiceError)
        .ok_or_record_failure(&req.job_id, &state.db)
        .await?
        .webhook_secret;
    let bulk_storage_service = state
        .required_bulk_storage_service()
        .map_err(|_| WorkerServiceError::NoBulkStorageServiceConfigured)
        .ok_or_record_failure(&req.job_id, &state.db)
        .await?;

    info!(
        transcription_key = %req.transcription_key,
        job_id = %req.job_id,
        "Sending audio transcription to categorization service"
    );

    let bytes = bulk_storage_service
        .get_file(&req.transcription_key)
        .await
        .map_err(|e| WorkerServiceError::BulkStorageServiceError(e.to_string()))
        .ok_or_record_failure(&req.job_id, &state.db)
        .await?;

    let transcription: Transcription = serde_json::from_slice(&bytes)?;

    let comments: Vec<Comment> = transcription
        .events
        .iter()
        .filter_map(|event| {
            event.speaker_id.as_ref().map(|speaker| Comment {
                id: Uuid::new_v4().to_string(), // TODO:
                comment: event.text.clone(),
                speaker: speaker.to_owned(),
            })
        })
        .collect();

    let analysis_job = categorization_service
        .create_analysis_job(
            comments,
            format!(
                "{}/api/conversation/{}/events/{}/rooms/{}/report",
                state.config.domain, req.conversation_id, req.event_id, req.room_id
            ),
            webhook_secret.to_string(),
        )
        .await
        .map_err(|e| WorkerServiceError::CategorizationServiceError(e.to_string()))
        .ok_or_record_failure(&req.job_id, &state.db)
        .await?;

    info!(
        job_id = %req.job_id,
        categorization_job_id = %analysis_job.id,
        "Report job created in categorization service"
    );

    // Mark this room's recording as completed once it has been submitted for
    // categorization.
    let _ = crate::models::audio_recording::update_status(
        &state.db,
        &req.room_id,
        crate::models::audio_recording::AudioRecordingStatus::Completed,
    )
    .await;

    job::complete(
        &state.db,
        req.job_id,
        "Transcription sensemaking pipeline completed successfully",
    )
    .await
    .map_err(|e| WorkerServiceError::DbError(e.to_string()))?;

    Ok::<_, _>(GoTo::Done(
        "Transcription sensemaking pipeline completed successfully",
    ))
}

#[cfg(test)]
mod tests {
    use aws_config::BehaviorVersion;
    use sqlx::PgPool;

    use crate::{
        bulk_storage_service::{s3_storage::S3StorageService, BulkStorageService},
        categorization_service::{tttc_categorizer::TttcCategorizer, CategorizationService},
        config::TranscriptionServiceConfig,
        test_helpers::{test_config, test_state},
        transcription_service::amazon_transcriber::AmazonTranscriber,
    };

    use super::*;

    use std::{error::Error, str::FromStr};

    async fn worker_state(
        pool: PgPool,
    ) -> std::result::Result<Data<Arc<ComhairleState>>, Box<dyn Error>> {
        let config = test_config().unwrap();

        let s3_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let bulk_storage_service = Arc::new(S3StorageService::new(
            &s3_config,
            "comhairle-media-test".to_owned(),
        )) as Arc<dyn BulkStorageService>;

        let categorization_service = config.categorization_service.as_ref().map(|config| {
            Arc::new(TttcCategorizer::new(&config.server_url, &config.api_key))
                as Arc<dyn CategorizationService>
        });

        let transcription_service = match &config.transcription_service {
            Some(TranscriptionServiceConfig::AmazonTranscribe(_)) => {
                Some(Arc::new(AmazonTranscriber::new().await)
                    as Arc<dyn crate::transcription_service::Transcriber>)
            }
            None => None,
        };

        let state = test_state()
            .db(pool)
            .bulk_storage_service(bulk_storage_service)
            .categorization_service(categorization_service.unwrap())
            .transcription_service(transcription_service.unwrap())
            .call()?;

        Ok(Data::new(Arc::new(state)))
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    #[ignore]
    async fn should_transcribe_audio_recording(
        pool: PgPool,
    ) -> std::result::Result<(), Box<dyn Error>> {
        let state = worker_state(pool).await?;

        let request = TranscribeRecording {
            event_id: Uuid::from_str("3c22d53d-07df-4d46-802e-486b79dd1a80").unwrap(),
            conversation_id: Uuid::new_v4(),
            room_id: Uuid::new_v4(),
            job_id: Uuid::new_v4(),
        };

        let _result = transcribe_recording(request, state).await?;

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    #[ignore]
    async fn should_generate_sense_making_report_from_transcript(
        pool: PgPool,
    ) -> std::result::Result<(), Box<dyn Error>> {
        let state = worker_state(pool).await?;

        let request = GenerateReport {
            transcription_key: "events/3c22d53d-07df-4d46-802e-486b79dd1a80/transcript.json"
                .to_string(),
            event_id: Uuid::from_str("3c22d53d-07df-4d46-802e-486b79dd1a80").unwrap(),
            conversation_id: Uuid::new_v4(),
            room_id: Uuid::new_v4(),
            job_id: Uuid::new_v4(),
        };

        let _result = generate_sensemaking_report(request, state).await?;

        Ok(())
    }
}
