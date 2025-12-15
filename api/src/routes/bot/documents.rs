use std::sync::Arc;

use aide::axum::{
    routing::{delete_with, get_with, post_with, put_with},
    ApiRouter,
};
use axum::{
    extract::{Json, Multipart, Path, Query, State},
    http::StatusCode,
    routing::post,
};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

use crate::{
    bot_service::ComhairleDocument, error::ComhairleError, routes::bot::GetQueryParams,
    ComhairleState,
};

#[instrument(err(Debug), skip(state))]
async fn list(
    State(state): State<Arc<ComhairleState>>,
    Path(knowledge_base_id): Path<String>,
    Query(params): Query<GetQueryParams>,
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
) -> Result<(StatusCode, Json<ComhairleDocument>), ComhairleError> {
    let (_, document) = state
        .bot_service
        .get_document(&document_id, &knowledge_base_id)
        .await?;

    Ok((StatusCode::OK, Json(document)))
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct UploadFileRequest {
    pub filename: String,
    pub bytes: Vec<u8>,
}

#[instrument(err(Debug), skip(state))]
async fn upload(
    State(state): State<Arc<ComhairleState>>,
    Path(knowledge_base_id): Path<String>,
    mut form_data: Multipart,
) -> Result<StatusCode, ComhairleError> {
    let mut files: Vec<UploadFileRequest> = Vec::new();

    while let Some(field) = form_data.next_field().await? {
        let filename = field.file_name().unwrap_or("<no filename>").to_string();
        let bytes = field.bytes().await?;

        let file = UploadFileRequest {
            filename,
            bytes: bytes.to_vec(),
        };
        files.push(file);
    }

    let _result = state
        .bot_service
        .upload_documents(&knowledge_base_id, files)
        .await?;

    Ok(StatusCode::CREATED)
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct UpdateDocumentRequest {
    pub name: Option<String>,
}

#[instrument(err(Debug), skip(state))]
async fn update(
    State(state): State<Arc<ComhairleState>>,
    Path((knowledge_base_id, document_id)): Path<(String, String)>,
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
) -> Result<StatusCode, ComhairleError> {
    let _ = state
        .bot_service
        .delete_document(document_id, knowledge_base_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(list, |op| {
                op.id("ListDocuments")
                    .summary("Get a list of documents from a knowledge base")
                    .response::<200, Json<Vec<ComhairleDocument>>>()
            }),
        )
        .api_route(
            "/{document_id}",
            get_with(get, |op| {
                op.id("GetDocuments")
                    .summary("Get a documents from a knowledge base by id")
                    .response::<200, Json<ComhairleDocument>>()
            }),
        )
        // .api_route(
        //     "/",
        //     post_with(upload, |op| {
        //         op.id("UploadDocument")
        //             .summary("Upload a document to a knowledge base")
        //             .response::<201, Json<ComhairleDocument>>()
        //     }),
        // )
        .route("/", post(upload))
        .api_route(
            "/{document_id}",
            put_with(update, |op| {
                op.id("UpdateDocument")
                    .summary("Update a document within a knowledge base")
                    .response::<200, Json<ComhairleDocument>>()
            }),
        )
        .api_route(
            "/{document_id}",
            delete_with(delete, |op| {
                op.id("DeleteDocument")
                    .summary("Delete a document from a knowledge base")
                    .response::<204, ()>()
            }),
        )
        .with_state(state)
}
