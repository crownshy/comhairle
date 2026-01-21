use std::sync::Arc;

use aide::axum::{
    routing::{delete_with, get_with, post_with, put_with},
    ApiRouter,
};
use axum::{
    body::Body,
    extract::{Json, Multipart, Path, Query, State},
    http::{Response, StatusCode},
    routing::post,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{
    bot_service::ComhairleDocument,
    error::ComhairleError,
    routes::{auth::RequiredAdminUser, bot::GetQueryParams},
    ComhairleState,
};

#[instrument(err(Debug), skip(state))]
async fn list(
    State(state): State<Arc<ComhairleState>>,
    Path(knowledge_base_id): Path<String>,
    Query(params): Query<GetQueryParams>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<Vec<ComhairleDocument>>), ComhairleError> {
    let (_, documents) = state
        .bot_service
        .list_documents(&knowledge_base_id, Some(params))
        .await?;

    Ok((StatusCode::OK, Json(documents)))
}

#[instrument(err(Debug), skip(state))]
async fn get(
    State(state): State<Arc<ComhairleState>>,
    Path((knowledge_base_id, document_id)): Path<(String, String)>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<ComhairleDocument>), ComhairleError> {
    let (_, document) = state
        .bot_service
        .get_document(&document_id, &knowledge_base_id)
        .await?;

    Ok((StatusCode::OK, Json(document)))
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Default)]
pub struct UpdateDocumentRequest {
    pub name: Option<String>,
}

#[instrument(err(Debug), skip(state))]
async fn update(
    State(state): State<Arc<ComhairleState>>,
    Path((knowledge_base_id, document_id)): Path<(String, String)>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(payload): Json<UpdateDocumentRequest>,
) -> Result<(StatusCode, Json<ComhairleDocument>), ComhairleError> {
    let (_, document) = state
        .bot_service
        .update_document(&document_id, &knowledge_base_id, payload)
        .await?;

    Ok((StatusCode::OK, Json(document)))
}

#[instrument(err(Debug), skip(state))]
async fn delete(
    State(state): State<Arc<ComhairleState>>,
    Path((knowledge_base_id, document_id)): Path<(String, String)>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<StatusCode, ComhairleError> {
    let _ = state
        .bot_service
        .delete_document(document_id, knowledge_base_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[instrument(err(Debug), skip(state))]
async fn parse_document(
    State(state): State<Arc<ComhairleState>>,
    Path((knowledge_base_id, document_id)): Path<(String, String)>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<StatusCode, ComhairleError> {
    let _ = state
        .bot_service
        .parse_document(document_id, knowledge_base_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[instrument(err(Debug), skip(state))]
async fn stop_parsing_document(
    State(state): State<Arc<ComhairleState>>,
    Path((knowledge_base_id, document_id)): Path<(String, String)>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<StatusCode, ComhairleError> {
    let _ = state
        .bot_service
        .stop_parsing_document(document_id, knowledge_base_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[instrument(err(Debug), skip(state))]
async fn download_document(
    State(state): State<Arc<ComhairleState>>,
    Path((knowledge_base_id, document_id)): Path<(String, String)>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<Response<Body>, ComhairleError> {
    let download_stream = state
        .bot_service
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

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(list, |op| {
                op.id("ListDocuments")
                    .tag("Bot Documents")
                    .summary("Get a list of documents from a knowledge base")
                    .security_requirement("JWT")
                    .response::<200, Json<Vec<ComhairleDocument>>>()
            }),
        )
        .api_route(
            "/{document_id}",
            get_with(get, |op| {
                op.id("GetDocument")
                    .tag("Bot Documents")
                    .summary("Get a documents from a knowledge base by id")
                    .security_requirement("JWT")
                    .response::<200, Json<ComhairleDocument>>()
            }),
        )
        .api_route(
            "/{document_id}",
            put_with(update, |op| {
                op.id("UpdateDocument")
                    .tag("Bot Documents")
                    .summary("Update a document within a knowledge base")
                    .security_requirement("JWT")
                    .response::<200, Json<ComhairleDocument>>()
            }),
        )
        .api_route(
            "/{document_id}",
            delete_with(delete, |op| {
                op.id("DeleteDocument")
                    .tag("Bot Documents")
                    .summary("Delete a document from a knowledge base")
                    .security_requirement("JWT")
                    .response::<204, ()>()
            }),
        )
        .api_route(
            "/{document_id}/parse",
            post_with(parse_document, |op| {
                op.id("ParseDocument")
                    .tag("Bot Documents")
                    .summary("Begin parsing a document")
                    .security_requirement("JWT")
                    .response::<204, ()>()
            }),
        )
        .api_route(
            "/{document_id}/stop_parse",
            post_with(stop_parsing_document, |op| {
                op.id("StopParsingDocument")
                    .tag("Bot Documents")
                    .summary("Stop parsing a document")
                    .security_requirement("JWT")
                    .response::<204, ()>()
            }),
        )
        .api_route(
            "/{document_id}/download",
            get_with(download_document, |op| {
                op.id("DownloadDocument")
                    .tag("Bot Documents")
                    .summary("Download a document")
                    .security_requirement("JWT")
                    .response::<204, Response<Body>>()
            }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot_service::MockComhairleBotService;
    use crate::{
        setup_server,
        test_helpers::{test_state, UserSession},
    };
    use std::error::Error;
    use std::sync::Arc;

    use axum::body::Body;
    use mockall::predicate::eq;
    use sqlx::PgPool;

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

        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_list_documents()
            .once()
            .with(eq("123"), eq(Some(params)))
            .returning(move |_, _| {
                Box::pin({
                    let document = document.clone();
                    async move { Ok((StatusCode::OK, vec![document.clone()])) }
                })
            });

        let state = test_state()
            .db(pool)
            .bot_service(Arc::new(bot_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;
        let (status, value, _) = admin_session
            .get(&app, "/bot/knowledge_bases/123/documents?page=2")
            .await?;
        let json: Vec<ComhairleDocument> = serde_json::from_value(value)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(json[0].id, "456".to_string(), "incorrect json response");

        Ok(())
    }

    #[sqlx::test]
    async fn should_return_single_document(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let document = ComhairleDocument {
            id: "456".to_string(),
            name: "test_document".to_string(),
            ..Default::default()
        };

        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_get_document()
            .once()
            .with(eq("456"), eq("123"))
            .returning(move |_, _| {
                Box::pin({
                    let document = document.clone();
                    async move { Ok((StatusCode::OK, document.clone())) }
                })
            });

        let state = test_state()
            .db(pool)
            .bot_service(Arc::new(bot_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;
        let (status, response, _) = admin_session
            .get(&app, "/bot/knowledge_bases/123/documents/456")
            .await?;
        let json: ComhairleDocument = serde_json::from_value(response)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(json.id, "456".to_string(), "incorrect json response");

        Ok(())
    }

    #[sqlx::test]
    async fn should_update_and_return_document(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let document = ComhairleDocument {
            id: "456".to_string(),
            name: "test_document".to_string(),
            ..Default::default()
        };

        let update_request = UpdateDocumentRequest {
            name: Some("test_document".to_string()),
            ..Default::default()
        };
        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_update_document()
            .once()
            .with(eq("456"), eq("123"), eq(update_request.clone()))
            .returning(move |_, _, _| {
                let document = document.clone();
                Box::pin(async move { Ok((StatusCode::OK, document.clone())) })
            });

        let state = test_state()
            .db(pool)
            .bot_service(Arc::new(bot_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        let bytes = serde_json::to_vec(&update_request)?;
        let body = Body::from(bytes);
        let (status, response, _) = admin_session
            .put(&app, "/bot/knowledge_bases/123/documents/456", body)
            .await?;

        assert!(status.is_success(), "error response status");
        assert_eq!(
            response.get("name").and_then(|v| v.as_str()).unwrap(),
            "test_document",
            "incorrect json response"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_delete_document(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_delete_document()
            .once()
            .with(eq("456".to_string()), eq("123".to_string()))
            .returning(|_, _| Box::pin(async move { Ok(StatusCode::OK) }));

        let state = test_state()
            .db(pool)
            .bot_service(Arc::new(bot_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        let (status, _, _) = admin_session
            .delete(&app, "/bot/knowledge_bases/123/documents/456")
            .await?;

        assert!(status.is_success(), "error response status");

        Ok(())
    }
}
