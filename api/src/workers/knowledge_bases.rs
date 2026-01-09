use std::{sync::Arc, time::Duration};

use apalis::prelude::Data;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::{self, conversation::PartialConversation, job::UpdateJob},
    routes::{bot::chats::UpdateChatRequest, conversations::UploadFileRequest},
    ComhairleState,
};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct KnowledgeBaseJob {
    pub job_id: Uuid,
    pub conversation_id: Uuid,
    pub chat_bot_id: String,
    pub step: String,
    pub documents: Vec<UploadFileRequest>,
}

pub async fn handle_knowledge_base_processing(
    mut job: KnowledgeBaseJob,
    state: Data<Arc<ComhairleState>>,
) -> Result<(), ComhairleError> {
    info!(
        job_id = %job.job_id,
        "Starting knowledge base processing job"
    );

    // TODO: logs at each step
    // TODO: make idempotent (allow for retries restarts)

    let conversation =
        models::conversation::get_localised_by_id(&state.db, &job.conversation_id).await?;

    // 1. create new `knowledge_base`
    info!(
        job_id = %job.job_id,
        conversation_id = %job.conversation_id,
        "Creating new knowledge base for conversation"
    );

    let (_, knowledge_base) = state
        .bot_service
        .create_knowledge_base(conversation.title, None)
        .await?;

    // 2. connect `conversation` to newly created `knowledge_base`
    info!(
        job_id = %job.job_id,
        conversation_id = %job.conversation_id,
        "Connecting knowledge_base to conversation"
    );

    let update_conversation = PartialConversation {
        knowledge_base_id: Some(Some(knowledge_base.id.clone())),
        ..Default::default()
    };
    let _conversation =
        models::conversation::update(&state.db, &job.conversation_id, &update_conversation).await?;

    // 3. upload document to newly created `knowledge_base`
    info!(
        job_id = %job.job_id,
        conversation_id = %job.conversation_id,
        knowledge_base_id = %knowledge_base.id,
        "Uploading documents to knowledge_base"
    );
    let (_, documents) = state
        .bot_service
        .upload_documents(&knowledge_base.id, job.documents)
        .await?;

    let max_attempts = 60; // TODO:
    let poll_interval = Duration::from_secs(10);

    let mut attempts = 0;

    // 4. poll document until full parsed
    info!(
        job_id = %job.job_id,
        document_id = %documents[0].id,
        "Polling document for parse status"
    );
    loop {
        attempts += 1;

        let (_, document) = state
            .bot_service
            .get_document(&documents[0].id, &knowledge_base.id)
            .await?;
        info!(
            job_id = %job.job_id,
            document_id = %documents[0].id,
            parse_progess = %document.parse_progress,
            "Poll document status"
        );

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
                status: Some(message.to_string()),
                ..Default::default()
            };
            let _ = models::job::update(&state.db, &job.job_id, update_job).await?;

            return Err(ComhairleError::BackgroundJobFailed(message.to_string()));
        }

        sleep(poll_interval).await;
    }

    // 5. point chat to new knowledge base
    info!(
        job_id = %job.job_id,
        chat_id = %job.chat_bot_id,
        "Connecting conversation chat bot to knowledge base"
    );

    let update_params = UpdateChatRequest {
        knowledge_base_ids: Some(vec![
            state.config.default_knowledge_base_id.clone(),
            knowledge_base.id.to_string(),
        ]),
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

    // 6. update conversation `enable_qa_chat_bot`
    info!(
        job_id = %job.job_id,
        conversation_id = %job.conversation_id,
        "Conversation chat bot and knowledge base linked"
    );
    let update_conversation = PartialConversation {
        enable_qa_chat_bot: Some(true),
        ..Default::default()
    };
    let conversation =
        models::conversation::update(&state.db, &job.conversation_id, &update_conversation).await?;

    info!(
        job_id = %job.job_id,
        "Job completed successfully"
    );

    let update_job = UpdateJob {
        status: Some("completed".to_string()),
        finished_at: Some(Utc::now()),
        completion_message: Some("Successfully created knowledge base with parsed document for conversation and connected to chat bot".to_string()),
        ..Default::default()
    };
    let _ = models::job::update(&state.db, &job.job_id, update_job).await?;

    // TODO: notify user of completion via webhooks?

    Ok(())
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct KBJob {
    job_id: Uuid,
    conversation_id: Uuid,
}

// TODO: rename
pub async fn handle_kb_processing_alt(
    job: KBJob,
    state: Data<Arc<ComhairleState>>,
) -> Result<(), ComhairleError> {
    let conversation =
        models::conversation::get_localised_by_id(&state.db, &job.conversation_id).await?;

    let chat_id = match conversation.chat_bot_id {
        Some(id) => id,
        None => {
            return Err(ComhairleError::CorruptedData(format!(
                "Missing chat_bot_id on conversation {}",
                conversation.id
            )))
        }
    };
    let kb_id = match conversation.knowledge_base_id {
        Some(id) => id,
        None => {
            return Err(ComhairleError::CorruptedData(format!(
                "Missing knowledge_base_id on conversation {}",
                conversation.id
            )))
        }
    };
    let (_, chat) = state.bot_service.get_chat(&chat_id).await?;
    let (_, knowledge_base) = state.bot_service.get_knowledge_base(&kb_id).await?;
    let (_, kb_documents) = state
        .bot_service
        .list_documents(&knowledge_base.id, None)
        .await?;

    if kb_documents.is_empty() {
        // Upload default document to knowledge_base
    }

    let (_, kb_documents) = state
        .bot_service
        .list_documents(&knowledge_base.id, None)
        .await?;

    let has_parsed_document = kb_documents
        .iter()
        .any(|doc| doc.parse_status == "DONE" && doc.parse_progress == 1.0);

    if !has_parsed_document {
        let unparsed_document = kb_documents
            .iter()
            .find(|doc| doc.parse_progress < 1.0 && doc.parse_status == "RUNNING");

        if let Some(document) = unparsed_document {
            // Poll document
            // Requires document_id from previous step to persist if job is stopped or retried
        } else {
            return Err(ComhairleError::BackgroundJobFailed(
                "Partially parsed document not found".to_string(),
            ));
        }
    }

    if !chat.knowledge_base_ids.contains(&kb_id) {
        // connect chat to knowledge base
    }

    if !conversation.enable_qa_chat_bot {
        // enable flag on conversation
    }

    Ok(())
}
