use std::sync::Arc;

use aide::axum::{routing::post_with, ApiRouter};
use axum::{
    extract::{Json, Multipart, Path, State},
    http::StatusCode,
    routing::post,
};
use axum_extra::extract::CookieJar;
use ragflow::UploadFile;
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

use crate::{error::ComhairleError, ComhairleState};

#[derive(Deserialize, Debug, JsonSchema)]
struct CreateKnowledgeBaseRequest {
    name: String,
    description: Option<String>,
    permission: Option<String>,
}

#[instrument(err(Debug), skip(state))]
async fn create_knowledgebase(
    State(state): State<Arc<ComhairleState>>,
    jar: CookieJar,
    Json(payload): Json<CreateKnowledgeBaseRequest>,
) -> Result<(CookieJar, StatusCode), ComhairleError> {
    let _result = state
        .bot_service
        .create_knowledgebase(payload.name, payload.description)
        .await?;

    Ok((jar, StatusCode::CREATED))
}

#[instrument(err(Debug), skip(state, form_data))]
async fn upload_documents(
    State(state): State<Arc<ComhairleState>>,
    Path(knowledgebase_id): Path<String>,
    mut form_data: Multipart,
) -> Result<StatusCode, ComhairleError> {
    let mut files: Vec<UploadFile> = Vec::new();

    while let Some(field) = form_data.next_field().await? {
        let filename = field.file_name().unwrap_or("<no filename>").to_string();
        let bytes = field.bytes().await?;

        let file = UploadFile {
            filename,
            bytes: bytes.to_vec(),
        };
        files.push(file);
    }

    let _result = state
        .bot_service
        .upload_documents(&knowledgebase_id, files)
        .await?;

    Ok(StatusCode::OK)
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/create_knowledgebase",
            post_with(create_knowledgebase, |op| {
                op.id("CreateKnowledgeBase")
                    .summary("Create a knowledgebase in RAG system")
                    .response::<201, ()>()
            }),
        )
        .route(
            "/upload_documents/{knowledgebase_id}",
            post(upload_documents),
        )
        .with_state(state)
}
