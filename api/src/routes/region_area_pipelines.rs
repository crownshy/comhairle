use std::sync::Arc;

use aide::axum::ApiRouter;
use aide::axum::routing::{get_with, post_with, put_with};
use axum::extract::{Json, Multipart, Path, Query, State};
use axum::http::StatusCode;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ComhairleState;
use crate::error::ComhairleError;
use crate::models::region_area_pipeline::reconcile::ReconciliationCounts;
use crate::models::region_area_pipeline::source::store_upload;
use crate::models::region_area_pipeline::tables::Pipeline;
use crate::models::region_area_pipeline::{PipelineDefinition, lifecycle};
use crate::routes::auth::RequiredSuperAdminUser;

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct UploadSourceResponse {
    pub upload_id: Uuid,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct DeletePipelineOptions {
    /// Also delete the pipeline's geographic areas and the Regions created for them.
    #[serde(default)]
    pub delete_areas: bool,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct DeletePipelineResponse {
    pub deleted_areas: usize,
}

async fn list(
    State(state): State<Arc<ComhairleState>>,
    RequiredSuperAdminUser(_user): RequiredSuperAdminUser,
) -> Result<Json<Vec<Pipeline>>, ComhairleError> {
    Ok(Json(lifecycle::list(&state.db).await?))
}

async fn create(
    State(state): State<Arc<ComhairleState>>,
    RequiredSuperAdminUser(_user): RequiredSuperAdminUser,
    Json(definition): Json<PipelineDefinition>,
) -> Result<(StatusCode, Json<Pipeline>), ComhairleError> {
    let pipeline = lifecycle::create(&state.db, &definition).await?;
    Ok((StatusCode::CREATED, Json(pipeline)))
}

async fn replace_draft(
    State(state): State<Arc<ComhairleState>>,
    RequiredSuperAdminUser(_user): RequiredSuperAdminUser,
    Path(pipeline_id): Path<Uuid>,
    Json(definition): Json<PipelineDefinition>,
) -> Result<Json<Pipeline>, ComhairleError> {
    Ok(Json(
        lifecycle::replace_draft(&state.db, pipeline_id, &definition).await?,
    ))
}

async fn get_preview(
    State(state): State<Arc<ComhairleState>>,
    RequiredSuperAdminUser(_user): RequiredSuperAdminUser,
    Path(pipeline_id): Path<Uuid>,
) -> Result<Json<Option<ReconciliationCounts>>, ComhairleError> {
    Ok(Json(lifecycle::get_preview(&state.db, pipeline_id).await?))
}

async fn preview(
    State(state): State<Arc<ComhairleState>>,
    RequiredSuperAdminUser(_user): RequiredSuperAdminUser,
    Path(pipeline_id): Path<Uuid>,
) -> Result<Json<ReconciliationCounts>, ComhairleError> {
    Ok(Json(
        lifecycle::preview(
            &state.db,
            pipeline_id,
            state.bulk_storage_service.as_deref(),
        )
        .await?,
    ))
}

async fn activate(
    State(state): State<Arc<ComhairleState>>,
    RequiredSuperAdminUser(_user): RequiredSuperAdminUser,
    Path(pipeline_id): Path<Uuid>,
) -> Result<Json<Pipeline>, ComhairleError> {
    state.required_worker_service()?;
    Ok(Json(
        lifecycle::activate(
            &state.db,
            pipeline_id,
            state.bulk_storage_service.as_deref(),
        )
        .await?,
    ))
}

async fn deactivate(
    State(state): State<Arc<ComhairleState>>,
    RequiredSuperAdminUser(_user): RequiredSuperAdminUser,
    Path(pipeline_id): Path<Uuid>,
) -> Result<Json<Pipeline>, ComhairleError> {
    Ok(Json(lifecycle::deactivate(&state.db, pipeline_id).await?))
}

async fn delete(
    State(state): State<Arc<ComhairleState>>,
    RequiredSuperAdminUser(_user): RequiredSuperAdminUser,
    Path(pipeline_id): Path<Uuid>,
    Query(options): Query<DeletePipelineOptions>,
) -> Result<Json<DeletePipelineResponse>, ComhairleError> {
    let deleted_areas = lifecycle::delete(&state.db, pipeline_id, options.delete_areas).await?;
    Ok(Json(DeletePipelineResponse { deleted_areas }))
}

async fn upload_source(
    State(state): State<Arc<ComhairleState>>,
    RequiredSuperAdminUser(_user): RequiredSuperAdminUser,
    mut form_data: Multipart,
) -> Result<(StatusCode, Json<UploadSourceResponse>), ComhairleError> {
    let storage = state.required_bulk_storage_service()?;
    let mut field = form_data
        .next_field()
        .await?
        .ok_or_else(|| ComhairleError::BadRequest("Upload one GeoJSON file".to_string()))?;
    if field.name() != Some("file") || field.file_name().is_none() {
        return Err(ComhairleError::BadRequest(
            "Upload one GeoJSON file using the file field".to_string(),
        ));
    }

    let mut bytes = Vec::new();
    while let Some(chunk) = field.chunk().await? {
        bytes.extend_from_slice(&chunk);
    }
    drop(field);
    if form_data.next_field().await?.is_some() {
        return Err(ComhairleError::BadRequest(
            "Upload only one GeoJSON file".to_string(),
        ));
    }
    let upload_id = store_upload(storage.as_ref(), bytes).await?;
    Ok((
        StatusCode::CREATED,
        Json(UploadSourceResponse { upload_id }),
    ))
}

pub fn router() -> ApiRouter<Arc<ComhairleState>> {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(list, |operation| {
                operation
                    .id("ListRegionAreaPipelines")
                    .tag("Region Area Pipelines")
                    .security_requirement("JWT")
                    .summary("List Region Area Pipelines")
                    .response::<200, Json<Vec<Pipeline>>>()
            })
            .post_with(create, |operation| {
                operation
                    .id("CreateRegionAreaPipeline")
                    .tag("Region Area Pipelines")
                    .security_requirement("JWT")
                    .summary("Create a draft Region Area Pipeline")
                    .response::<201, Json<Pipeline>>()
            }),
        )
        .api_route(
            "/{pipeline_id}",
            put_with(replace_draft, |operation| {
                operation
                    .id("ReplaceRegionAreaPipelineDraft")
                    .tag("Region Area Pipelines")
                    .security_requirement("JWT")
                    .summary("Replace a pipeline draft definition")
                    .response::<200, Json<Pipeline>>()
            })
            .delete_with(delete, |operation| {
                operation
                    .id("DeleteRegionAreaPipeline")
                    .tag("Region Area Pipelines")
                    .security_requirement("JWT")
                    .summary("Delete a pipeline, optionally with its geographic areas")
                    .response::<200, Json<DeletePipelineResponse>>()
            }),
        )
        .api_route(
            "/{pipeline_id}/preview",
            get_with(get_preview, |operation| {
                operation
                    .id("GetRegionAreaPipelinePreview")
                    .tag("Region Area Pipelines")
                    .security_requirement("JWT")
                    .summary("Get the latest pipeline preview")
                    .response::<200, Json<Option<ReconciliationCounts>>>()
            })
            .post_with(preview, |operation| {
                operation
                    .id("PreviewRegionAreaPipeline")
                    .tag("Region Area Pipelines")
                    .security_requirement("JWT")
                    .summary("Preview a pipeline draft without publishing")
                    .response::<200, Json<ReconciliationCounts>>()
            }),
        )
        .api_route(
            "/{pipeline_id}/activate",
            post_with(activate, |operation| {
                operation
                    .id("ActivateRegionAreaPipeline")
                    .tag("Region Area Pipelines")
                    .security_requirement("JWT")
                    .summary("Publish the current preview and activate the pipeline")
                    .response::<200, Json<Pipeline>>()
            }),
        )
        .api_route(
            "/{pipeline_id}/deactivate",
            post_with(deactivate, |operation| {
                operation
                    .id("DeactivateRegionAreaPipeline")
                    .tag("Region Area Pipelines")
                    .security_requirement("JWT")
                    .summary("Deactivate a pipeline")
                    .response::<200, Json<Pipeline>>()
            }),
        )
        .api_route(
            "/uploads",
            post_with(upload_source, |operation| {
                operation
                    .id("UploadRegionAreaPipelineSource")
                    .tag("Region Area Pipelines")
                    .security_requirement("JWT")
                    .summary("Upload a private GeoJSON pipeline source")
                    .response::<201, Json<UploadSourceResponse>>()
            }),
        )
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use axum::body::Body;
    use sqlx::PgPool;

    use super::*;
    use crate::bulk_storage_service::{MockBulkStorageService, UploadResult};
    use crate::models::model_test_helpers::setup_app_and_session_with_state;
    use crate::test_helpers::{TEST_PASSWORD, UserSession, test_state};

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn only_super_admins_can_upload_private_sources(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let mut storage = MockBulkStorageService::new();
        storage
            .expect_upload_file()
            .once()
            .returning(|key, bytes, metadata| {
                assert!(key.starts_with("region-area-pipelines/uploads/"));
                assert_eq!(bytes, b"example");
                assert!(!metadata.is_public);
                Box::pin(async { Ok(UploadResult { url: String::new() }) })
            });
        let state = Arc::new(
            test_state()
                .db(pool)
                .bulk_storage_service(Arc::new(storage))
                .call()?,
        );
        let (app, mut admin) = setup_app_and_session_with_state(state).await?;
        let mut user = UserSession::new("regular", TEST_PASSWORD, "regular@example.com");
        user.signup(&app).await?;
        let body = || {
            concat!(
                "--test-boundary\r\n",
                "Content-Disposition: form-data; name=\"file\"; filename=\"source.geojson\"\r\n",
                "Content-Type: application/geo+json\r\n\r\n",
                "example\r\n--test-boundary--\r\n"
            )
        };
        let (status, _, _) = user
            .post_multipart(
                &app,
                "/region_area_pipelines/uploads",
                "test-boundary",
                Body::from(body()),
            )
            .await?;
        assert_eq!(status, StatusCode::FORBIDDEN);
        let (status, response, _) = admin
            .post_multipart(
                &app,
                "/region_area_pipelines/uploads",
                "test-boundary",
                Body::from(body()),
            )
            .await?;
        assert_eq!(status, StatusCode::CREATED, "{response}");
        let response: UploadSourceResponse = serde_json::from_value(response)?;
        assert_ne!(response.upload_id, Uuid::nil());
        Ok(())
    }
}
