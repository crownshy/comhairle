use std::sync::Arc;

use aide::axum::{
    ApiRouter,
    routing::{delete_with, get_with, patch_with, post_with, put_with},
};
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    ComhairleState,
    error::ComhairleError,
    models::{
        pagination::{PageOptions, PaginatedResults},
        region::{self, CreateRegion, PartialRegion, RegionFilterOptions, RegionOrderOptions},
        region_area,
    },
    routes::{
        auth::{RequiredAdminUser, RequiredUser},
        regions::dto::{
            LocalizedRegionDto, RegionAreaLinksDto, RegionAreaLinksRequestDto, RegionDto,
        },
        translations::LocaleExtractor,
    },
};

pub mod dto;

#[instrument(err(Debug), skip(state))]
async fn list(
    State(state): State<Arc<ComhairleState>>,
    Query(order_options): Query<RegionOrderOptions>,
    Query(filter_options): Query<RegionFilterOptions>,
    Query(page_options): Query<PageOptions>,
    LocaleExtractor(locale): LocaleExtractor,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<PaginatedResults<LocalizedRegionDto>>), ComhairleError> {
    let regions = region::list(
        &state.db,
        page_options,
        filter_options,
        order_options,
        &locale,
    )
    .await?
    .into();

    Ok((StatusCode::OK, Json(regions)))
}

#[instrument(err(Debug), skip(state))]
async fn get(
    State(state): State<Arc<ComhairleState>>,
    Path(region_id): Path<Uuid>,
    RequiredUser(user): RequiredUser,
    LocaleExtractor(locale): LocaleExtractor,
) -> Result<(StatusCode, Json<LocalizedRegionDto>), ComhairleError> {
    let region = region::get_localized_by_id(&state.db, &region_id, &locale)
        .await?
        .into();

    Ok((StatusCode::OK, Json(region)))
}

#[instrument(err(Debug), skip(state))]
async fn create(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(_user): RequiredAdminUser,
    LocaleExtractor(locale): LocaleExtractor,
    Json(payload): Json<CreateRegion>,
) -> Result<(StatusCode, Json<RegionDto>), ComhairleError> {
    let region = region::create(&state.db, &payload, &locale).await?.into();

    Ok((StatusCode::CREATED, Json(region)))
}

#[instrument(err(Debug), skip(state))]
async fn update(
    State(state): State<Arc<ComhairleState>>,
    Path(region_id): Path<Uuid>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(region): Json<PartialRegion>,
) -> Result<(StatusCode, Json<RegionDto>), ComhairleError> {
    let region = region::update(&state.db, &region_id, &region).await?.into();

    Ok((StatusCode::OK, Json(region)))
}

/// Get the region's `metadata` jsonb column.
#[instrument(err(Debug), skip(state))]
async fn get_region_metadata(
    State(state): State<Arc<ComhairleState>>,
    Path(region_id): Path<Uuid>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<Option<serde_json::Value>>), ComhairleError> {
    let metadata = region::get_metadata(&state.db, &region_id).await?;

    Ok((StatusCode::OK, Json(metadata)))
}

/// Shallow-merge the request body into the region's `metadata` jsonb column.
#[instrument(err(Debug), skip(state))]
async fn patch_region_metadata(
    State(state): State<Arc<ComhairleState>>,
    Path(region_id): Path<Uuid>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(patch): Json<serde_json::Value>,
) -> Result<(StatusCode, Json<RegionDto>), ComhairleError> {
    let region = region::patch_metadata(&state.db, &region_id, patch).await?;
    let region: RegionDto = region.into();

    Ok((StatusCode::OK, Json(region)))
}

#[instrument(err(Debug), skip(state))]
async fn delete(
    State(state): State<Arc<ComhairleState>>,
    Path(region_id): Path<Uuid>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<RegionDto>), ComhairleError> {
    let region = region::delete(&state.db, &region_id).await?.into();

    Ok((StatusCode::OK, Json(region)))
}

#[instrument(err(Debug), skip(state))]
async fn get_area_links(
    State(state): State<Arc<ComhairleState>>,
    Path(region_id): Path<Uuid>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<RegionAreaLinksDto>), ComhairleError> {
    let _ = region::get_by_id(&state.db, &region_id).await?;
    let area_ids = region::list_area_ids(&state.db, &region_id).await?;

    Ok((
        StatusCode::OK,
        Json(RegionAreaLinksDto {
            region_id,
            area_ids,
        }),
    ))
}

#[instrument(err(Debug), skip(state))]
async fn set_area_links(
    State(state): State<Arc<ComhairleState>>,
    Path(region_id): Path<Uuid>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(payload): Json<RegionAreaLinksRequestDto>,
) -> Result<(StatusCode, Json<RegionAreaLinksDto>), ComhairleError> {
    let _ = region::get_by_id(&state.db, &region_id).await?;
    for area_id in &payload.area_ids {
        let _ = region_area::get_by_id(&state.db, area_id).await?;
    }

    region::set_area_links(&state.db, &region_id, &payload.area_ids).await?;
    let area_ids = region::list_area_ids(&state.db, &region_id).await?;

    Ok((
        StatusCode::OK,
        Json(RegionAreaLinksDto {
            region_id,
            area_ids,
        }),
    ))
}

#[instrument(err(Debug), skip(state))]
async fn add_area_link(
    State(state): State<Arc<ComhairleState>>,
    Path((region_id, area_id)): Path<(Uuid, Uuid)>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<RegionAreaLinksDto>), ComhairleError> {
    let _ = region::get_by_id(&state.db, &region_id).await?;
    let _ = region_area::get_by_id(&state.db, &area_id).await?;

    region::add_area_link(&state.db, &region_id, &area_id).await?;
    let area_ids = region::list_area_ids(&state.db, &region_id).await?;

    Ok((
        StatusCode::OK,
        Json(RegionAreaLinksDto {
            region_id,
            area_ids,
        }),
    ))
}

#[instrument(err(Debug), skip(state))]
async fn remove_area_link(
    State(state): State<Arc<ComhairleState>>,
    Path((region_id, area_id)): Path<(Uuid, Uuid)>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<RegionAreaLinksDto>), ComhairleError> {
    let _ = region::get_by_id(&state.db, &region_id).await?;
    let _ = region_area::get_by_id(&state.db, &area_id).await?;

    region::remove_area_link(&state.db, &region_id, &area_id).await?;
    let area_ids = region::list_area_ids(&state.db, &region_id).await?;

    Ok((
        StatusCode::OK,
        Json(RegionAreaLinksDto {
            region_id,
            area_ids,
        }),
    ))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(list, |op| {
                op.id("ListRegions")
                    .tag("Regions")
                    .summary("List of regions")
                    .description("Paginated list of regions with optional ordering")
                    .security_requirement("JWT")
                    .response::<200, Json<PaginatedResults<LocalizedRegionDto>>>()
            }),
        )
        .api_route(
            "/{region_id}",
            get_with(get, |op| {
                op.id("GetRegion")
                    .tag("Regions")
                    .summary("Get a region by id")
                    .description("Get a region by id")
                    .security_requirement("JWT")
                    .response::<200, Json<LocalizedRegionDto>>()
            }),
        )
        .api_route(
            "/",
            post_with(create, |op| {
                op.id("CreateRegion")
                    .tag("Regions")
                    .summary("Create a new region")
                    .description("Create a new region")
                    .security_requirement("JWT")
                    .response::<201, Json<RegionDto>>()
            }),
        )
        .api_route(
            "/{region_id}",
            put_with(update, |op| {
                op.id("UpdateRegion")
                    .tag("Regions")
                    .summary("Update a region")
                    .description("Update a region")
                    .security_requirement("JWT")
                    .response::<200, Json<RegionDto>>()
            }),
        )
        .api_route(
            "/{region_id}/metadata",
            get_with(get_region_metadata, |op| {
                op.id("GetRegionMetadata")
                    .tag("Regions")
                    .summary("Get region metadata")
                    .description("Get region metadata")
                    .security_requirement("JWT")
                    .response::<200, Json<Option<serde_json::Value>>>()
            }),
        )
        .api_route(
            "/{region_id}/metadata",
            patch_with(patch_region_metadata, |op| {
                op.id("PatchRegionMetadata")
                    .tag("Regions")
                    .summary("Shallow-merge region metadata")
                    .description(
                        "Merge a JSON object into region.metadata at the top level using jsonb concatenation",
                    )
                    .security_requirement("JWT")
                    .response::<200, Json<RegionDto>>()
            }),
        )
        .api_route(
            "/{region_id}",
            delete_with(delete, |op| {
                op.id("DeleteRegion")
                    .tag("Regions")
                    .summary("Delete a region")
                    .description("Delete a region")
                    .security_requirement("JWT")
                    .response::<200, Json<RegionDto>>()
            }),
        )
        .api_route(
            "/{region_id}/areas",
            get_with(get_area_links, |op| {
                op.id("GetRegionAreaLinks")
                    .tag("Regions")
                    .summary("List region area links")
                    .description("List region area links")
                    .security_requirement("JWT")
                    .response::<200, Json<RegionAreaLinksDto>>()
            }),
        )
        .api_route(
            "/{region_id}/areas",
            put_with(set_area_links, |op| {
                op.id("SetRegionAreaLinks")
                    .tag("Regions")
                    .summary("Replace region area links")
                    .description("Replace region area links")
                    .security_requirement("JWT")
                    .response::<200, Json<RegionAreaLinksDto>>()
            }),
        )
        .api_route(
            "/{region_id}/areas/{area_id}",
            post_with(add_area_link, |op| {
                op.id("AddRegionAreaLink")
                    .tag("Regions")
                    .summary("Add region area link")
                    .description("Add region area link")
                    .security_requirement("JWT")
                    .response::<200, Json<RegionAreaLinksDto>>()
            }),
        )
        .api_route(
            "/{region_id}/areas/{area_id}",
            delete_with(remove_area_link, |op| {
                op.id("RemoveRegionAreaLink")
                    .tag("Regions")
                    .summary("Remove region area link")
                    .description("Remove region area link")
                    .security_requirement("JWT")
                    .response::<200, Json<RegionAreaLinksDto>>()
            }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use sqlx::PgPool;
    use std::error::Error;

    use crate::models::region::RegionType;
    use crate::models::{model_test_helpers::setup_default_app_and_session, region_area};

    use super::*;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_a_region(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;

        let new_region = CreateRegion {
            name: "test_region".to_string(),
            description: "test_desc".to_string(),
            region_type: RegionType::Official,
            ..Default::default()
        };

        let body = serde_json::to_vec(&new_region)?;
        let (status, response, _) = session.post(&app, "/regions", body.into()).await?;

        let region: RegionDto = serde_json::from_value(response)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(
            region.region_type,
            RegionType::Official,
            "incorrect org_type"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_a_region_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;

        let (_, response, _) = session.create_random_region(&app).await?;
        let region: RegionDto = serde_json::from_value(response)?;

        let (status, response, _) = session
            .get(&app, &format!("/regions/{}", region.id))
            .await?;
        let region: LocalizedRegionDto = serde_json::from_value(response)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(
            region.name,
            "test_region".to_string(),
            "incorrect region name"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_list_regions(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;

        let _ = session.create_random_region(&app).await?;
        let _ = session.create_random_region(&app).await?;
        let _ = session.create_random_region(&app).await?;

        let (status, response, _) = session.get(&app, "/regions").await?;
        let regions: PaginatedResults<LocalizedRegionDto> = serde_json::from_value(response)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(regions.total, 3, "incorrect number of regions");
        assert_eq!(
            regions.records[0].name,
            "test_region".to_string(),
            "incorrect region json"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_return_ordered_list_of_regions(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;

        let _ = session
            .create_region(
                &app,
                json!({
                "name": "bar",
                "description": "1",
                "region_type": "custom",
                }),
            )
            .await?;
        let _ = session
            .create_region(
                &app,
                json!({
                "name": "foo",
                "description": "2",
                "region_type": "custom",
                }),
            )
            .await?;
        let _ = session
            .create_region(
                &app,
                json!({
                "name": "baz",
                "description": "3",
                "region_type": "custom",
                }),
            )
            .await?;

        let (_, response, _) = session.get(&app, "/regions?created_at=desc").await?;
        let regions: PaginatedResults<LocalizedRegionDto> = serde_json::from_value(response)?;
        assert_eq!(
            regions.records[0].name,
            "baz".to_string(),
            "incorrect first region [created_at=desc]"
        );
        assert_eq!(
            regions.records[2].name,
            "bar".to_string(),
            "incorrect last region [created_at=desc]"
        );

        let (_, response, _) = session.get(&app, "/regions?name=asc").await?;
        let regions: PaginatedResults<LocalizedRegionDto> = serde_json::from_value(response)?;
        assert_eq!(
            regions.records[0].name,
            "bar".to_string(),
            "incorrect first region [name=asc]"
        );
        assert_eq!(
            regions.records[2].name,
            "foo".to_string(),
            "incorrect last region [name=asc]"
        );

        let (_, response, _) = session.get(&app, "/regions?name=desc").await?;
        let regions: PaginatedResults<LocalizedRegionDto> = serde_json::from_value(response)?;
        assert_eq!(
            regions.records[0].name,
            "foo".to_string(),
            "incorrect first region [name=desc]"
        );
        assert_eq!(
            regions.records[2].name,
            "bar".to_string(),
            "incorrect last region [name=desc]"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_update_a_region(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, response, _) = session.create_random_region(&app).await?;
        let region: RegionDto = serde_json::from_value(response)?;
        assert_eq!(
            region.region_type,
            RegionType::Custom,
            "incorrect region_type before update"
        );

        let update = PartialRegion {
            region_type: Some(RegionType::Official),
            ..Default::default()
        };
        let body = serde_json::to_vec(&update)?;
        let (status, response, _) = session
            .put(&app, &format!("/regions/{}", region.id), body.into())
            .await?;
        let region: RegionDto = serde_json::from_value(response)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(
            region.region_type,
            RegionType::Official,
            "incorrect region_type after update"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_delete_a_region(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;

        let (_, response, _) = session.create_random_region(&app).await?;
        let region: RegionDto = serde_json::from_value(response)?;

        let _ = session
            .delete(&app, &format!("/regions/{}", region.id))
            .await?;

        let (_, response, _) = session
            .get(&app, &format!("/regions/{}", region.id))
            .await?;

        assert_eq!(
            response.get("err").and_then(|v| v.as_str()).unwrap(),
            "Region not found",
            "incorrect error message"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_manage_region_area_links(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;

        let (_, region_res, _) = session.create_random_region(&app).await?;
        let region: RegionDto = serde_json::from_value(region_res)?;

        let (_, area_res_a, _) = session.create_random_region_area(&app).await?;
        let area_a: crate::routes::region_areas::dto::RegionAreaDto =
            serde_json::from_value(area_res_a)?;

        let (_, area_res_b, _) = session.create_random_region_area(&app).await?;
        let area_b: crate::routes::region_areas::dto::RegionAreaDto =
            serde_json::from_value(area_res_b)?;

        let set_body = serde_json::to_vec(&RegionAreaLinksRequestDto {
            area_ids: vec![area_a.id, area_b.id],
        })?;
        let (status, response, _) = session
            .put(
                &app,
                &format!("/regions/{}/areas", region.id),
                set_body.into(),
            )
            .await?;
        let links: RegionAreaLinksDto = serde_json::from_value(response)?;
        assert!(status.is_success(), "error response status");
        assert_eq!(links.area_ids.len(), 2, "incorrect link count after set");

        let (status, response, _) = session
            .delete(&app, &format!("/regions/{}/areas/{}", region.id, area_a.id))
            .await?;
        let links: RegionAreaLinksDto = serde_json::from_value(response)?;
        assert!(status.is_success(), "error response status");
        assert_eq!(links.area_ids.len(), 1, "incorrect link count after remove");
        assert_eq!(links.area_ids[0], area_b.id, "wrong area remains linked");

        let (status, response, _) = session
            .post(
                &app,
                &format!("/regions/{}/areas/{}", region.id, area_a.id),
                "{}".to_string().into(),
            )
            .await?;
        let links: RegionAreaLinksDto = serde_json::from_value(response)?;
        assert!(status.is_success(), "error response status");
        assert_eq!(links.area_ids.len(), 2, "incorrect link count after add");

        let (status, response, _) = session
            .get(&app, &format!("/regions/{}/areas", region.id))
            .await?;
        let links: RegionAreaLinksDto = serde_json::from_value(response)?;
        assert!(status.is_success(), "error response status");
        assert_eq!(links.area_ids.len(), 2, "incorrect link count from get");
        assert!(links.area_ids.contains(&area_a.id), "missing area_a");
        assert!(links.area_ids.contains(&area_b.id), "missing area_b");

        let area_ids = region::list_area_ids(&pool, &region.id).await?;
        assert_eq!(area_ids.len(), 2, "model links not in sync");

        let _ = region_area::get_by_id(&pool, &area_a.id).await?;

        Ok(())
    }
}
