use std::sync::Arc;

use aide::axum::{
    ApiRouter,
    routing::{delete_with, get_with, post_with},
};
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
    ComhairleState,
    bot_service::{ComhairleDocument, GetQueryParams, UploadFileRequest},
    error::ComhairleError,
    models::{
        conversation,
        job::{self, CreateJob},
        translations, user_participation, workflow, workflow_step,
    },
    routes::auth::{OptionalUser, RequiredAdminUser, is_user_admin},
    tools::{
        ToolConfig,
        learn::{LearnPageEntry, PageContent},
    },
    worker_service::process_documents::DocumentJob,
};

/// Reserved knowledge-base document name for the auto-synced learn-step content.
///
/// A conversation has at most one of these in its knowledge base; a re-sync
/// replaces it
pub const LEARN_CONTENT_DOCUMENT_NAME: &str = "learn-step-content.md";

/// Not sure if this is the desired behaviour. I made a few assumptions:
/// - The user owns the conversation
/// - The user is a participant in any workflow of the conversation
/// - No user is logged in but the conversation is public and live
#[instrument(err(Debug), skip(state))]
async fn require_conversation_document_access(
    state: &Arc<ComhairleState>,
    user: &OptionalUser,
    conversation_id: &Uuid,
) -> Result<(), ComhairleError> {
    let conversation = conversation::get_by_id(&state.db, conversation_id).await?;

    if conversation.is_public && conversation.is_live {
        return Ok(());
    }

    let Some(ref user) = user.0 else {
        return Err(ComhairleError::UserNotAuthorized);
    };

    if is_user_admin(&state, user).await {
        return Ok(());
    }

    if conversation.owner_id == user.id {
        return Ok(());
    }

    let participant_ids =
        user_participation::get_participant_user_ids_for_conversation(&state.db, conversation_id)
            .await?;

    if participant_ids.contains(&user.id) {
        return Ok(());
    }

    Err(ComhairleError::UserNotAuthorized)
}

#[instrument(err(Debug), skip(state))]
async fn list(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    Query(params): Query<GetQueryParams>,
    user: OptionalUser,
) -> Result<(StatusCode, Json<Vec<ComhairleDocument>>), ComhairleError> {
    require_conversation_document_access(&state, &user, &conversation_id).await?;
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
    user: OptionalUser,
) -> Result<Response<Body>, ComhairleError> {
    require_conversation_document_access(&state, &user, &conversation_id).await?;
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
    let worker_service = state.required_worker_service()?;

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
    worker_service.push_document_job(worker_job).await?;

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
            )));
        }
    };

    Ok(knowledge_base_id)
}

/// Resolve a single learn page entry to its markdown text in the given locale.
///
/// Modern pages reference `text_content`; legacy pages carry inline markdown.
/// Returns `None` when the page has no text for this locale.
async fn resolve_page_content(
    db: &sqlx::PgPool,
    entry: &LearnPageEntry,
    locale: &str,
) -> Result<Option<String>, ComhairleError> {
    match entry {
        LearnPageEntry::TextContent(page) => {
            Ok(
                translations::get_text_translation_optional(db, &page.text_content_id, locale)
                    .await?
                    .map(|translation| translation.content),
            )
        }
        LearnPageEntry::Legacy(localized_pages) => {
            let content = localized_pages
                .iter()
                .find(|page| page.lang == locale)
                .or_else(|| localized_pages.first())
                .map(|page| match &page.content {
                    PageContent::Markdown(markdown) => markdown.clone(),
                });
            Ok(content)
        }
    }
}

/// Gather every learn step's authored content for a conversation into a single
/// markdown document, with per-step headings and page separators.
///
/// Only the live (published) `tool_config` is synced, so the assistant knows
/// exactly what a participant can read. Returns `None` when there is nothing to
/// sync (no learn steps, or none with resolvable content).
#[instrument(err(Debug), skip(state))]
async fn build_learn_content_document(
    state: &Arc<ComhairleState>,
    conversation_id: &Uuid,
) -> Result<Option<String>, ComhairleError> {
    let conversation = conversation::get_by_id(&state.db, conversation_id).await?;
    let locale = &conversation.primary_locale;

    let workflows = workflow::list(&state.db, *conversation_id, None).await?;

    let mut sections: Vec<String> = Vec::new();

    for workflow in workflows {
        let steps = workflow_step::list(&state.db, &workflow.id).await?;
        for step in steps {
            let Some(ToolConfig::Learn(config)) = step.tool_config.as_ref() else {
                continue;
            };

            let heading =
                translations::get_text_translation_optional(&state.db, &step.name, locale)
                    .await?
                    .map(|translation| translation.content)
                    .filter(|content| !content.trim().is_empty())
                    .unwrap_or_else(|| "Untitled step".to_string());

            let mut pages: Vec<String> = Vec::new();
            for entry in &config.pages {
                if let Some(text) = resolve_page_content(&state.db, entry, locale).await? {
                    let trimmed = text.trim();
                    if !trimmed.is_empty() {
                        pages.push(trimmed.to_string());
                    }
                }
            }

            if pages.is_empty() {
                continue;
            }

            sections.push(format!("# {heading}\n\n{}", pages.join("\n\n")));
        }
    }

    if sections.is_empty() {
        return Ok(None);
    }

    Ok(Some(sections.join("\n\n---\n\n")))
}

#[derive(Serialize, JsonSchema, Debug)]
pub struct SyncLearningContentResponse {
    message: String,
    job_id: Option<Uuid>,
    document: Option<ComhairleDocument>,
}

/// Rebuild the conversation's learn-content knowledge-base document from the
/// current learn-step content and kick off a re-parse.
///
/// The reserved-name document is deleted first (RAGFlow re-chunks on parse, so
/// we replace rather than update in place) and the same background parse job as
/// a normal upload connects the chat bot once parsing completes.
#[instrument(err(Debug), skip(state))]
async fn sync_learning_content(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<SyncLearningContentResponse>), ComhairleError> {
    let bot_service = state.required_bot_service()?;
    let worker_service = state.required_worker_service()?;

    let knowledge_base_id = get_knowledge_base_id(&state, &conversation_id).await?;

    // Drop any previously-synced learn-content document so retrieval never
    // serves stale chunks.
    let (_, existing) = bot_service
        .list_documents(
            &knowledge_base_id,
            Some(GetQueryParams {
                name: Some(LEARN_CONTENT_DOCUMENT_NAME.to_string()),
                ..Default::default()
            }),
        )
        .await?;
    for document in existing
        .into_iter()
        .filter(|document| document.name == LEARN_CONTENT_DOCUMENT_NAME)
    {
        bot_service
            .delete_document(document.id, knowledge_base_id.clone())
            .await?;
    }

    let Some(content) = build_learn_content_document(&state, &conversation_id).await? else {
        return Ok((
            StatusCode::OK,
            Json(SyncLearningContentResponse {
                message: "No learn-step content to sync".to_string(),
                job_id: None,
                document: None,
            }),
        ));
    };

    let file = UploadFileRequest {
        filename: LEARN_CONTENT_DOCUMENT_NAME.to_string(),
        bytes: content.into_bytes(),
    };
    let (_, document) = bot_service
        .upload_document(&knowledge_base_id, file)
        .await?;

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
    worker_service.push_document_job(worker_job).await?;

    Ok((
        StatusCode::OK,
        Json(SyncLearningContentResponse {
            message: "Learn content sync started".to_string(),
            job_id: Some(job.id),
            document: Some(document),
        }),
    ))
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
            "/sync_learning_content",
            post_with(sync_learning_content, |op| {
                op.id("SyncLearningContent")
                    .tag("Documents")
                    .summary(
                        "Rebuild the conversation's learn-step content knowledge-base document",
                    )
                    .security_requirement("JWT")
                    .response::<200, Json<SyncLearningContentResponse>>()
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
    use axum::{Router, body::Body, http::StatusCode};
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

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn sync_learning_content_no_learn_steps_is_noop(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let kb_id = "kb-123".to_string();
        let (app, mut session, conversation_id) =
            setup_test_app_with_conversation(pool, kb_id, |bot_service| {
                // No reserved document exists yet.
                bot_service
                    .expect_list_documents()
                    .once()
                    .returning(|_, _| Box::pin(async move { Ok((StatusCode::OK, Vec::new())) }));
                // A conversation with no learn steps has nothing to upload.
                bot_service.expect_upload_document().never();
            })
            .await?;

        let (status, value, _) = session
            .post(
                &app,
                &format!("/conversation/{conversation_id}/documents/sync_learning_content"),
                Body::empty(),
            )
            .await?;

        assert!(status.is_success(), "sync should succeed: {status}");
        assert!(
            value.get("job_id").map(|v| v.is_null()).unwrap_or(false),
            "no job should be created when there is nothing to sync: {value}"
        );

        Ok(())
    }

    fn document_list_returning(bot_service: &mut MockComhairleBotService, kb_id: String) {
        bot_service
            .expect_list_documents()
            .with(eq(kb_id), eq(Some(GetQueryParams::default())))
            .returning(|_, _| {
                Box::pin(async move {
                    Ok((
                        StatusCode::OK,
                        vec![ComhairleDocument {
                            id: "doc-1".to_string(),
                            name: "doc".to_string(),
                            ..Default::default()
                        }],
                    ))
                })
            });
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn anon_can_list_documents_when_public_and_live(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let kb_id = "kb-123".to_string();
        let (app, mut admin, conversation_id) =
            setup_test_app_with_conversation(pool, kb_id.clone(), |bs| {
                document_list_returning(bs, kb_id.clone());
            })
            .await?;

        // Flip conversation to public (default seeded as private + live).
        admin
            .update_conversation(&app, &conversation_id, json!({ "is_public": true }))
            .await?;

        let mut anon = UserSession::new_anon();
        let (status, _, _) = anon
            .get(&app, &format!("/conversation/{conversation_id}/documents"))
            .await?;

        assert!(
            status.is_success(),
            "anon should access public+live docs: {status}"
        );
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn anon_forbidden_when_conversation_private(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let kb_id = "kb-123".to_string();
        let (app, _admin, conversation_id) = setup_test_app_with_conversation(pool, kb_id, |_bs| {
            // list_documents must NOT be invoked when access is denied.
        })
        .await?;

        let mut anon = UserSession::new_anon();
        let (status, _, _) = anon
            .get(&app, &format!("/conversation/{conversation_id}/documents"))
            .await?;

        assert_eq!(
            status,
            StatusCode::FORBIDDEN,
            "anon must be denied on private conv"
        );
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn non_participant_user_forbidden(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let kb_id = "kb-123".to_string();
        let (app, _admin, conversation_id) = setup_test_app_with_conversation(pool, kb_id, |_bs| {
            // list_documents must NOT be invoked when access is denied.
        })
        .await?;

        let mut outsider = UserSession::new(
            "outsider",
            crate::test_helpers::TEST_PASSWORD,
            "outsider@example.com",
        );
        outsider.signup(&app).await?;

        let (status, _, _) = outsider
            .get(&app, &format!("/conversation/{conversation_id}/documents"))
            .await?;

        assert_eq!(
            status,
            StatusCode::FORBIDDEN,
            "non-admin non-participant must be denied"
        );
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn participant_can_list_documents(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let kb_id = "kb-123".to_string();
        let (app, mut admin, conversation_id) =
            setup_test_app_with_conversation(pool, kb_id.clone(), |bs| {
                document_list_returning(bs, kb_id.clone());
            })
            .await?;

        // Admin sets up a workflow the participant can register on.
        let (_, workflow, _) = admin.create_random_workflow(&app, &conversation_id).await?;
        let workflow_id = workflow["id"].as_str().unwrap().to_string();

        let mut participant = UserSession::new(
            "participant",
            crate::test_helpers::TEST_PASSWORD,
            "participant@example.com",
        );
        participant.signup(&app).await?;
        participant
            .post(
                &app,
                &format!("/conversation/{conversation_id}/workflow/{workflow_id}/register"),
                Body::empty(),
            )
            .await?;

        let (status, _, _) = participant
            .get(&app, &format!("/conversation/{conversation_id}/documents"))
            .await?;

        assert!(
            status.is_success(),
            "participant should list docs: {status}"
        );
        Ok(())
    }
}
