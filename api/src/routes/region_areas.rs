use std::sync::Arc;

use aide::axum::{
    ApiRouter,
    routing::{delete_with, get_with, post_with, put_with},
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    ComhairleState,
    error::ComhairleError,
    models::region_area::{self, CreateRegionArea, PartialRegionArea},
    routes::{auth::RequiredAdminUser, region_areas::dto::RegionAreaDto},
};

pub mod dto;

async fn create_region_area(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(create_request): Json<CreateRegionArea>,
) -> Result<(StatusCode, Json<RegionAreaDto>), ComhairleError> {
    let area = region_area::create(&state.db, &create_request)
        .await?
        .into();
    Ok((StatusCode::CREATED, Json(area)))
}

async fn list_region_areas(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<Vec<RegionAreaDto>>), ComhairleError> {
    let areas = region_area::list(&state.db)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok((StatusCode::OK, Json(areas)))
}

async fn get_region_area(
    State(state): State<Arc<ComhairleState>>,
    Path(region_area_id): Path<Uuid>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<RegionAreaDto>), ComhairleError> {
    let area = region_area::get_by_id(&state.db, &region_area_id)
        .await?
        .into();
    Ok((StatusCode::OK, Json(area)))
}

async fn update_region_area(
    State(state): State<Arc<ComhairleState>>,
    Path(region_area_id): Path<Uuid>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(update_request): Json<PartialRegionArea>,
) -> Result<(StatusCode, Json<RegionAreaDto>), ComhairleError> {
    let area = region_area::update(&state.db, &region_area_id, &update_request)
        .await?
        .into();
    Ok((StatusCode::OK, Json(area)))
}

async fn delete_region_area(
    State(state): State<Arc<ComhairleState>>,
    Path(region_area_id): Path<Uuid>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<RegionAreaDto>), ComhairleError> {
    let area = region_area::delete(&state.db, &region_area_id)
        .await?
        .into();
    Ok((StatusCode::OK, Json(area)))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
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
                    .response::<200, Json<Vec<RegionAreaDto>>>()
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
        .with_state(state)
}
