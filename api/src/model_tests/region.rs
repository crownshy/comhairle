//! Tests for [`crate::models::region`], kept in the api crate because they
//! boot the HTTP app and build fixtures through the API.

mod tests {
    use crate::models::error::DataError;
    use serde_json::json;

    use crate::{
        models::{
            model_test_helpers::setup_default_app_and_session,
            region_area::{self, CreateRegionArea},
        },
        routes::organizations::dto::OrganizationDto,
    };

    #[allow(unused_imports)]
    use crate::models::error::{
        AuthError, ConversationError, EventError, InviteError, ModelError, PermissionError,
        ReportError, UserError, ValidationError, WorkflowError,
    };
    #[allow(unused_imports)]
    use crate::models::moderation_status::ModerationStatus;
    #[allow(unused_imports)]
    use crate::models::pagination::{Order, PageOptions, PaginatedResults};
    use crate::models::region::*;
    #[allow(unused_imports)]
    use crate::models::request_context::{ClientIp, ClientUserAgent};
    #[allow(unused_imports)]
    use crate::models::user_progress::ProgressStatus;
    #[allow(unused_imports)]
    use crate::models::users::User;
    #[allow(unused_imports)]
    use crate::models::{proposal_section, user_progress, users};
    #[allow(unused_imports)]
    use ::std::collections::{HashMap, HashSet};
    #[allow(unused_imports)]
    use chrono::{DateTime, Utc};
    #[allow(unused_imports)]
    use schemars::JsonSchema;
    #[allow(unused_imports)]
    use sqlx::PgPool;
    use std::error::Error;
    #[allow(unused_imports)]
    use uuid::Uuid;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_a_region(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let _ = setup_default_app_and_session(&pool).await?;

        let new_region = CreateRegion {
            name: "Glasgow".to_string(),
            description: "Largest city in Scotland".to_string(),
            region_type: RegionType::Official,
            official_id: Some("G".to_string()),
        };

        let region = create(&pool, &new_region, "en").await?;

        assert_eq!(
            region.region_type,
            RegionType::Official,
            "incorrect region_type"
        );
        assert_eq!(
            region.official_id.unwrap(),
            "G".to_string(),
            "incorrect official_id"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_update_a_region(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let _ = setup_default_app_and_session(&pool).await?;

        let new_region = CreateRegion {
            name: "Glasgow".to_string(),
            description: "Largest city in Scotland".to_string(),
            region_type: RegionType::Official,
            official_id: Some("G".to_string()),
        };

        let region = create(&pool, &new_region, "en").await?;
        assert_eq!(
            region.region_type,
            RegionType::Official,
            "incorrect region after creation"
        );
        assert_eq!(
            region.official_id.unwrap(),
            "G".to_string(),
            "incorrect official_id after creation"
        );

        let update_region = PartialRegion {
            region_type: Some(RegionType::Custom),
            official_id: Some("G1".to_string()),
            ..Default::default()
        };
        let updated_region = update(&pool, &region.id, &update_region).await?;

        assert_eq!(
            updated_region.region_type,
            RegionType::Custom,
            "incorrect region after update"
        );
        assert_eq!(
            updated_region.official_id.unwrap(),
            "G1".to_string(),
            "incorrect official_id after update"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_list_ordered_regions(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let _ = setup_default_app_and_session(&pool).await?;

        let new_region_1 = CreateRegion {
            name: "region_a".to_string(),
            description: "region_a".to_string(),
            ..Default::default()
        };
        let new_region_2 = CreateRegion {
            name: "region_c".to_string(),
            description: "region_c".to_string(),
            ..Default::default()
        };
        let new_region_3 = CreateRegion {
            name: "region_d".to_string(),
            description: "region_d".to_string(),
            ..Default::default()
        };
        let new_region_4 = CreateRegion {
            name: "region_b".to_string(),
            description: "region_b".to_string(),
            ..Default::default()
        };
        let _ = create(&pool, &new_region_1, "en").await?;
        let _ = create(&pool, &new_region_2, "en").await?;
        let _ = create(&pool, &new_region_3, "en").await?;
        let _ = create(&pool, &new_region_4, "en").await?;

        let page_options = PageOptions {
            offset: None,
            limit: None,
        };
        let filter_options = RegionFilterOptions {
            ..Default::default()
        };
        let order_options = RegionOrderOptions {
            created_at: Some(Order::Asc),
            ..Default::default()
        };
        let results = list(&pool, page_options, filter_options, order_options, "en").await?;

        assert_eq!(results.total, 4, "incorrect number of regions");
        assert_eq!(
            results.records[0].name,
            "region_a".to_string(),
            "incorrect first region [created_at: asc]"
        );
        assert_eq!(
            results.records[3].name,
            "region_b".to_string(),
            "incorrect last region [created_at: desc]"
        );

        let page_options = PageOptions {
            offset: None,
            limit: None,
        };
        let filter_options = RegionFilterOptions {
            ..Default::default()
        };
        let order_options = RegionOrderOptions {
            name: Some(Order::Desc),
            ..Default::default()
        };
        let results = list(&pool, page_options, filter_options, order_options, "en").await?;

        assert_eq!(results.total, 4, "incorrect number of regions");
        assert_eq!(
            results.records[0].name,
            "region_d".to_string(),
            "incorrect first region [name: desc]"
        );
        assert_eq!(
            results.records[3].name,
            "region_a".to_string(),
            "incorrect last region [name: desc]"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_list_regions_filtered_by_organization(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let new_region_1 = CreateRegion {
            name: "Glasgow North".to_string(),
            description: "Glasgow North".to_string(),
            region_type: RegionType::Official,
            official_id: Some("G1".to_string()),
        };
        let new_region_2 = CreateRegion {
            name: "Glasgow South".to_string(),
            description: "Glasgow South".to_string(),
            region_type: RegionType::Official,
            official_id: Some("G2".to_string()),
        };
        let new_region_3 = CreateRegion {
            name: "Glasgow East".to_string(),
            description: "Glasgow East".to_string(),
            region_type: RegionType::Official,
            official_id: Some("G3".to_string()),
        };
        let region_1 = create(&pool, &new_region_1, "en").await?;
        let region_2 = create(&pool, &new_region_2, "en").await?;
        let _ = create(&pool, &new_region_3, "en").await?;

        let (_, org_res, _) = session.create_random_organization(&app).await?;
        let organization: OrganizationDto = serde_json::from_value(org_res)?;
        let (status, updated_org_res, _) = session
            .put(
                &app,
                &format!("/organizations/{}", organization.id),
                json!({
                    "regions": vec![region_1.id, region_2.id]
                })
                .to_string()
                .into(),
            )
            .await?;
        assert!(
            status.is_success(),
            "organization update failed: {}",
            updated_org_res
        );
        let page_options = PageOptions {
            limit: None,
            offset: None,
        };
        let order_options = RegionOrderOptions {
            ..Default::default()
        };
        let filter_options = RegionFilterOptions {
            organization_id: Some(organization.id),
        };
        let results = list(&pool, page_options, filter_options, order_options, "en").await?;

        assert_eq!(results.total, 2, "incorrect total");
        assert_eq!(results.records[0].id, region_1.id, "incorrect first id");
        assert_eq!(results.records[1].id, region_2.id, "incorrect second id");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_link_region_to_multiple_areas(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let _ = setup_default_app_and_session(&pool).await?;

        let region = create(
            &pool,
            &CreateRegion {
                name: "Glasgow".to_string(),
                description: "Largest city in Scotland".to_string(),
                region_type: RegionType::Official,
                official_id: Some("G".to_string()),
            },
            "en",
        )
        .await?;

        let area_a = region_area::create(
            &pool,
            CreateRegionArea {
                zip_prefix: "G1".to_string(),
            },
        )
        .await?;
        let area_b = region_area::create(
            &pool,
            CreateRegionArea {
                zip_prefix: "G2".to_string(),
            },
        )
        .await?;

        set_area_links(&pool, &region.id, &[area_a.id, area_b.id]).await?;
        let area_ids = list_area_ids(&pool, &region.id).await?;

        assert_eq!(area_ids.len(), 2, "incorrect number of linked areas");
        assert!(area_ids.contains(&area_a.id), "missing first linked area");
        assert!(area_ids.contains(&area_b.id), "missing second linked area");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_a_localized_region(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let _ = setup_default_app_and_session(&pool).await?;
        let new_region = CreateRegion {
            name: "Glasgow".to_string(),
            description: "Largest city in Scotland".to_string(),
            region_type: RegionType::Official,
            official_id: Some("G".to_string()),
        };

        let region = create(&pool, &new_region, "en").await?;
        let region = get_localized_by_id(&pool, &region.id, "en").await?;

        assert_eq!(region.name, "Glasgow".to_string(), "incorrect name");
        assert_eq!(
            region.description,
            "Largest city in Scotland".to_string(),
            "incorrect description"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_delete_a_region(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let _ = setup_default_app_and_session(&pool).await?;
        let new_region = CreateRegion {
            name: "Glasgow".to_string(),
            description: "Largest city in Scotland".to_string(),
            region_type: RegionType::Official,
            official_id: Some("G".to_string()),
        };

        let region = create(&pool, &new_region, "en").await?;
        let _ = delete(&pool, &region.id).await?;

        let err = get_localized_by_id(&pool, &region.id, "en")
            .await
            .unwrap_err();

        match err {
            ModelError::Data(DataError::ResourceNotFound(e)) => {
                assert_eq!(e, "Region".to_string(), "incorrect error message");
            }
            _ => panic!("Expected ResourceNotFound error"),
        }

        Ok(())
    }
}
