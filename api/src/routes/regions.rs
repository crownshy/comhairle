use std::sync::Arc;

use aide::axum::{
    routing::{delete_with, get_with, post_with, put_with},
    ApiRouter,
};
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::{
        pagination::{PageOptions, PaginatedResults},
        region::{self, CreateRegion, PartialRegion, RegionOrderOptions},
    },
    routes::{
        auth::{RequiredAdminUser, RequiredUser},
        regions::dto::{LocalizedRegionDto, RegionDto},
        translations::LocaleExtractor,
    },
    ComhairleState,
};

pub mod dto;

#[instrument(err(Debug), skip(state))]
async fn list(
    State(state): State<Arc<ComhairleState>>,
    Query(order_options): Query<RegionOrderOptions>,
    Query(page_options): Query<PageOptions>,
    LocaleExtractor(locale): LocaleExtractor,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<PaginatedResults<LocalizedRegionDto>>), ComhairleError> {
    let regions = region::list(&state.db, page_options, order_options, &locale)
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

#[instrument(err(Debug), skip(state))]
async fn delete(
    State(state): State<Arc<ComhairleState>>,
    Path(region_id): Path<Uuid>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<RegionDto>), ComhairleError> {
    let region = region::delete(&state.db, &region_id).await?.into();

    Ok((StatusCode::OK, Json(region)))
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
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use sqlx::PgPool;
    use std::error::Error;

    use crate::models::{model_test_helpers::setup_default_app_and_session, region::RegionType};

    use super::*;

    #[sqlx::test]
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

    #[sqlx::test]
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

    #[sqlx::test]
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

    #[sqlx::test]
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

    #[sqlx::test]
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

    #[sqlx::test]
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
}
