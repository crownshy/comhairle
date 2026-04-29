use apalis::prelude::*;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

use crate::{
    categorization_service::Comment,
    models::job::{self, UpdateJob},
    transcription_service::Transcription,
    ComhairleState,
};

use super::error::{RecordWorkerError, Result, WorkerServiceError};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TranscribeRecording {
    pub event_id: Uuid,
    pub conversation_id: Uuid,
    pub room_id: Option<String>,
    pub job_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerateReport {
    pub transcription_key: String,
    pub event_id: Uuid,
    pub conversation_id: Uuid,
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
        transcription_key: format!("{recording_location}/transcript.json"),
        event_id: req.event_id,
        conversation_id: req.conversation_id,
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
    let webhook_signature = &state
        .config
        .categorization_service
        .as_ref()
        .ok_or(WorkerServiceError::NoCategorizationServiceError)
        .ok_or_record_failure(&req.job_id, &state.db)
        .await?
        .webhook_signature;

    info!(
        transcription_id = %req.transcription_key,
        job_id = %req.job_id,
        "Run audio transcription through sense making service and generate report"
    );

    let bytes = state
        .bulk_storage_service
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

    let _analysis_job = categorization_service
        .create_analysis_job(
            comments,
            format!(
                "{}/api/conversation/{}/events/{}/report",
                state.config.domain, req.conversation_id, req.event_id
            ),
            webhook_signature.to_string(),
        )
        .await
        .map_err(|e| WorkerServiceError::CategorizationServiceError(e.to_string()))
        .ok_or_record_failure(&req.job_id, &state.db)
        .await?;

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

#[cfg(test)]
mod tests {
    use aws_config::BehaviorVersion;
    use sqlx::PgPool;

    use crate::{
        bulk_storage::{s3_storage::S3StorageService, BulkStorageService},
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

    #[sqlx::test]
    #[ignore]
    async fn should_transcribe_audio_recording(
        pool: PgPool,
    ) -> std::result::Result<(), Box<dyn Error>> {
        let state = worker_state(pool).await?;

        let request = TranscribeRecording {
            event_id: Uuid::from_str("3c22d53d-07df-4d46-802e-486b79dd1a80").unwrap(),
            conversation_id: Uuid::new_v4(),
            room_id: None,
            job_id: Uuid::new_v4(),
        };

        let _result = transcribe_recording(request, state).await?;

        Ok(())
    }

    #[sqlx::test]
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
            job_id: Uuid::new_v4(),
        };

        let _result = generate_sensemaking_report(request, state).await?;

        Ok(())
    }
}
