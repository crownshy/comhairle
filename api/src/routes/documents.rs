use std::sync::Arc;

use aide::axum::{routing::post_with, ApiRouter};
use apalis::prelude::MessageQueue;
use axum::{
    extract::{Json, Multipart, Path, State},
    http::StatusCode,
};
use schemars::JsonSchema;
use serde::Serialize;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    bot_service::{ComhairleDocument, UploadFileRequest},
    error::ComhairleError,
    models::{
        conversation,
        job::{self, CreateJob},
    },
    routes::auth::RequiredAdminUser,
    workers::process_documents::DocumentJob,
    ComhairleState,
};

#[derive(Serialize, JsonSchema, Debug)]
pub struct UploadFileResponse {
    message: String,
    job_id: Uuid,
    document: ComhairleDocument,
}

#[instrument(err(Debug), skip(state))]
pub async fn upload(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    RequiredAdminUser(_user): RequiredAdminUser,
    mut form_data: Multipart,
) -> Result<(StatusCode, Json<UploadFileResponse>), ComhairleError> {
    let conversation = conversation::get_by_id(&state.db, &conversation_id).await?;
    let knowledge_base_id = match conversation.knowledge_base_id {
        Some(id) => id,
        None => {
            return Err(ComhairleError::CorruptedData(format!(
                "Missing knowledge_base_id on conversation {}",
                conversation.id
            )))
        }
    };

    // Get file data and upload document
    let (filename, bytes) = match form_data.next_field().await? {
        Some(field) => {
            let filename = field.file_name().unwrap_or("<no filename>").to_string();
            let bytes = field.bytes().await?.to_vec();
            (filename, bytes)
        }
        None => return Err(ComhairleError::BadRequest("Missing form field".to_string())),
    };
    if form_data.next_field().await?.is_some() {
        return Err(ComhairleError::BadRequest(
            "Only one document upload allowed".to_string(),
        ));
    }
    let file = UploadFileRequest { filename, bytes };
    let (_, document) = state
        .bot_service
        .upload_document(&knowledge_base_id, file)
        .await?;

    // Create background job for parsing
    let create_job = CreateJob {
        progress: Some(0.0),
        ..Default::default()
    };
    let job = job::create(&state.db, create_job).await?;
    let worker_job = DocumentJob {
        job_id: job.id,
        conversation_id,
        document_id: document.id.clone(),
    };
    let mut lock = state.jobs.process_documents.lock().await;
    lock.enqueue(worker_job)
        .await
        .map_err(|_| ComhairleError::BackgroundJobFailedToQueue)?;

    let json = UploadFileResponse {
        message: "Document parsing moved to background job".to_string(),
        job_id: job.id,
        document,
    };

    Ok((StatusCode::OK, Json(json)))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(upload, |op| {
                op.tag("Documents")
                    .description(
                        "⚠️ This endpoint requires multipart/form-data.\n\n\
                            Generated API clients may not support file uploads.\n\n\
                            Use FormData and a raw HTTP request.\n\n\
                            **Example (curl):**\n\
```bash
curl -X POST \\
-H 'Cookie: auth-token=...;' \\
'localhost:3000/conversation/__CONVERSATION_ID__/upload_documents' \\
--form 'file=@/path-to-document.pdf'
```
                            ",
                    )
                    .summary(
                        "Upload a document to a conversation's knowledge base and begin parsing",
                    )
                    .security_requirement("JWT")
                    .response::<200, Json<UploadFileResponse>>()
            }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::bot_service::{ComhairleChat, ComhairleKnowledgeBase, MockComhairleBotService};
    use crate::test_helpers::test_state;
    use crate::{setup_server, test_helpers::UserSession};
    use axum::{body::Body, http::StatusCode};
    use mockall::predicate::eq;
    use serde_json::json;
    use sqlx::PgPool;
    use std::error::Error;
    use std::sync::Arc;

    #[sqlx::test]
    async fn should_upload_a_document(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let upload_request = UploadFileRequest {
            filename: "test.txt".to_string(),
            bytes: b"test multipart".to_vec(),
        };
        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_create_knowledge_base()
            .once()
            .returning(|_, _| {
                Box::pin(async move {
                    Ok((
                        StatusCode::CREATED,
                        ComhairleKnowledgeBase {
                            id: "123".to_string(),
                            ..Default::default()
                        },
                    ))
                })
            });
        bot_service.expect_create_chat().once().returning(|_| {
            Box::pin(async move {
                Ok((
                    StatusCode::CREATED,
                    ComhairleChat {
                        id: "123".to_string(),
                        ..Default::default()
                    },
                ))
            })
        });
        bot_service
            .expect_upload_document()
            .once()
            .with(eq("123"), eq(upload_request))
            .returning(|_, _| {
                Box::pin(async move {
                    Ok((
                        StatusCode::OK,
                        ComhairleDocument {
                            id: "123".to_string(),
                            name: "test_doc".to_string(),
                            ..Default::default()
                        },
                    ))
                })
            });

        let state = test_state()
            .db(pool)
            .bot_service(Arc::new(bot_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_admin();
        session.signup(&app).await?;
        let (_, conversation, _) = session
            .create_conversation(
                &app,
                json!({
                    "title" : "Test conversation",
                    "short_description" : "A test conversation",
                    "description" : "A longer description",
                    "image_url" : "http://someimage.png",
                    "tags" : ["one", "two", "three"],
                    "is_public" : false,
                    "is_live": true,
                    "is_invite_only" : false,
                    "slug" : "new_conversation",
                    "primary_locale" : "en",
                    "supported_languages" : ["en"],
                    "knowledge_base_id": "123",
                    "chat_bot_id": "123"
                }),
            )
            .await?;
        let conversation_id = conversation.get("id").and_then(|v| v.as_str()).unwrap();

        let boundary = "test-boundary";
        let body = format!(
            "--{boundary}\r\n\
            Content-Disposition: form-data; name=\"file\"; filename=\"test.txt\"\r\n\
            Content-Type: text/plain\r\n\
            \r\n\
            test multipart\r\n\
            --{boundary}--\r\n"
        );
        let body = Body::from(body);

        let (status, value, _) = session
            .post_multipart(
                &app,
                &format!("/conversation/{conversation_id}/documents"),
                boundary,
                body,
            )
            .await?;

        let document = value.get("document").unwrap().to_owned();
        let id = document.get("id").and_then(|v| v.as_str()).unwrap();
        let name = document.get("name").and_then(|v| v.as_str()).unwrap();

        assert!(status.is_success(), "error response status");
        assert_eq!(id, "123".to_string(), "incorrect json response");
        assert_eq!(name, "test_doc".to_string(), "incorrect json response");

        Ok(())
    }
}
