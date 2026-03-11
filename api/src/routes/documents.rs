use std::sync::Arc;

use aide::axum::{
    routing::{delete_with, get_with, post_with},
    ApiRouter,
};
use apalis::prelude::MessageQueue;
use axum::{
    body::Body,
    extract::{Json, Multipart, Path, Query, State},
    http::StatusCode,
    response::Response,
};
use schemars::JsonSchema;
use serde::Serialize;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    bot_service::{ComhairleDocument, GetQueryParams, UploadFileRequest},
    error::ComhairleError,
    models::{
        conversation,
        job::{self, CreateJob},
    },
    routes::auth::RequiredAdminUser,
    workers::process_documents::DocumentJob,
    ComhairleState,
};

#[instrument(err(Debug), skip(state))]
async fn list(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    Query(params): Query<GetQueryParams>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<Vec<ComhairleDocument>>), ComhairleError> {
    let bot_service = state.required_bot_service()?;

    let knowledge_base_id = get_knowledge_base_id(&state, &conversation_id).await?;

    let (_, documents) = bot_service
        .list_documents(&knowledge_base_id, Some(params))
        .await?;

    Ok((StatusCode::OK, Json(documents)))
}

#[instrument(err(Debug), skip(state))]
async fn get(
    State(state): State<Arc<ComhairleState>>,
    Path((conversation_id, document_id)): Path<(Uuid, String)>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<ComhairleDocument>), ComhairleError> {
    let bot_service = state.required_bot_service()?;

    let knowledge_base_id = get_knowledge_base_id(&state, &conversation_id).await?;

    let (_, document) = bot_service
        .get_document(&document_id, &knowledge_base_id)
        .await?;

    Ok((StatusCode::OK, Json(document)))
}

#[instrument(err(Debug), skip(state))]
async fn delete(
    State(state): State<Arc<ComhairleState>>,
    Path((conversation_id, document_id)): Path<(Uuid, String)>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<StatusCode, ComhairleError> {
    let bot_service = state.required_bot_service()?;

    let knowledge_base_id = get_knowledge_base_id(&state, &conversation_id).await?;

    let _ = bot_service
        .delete_document(document_id, knowledge_base_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[instrument(err(Debug), skip(state))]
async fn parse_document(
    State(state): State<Arc<ComhairleState>>,
    Path((conversation_id, document_id)): Path<(Uuid, String)>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<StatusCode, ComhairleError> {
    let bot_service = state.required_bot_service()?;

    let knowledge_base_id = get_knowledge_base_id(&state, &conversation_id).await?;

    let _ = bot_service
        .parse_document(document_id, knowledge_base_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[instrument(err(Debug), skip(state))]
async fn stop_parsing_document(
    State(state): State<Arc<ComhairleState>>,
    Path((conversation_id, document_id)): Path<(Uuid, String)>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<StatusCode, ComhairleError> {
    let bot_service = state.required_bot_service()?;

    let knowledge_base_id = get_knowledge_base_id(&state, &conversation_id).await?;

    let _ = bot_service
        .stop_parsing_document(document_id, knowledge_base_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[instrument(err(Debug), skip(state))]
async fn download_document(
    State(state): State<Arc<ComhairleState>>,
    Path((conversation_id, document_id)): Path<(Uuid, String)>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<Response<Body>, ComhairleError> {
    let bot_service = state.required_bot_service()?;

    let knowledge_base_id = get_knowledge_base_id(&state, &conversation_id).await?;
    let download_stream = bot_service
        .download_document(document_id, knowledge_base_id)
        .await?;

    let status = download_stream.status();
    let headers = download_stream.headers().clone();

    if !status.is_success() {
        return Err(ComhairleError::DownloadError(
            "Unable to download document: {document_id}".to_string(),
        ));
    }

    let mut response = Response::new(Body::from_stream(download_stream.bytes_stream()));

    *response.status_mut() = status;
    *response.headers_mut() = headers;

    Ok(response)
}

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
    let bot_service = state.required_bot_service()?;

    let knowledge_base_id = get_knowledge_base_id(&state, &conversation_id).await?;

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
    let (_, document) = bot_service
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

async fn get_knowledge_base_id(
    state: &Arc<ComhairleState>,
    conversation_id: &Uuid,
) -> Result<String, ComhairleError> {
    let conversation = conversation::get_by_id(&state.db, conversation_id).await?;
    let knowledge_base_id = match conversation.knowledge_base_id {
        Some(id) => id,
        None => {
            return Err(ComhairleError::CorruptedData(format!(
                "Missing knowledge_base_id on conversation {}",
                conversation.id
            )))
        }
    };

    Ok(knowledge_base_id)
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(list, |op| {
                op.id("ListDocuments")
                    .tag("Documents")
                    .summary("Get a list of documents from a conversation's knowledge base")
                    .security_requirement("JWT")
                    .response::<200, Json<Vec<ComhairleDocument>>>()
            }),
        )
        .api_route(
            "/{document_id}",
            get_with(get, |op| {
                op.id("GetDocument")
                    .tag("Documents")
                    .summary("Get a document from a conversation's knowledge base by id")
                    .security_requirement("JWT")
                    .response::<200, Json<ComhairleDocument>>()
            }),
        )
        .api_route(
            "/{document_id}",
            delete_with(delete, |op| {
                op.id("DeleteDocument")
                    .tag("Documents")
                    .summary("Delete a document from a conversation's knowledge base")
                    .security_requirement("JWT")
                    .response::<204, ()>()
            }),
        )
        .api_route(
            "/{document_id}/parse",
            post_with(parse_document, |op| {
                op.id("ParseDocument")
                    .tag("Documents")
                    .summary("Begin parsing a document")
                    .security_requirement("JWT")
                    .response::<204, ()>()
            }),
        )
        .api_route(
            "/{document_id}/stop_parse",
            post_with(stop_parsing_document, |op| {
                op.id("StopParsingDocument")
                    .tag("Documents")
                    .summary("Stop parsing a document")
                    .security_requirement("JWT")
                    .response::<204, ()>()
            }),
        )
        .api_route(
            "/{document_id}/download",
            get_with(download_document, |op| {
                op.id("DownloadDocument")
                    .tag("Documents")
                    .summary("Download a document")
                    .security_requirement("JWT")
                    .response::<204, Response<Body>>()
            }),
        )
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
    use axum::{body::Body, http::StatusCode, Router};
    use mockall::predicate::eq;
    use serde_json::json;
    use sqlx::PgPool;
    use std::error::Error;
    use std::sync::Arc;

    fn mock_bot_service_for_conversation(kb_id: String) -> MockComhairleBotService {
        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_create_knowledge_base()
            .returning(move |_, _| {
                let kb_id = kb_id.clone();
                Box::pin(async move {
                    Ok((
                        StatusCode::CREATED,
                        ComhairleKnowledgeBase {
                            id: kb_id,
                            ..Default::default()
                        },
                    ))
                })
            });
        bot_service.expect_create_chat().returning(|_| {
            Box::pin(async move {
                Ok((
                    StatusCode::CREATED,
                    ComhairleChat {
                        id: "chat-123".to_string(),
                        ..Default::default()
                    },
                ))
            })
        });

        bot_service
    }

    fn build_bot_service<F>(configure: F, kb_id: String) -> MockComhairleBotService
    where
        F: FnOnce(&mut MockComhairleBotService),
    {
        let mut bot_service = mock_bot_service_for_conversation(kb_id);
        configure(&mut bot_service);
        bot_service
    }

    async fn setup_test_app_with_conversation<F>(
        pool: PgPool,
        kb_id: String,
        configure_bot_service: F,
    ) -> Result<(Router, UserSession, String), Box<dyn Error>>
    where
        F: FnOnce(&mut MockComhairleBotService),
    {
        let bot_service = build_bot_service(configure_bot_service, kb_id.clone());
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
                    "knowledge_base_id": kb_id,
                    "chat_bot_id": "123"
                }),
            )
            .await?;

        let id = conversation["id"].as_str().unwrap().to_string();

        Ok((app, session, id))
    }

    #[sqlx::test]
    async fn should_return_document_list(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let document = ComhairleDocument {
            id: "456".to_string(),
            name: "test_document".to_string(),
            ..Default::default()
        };
        let params = GetQueryParams {
            page: Some(2),
            ..Default::default()
        };
        let kb_id = "kb-123".to_string();

        let (app, mut session, conversation_id) =
            setup_test_app_with_conversation(pool, kb_id.clone(), |bot_service| {
                bot_service
                    .expect_list_documents()
                    .once()
                    .with(eq(kb_id), eq(Some(params)))
                    .returning(move |_, _| {
                        Box::pin({
                            let document = document.clone();
                            async move { Ok((StatusCode::OK, vec![document.clone()])) }
                        })
                    });
            })
            .await?;

        let (status, value, _) = session
            .get(
                &app,
                &format!("/conversation/{conversation_id}/documents?page=2"),
            )
            .await?;
        let json: Vec<ComhairleDocument> = serde_json::from_value(value)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(json[0].id, "456".to_string(), "incorrect json response");

        Ok(())
    }

    #[sqlx::test]
    async fn should_return_single_document(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let document = ComhairleDocument {
            id: "doc-456".to_string(),
            name: "test_document".to_string(),
            ..Default::default()
        };
        let kb_id = "kb-123".to_string();

        let (app, mut session, conversation_id) =
            setup_test_app_with_conversation(pool, kb_id.clone(), |bot_service| {
                bot_service
                    .expect_get_document()
                    .once()
                    .with(eq("doc-456"), eq(kb_id))
                    .returning(move |_, _| {
                        Box::pin({
                            let document = document.clone();
                            async move { Ok((StatusCode::OK, document.clone())) }
                        })
                    });
            })
            .await?;

        let (status, response, _) = session
            .get(
                &app,
                &format!("/conversation/{conversation_id}/documents/doc-456"),
            )
            .await?;
        let json: ComhairleDocument = serde_json::from_value(response)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(json.id, "doc-456".to_string(), "incorrect json response");

        Ok(())
    }

    #[sqlx::test]
    async fn should_delete_document(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let kb_id = "kb-123".to_string();
        let (app, mut session, conversation_id) =
            setup_test_app_with_conversation(pool, kb_id.clone(), |bot_service| {
                bot_service
                    .expect_delete_document()
                    .once()
                    .with(eq("doc-456".to_string()), eq(kb_id))
                    .returning(|_, _| Box::pin(async move { Ok(StatusCode::OK) }));
            })
            .await?;

        let (status, _, _) = session
            .delete(
                &app,
                &format!("/conversation/{conversation_id}/documents/doc-456"),
            )
            .await?;

        assert!(status.is_success(), "error response status");

        Ok(())
    }

    #[sqlx::test]
    async fn should_upload_a_document(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let upload_request = UploadFileRequest {
            filename: "test.txt".to_string(),
            bytes: b"test multipart".to_vec(),
        };
        let kb_id = "kb-123".to_string();

        let (app, mut session, conversation_id) =
            setup_test_app_with_conversation(pool, kb_id, |bot_service| {
                bot_service
                    .expect_upload_document()
                    .once()
                    .with(eq("kb-123"), eq(upload_request))
                    .returning(|_, _| {
                        Box::pin(async move {
                            Ok((
                                StatusCode::OK,
                                ComhairleDocument {
                                    id: "kb-123".to_string(),
                                    name: "test_doc".to_string(),
                                    ..Default::default()
                                },
                            ))
                        })
                    });
            })
            .await?;

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
        assert_eq!(id, "kb-123".to_string(), "incorrect json response");
        assert_eq!(name, "test_doc".to_string(), "incorrect json response");

        Ok(())
    }
}
