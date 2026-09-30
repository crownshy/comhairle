//! Tests for [`crate::models::organization`], kept in the api crate because they
//! boot the HTTP app and build fixtures through the API.

mod tests {
    use crate::models::error::DataError;
    use crate::{
        models::{
            model_test_helpers::setup_default_app_and_session,
            users::{self, create_user},
        },
        routes::{auth::SignupRequest, regions::dto::RegionDto},
    };

    #[allow(unused_imports)]
    use crate::models::error::{
        AuthError, ConversationError, EventError, InviteError, ModelError, PermissionError,
        ReportError, UserError, ValidationError, WorkflowError,
    };
    #[allow(unused_imports)]
    use crate::models::moderation_status::ModerationStatus;
    use crate::models::organization::*;
    #[allow(unused_imports)]
    use crate::models::pagination::{Order, PageOptions, PaginatedResults};
    #[allow(unused_imports)]
    use crate::models::request_context::{ClientIp, ClientUserAgent};
    #[allow(unused_imports)]
    use crate::models::user_progress::ProgressStatus;
    #[allow(unused_imports)]
    use crate::models::users::User;
    #[allow(unused_imports)]
    use crate::models::{proposal_section, user_progress};
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
    async fn should_create_organization(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let _ = setup_default_app_and_session(&pool).await?;

        let new_org = CreateOrganization {
            name: "test_org".to_string(),
            description: "test_org".to_string(),
            mission: "to_pass_test".to_string(),
            org_type: OrganizationType::NonProfit,
            contact_email: Some("test@org.com".to_string()),
            external_url: Some("test.com".to_string()),
            ..Default::default()
        };

        let org = create(&pool, &new_org, "en").await?;
        let fetched_org = get_by_id(&pool, &org.id).await?;

        assert_eq!(org.name, "test_org".to_string(), "incorrect name");
        assert_eq!(
            org.org_type,
            OrganizationType::NonProfit,
            "incorrect org_type"
        );
        assert!(
            org.regions.is_empty(),
            "regions not initialized as empty vec"
        );
        assert_eq!(
            fetched_org.contact_email,
            Some("test@org.com".to_string()),
            "contact_email not persisted"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_delete_organization_when_related_users_exist(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let _ = setup_default_app_and_session(&pool).await?;

        let new_org = CreateOrganization {
            name: "test_org".to_string(),
            description: "test_org".to_string(),
            mission: "to_pass_test".to_string(),
            org_type: OrganizationType::NonProfit,
            contact_email: None,
            external_url: None,
            ..Default::default()
        };

        let organization = create(&pool, &new_org, "en").await?;

        let user = create_user(
            &SignupRequest {
                username: "test_org_user".to_string(),
                password: "StrongPass123!".to_string(),
                email: "test_org_user@example.com".to_string(),
                avatar_url: None,
            },
            &pool,
        )
        .await?;

        users::update_user(
            &user.id,
            &users::UpdateUserRequest {
                organization_id: Some(organization.id),
                ..Default::default()
            },
            &pool,
        )
        .await?;

        let deleted_organization = delete(&pool, &organization.id).await?;

        assert_eq!(deleted_organization.id, organization.id);

        let updated_user = users::get_user_by_id(&user.id, &pool).await?;
        assert_eq!(updated_user.organization_id, None);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_update_an_organization(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let _ = setup_default_app_and_session(&pool).await?;
        let new_org = CreateOrganization {
            name: "test_org".to_string(),
            description: "test_org".to_string(),
            mission: "to_pass_test".to_string(),
            org_type: OrganizationType::NonProfit,
            external_url: Some("test.com".to_string()),
            ..Default::default()
        };

        let org = create(&pool, &new_org, "en").await?;

        assert_eq!(
            org.org_type,
            OrganizationType::NonProfit,
            "incorrect org_type after creation"
        );
        assert!(org.regions.is_empty(), "incorrect regions after creation");

        let update_org = PartialOrganization {
            org_type: Some(OrganizationType::Governmental),
            regions: Some(vec![Uuid::new_v4(), Uuid::new_v4()]),
            ..Default::default()
        };

        let updated_org = update(&pool, &org.id, &update_org).await?;

        assert_eq!(
            updated_org.org_type,
            OrganizationType::Governmental,
            "incorrect org_type after update"
        );
        assert_eq!(
            updated_org.regions.len(),
            2,
            "incorrect regions after update"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_a_list_of_localized_organizations(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let _ = setup_default_app_and_session(&pool).await?;
        let new_org_1 = CreateOrganization {
            name: "test_org_1".to_string(),
            description: "test_org_1".to_string(),
            mission: "to_pass_test".to_string(),
            org_type: OrganizationType::NonProfit,
            external_url: Some("test.com".to_string()),
            ..Default::default()
        };
        let new_org_2 = CreateOrganization {
            name: "test_org_2".to_string(),
            description: "test_org_2".to_string(),
            mission: "to_pass_test".to_string(),
            org_type: OrganizationType::NonProfit,
            external_url: Some("test.com".to_string()),
            ..Default::default()
        };
        let new_org_3 = CreateOrganization {
            name: "test_org_3".to_string(),
            description: "test_org_3".to_string(),
            mission: "to_pass_test".to_string(),
            org_type: OrganizationType::NonProfit,
            external_url: Some("test.com".to_string()),
            ..Default::default()
        };

        let _ = create(&pool, &new_org_1, "en").await?;
        let _ = create(&pool, &new_org_2, "en").await?;
        let _ = create(&pool, &new_org_3, "en").await?;

        let page_options = PageOptions {
            offset: None,
            limit: None,
        };
        let order_options = OrganizationOrderOptions {
            ..Default::default()
        };
        let filter_options = OrganizationFilterOptions {
            ..Default::default()
        };
        let results = list(&pool, page_options, filter_options, order_options, "en").await?;

        assert_eq!(results.total, 3, "incorrect number of organizations");
        assert_eq!(
            results.records[1].name,
            "test_org_2".to_string(),
            "incorrect organization name"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_filter_organizations_by_region(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, region_res_1, _) = session.create_random_region(&app).await?;
        let (_, region_res_2, _) = session.create_random_region(&app).await?;
        let region_1: RegionDto = serde_json::from_value(region_res_1)?;
        let region_2: RegionDto = serde_json::from_value(region_res_2)?;

        let new_org_1 = CreateOrganization {
            name: "test_org_1".to_string(),
            description: "test_org_1".to_string(),
            mission: "to_pass_test".to_string(),
            org_type: OrganizationType::NonProfit,
            contact_email: None,
            external_url: Some("test.com".to_string()),
            regions: Some(vec![region_1.id]),
        };
        let new_org_2 = CreateOrganization {
            name: "test_org_2".to_string(),
            description: "test_org_2".to_string(),
            mission: "to_pass_test".to_string(),
            org_type: OrganizationType::NonProfit,
            contact_email: None,
            external_url: Some("test.com".to_string()),
            regions: Some(vec![region_2.id]),
        };
        let new_org_3 = CreateOrganization {
            name: "test_org_3".to_string(),
            description: "test_org_3".to_string(),
            mission: "to_pass_test".to_string(),
            org_type: OrganizationType::NonProfit,
            contact_email: None,
            external_url: Some("test.com".to_string()),
            regions: Some(vec![region_1.id]),
        };

        let _ = create(&pool, &new_org_1, "en").await?;
        let _ = create(&pool, &new_org_2, "en").await?;
        let _ = create(&pool, &new_org_3, "en").await?;

        let page_options = PageOptions {
            offset: None,
            limit: None,
        };
        let order_options = OrganizationOrderOptions {
            ..Default::default()
        };
        let filter_options = OrganizationFilterOptions {
            region_id: Some(region_1.id),
        };
        let results = list(
            &pool,
            page_options.clone(),
            filter_options,
            order_options,
            "en",
        )
        .await?;

        assert_eq!(
            results.total, 2,
            "incorrect number of organizations: [first results]"
        );
        assert_eq!(
            results.records[0].name,
            "test_org_1".to_string(),
            "incorrect first organization name: [first results]"
        );
        assert_eq!(
            results.records[1].name,
            "test_org_3".to_string(),
            "incorrect second organization name: [first results]"
        );

        let order_options = OrganizationOrderOptions {
            ..Default::default()
        };
        let filter_options = OrganizationFilterOptions {
            region_id: Some(region_2.id),
        };
        let results = list(
            &pool,
            page_options.clone(),
            filter_options,
            order_options,
            "en",
        )
        .await?;

        assert_eq!(
            results.total, 1,
            "incorrect number of organizations: [second results]"
        );
        assert_eq!(
            results.records[0].name,
            "test_org_2".to_string(),
            "incorrect first organization name: [second results]"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_a_localized_organization(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let _ = setup_default_app_and_session(&pool).await?;
        let new_org = CreateOrganization {
            name: "test_org".to_string(),
            description: "test_org".to_string(),
            mission: "to_pass_test".to_string(),
            org_type: OrganizationType::NonProfit,
            external_url: Some("test.com".to_string()),
            ..Default::default()
        };

        let org = create(&pool, &new_org, "en").await?;

        let org = get_localized_by_id(&pool, &org.id, "en").await?;

        assert_eq!(org.name, "test_org".to_string(), "incorrect name");
        assert_eq!(org.mission, "to_pass_test".to_string(), "incorrect mission");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_an_organization(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let _ = setup_default_app_and_session(&pool).await?;
        let new_org = CreateOrganization {
            name: "test_org".to_string(),
            description: "test_org".to_string(),
            mission: "to_pass_test".to_string(),
            org_type: OrganizationType::NonProfit,
            external_url: Some("test.com".to_string()),
            ..Default::default()
        };

        let org = create(&pool, &new_org, "en").await?;

        let org = get_by_id(&pool, &org.id).await?;

        assert_eq!(org.name, "test_org".to_string(), "incorrect name");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_delete_an_organization(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let _ = setup_default_app_and_session(&pool).await?;
        let new_org = CreateOrganization {
            name: "test_org".to_string(),
            description: "test_org".to_string(),
            mission: "to_pass_test".to_string(),
            org_type: OrganizationType::NonProfit,
            external_url: Some("test.com".to_string()),
            ..Default::default()
        };

        let org = create(&pool, &new_org, "en").await?;

        let _ = delete(&pool, &org.id).await?;

        let err = get_localized_by_id(&pool, &org.id, "en").await.unwrap_err();

        match err {
            ModelError::Data(DataError::ResourceNotFound(e)) => {
                assert_eq!(e, "Organization".to_string(), "incorrect error message");
            }
            _ => panic!("Expected ResourceNotFound error"),
        }

        Ok(())
    }
}
