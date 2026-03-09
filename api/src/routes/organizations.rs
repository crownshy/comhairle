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
        organization::{self, CreateOrganization, OrganizationOrderOptions, PartialOrganization},
        pagination::{PageOptions, PaginatedResults},
    },
    routes::{
        auth::{RequiredAdminUser, RequiredUser},
        organizations::dto::{LocalizedOrganizationDto, OrganizationDto},
        translations::LocaleExtractor,
    },
    ComhairleState,
};

pub mod dto;

#[instrument(err(Debug), skip(state))]
async fn list(
    State(state): State<Arc<ComhairleState>>,
    Query(order_options): Query<OrganizationOrderOptions>,
    Query(page_options): Query<PageOptions>,
    LocaleExtractor(locale): LocaleExtractor,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<PaginatedResults<LocalizedOrganizationDto>>), ComhairleError> {
    let organizations = organization::list(&state.db, page_options, order_options, &locale)
        .await?
        .into();

    Ok((StatusCode::OK, Json(organizations)))
}

#[instrument(err(Debug), skip(state))]
async fn get(
    State(state): State<Arc<ComhairleState>>,
    Path(organization_id): Path<Uuid>,
    RequiredUser(user): RequiredUser,
    LocaleExtractor(locale): LocaleExtractor,
) -> Result<(StatusCode, Json<LocalizedOrganizationDto>), ComhairleError> {
    let organization = organization::get_localized_by_id(&state.db, &organization_id, &locale)
        .await?
        .into();

    Ok((StatusCode::OK, Json(organization)))
}

#[instrument(err(Debug), skip(state))]
async fn create(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(_user): RequiredAdminUser,
    LocaleExtractor(locale): LocaleExtractor,
    Json(payload): Json<CreateOrganization>,
) -> Result<(StatusCode, Json<OrganizationDto>), ComhairleError> {
    let organization = organization::create(&state.db, &payload, &locale)
        .await?
        .into();

    Ok((StatusCode::CREATED, Json(organization)))
}

#[instrument(err(Debug), skip(state))]
async fn update(
    State(state): State<Arc<ComhairleState>>,
    Path(organization_id): Path<Uuid>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(organization): Json<PartialOrganization>,
) -> Result<(StatusCode, Json<OrganizationDto>), ComhairleError> {
    let organization = organization::update(&state.db, &organization_id, &organization)
        .await?
        .into();

    Ok((StatusCode::OK, Json(organization)))
}

#[instrument(err(Debug), skip(state))]
async fn delete(
    State(state): State<Arc<ComhairleState>>,
    Path(organization_id): Path<Uuid>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<OrganizationDto>), ComhairleError> {
    let organization = organization::delete(&state.db, &organization_id)
        .await?
        .into();

    Ok((StatusCode::OK, Json(organization)))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(list, |op| {
                op.id("ListOrganizations")
                    .tag("Organizations")
                    .summary("List of organizations")
                    .description("Paginated list of organizations with optional ordering")
                    .security_requirement("JWT")
                    .response::<200, Json<PaginatedResults<LocalizedOrganizationDto>>>()
            }),
        )
        .api_route(
            "/{organization_id}",
            get_with(get, |op| {
                op.id("GetOrganization")
                    .tag("Organizations")
                    .summary("Get an organization by id")
                    .description("Get an organization by id")
                    .security_requirement("JWT")
                    .response::<200, Json<LocalizedOrganizationDto>>()
            }),
        )
        .api_route(
            "/",
            post_with(create, |op| {
                op.id("CreateOrganization")
                    .tag("Organizations")
                    .summary("Create a new organization")
                    .description("Create a new organization")
                    .security_requirement("JWT")
                    .response::<201, Json<OrganizationDto>>()
            }),
        )
        .api_route(
            "/{organization_id}",
            put_with(update, |op| {
                op.id("UpdateOrganization")
                    .tag("Organizations")
                    .summary("Update an organization")
                    .description("Update an organization")
                    .security_requirement("JWT")
                    .response::<200, Json<OrganizationDto>>()
            }),
        )
        .api_route(
            "/{organization_id}",
            delete_with(delete, |op| {
                op.id("DeleteOrganization")
                    .tag("Organizations")
                    .summary("Delete an organization")
                    .description("Delete an organization")
                    .security_requirement("JWT")
                    .response::<200, Json<OrganizationDto>>()
            }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use sqlx::PgPool;
    use std::error::Error;

    use crate::models::{
        model_test_helpers::setup_default_app_and_session, organization::OrganizationType,
    };

    use super::*;

    #[sqlx::test]
    async fn should_create_an_organization(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;

        let new_organization = CreateOrganization {
            name: "test_organization".to_string(),
            description: "test_desc".to_string(),
            mission: "test_mission".to_string(),
            org_type: OrganizationType::NonProfit,
            ..Default::default()
        };

        let body = serde_json::to_vec(&new_organization)?;
        let (status, response, _) = session.post(&app, "/organizations", body.into()).await?;

        let organization: OrganizationDto = serde_json::from_value(response)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(
            organization.org_type,
            OrganizationType::NonProfit,
            "incorrect org_type"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_get_an_organization_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;

        let (_, response, _) = session.create_random_organization(&app).await?;
        let organization: OrganizationDto = serde_json::from_value(response)?;

        let (status, response, _) = session
            .get(&app, &format!("/organizations/{}", organization.id))
            .await?;
        let organization: LocalizedOrganizationDto = serde_json::from_value(response)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(
            organization.name,
            "test_organization".to_string(),
            "incorrect organization name"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_list_organizations(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;

        let _ = session.create_random_organization(&app).await?;
        let _ = session.create_random_organization(&app).await?;
        let _ = session.create_random_organization(&app).await?;

        let (status, response, _) = session.get(&app, "/organizations").await?;
        let organizations: PaginatedResults<LocalizedOrganizationDto> =
            serde_json::from_value(response)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(organizations.total, 3, "incorrect number of organizations");
        assert_eq!(
            organizations.records[0].name,
            "test_organization".to_string(),
            "incorrect organization json"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_return_ordered_list_of_organizations(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;

        let _ = session
            .create_organization(
                &app,
                json!({
                "name": "bar",
                "description": "1",
                "mission": "test_mission",
                "org_type": "non_profit",
                }),
            )
            .await?;
        let _ = session
            .create_organization(
                &app,
                json!({
                "name": "foo",
                "description": "2",
                "mission": "test_mission",
                "org_type": "non_profit",
                }),
            )
            .await?;
        let _ = session
            .create_organization(
                &app,
                json!({
                "name": "baz",
                "description": "3",
                "mission": "test_mission",
                "org_type": "non_profit",
                }),
            )
            .await?;

        let (_, response, _) = session
            .get(&app, "/organizations?created_at=desc")
            .await?;
        let organizations: PaginatedResults<LocalizedOrganizationDto> =
            serde_json::from_value(response)?;
        assert_eq!(
            organizations.records[0].name,
            "baz".to_string(),
            "incorrect first organization [created_at=desc]"
        );
        assert_eq!(
            organizations.records[2].name,
            "bar".to_string(),
            "incorrect last organization [created_at=desc]"
        );

        let (_, response, _) = session
            .get(
                &app,
                "/organizations?name=asc",
            )
            .await?;
        let organizations: PaginatedResults<LocalizedOrganizationDto> =
            serde_json::from_value(response)?;
        assert_eq!(
            organizations.records[0].name,
            "bar".to_string(),
            "incorrect first organization [name=asc]"
        );
        assert_eq!(
            organizations.records[2].name,
            "foo".to_string(),
            "incorrect last organization [name=asc]"
        );

        let (_, response, _) = session
            .get(
                &app,
                "/organizations?name=desc",
            )
            .await?;
        let organizations: PaginatedResults<LocalizedOrganizationDto> =
            serde_json::from_value(response)?;
        assert_eq!(
            organizations.records[0].name,
            "foo".to_string(),
            "incorrect first organization [name=desc]"
        );
        assert_eq!(
            organizations.records[2].name,
            "bar".to_string(),
            "incorrect last organization [name=desc]"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_update_an_organization(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, response, _) = session.create_random_organization(&app).await?;
        let organization: OrganizationDto = serde_json::from_value(response)?;

        let update = PartialOrganization {
            org_type: Some(OrganizationType::Other),
            ..Default::default()
        };
        let body = serde_json::to_vec(&update)?;
        let (status, response, _) = session
            .put(
                &app,
                &format!("/organizations/{}", organization.id),
                body.into(),
            )
            .await?;
        let organization: OrganizationDto = serde_json::from_value(response)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(
            organization.org_type,
            OrganizationType::Other,
            "incorrect org_type"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_delete_an_organization(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;

        let (_, response, _) = session.create_random_organization(&app).await?;
        let organization: OrganizationDto = serde_json::from_value(response)?;

        let _ = session
            .delete(&app, &format!("/organizations/{}", organization.id))
            .await?;

        let (_, response, _) = session
            .get(&app, &format!("/organizations/{}", organization.id))
            .await?;

        assert_eq!(
            response.get("err").and_then(|v| v.as_str()).unwrap(),
            "Organization not found",
            "incorrect error message"
        );

        Ok(())
    }
}
