pub mod config;
pub mod error;
pub mod process_documents;
pub mod process_video_call_transcriptions;
pub mod schedule_event_emails;

use std::{future::Future, str::FromStr, sync::Arc, time::Duration};

use apalis::prelude::*;
use apalis_cron::{CronStream, Schedule};
use apalis_redis::RedisStorage;
use async_trait::async_trait;
use tokio::{sync::Mutex, time::timeout};
use tracing::{error, instrument};

use crate::{
    config::WorkerConfig,
    error::ComhairleError,
    worker_service::{
        process_documents::{process_document_handler, DocumentJob},
        process_video_call_transcriptions::{
            generate_sensemaking_report, transcribe_recording, TranscribeRecording,
        },
        schedule_event_emails::{schedule_event_emails, EventEmailRequest},
    },
    ComhairleState,
};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[async_trait]
#[cfg_attr(test, automock)]
pub trait WorkerService: Send + Sync {
    async fn push_document_job(&self, job: DocumentJob) -> Result<(), ComhairleError>;

    async fn push_transcription_job(&self, job: TranscribeRecording) -> Result<(), ComhairleError>;
}

#[derive(Clone, Debug)]
pub struct ComhairleWorkerService {
    pub process_documents: Arc<Mutex<RedisStorage<DocumentJob>>>,
    pub process_transcriptions: Arc<Mutex<RedisStorage<StepRequest<Vec<u8>>>>>,
    pub schedule_event_emails: Arc<Mutex<RedisStorage<EventEmailRequest>>>,
}

#[async_trait]
impl WorkerService for ComhairleWorkerService {
    #[instrument(err(Debug))]
    async fn push_document_job(&self, job: DocumentJob) -> Result<(), ComhairleError> {
        let mut lock = self.process_documents.lock().await;
        lock.push(job)
            .await
            .map_err(|_| ComhairleError::BackgroundJobFailedToQueue)?;

        Ok(())
    }

    #[instrument(err(Debug))]
    async fn push_transcription_job(&self, job: TranscribeRecording) -> Result<(), ComhairleError> {
        let mut lock = self.process_transcriptions.lock().await;
        lock.start_stepped(job)
            .await
            .map_err(|_| ComhairleError::BackgroundJobFailedToQueue)?;

        Ok(())
    }
}

pub struct WorkerStorage {
    pub documents: RedisStorage<DocumentJob>,
    pub transcriptions: RedisStorage<StepRequest<Vec<u8>>>,
    pub event_emails: RedisStorage<EventEmailRequest>,
}

pub async fn init_worker_service(
    config: &Option<WorkerConfig>,
) -> Option<(Arc<dyn WorkerService>, WorkerStorage)> {
    let config = config.as_ref()?;

    // Manually handle connection timeout as apalis default too long
    let redis_connection = timeout(
        Duration::from_secs(10),
        apalis_redis::connect(config.redis_url.clone()),
    )
    .await
    .ok()
    .and_then(|r| r.ok())
    .or_else(|| {
        error!("Timed out attempting to establish connection to Redis");
        error!("Worker service unavailable");
        None
    })?;

    let documents_config =
        apalis_redis::Config::default().set_namespace("worker_service_documents");
    let transcriptions_config =
        apalis_redis::Config::default().set_namespace("worker_service_transcriptions");
    let event_email_scheduler_config =
        apalis_redis::Config::default().set_namespace("worker_service_event_email_scheduler");

    let documents_storage =
        RedisStorage::new_with_config(redis_connection.clone(), documents_config);
    let transcriptions_storage =
        RedisStorage::new_with_config(redis_connection.clone(), transcriptions_config);
    let event_email_scheduler_storage =
        RedisStorage::new_with_config(redis_connection.clone(), event_email_scheduler_config);

    let worker_service = Arc::new(ComhairleWorkerService {
        process_documents: Arc::new(Mutex::new(documents_storage.clone())),
        process_transcriptions: Arc::new(Mutex::new(transcriptions_storage.clone())),
        schedule_event_emails: Arc::new(Mutex::new(event_email_scheduler_storage.clone())),
    });

    Some((
        worker_service,
        WorkerStorage {
            documents: documents_storage,
            transcriptions: transcriptions_storage,
            event_emails: event_email_scheduler_storage,
        },
    ))
}

pub fn init_monitor(
    storage: Option<WorkerStorage>,
    state: &Arc<ComhairleState>,
) -> Option<impl Future<Output = std::io::Result<()>>> {
    if let Some(storage) = storage {
        let mut monitor = Monitor::new();

        let process_documents_worker = WorkerBuilder::new("process_document_worker")
            .data(state.clone())
            .data(())
            .enable_tracing()
            .backend(storage.documents.clone())
            .build_fn(process_document_handler);

        let transcription_worker_steps = StepBuilder::new()
            .step_fn(transcribe_recording)
            .step_fn(generate_sensemaking_report);

        let process_transcriptions_worker = WorkerBuilder::new("process_transcriptions_worker")
            .data(state.clone())
            .data(())
            .enable_tracing()
            .backend(storage.transcriptions.clone())
            .build_stepped(transcription_worker_steps);

        // TODO: update for testing
        // let event_emails_schedule = Schedule::from_str("0 */15 * * * *").unwrap(); // TODO:
        let event_emails_schedule = Schedule::from_str("*/10 * * * * *").unwrap(); // TODO:
        let event_emails_cron_stream = CronStream::new(event_emails_schedule);
        let event_emails_backend =
            event_emails_cron_stream.pipe_to_storage(storage.event_emails.clone());

        let schedule_event_emails_worker = WorkerBuilder::new("schedule_event_emails")
            .data(state.clone())
            .data(())
            .enable_tracing()
            .backend(event_emails_backend)
            .build_fn(schedule_event_emails);

        monitor = monitor
            .register(process_documents_worker)
            .register(process_transcriptions_worker)
            .register(schedule_event_emails_worker);

        Some(monitor.run())
    } else {
        None
    }
}

#[cfg(test)]
impl MockWorkerService {
    pub fn base() -> MockWorkerService {
        let mut worker_service = MockWorkerService::new();

        worker_service
            .expect_push_document_job()
            .returning(|_| Box::pin(async move { Ok(()) }));

        worker_service
            .expect_push_transcription_job()
            .returning(|_| Box::pin(async move { Ok(()) }));

        worker_service
    }
}
