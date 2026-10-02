use std::sync::Arc;

use aide::axum::{
    ApiRouter,
    routing::{delete_with, get_with, post_with, put_with},
};
use axum::{
    Json,
    extract::{DefaultBodyLimit, Path, Query, State},
    http::StatusCode,
};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;
use uuid::Uuid;

use crate::error::ComhairleError;
use crate::models::region_area::{self, AreaLocation, CreateRegionArea, PartialRegionArea};
use crate::routes::{auth::RequiredSuperAdminUser, region_areas::dto::RegionAreaDto};
use crate::{
    ComhairleState,
    models::pagination::{PageOptions, PaginatedResults},
};

pub mod dto;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ImportRegionAreasRequest {
    pub areas: Vec<CreateRegionArea>,
}

pub use crate::models::region_area::RegionAreaListOptions;

async fn intersecting_region_areas(
    State(state): State<Arc<ComhairleState>>,
    RequiredSuperAdminUser(_user): RequiredSuperAdminUser,
    Query(location): Query<AreaLocation>,
) -> Result<Json<PaginatedResults<RegionAreaDto>>, ComhairleError> {
    let areas = region_area::intersecting(&state.db, &location).await?;
    let area_dtos = PaginatedResults {
        total: areas.len() as i32,
        records: areas.into_iter().map(Into::into).collect(),
    };
    Ok(Json(area_dtos))
}

#[instrument(err(Debug), skip(state))]
async fn create_region_area(
    State(state): State<Arc<ComhairleState>>,
    RequiredSuperAdminUser(_user): RequiredSuperAdminUser,
    Json(create_request): Json<CreateRegionArea>,
) -> Result<(StatusCode, Json<RegionAreaDto>), ComhairleError> {
    let area = region_area::create(&state.db, create_request).await?.into();
    Ok((StatusCode::CREATED, Json(area)))
}

#[instrument(err(Debug), skip(state))]
async fn list_region_areas(
    State(state): State<Arc<ComhairleState>>,
    RequiredSuperAdminUser(_user): RequiredSuperAdminUser,
    Query(options): Query<RegionAreaListOptions>,
    Query(page_options): Query<PageOptions>,
) -> Result<(StatusCode, Json<PaginatedResults<RegionAreaDto>>), ComhairleError> {
    let areas = region_area::list_with_geometry(&state.db, options, page_options).await?;
    let area_dtos = PaginatedResults {
        total: areas.total,
        records: areas.records.into_iter().map(Into::into).collect(),
    };
    Ok((StatusCode::OK, Json(area_dtos)))
}

#[instrument(err(Debug), skip(state))]
async fn get_region_area(
    State(state): State<Arc<ComhairleState>>,
    Path(region_area_id): Path<Uuid>,
    RequiredSuperAdminUser(_user): RequiredSuperAdminUser,
) -> Result<(StatusCode, Json<RegionAreaDto>), ComhairleError> {
    let area = region_area::get_by_id(&state.db, &region_area_id)
        .await?
        .into();
    Ok((StatusCode::OK, Json(area)))
}

#[instrument(err(Debug), skip(state))]
async fn update_region_area(
    State(state): State<Arc<ComhairleState>>,
    Path(region_area_id): Path<Uuid>,
    RequiredSuperAdminUser(_user): RequiredSuperAdminUser,
    Json(update_request): Json<PartialRegionArea>,
) -> Result<(StatusCode, Json<RegionAreaDto>), ComhairleError> {
    let area = region_area::update(&state.db, &region_area_id, &update_request)
        .await?
        .into();
    Ok((StatusCode::OK, Json(area)))
}

#[instrument(err(Debug), skip(state))]
async fn delete_region_area(
    State(state): State<Arc<ComhairleState>>,
    Path(region_area_id): Path<Uuid>,
    RequiredSuperAdminUser(_user): RequiredSuperAdminUser,
) -> Result<(StatusCode, Json<RegionAreaDto>), ComhairleError> {
    let area = region_area::delete(&state.db, &region_area_id)
        .await?
        .into();
    Ok((StatusCode::OK, Json(area)))
}

#[instrument(err(Debug), skip(state, import_request))]
async fn import_region_areas(
    State(state): State<Arc<ComhairleState>>,
    RequiredSuperAdminUser(_user): RequiredSuperAdminUser,
    Json(import_request): Json<ImportRegionAreasRequest>,
) -> Result<(StatusCode, Json<Vec<RegionAreaDto>>), ComhairleError> {
    let areas = region_area::create_many(&state.db, import_request.areas)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();

    Ok((StatusCode::CREATED, Json(areas)))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/intersecting",
            get_with(intersecting_region_areas, |op| {
                op.id("IntersectingRegionAreas")
                    .tag("Region Areas")
                    .security_requirement("JWT")
                    .summary(
                        "Find all region areas intersecting a WGS84 point, including boundaries",
                    )
                    .response::<200, Json<PaginatedResults<RegionAreaDto>>>()
            }),
        )
        .api_route(
            "/import",
            post_with(import_region_areas, |op| {
                op.id("ImportRegionAreas")
                    .tag("Region Areas")
                    .security_requirement("JWT")
                    .summary("Import region areas")
                    .response::<201, Json<Vec<RegionAreaDto>>>()
            }),
        )
        .api_route(
            "/",
            post_with(create_region_area, |op| {
                op.id("CreateRegionArea")
                    .tag("Region Areas")
                    .security_requirement("JWT")
                    .summary("Create a region area")
                    .response::<201, Json<RegionAreaDto>>()
            }),
        )
        .api_route(
            "/",
            get_with(list_region_areas, |op| {
                op.id("ListRegionAreas")
                    .tag("Region Areas")
                    .security_requirement("JWT")
                    .summary("List region areas")
                    .response::<200, Json<PaginatedResults<RegionAreaDto>>>()
            }),
        )
        .api_route(
            "/{region_area_id}",
            get_with(get_region_area, |op| {
                op.id("GetRegionArea")
                    .tag("Region Areas")
                    .security_requirement("JWT")
                    .summary("Get a region area by id")
                    .response::<200, Json<RegionAreaDto>>()
            }),
        )
        .api_route(
            "/{region_area_id}",
            put_with(update_region_area, |op| {
                op.id("UpdateRegionArea")
                    .tag("Region Areas")
                    .security_requirement("JWT")
                    .summary("Update a region area")
                    .response::<200, Json<RegionAreaDto>>()
            }),
        )
        .api_route(
            "/{region_area_id}",
            delete_with(delete_region_area, |op| {
                op.id("DeleteRegionArea")
                    .tag("Region Areas")
                    .security_requirement("JWT")
                    .summary("Delete a region area")
                    .response::<200, Json<RegionAreaDto>>()
            }),
        )
        .layer(DefaultBodyLimit::max(256 * 1024 * 1024))
        .with_state(state)
}
