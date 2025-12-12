use std::sync::Arc;

use aide::axum::{
    routing::{delete_with, get_with, post_with},
    ApiRouter,
};
use axum::{
    extract::{Json, Multipart, Path, Query, State},
    http::StatusCode,
};
use axum_extra::extract::CookieJar;
use ragflow::document::Document;
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

use crate::{error::ComhairleError, routes::bot::GetQueryParams, ComhairleState};

#[instrument(err(Debug), skip(state))]
async fn get(
    State(state): State<Arc<ComhairleState>>,
    Path(knowledge_base_id): Path<String>,
    Query(params): Query<GetQueryParams>, // TODO: should the params be optional here?
) -> Result<(StatusCode, Json<Vec<Document>>), ComhairleError> {
    let (_, documents) = state
        .bot_service
        .get_documents(knowledge_base_id, Some(params))
        .await?;

    Ok((StatusCode::OK, Json(documents)))
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

#[instrument(err(Debug), skip(state))]
async fn upload(
    State(state): State<Arc<ComhairleState>>,
    jar: CookieJar,
    // mut form_data: Multipart, // TODO: multi form data
) -> Result<(CookieJar, StatusCode), ComhairleError> {
    todo!();
    // let mut files: Vec<UploadFile> = Vec::new();
    //
    // while let Some(field) = form_data.next_field().await? {
    //     let filename = field.file_name().unwrap_or("<no filename>").to_string();
    //     let bytes = field.bytes().await?;
    //
    //     let file = UploadFile {
    //         filename,
    //         bytes: bytes.to_vec(),
    //     };
    //     files.push(file);
    // }
    //
    // let _result = state
    //     .bot_service
    //     .upload_documents(&knowledge_base_id, files)
    //     .await?;

    Ok((jar, StatusCode::CREATED))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(get, |op| {
                op.id("GetDocuments")
                    .summary("Get a list of documents from a knowledge base")
                    .response::<200, ()>()
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
        .api_route(
            "/",
            post_with(upload, |op| {
                op.id("UploadDocument")
                    .summary("Upload a document to a knowledge base")
                    .response::<201, ()>()
            }),
        )
        .with_state(state)
}
