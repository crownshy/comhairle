use std::{sync::Arc, time::Duration};

use apalis::prelude::Data;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    bot_service::ComhairleDocument,
    error::ComhairleError,
    routes::{bot::chats::UpdateChatRequest, conversations::UploadFileRequest},
    workers::JobMetadata,
    ComhairleState,
};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct KnowledgeBaseJob {
    pub job_id: Uuid,
    pub conversation_id: Uuid,
    pub knowledge_base_id: String,
    pub chat_bot_id: String,
    pub metadata: JobMetadata,
}

const DEFAULT_DOCUMENT: &[u8] = include_bytes!("knowledge-base-welcome.txt");

async fn upload_default_document(
    state: &Arc<ComhairleState>,
    knowledge_base_id: &str,
) -> Result<(StatusCode, Vec<ComhairleDocument>), ComhairleError> {
    let file = UploadFileRequest {
        filename: "knowledge-base-welcome.txt".to_string(),
        bytes: DEFAULT_DOCUMENT.to_vec(),
    };

    state
        .bot_service
        .upload_documents(knowledge_base_id, vec![file])
        .await
}

pub async fn handle_knowledge_base_processing(
    job: KnowledgeBaseJob,
    state: Data<Arc<ComhairleState>>,
) -> Result<(), ComhairleError> {
    info!(
        job_id = %job.job_id,
        "Starting knowledge base processing job"
    );

    // TODO: Mark job as running

    let (_, documents) = upload_default_document(&state, &job.knowledge_base_id).await?;
    let default_document_id = &documents[0].id;

    info!(
        job_id = %job.job_id,
        document_id = default_document_id,
        "Default document uploaded and parsing begun"
    );

    let max_attempts = 60; // TODO:
    let poll_interval = Duration::from_secs(10);

    let mut attempts = 0;

    loop {
        attempts += 1;

        let (_, document) = state
            .bot_service
            .get_document(default_document_id, &job.knowledge_base_id)
            .await?;

        if document.parse_status == "DONE" && document.parse_progress >= 1.0 {
            info!(
                job_id = %job.job_id,
                document_id = %default_document_id,
                "Default document parsing complete"
            );

            break;
        }

        if attempts >= max_attempts {
            let message = "Document parsing timed out";
            error!(job_id = %job.job_id, "{message}");

            // TODO: mark job as errored

            return Err(ComhairleError::BackgroundJobFailed(message.to_string()));
        }

        sleep(poll_interval).await;
    }

    info!(
        job_id = %job.job_id,
        chat_id = %job.chat_bot_id,
        "Connecting conversation chat bot to updated knowledge base"
    );

    let update_params = UpdateChatRequest {
        knowledge_base_ids: Some(vec![job.knowledge_base_id.to_string()]),
        ..Default::default()
    };

    let _ = state
        .bot_service
        .update_chat(&job.chat_bot_id, update_params)
        .await?;

    info!(
        job_id = %job.job_id,
        chat_id = %job.chat_bot_id,
        "Conversation chat bot and knowledge base linked"
    );

    info!(
        job_id = %job.job_id,
        "Job completed successfully"
    );

    Ok(())
}
