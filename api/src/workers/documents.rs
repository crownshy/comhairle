use std::{sync::Arc, time::Duration};

use apalis::prelude::Data;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::{self, job::UpdateJob},
    routes::{bot::chats::UpdateChatRequest, conversations::UploadFileRequest},
    ComhairleState,
};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct DocumentJob {
    pub job_id: Uuid,
    pub conversation_id: Uuid,
    pub document: UploadFileRequest,
}

/// Background job for RAG system document processing.
///
/// Check's for parse completion of document and subsequently connects conversation's chat bot to
/// its knowledge base if not already connected.
///
/// # Purpose
///
/// A `Conversation` owns both a `knowledge_base_id` (dataset) and a `chat_bot_id` in Ragflow.
/// While both are created when the conversation is initialized, Ragflow does not allow a chat
/// bot to be connected to a knowledge base until the knowledge base contains at least one
/// fully parsed document. Document parsing can take some time.
///
/// This job bridges that gap by waiting for the a document to be fully processed before
/// ensuring the chat bot is correctly connected to the conversation’s knowledge base.
///
/// # Steps
///
/// 1. Uploads a document to the conversation’s knowledge base, triggering parsing in Ragflow.
/// 2. Polls Ragflow to monitor the document’s parsing status.
/// 3. Updates progress on job in postgres to allow front end to poll parse status from job.
/// 4. Once parsing is complete, verifies whether the conversation’s chat bot is connected to the knowledge base.
/// 5. Connects the chat bot to the knowledge base if it is not already connected.
pub async fn handle_document_processing(
    job: DocumentJob,
    state: Data<Arc<ComhairleState>>,
) -> Result<(), ComhairleError> {
    info!(
        job_id = %job.job_id,
        "Starting document processing job"
    );

    let conversation =
        models::conversation::get_localised_by_id(&state.db, &job.conversation_id).await?;
    let knowledge_base_id = match conversation.knowledge_base_id {
        Some(id) => id,
        None => {
            return Err(ComhairleError::BackgroundJobFailed(format!(
                "Missing knowledge_base_id on conversation {}",
                conversation.id
            )))
        }
    };
    let chat_bot_id = match conversation.chat_bot_id {
        Some(id) => id,
        None => {
            return Err(ComhairleError::BackgroundJobFailed(format!(
                "Missing chat_bot_id on conversation {}",
                conversation.id
            )))
        }
    };

    info!(
        job_id = %job.job_id,
        conversation_id = %job.conversation_id,
        knowledge_base_id = %knowledge_base_id,
        "Uploading document to knowledge_base"
    );
    let (_, documents) = state
        .bot_service
        .upload_documents(&knowledge_base_id, vec![job.document])
        .await?;

    let max_attempts = 120; // 20 minutes
    let poll_interval = Duration::from_secs(10);
    let mut attempts = 0;

    info!(
        job_id = %job.job_id,
        document_id = %documents[0].id,
        "Polling document for parse status"
    );
    loop {
        attempts += 1;

        let (_, document) = state
            .bot_service
            .get_document(&documents[0].id, &knowledge_base_id)
            .await?;

        let update_job = UpdateJob {
            progress: Some(document.parse_progress),
            ..Default::default()
        };
        let _job = models::job::update(&state.db, &job.job_id, update_job).await?;

        if document.parse_status == "DONE" && document.parse_progress >= 1.0 {
            info!(
                job_id = %job.job_id,
                document_id = %documents[0].id,
                "Document parsing complete"
            );

            break;
        }

        if attempts >= max_attempts {
            let message = "Document parsing timed out";
            error!(job_id = %job.job_id, "{message}");

            let update_job = UpdateJob {
                status: Some("error".to_string()),
                error: Some(message.to_string()),
                ..Default::default()
            };
            let _ = models::job::update(&state.db, &job.job_id, update_job).await?;

            return Err(ComhairleError::BackgroundJobFailed(message.to_string()));
        }

        sleep(poll_interval).await;
    }

    let (_, chat) = state.bot_service.get_chat(&chat_bot_id).await?;

    if !chat.knowledge_base_ids.contains(&knowledge_base_id) {
        info!(
            job_id = %job.job_id,
            chat_id = %chat_bot_id,
            "Connecting conversation chat bot to knowledge base"
        );

        let update_params = UpdateChatRequest {
            knowledge_base_ids: Some(vec![
                state.config.default_knowledge_base_id.clone(),
                knowledge_base_id.to_string(),
            ]),
            ..Default::default()
        };

        let _ = state
            .bot_service
            .update_chat(&chat_bot_id, update_params)
            .await?;

        info!(
            job_id = %job.job_id,
            chat_id = %chat_bot_id,
            "Conversation chat bot and knowledge base linked"
        );
    }

    info!(
        job_id = %job.job_id,
        "Job completed successfully"
    );

    // update job as complete
    let update_job = UpdateJob {
        status: Some("completed".to_string()),
        finished_at: Some(Utc::now()),
        completion_message: Some("Document successfully parsed".to_string()),
        ..Default::default()
    };
    let _ = models::job::update(&state.db, &job.job_id, update_job).await?;

    Ok(())
}
