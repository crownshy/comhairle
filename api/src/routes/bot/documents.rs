use std::sync::Arc;

use aide::axum::{routing::post_with, ApiRouter};
use axum::{
    extract::{Json, Multipart, Path, State},
    http::StatusCode,
};
use axum_extra::extract::CookieJar;
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

use crate::{error::ComhairleError, ComhairleState};

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
    //     .upload_documents(&knowledgebase_id, files)
    //     .await?;

    Ok((jar, StatusCode::CREATED))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
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
