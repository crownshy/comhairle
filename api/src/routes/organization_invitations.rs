use std::sync::Arc;

use aide::axum::ApiRouter;
use aide::axum::routing::{get_with, post_with};
use axum::extract::{Json, Path, State};
use axum::middleware::from_fn;
use tracing::instrument;
use uuid::Uuid;

use crate::ComhairleState;
use crate::error::ComhairleError;
use crate::middleware::permissions::authenticate;
use crate::models::organization_invitation::{self, InvitationDetails};
use crate::routes::auth::RequiredUser;

#[instrument(err(Debug), skip(state))]
async fn get_invitation(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(caller): RequiredUser,
    Path(invitation_id): Path<Uuid>,
) -> Result<Json<InvitationDetails>, ComhairleError> {
    Ok(Json(
        organization_invitation::details(&state, invitation_id, caller.id).await?,
    ))
}

#[instrument(err(Debug), skip(state))]
async fn accept_invitation(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(caller): RequiredUser,
    Path(invitation_id): Path<Uuid>,
) -> Result<Json<InvitationDetails>, ComhairleError> {
    Ok(Json(
        organization_invitation::accept(&state, invitation_id, caller.id).await?,
    ))
}

pub fn router() -> ApiRouter<Arc<ComhairleState>> {
    ApiRouter::new()
        .api_route(
            "/{invitation_id}",
            get_with(get_invitation, |operation| {
                operation
                    .id("GetOrganizationConversationInvitation")
                    .tag("Organizations")
                    .summary("Get an invitation for an organization to join a conversation")
                    .security_requirement("JWT")
            })
            .route_layer(from_fn(authenticate::<RequiredUser>)),
        )
        .api_route(
            "/{invitation_id}/accept",
            post_with(accept_invitation, |operation| {
                operation
                    .id("AcceptOrganizationConversationInvitation")
                    .tag("Organizations")
                    .summary("Accept an invitation as an administrator of the organization")
                    .security_requirement("JWT")
            })
            .route_layer(from_fn(authenticate::<RequiredUser>)),
        )
}
