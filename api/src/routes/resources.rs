use std::sync::Arc;

use aide::axum::{
    routing::{get_with, post_with},
    ApiRouter,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::{
        self,
        resource::{
            get_signed_upload_url, CreateResource, ResourceResponse, ResourceUploadResponse,
        },
    },
    ComhairleState,
};

use super::auth::RequiredUser;

async fn get_resource(
    State(state): State<Arc<ComhairleState>>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<ResourceResponse>), ComhairleError> {
    let resource = models::resource::get(&state.db, id).await?;
    let res = resource
        .to_resource_response(&state.s3_client, &state.config.resource_bucket)
        .await?;
    Ok((StatusCode::OK, Json(res)))
}

async fn create_resource(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Json(new_resource): Json<CreateResource>,
) -> Result<(StatusCode, Json<ResourceResponse>), ComhairleError> {
    let resource = models::resource::create_resource(&state.db, new_resource, user.id).await?;
    let resource_response = resource
        .to_resource_response(&state.s3_client, &state.config.resource_bucket)
        .await?;
    Ok((StatusCode::CREATED, Json(resource_response)))
}

async fn upload_request(
    State(state): State<Arc<ComhairleState>>,
) -> Result<(StatusCode, Json<ResourceUploadResponse>), ComhairleError> {
    let id = Uuid::new_v4();
    let url = get_signed_upload_url(
        &id.to_string(),
        &state.s3_client,
        &state.config.resource_bucket,
    )
    .await?;
    Ok((
        StatusCode::CREATED,
        Json(ResourceUploadResponse { url, id }),
    ))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/resource/{id}",
            get_with(get_resource, |op| {
                op.id("GetResource").description("Get resource by id")
            }),
        )
        .api_route(
            "/resource",
            post_with(create_resource, |op| {
                op.id("GetResource").description("Get resource by id")
            }),
        )
        .api_route(
            "/upload_request",
            post_with(upload_request, |op| {
                op.id("UploadRequst")
                    .description("Request an upload url for a resource")
            }),
        )
        .with_state(state)
}
