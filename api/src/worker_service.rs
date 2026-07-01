pub mod config;
pub mod error;
pub mod process_documents;
pub mod process_video_call_transcriptions;
pub mod scheduled_emails;

use std::{future::Future, str::FromStr, sync::Arc, time::Duration};

use apalis::prelude::*;
use apalis_cron::{CronStream, Schedule};
use apalis_redis::RedisStorage;
use async_trait::async_trait;
use tokio::{sync::Mutex, time::timeout};
use tracing::{error, instrument};

use crate::{
    ComhairleState,
    config::WorkerConfig,
    error::ComhairleError,
    worker_service::{
        process_documents::{DocumentJob, process_document_handler},
        process_video_call_transcriptions::{
            TranscribeRecording, generate_sensemaking_report, transcribe_recording,
        },
        scheduled_emails::{
            ScheduledEmailsRequest, SendScheduledEmailJob, retrieve_and_enqueue_scheduled_emails,
            send_scheduled_email,
        },
    },
};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[async_trait]
#[cfg_attr(test, automock)]
pub trait WorkerService: Send + Sync {
    async fn push_document_job(&self, job: DocumentJob) -> Result<(), ComhairleError>;

    async fn push_transcription_job(&self, job: TranscribeRecording) -> Result<(), ComhairleError>;

    async fn push_send_scheduled_email_job(
        &self,
        job: SendScheduledEmailJob,
    ) -> Result<(), ComhairleError>;
}

#[derive(Clone, Debug)]
pub struct ComhairleWorkerService {
    pub process_documents: Arc<Mutex<RedisStorage<DocumentJob>>>,
    pub process_transcriptions: Arc<Mutex<RedisStorage<StepRequest<Vec<u8>>>>>,
    pub scheduled_emails: Arc<Mutex<RedisStorage<ScheduledEmailsRequest>>>,
    pub send_scheduled_email: Arc<Mutex<RedisStorage<SendScheduledEmailJob>>>,
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

    #[instrument(err(Debug))]
    async fn push_send_scheduled_email_job(
        &self,
        job: SendScheduledEmailJob,
    ) -> Result<(), ComhairleError> {
        let mut lock = self.send_scheduled_email.lock().await;
        lock.push(job)
            .await
            .map_err(|_| ComhairleError::BackgroundJobFailedToQueue)?;

        Ok(())
    }
}

pub struct WorkerStorage {
    pub documents: RedisStorage<DocumentJob>,
    pub transcriptions: RedisStorage<StepRequest<Vec<u8>>>,
    pub scheduled_emails: RedisStorage<ScheduledEmailsRequest>,
    pub send_scheduled_email: RedisStorage<SendScheduledEmailJob>,
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
    let scheduled_emails_config =
        apalis_redis::Config::default().set_namespace("worker_service_scheduled_emails");
    let send_scheduled_email_config =
        apalis_redis::Config::default().set_namespace("worker_service_send_scheduled_email");

    let documents_storage =
        RedisStorage::new_with_config(redis_connection.clone(), documents_config);
    let transcriptions_storage =
        RedisStorage::new_with_config(redis_connection.clone(), transcriptions_config);
    let scheduled_emails_storage =
        RedisStorage::new_with_config(redis_connection.clone(), scheduled_emails_config);
    let send_scheduled_email_storage =
        RedisStorage::new_with_config(redis_connection.clone(), send_scheduled_email_config);

    let worker_service = Arc::new(ComhairleWorkerService {
        process_documents: Arc::new(Mutex::new(documents_storage.clone())),
        process_transcriptions: Arc::new(Mutex::new(transcriptions_storage.clone())),
        scheduled_emails: Arc::new(Mutex::new(scheduled_emails_storage.clone())),
        send_scheduled_email: Arc::new(Mutex::new(send_scheduled_email_storage.clone())),
    });

    Some((
        worker_service,
        WorkerStorage {
            documents: documents_storage,
            transcriptions: transcriptions_storage,
            scheduled_emails: scheduled_emails_storage,
            send_scheduled_email: send_scheduled_email_storage,
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

        let scheduled_emails_schedule = Schedule::from_str("0 */5 * * * *")
            .expect("Unable to create cron schedule for background worker");
        let scheduled_emails_cron_stream = CronStream::new(scheduled_emails_schedule);
        let scheduled_emails_backend =
            scheduled_emails_cron_stream.pipe_to_storage(storage.scheduled_emails.clone());

        let scheduled_emails_worker = WorkerBuilder::new("scheduled_emails_worker")
            .data(state.clone())
            .data(())
            .enable_tracing()
            .backend(scheduled_emails_backend)
            .build_fn(retrieve_and_enqueue_scheduled_emails);

        let send_scheduled_email_worker = WorkerBuilder::new("send_scheduled_email_worker")
            .data(state.clone())
            .data(())
            .enable_tracing()
            .backend(storage.send_scheduled_email.clone())
            .build_fn(send_scheduled_email);

        monitor = monitor
            .register(process_documents_worker)
            .register(process_transcriptions_worker)
            .register(scheduled_emails_worker)
            .register(send_scheduled_email_worker);

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
            .expect_push_send_scheduled_email_job()
            .returning(|_| Box::pin(async move { Ok(()) }));

        worker_service
    }
}
