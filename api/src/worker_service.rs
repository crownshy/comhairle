pub mod process_documents;
pub mod process_video_call_transcriptions;

use std::sync::Arc;

use apalis::prelude::*;
use apalis_redis::RedisStorage;
use async_trait::async_trait;
use tokio::sync::Mutex;
use tracing::instrument;

use crate::{
    error::ComhairleError,
    worker_service::{
        process_documents::DocumentJob, process_video_call_transcriptions::TranscribeRecording,
    },
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
    pub process_documents: Arc<Mutex<MemoryStorage<DocumentJob>>>,
    pub process_transcriptions: Arc<Mutex<RedisStorage<StepRequest<Vec<u8>>>>>,
}

#[async_trait]
impl WorkerService for ComhairleWorkerService {
    #[instrument(err(Debug))]
    async fn push_document_job(&self, job: DocumentJob) -> Result<(), ComhairleError> {
        let mut lock = self.process_documents.lock().await;
        lock.enqueue(job)
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

#[cfg(test)]
impl MockWorkerService {
    pub fn base() -> MockWorkerService {
        let mut worker_service = MockWorkerService::new();

        worker_service
            .expect_push_document_job()
            .returning(|_| Box::pin(async move { Ok(()) }));

        worker_service
    }
}
