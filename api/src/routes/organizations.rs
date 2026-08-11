use std::sync::Arc;

use aide::{
    OperationIo,
    axum::{
        ApiRouter,
        routing::{delete_with, get_with, patch_with, post_with, put_with},
    },
};
use axum::{
    extract::{FromRequestParts, Json, Path, Query, State},
    http::StatusCode,
};
use minijinja::context;
use schemars::JsonSchema;
use tracing::{instrument, warn};
use uuid::Uuid;

use crate::{
    ComhairleState,
    error::ComhairleError,
    models::{
        organization::{
            self, CreateOrganization, OrganizationFilterOptions, OrganizationOrderOptions,
            PartialOrganization,
        },
        pagination::{PageOptions, PaginatedResults},
        permissions::{
            Action, ExtractResourceId, GrantRoleRequest, OwnedResource, RevokeRoleRequest, Role,
            UserOrOrganizationId, grant_role, list_users_with_permission, revoke_role,
        },
        translations, users,
    },
    routes::{
        auth::{EmailLinkClaims, RequiredAdminUser, RequiredUser, authorize, generate_jwt},
        organizations::dto::{
            CreateOrganizationResponseDto, LocalizedOrganizationDto,
            OrganizationAdminBootstrapSummaryDto, OrganizationDto,
        },
        translations::LocaleExtractor,
    },
};

pub mod dto;

#[derive(Debug, serde::Deserialize)]
struct OrganizationPath {
    organization_id: Uuid,
}

#[derive(Debug, OperationIo)]
struct OrganizationResource {
    resource_id: Uuid,
    owner_id: Option<Uuid>,
}

impl FromRequestParts<Arc<ComhairleState>> for OrganizationResource {
    type Rejection = ComhairleError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &Arc<ComhairleState>,
    ) -> Result<Self, Self::Rejection> {
        let Path(OrganizationPath { organization_id }) =
            Path::<OrganizationPath>::from_request_parts(parts, state)
                .await
                .map_err(|_| {
                    ComhairleError::ResourceNotFound(
                        "Path must contain an organization_id".to_string(),
                    )
                })?;

        Ok(Self {
            resource_id: organization_id,
            owner_id: None,
        })
    }
}

impl ExtractResourceId for OrganizationResource {
    fn resource_id(&self) -> Uuid {
        self.resource_id
    }
}

impl OwnedResource for OrganizationResource {
    fn owner_id(&self) -> Option<Uuid> {
        self.owner_id
    }
}

#[derive(Debug, serde::Deserialize, JsonSchema)]
struct OrganizationMemberPath {
    organization_id: Uuid,
    user_id: Uuid,
}

#[derive(Debug, serde::Deserialize, JsonSchema)]
struct UpsertOrganizationUserBody {
    email: String,
    role: Option<OrganizationTeamRole>,
    allow_create_user: Option<bool>,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum OrganizationTeamRole {
    Member,
    Admin,
}

#[derive(Debug, serde::Deserialize, JsonSchema)]
struct UpdateOrganizationMemberRoleBody {
    role: OrganizationTeamRole,
}

#[derive(Debug, serde::Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct OrganizationTeamUserDto {
    id: Uuid,
    username: Option<String>,
    email: Option<String>,
    role: OrganizationTeamRole,
}

#[derive(Debug, serde::Deserialize, JsonSchema)]
struct UpdateOrganizationBody {
    name: Option<String>,
    description: Option<String>,
    mission: Option<String>,
    org_type: Option<organization::OrganizationType>,
    contact_email: Option<Option<String>>,
    external_url: Option<Option<String>>,
    regions: Option<Vec<Uuid>>,
    metadata: Option<serde_json::Value>,
}

#[derive(Debug, serde::Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct UpsertOrganizationUserResponseDto {
    user: OrganizationTeamUserDto,
    created_account: bool,
    emailed: bool,
}

#[derive(Debug, serde::Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct OrganizationTeamResponseDto {
    members: Vec<OrganizationTeamUserDto>,
}

async fn set_member_admin_role(
    state: &Arc<ComhairleState>,
    organization_id: Uuid,
    user_id: Uuid,
    granted_by: Uuid,
    role: OrganizationTeamRole,
) -> Result<(), ComhairleError> {
    match role {
        OrganizationTeamRole::Admin => {
            let grant_result = grant_role(
                state,
                GrantRoleRequest {
                    actor_id: UserOrOrganizationId::User(user_id),
                    permission_triplet: Role::OrganizationAdmin.triplet(&organization_id),
                    granted_by: &granted_by,
                    grant_reason: "Organization team management",
                },
            )
            .await;

            if let Err(error) = grant_result
                && !matches!(error, ComhairleError::RoleAlreadyGranted(_))
            {
                return Err(error);
            }
        }
        OrganizationTeamRole::Member => {
            let revoke_result = revoke_role(
                state,
                RevokeRoleRequest {
                    actor_id: UserOrOrganizationId::User(user_id),
                    permission_triplet: Role::OrganizationAdmin.triplet(&organization_id),
                },
            )
            .await;

            if let Err(error) = revoke_result
                && !matches!(error, ComhairleError::RoleNotFound(_))
            {
                return Err(error);
            }
        }
    }

    Ok(())
}

fn role_label(role: OrganizationTeamRole) -> &'static str {
    match role {
        OrganizationTeamRole::Member => "member",
        OrganizationTeamRole::Admin => "administrator",
    }
}

fn notify_team_addition(
    state: &Arc<ComhairleState>,
    recipient_email: &str,
    organization_name: &str,
    role: OrganizationTeamRole,
) {
    let subject = "You were added to an organization on Comhairle";
    let admin_sign_in_link = format!("{}/auth/login?backTo=/admin", state.config.domain);

    let _ = state.mailer.send_email(
        recipient_email,
        subject,
        "organization_team_onboarding.html",
        context! {
            subject,
            organization_name,
            role_label => role_label(role),
            admin_sign_in_link,
        },
        None,
    );
}

async fn resolve_or_create_user_by_email(
    state: &Arc<ComhairleState>,
    email: &str,
    allow_create_user: bool,
) -> Result<(users::User, bool, bool), ComhairleError> {
    let trimmed = email.trim().to_lowercase();
    if trimmed.is_empty() {
        return Err(ComhairleError::BadRequest(
            "Email cannot be empty".to_string(),
        ));
    }

    match users::get_user_by_email(&trimmed, &state.db).await {
        Ok(user) => Ok((user, false, false)),
        Err(ComhairleError::NoUserFoundForEmail(_)) => {
            if !allow_create_user {
                return Err(ComhairleError::NoUserFoundForEmail(trimmed));
            }

            let user = users::create_organization_admin_user(state, &trimmed).await?;

            let token = generate_jwt()
                .user(&user)
                .secret(&state.config.jwt_secret)
                .custom_claims(EmailLinkClaims {
                    email: user.email.clone(),
                })
                .duration(chrono::Duration::hours(24))
                .call();
            let reset_link = format!(
                "{}/auth/password-reset/update?token={}",
                state.config.domain, token
            );

            let emailed = state
                .mailer
                .send_user_account_created_email(&user.email, &user.username, reset_link)
                .is_ok();

            Ok((user, true, emailed))
        }
        Err(error) => Err(error),
    }
}

#[instrument(err(Debug), skip(state))]
async fn list(
    State(state): State<Arc<ComhairleState>>,
    Query(order_options): Query<OrganizationOrderOptions>,
    Query(filter_options): Query<OrganizationFilterOptions>,
    Query(page_options): Query<PageOptions>,
    LocaleExtractor(locale): LocaleExtractor,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<PaginatedResults<LocalizedOrganizationDto>>), ComhairleError> {
    let organizations = organization::list(
        &state.db,
        page_options,
        filter_options,
        order_options,
        &locale,
    )
    .await?
    .into();

    Ok((StatusCode::OK, Json(organizations)))
}

#[instrument(err(Debug), skip(state))]
async fn get(
    State(state): State<Arc<ComhairleState>>,
    Path(organization_id): Path<Uuid>,
    RequiredUser(_user): RequiredUser,
    LocaleExtractor(locale): LocaleExtractor,
) -> Result<(StatusCode, Json<LocalizedOrganizationDto>), ComhairleError> {
    let organization = organization::get_localized_by_id(&state.db, &organization_id, &locale)
        .await?
        .into();

    Ok((StatusCode::OK, Json(organization)))
}

#[instrument(err(Debug), skip(state))]
async fn get_team(
    State(state): State<Arc<ComhairleState>>,
    Path(organization_id): Path<Uuid>,
    RequiredUser(user): RequiredUser,
    resource: OrganizationResource,
) -> Result<(StatusCode, Json<OrganizationTeamResponseDto>), ComhairleError> {
    authorize(&state, &user, Action::OrganizationUpdate, &resource).await?;

    let admins = list_users_with_permission(
        &state.db,
        Role::OrganizationAdmin.resource_type().as_ref(),
        organization_id,
        Some(Role::OrganizationAdmin.as_ref()),
    )
    .await?;

    let admin_ids = admins
        .into_iter()
        .map(|admin| admin.id)
        .collect::<std::collections::HashSet<_>>();

    let members = users::list_by_organization_id(&organization_id, &state.db)
        .await?
        .into_iter()
        .map(|member| OrganizationTeamUserDto {
            role: if admin_ids.contains(&member.id) {
                OrganizationTeamRole::Admin
            } else {
                OrganizationTeamRole::Member
            },
            id: member.id,
            username: member.username,
            email: member.email,
        })
        .collect();

    Ok((
        StatusCode::OK,
        Json(OrganizationTeamResponseDto { members }),
    ))
}

#[instrument(err(Debug), skip(state))]
async fn add_member(
    State(state): State<Arc<ComhairleState>>,
    Path(organization_id): Path<Uuid>,
    RequiredUser(user): RequiredUser,
    resource: OrganizationResource,
    Json(payload): Json<UpsertOrganizationUserBody>,
) -> Result<(StatusCode, Json<UpsertOrganizationUserResponseDto>), ComhairleError> {
    authorize(&state, &user, Action::OrganizationUpdate, &resource).await?;

    let allow_create_user = payload.allow_create_user.unwrap_or(false);
    let (resolved_user, created_account, emailed) =
        resolve_or_create_user_by_email(&state, &payload.email, allow_create_user).await?;
    let role = payload.role.unwrap_or(OrganizationTeamRole::Member);

    let updated_user =
        users::set_user_organization_id(&resolved_user.id, Some(organization_id), &state.db)
            .await?;

    set_member_admin_role(&state, organization_id, updated_user.id, user.id, role).await?;

    if let Some(email) = updated_user.email.as_deref() {
        let organization = organization::get_by_id(&state.db, &organization_id).await?;
        notify_team_addition(&state, email, &organization.name, role);
    }

    Ok((
        StatusCode::OK,
        Json(UpsertOrganizationUserResponseDto {
            user: OrganizationTeamUserDto {
                id: updated_user.id,
                username: updated_user.username,
                email: updated_user.email,
                role,
            },
            created_account,
            emailed,
        }),
    ))
}

#[instrument(err(Debug), skip(state))]
async fn update_member_role(
    State(state): State<Arc<ComhairleState>>,
    Path(OrganizationMemberPath {
        organization_id,
        user_id,
    }): Path<OrganizationMemberPath>,
    RequiredUser(user): RequiredUser,
    resource: OrganizationResource,
    Json(payload): Json<UpdateOrganizationMemberRoleBody>,
) -> Result<StatusCode, ComhairleError> {
    authorize(&state, &user, Action::OrganizationUpdate, &resource).await?;

    let target_user = users::get_user_by_id(&user_id, &state.db).await?;
    if !target_user
        .organization_id
        .is_some_and(|member_org_id| member_org_id == organization_id)
    {
        return Err(ComhairleError::BadRequest(
            "User is not a member of this organization".to_string(),
        ));
    }

    set_member_admin_role(&state, organization_id, user_id, user.id, payload.role).await?;

    Ok(StatusCode::OK)
}

#[instrument(err(Debug), skip(state))]
async fn remove_member(
    State(state): State<Arc<ComhairleState>>,
    Path(OrganizationMemberPath {
        organization_id,
        user_id,
    }): Path<OrganizationMemberPath>,
    RequiredUser(user): RequiredUser,
    resource: OrganizationResource,
) -> Result<StatusCode, ComhairleError> {
    authorize(&state, &user, Action::OrganizationUpdate, &resource).await?;

    let target_user = users::get_user_by_id(&user_id, &state.db).await?;

    if target_user
        .organization_id
        .is_some_and(|id| id == organization_id)
    {
        users::set_user_organization_id(&user_id, None, &state.db).await?;
    }

    set_member_admin_role(
        &state,
        organization_id,
        user_id,
        user.id,
        OrganizationTeamRole::Member,
    )
    .await?;

    Ok(StatusCode::OK)
}

#[instrument(err(Debug), skip(state))]
async fn create(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    LocaleExtractor(locale): LocaleExtractor,
    Json(payload): Json<CreateOrganization>,
) -> Result<(StatusCode, Json<CreateOrganizationResponseDto>), ComhairleError> {
    let created_organization = organization::create(&state.db, &payload, &locale).await?;

    grant_role(
        &state,
        GrantRoleRequest {
            actor_id: UserOrOrganizationId::User(user.id),
            permission_triplet: Role::OrganizationAdmin.triplet(&created_organization.id),
            granted_by: &user.id,
            grant_reason: "Organization creator bootstrap",
        },
    )
    .await?;

    if let Some(user_emails) = payload.user_emails.as_deref() {
        if let Err(error) =
            organization::add_member_emails(&state.db, &created_organization.id, user_emails).await
        {
            warn!(
                "Failed to add organization members for {}: {:?}",
                created_organization.id, error
            );
        }
    }

    let mut admin_bootstrap_results =
        if let Some(admin_emails) = payload.organization_admin_emails.as_deref() {
            organization::bootstrap_organization_admin_accounts(
                &state,
                &created_organization.id,
                admin_emails,
            )
            .await
        } else {
            Vec::new()
        };

    for result in &mut admin_bootstrap_results {
        if !result.assigned {
            continue;
        }

        let Some(user_id) = result.user_id else {
            result.assigned = false;
            result.error = Some("Missing user id for admin assignment".to_string());
            continue;
        };

        if let Err(error) = grant_role(
            &state,
            GrantRoleRequest {
                actor_id: UserOrOrganizationId::User(user_id),
                permission_triplet: Role::OrganizationAdmin.triplet(&created_organization.id),
                granted_by: &user.id,
                grant_reason: "Organization admin bootstrap",
            },
        )
        .await
        {
            warn!(
                "Failed to grant organization admin role for user {} and organization {}: {:?}",
                user_id, created_organization.id, error
            );
            result.assigned = false;
            result.error = Some(error.to_string());
            continue;
        }

        let admin_user = match users::get_user_by_id(&user_id, &state.db).await {
            Ok(user) => user,
            Err(error) => {
                warn!(
                    "Failed to load organization admin user {} for onboarding email: {:?}",
                    user_id, error
                );
                result.error = Some(error.to_string());
                continue;
            }
        };

        if result.created_account {
            let token = generate_jwt()
                .user(&admin_user)
                .secret(&state.config.jwt_secret)
                .custom_claims(EmailLinkClaims {
                    email: admin_user.email.clone(),
                })
                .duration(chrono::Duration::hours(24))
                .call();
            let reset_link = format!(
                "{}/auth/password-reset/update?token={}",
                state.config.domain, token
            );

            if let Err(error) = state.mailer.send_user_account_created_email(
                &admin_user.email,
                &admin_user.username,
                reset_link,
            ) {
                warn!(
                    "Failed to send organization admin account created email to {}: {:?}",
                    result.email, error
                );
                result.error = Some(error.to_string());
                continue;
            }

            result.emailed = true;
        }
    }

    let organization = created_organization.into();
    let admin_bootstrap_summary =
        OrganizationAdminBootstrapSummaryDto::from_results(&admin_bootstrap_results);

    Ok((
        StatusCode::CREATED,
        Json(CreateOrganizationResponseDto {
            organization,
            admin_bootstrap_summary,
        }),
    ))
}

#[instrument(err(Debug), skip(state))]
async fn update(
    State(state): State<Arc<ComhairleState>>,
    Path(organization_id): Path<Uuid>,
    RequiredAdminUser(user): RequiredAdminUser,
    resource: OrganizationResource,
    LocaleExtractor(locale): LocaleExtractor,
    Json(payload): Json<UpdateOrganizationBody>,
) -> Result<(StatusCode, Json<OrganizationDto>), ComhairleError> {
    authorize(&state, &user, Action::OrganizationUpdate, &resource).await?;

    let existing = organization::get_by_id(&state.db, &organization_id).await?;

    if let Some(description) = payload.description.as_ref() {
        update_localized_text_content(&state.db, &existing.description, &locale, description)
            .await?;
    }

    if let Some(mission) = payload.mission.as_ref() {
        update_localized_text_content(&state.db, &existing.mission, &locale, mission).await?;
    }

    let partial = PartialOrganization {
        name: payload.name,
        org_type: payload.org_type,
        contact_email: payload.contact_email,
        external_url: payload.external_url,
        regions: payload.regions,
        metadata: payload.metadata.into(),
    };

    let organization = if partial.to_values().is_empty() {
        existing
    } else {
        organization::update(&state.db, &organization_id, &partial).await?
    }
    .into();

    Ok((StatusCode::OK, Json(organization)))
}

/// Get the organization's `metadata` jsonb column.
#[instrument(err(Debug), skip(state))]
async fn get_metadata(
    State(state): State<Arc<ComhairleState>>,
    Path(organization_id): Path<Uuid>,
    RequiredAdminUser(_user): RequiredAdminUser,
    resource: OrganizationResource,
) -> Result<(StatusCode, Json<Option<serde_json::Value>>), ComhairleError> {
    authorize(&state, &_user, Action::OrganizationRead, &resource).await?;

    let metadata = organization::get_metadata(&state.db, &organization_id).await?;

    Ok((StatusCode::OK, Json(metadata)))
}

/// Shallow-merge the request body into the organization's `metadata` jsonb
/// column. The body must be a JSON object.
#[instrument(err(Debug), skip(state))]
async fn patch_metadata(
    State(state): State<Arc<ComhairleState>>,
    Path(organization_id): Path<Uuid>,
    RequiredAdminUser(user): RequiredAdminUser,
    resource: OrganizationResource,
    Json(patch): Json<serde_json::Value>,
) -> Result<(StatusCode, Json<OrganizationDto>), ComhairleError> {
    authorize(&state, &user, Action::OrganizationUpdate, &resource).await?;

    let organization = organization::patch_metadata(&state.db, &organization_id, patch)
        .await?
        .into();

    Ok((StatusCode::OK, Json(organization)))
}

async fn update_localized_text_content(
    db: &sqlx::PgPool,
    content_id: &translations::TextContentId,
    locale: &str,
    content: &str,
) -> Result<(), ComhairleError> {
    match translations::get_text_translation_by_content_and_locale(db, content_id, locale).await {
        Ok(existing_translation) => {
            translations::update_text_translation(
                db,
                &existing_translation.id,
                &translations::UpdateTextTranslation {
                    content: Some(content.to_string()),
                    ..Default::default()
                },
            )
            .await?;
        }
        Err(ComhairleError::ResourceNotFound(_)) => {
            translations::create_text_translation(
                db,
                &translations::CreateTextTranslation {
                    content_id: *content_id,
                    locale: locale.to_string(),
                    content: content.to_string(),
                    ai_generated: Some(false),
                    requires_validation: Some(false),
                },
            )
            .await?;
        }
        Err(error) => return Err(error),
    }

    Ok(())
}

#[instrument(err(Debug), skip(state))]
async fn delete(
    State(state): State<Arc<ComhairleState>>,
    Path(organization_id): Path<Uuid>,
    RequiredAdminUser(user): RequiredAdminUser,
    resource: OrganizationResource,
) -> Result<(StatusCode, Json<OrganizationDto>), ComhairleError> {
    authorize(&state, &user, Action::OrganizationDelete, &resource).await?;

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
            "/{organization_id}/team",
            get_with(get_team, |op| {
                op.id("GetOrganizationTeam")
                    .tag("Organizations")
                    .summary("Get organization team")
                    .description("Returns members and administrators for an organization")
                    .security_requirement("JWT")
                    .response::<200, Json<OrganizationTeamResponseDto>>()
            }),
        )
        .api_route(
            "/{organization_id}/members",
            post_with(add_member, |op| {
                op.id("AddOrganizationMember")
                    .tag("Organizations")
                    .summary("Add organization member")
                    .description("Adds a member by email and bootstraps an account when needed")
                    .security_requirement("JWT")
                    .response::<200, Json<UpsertOrganizationUserResponseDto>>()
            }),
        )
        .api_route(
            "/{organization_id}/members/{user_id}",
            delete_with(remove_member, |op| {
                op.id("RemoveOrganizationMember")
                    .tag("Organizations")
                    .summary("Remove organization member")
                    .description("Removes a user's organization membership")
                    .security_requirement("JWT")
                    .response::<200, ()>()
            }),
        )
        .api_route(
            "/{organization_id}/members/{user_id}/role",
            put_with(update_member_role, |op| {
                op.id("UpdateOrganizationMemberRole")
                    .tag("Organizations")
                    .summary("Update organization member role")
                    .description("Updates organization member role between member and admin")
                    .security_requirement("JWT")
                    .response::<200, ()>()
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
                    .response::<201, Json<CreateOrganizationResponseDto>>()
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
            "/{organization_id}/metadata",
            get_with(get_metadata, |op| {
                op.id("GetOrganizationMetadata")
                    .tag("Organizations")
                    .summary("Get organization metadata")
                    .description("Get organization metadata")
                    .security_requirement("JWT")
                    .response::<200, Json<Option<serde_json::Value>>>()
            }),
        )
        .api_route(
            "/{organization_id}/metadata",
            patch_with(patch_metadata, |op| {
                op.id("PatchOrganizationMetadata")
                    .tag("Organizations")
                    .summary("Shallow-merge organization metadata")
                    .description(
                        "Merge a JSON object into organization.metadata at the top level using jsonb concatenation",
                    )
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
    use uuid::Uuid;

    use crate::{
        error::ComhairleError,
        mailer::MockComhairleMailer,
        models::{
            model_test_helpers::setup_default_app_and_session,
            organization::OrganizationType,
            permissions::{Role, has_resource_permission},
            users::{create_user, get_user_by_email},
        },
        routes::auth::SignupRequest,
        setup_server,
        test_helpers::{UserSession, test_state},
    };

    use super::*;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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

        assert!(
            status.is_success(),
            "error response status: {status}, body: {response}"
        );

        let organization: OrganizationDto = serde_json::from_value(response)?;
        assert_eq!(
            organization.org_type,
            OrganizationType::NonProfit,
            "incorrect org_type"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_bootstrap_organization_admins_and_return_summary(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let mut mailer = MockComhairleMailer::new();
        mailer
            .expect_send_welcome_email()
            .once()
            .returning(|_, _| Ok(()));
        mailer
            .expect_send_user_account_created_email()
            .once()
            .returning(|_, _, _| Ok(()));

        let state = Arc::new(test_state().db(pool).mailer(Arc::new(mailer)).call()?);
        let app = setup_server(state.clone()).await?;
        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let existing_admin = create_user(
            &SignupRequest {
                username: "existing_org_admin".to_string(),
                password: "StrongPass123!".to_string(),
                email: "existing-admin@example.com".to_string(),
                avatar_url: None,
            },
            &state.db,
        )
        .await?;

        let (status, response, _) = session
            .create_organization(
                &app,
                json!({
                    "name": "org_with_admins",
                    "description": "org_with_admins",
                    "mission": "org_with_admins",
                    "org_type": "non_profit",
                    "organization_admin_emails": [
                        "existing-admin@example.com",
                        "new-admin@example.com"
                    ]
                }),
            )
            .await?;

        assert_eq!(status, StatusCode::CREATED);

        let organization_id = Uuid::parse_str(
            response
                .get("id")
                .and_then(|value| value.as_str())
                .ok_or("missing id")?,
        )?;

        let summary = response
            .get("adminBootstrapSummary")
            .ok_or("missing adminBootstrapSummary")?;

        assert_eq!(summary.get("attempted").and_then(|v| v.as_u64()), Some(2));
        assert_eq!(summary.get("assigned").and_then(|v| v.as_u64()), Some(2));
        assert_eq!(
            summary.get("createdAccounts").and_then(|v| v.as_u64()),
            Some(1)
        );
        assert_eq!(summary.get("emailed").and_then(|v| v.as_u64()), Some(1));

        let new_admin = get_user_by_email("new-admin@example.com", &state.db).await?;
        assert_eq!(
            new_admin.auth_type,
            crate::models::users::UserAuthType::EmailPassword,
            "new organization admin should be a full email-password user"
        );

        let existing_has_permission = has_resource_permission(
            &state,
            Role::OrganizationAdmin.triplet(&organization_id),
            &existing_admin.id,
            existing_admin.organization_id.as_ref(),
        )
        .await?;
        let new_has_permission = has_resource_permission(
            &state,
            Role::OrganizationAdmin.triplet(&organization_id),
            &new_admin.id,
            new_admin.organization_id.as_ref(),
        )
        .await?;

        assert!(existing_has_permission);
        assert!(new_has_permission);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_not_rollback_organization_when_admin_account_created_email_fails(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let mut mailer = MockComhairleMailer::new();
        mailer
            .expect_send_welcome_email()
            .once()
            .returning(|_, _| Ok(()));
        mailer
            .expect_send_user_account_created_email()
            .once()
            .returning(|_, _, _| Err(ComhairleError::WrongUserType));

        let state = Arc::new(test_state().db(pool).mailer(Arc::new(mailer)).call()?);
        let app = setup_server(state.clone()).await?;
        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let (status, response, _) = session
            .create_organization(
                &app,
                json!({
                    "name": "org_with_email_failure",
                    "description": "org_with_email_failure",
                    "mission": "org_with_email_failure",
                    "org_type": "non_profit",
                    "organization_admin_emails": ["new-admin-failure@example.com"]
                }),
            )
            .await?;

        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(
            response.get("name").and_then(|value| value.as_str()),
            Some("org_with_email_failure")
        );

        let summary = response
            .get("adminBootstrapSummary")
            .ok_or("missing adminBootstrapSummary")?;
        assert_eq!(summary.get("attempted").and_then(|v| v.as_u64()), Some(1));
        assert_eq!(summary.get("assigned").and_then(|v| v.as_u64()), Some(1));
        assert_eq!(summary.get("emailed").and_then(|v| v.as_u64()), Some(0));
        assert_eq!(
            summary
                .get("failures")
                .and_then(|value| value.as_array())
                .map(|value| !value.is_empty()),
            Some(true)
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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

        let (_, response, _) = session.get(&app, "/organizations?created_at=desc").await?;
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

        let (_, response, _) = session.get(&app, "/organizations?name=asc").await?;
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

        let (_, response, _) = session.get(&app, "/organizations?name=desc").await?;
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

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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
