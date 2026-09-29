//! User orchestration that reaches beyond the users table.

use std::sync::Arc;

use tracing::instrument;

use crate::models::permissions::{self, GrantRoleRequest};
use crate::models::users::{
    User, create_user, organization_admin_temporary_password, organization_admin_username,
};
use crate::routes::auth::SignupRequest;
use crate::services::permissions::grant_role;
use crate::{ComhairleState, error::ComhairleError};

/// Create an admin account for an organization and grant it the system admin role.
///
/// Two writes that have to happen together, one of which goes through the
/// permission cache, so this coordinates rather than persists.
#[instrument(err(Debug), skip(state))]
pub async fn create_organization_admin_user(
    state: &Arc<ComhairleState>,
    email: &str,
) -> Result<User, ComhairleError> {
    let signup_request = SignupRequest {
        username: organization_admin_username(email),
        password: organization_admin_temporary_password(),
        avatar_url: None,
        email: email.to_string(),
    };

    let user = create_user(&signup_request, &state.db).await?;

    // Grant the user the admin role so that they can use the admin interface
    let _ = grant_role(
        &state,
        GrantRoleRequest {
            actor_id: permissions::UserOrOrganizationId::User(user.id),
            granted_by: &user.id,
            grant_reason: "Admin user created for organization",
            permission_triplet: permissions::Role::Admin.system_triplet(),
        },
    )
    .await?;

    Ok(user)
}
