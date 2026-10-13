use std::marker::PhantomData;
use std::sync::Arc;

use axum::Extension;
use axum::extract::{FromRequestParts, Request, State};
use axum::middleware::Next;
use axum::response::Response;
use uuid::Uuid;

use crate::ComhairleState;
use crate::error::ComhairleError;
use crate::models::permissions::PermissionResource;
use crate::routes::auth::RequiredUser;

pub struct PermissionRequirement<Resource: PermissionResource> {
    action: Resource::Action,
    resource: PhantomData<fn() -> Resource>,
}

impl<Resource: PermissionResource> Clone for PermissionRequirement<Resource> {
    fn clone(&self) -> Self {
        Self::new(self.action)
    }
}

impl<Resource: PermissionResource> PermissionRequirement<Resource> {
    pub fn new(action: Resource::Action) -> Self {
        Self {
            action,
            resource: PhantomData,
        }
    }
}

/// An extracted target that identifies requests scoped to the authenticated caller.
pub trait PermissionUserFilter: FromRequestParts<Arc<ComhairleState>> + Send
where
    Self::Rejection: std::fmt::Display,
{
    fn is_caller(&self, caller_id: &Uuid) -> bool;
}

pub async fn authorize<Resource: PermissionResource>(
    State(requirement): State<PermissionRequirement<Resource>>,
    Extension(state): Extension<Arc<ComhairleState>>,
    request: Request,
    next: Next,
) -> Result<Response, ComhairleError> {
    let (mut parts, body) = request.into_parts();
    let RequiredUser(caller) = RequiredUser::from_request_parts(&mut parts, &state).await?;
    let resource = Resource::from_request_parts(&mut parts, &state).await?;
    if resource.owner_id() == Some(caller.id) {
        return Ok(next.run(Request::from_parts(parts, body)).await);
    }
    if !resource
        .can_perform_action(&state, requirement.action, &caller.id)
        .await?
    {
        return Err(ComhairleError::UserNotAuthorized);
    }
    Ok(next.run(Request::from_parts(parts, body)).await)
}

/// Requires the configured action unless filtering for the authenticated caller.
pub async fn authorize_or_self<Resource, UserFilter>(
    State(requirement): State<PermissionRequirement<Resource>>,
    Extension(state): Extension<Arc<ComhairleState>>,
    request: Request,
    next: Next,
) -> Result<Response, ComhairleError>
where
    Resource: PermissionResource,
    UserFilter: PermissionUserFilter,
    <UserFilter as FromRequestParts<Arc<ComhairleState>>>::Rejection: std::fmt::Display,
{
    let (mut parts, body) = request.into_parts();
    let RequiredUser(caller) = RequiredUser::from_request_parts(&mut parts, &state).await?;
    let resource = Resource::from_request_parts(&mut parts, &state).await?;
    let target = UserFilter::from_request_parts(&mut parts, &state)
        .await
        .map_err(|error| ComhairleError::BadRequest(error.to_string()))?;
    if !target.is_caller(&caller.id)
        && resource.owner_id() != Some(caller.id)
        && !resource
            .can_perform_action(&state, requirement.action, &caller.id)
            .await?
    {
        return Err(ComhairleError::UserNotAuthorized);
    }
    Ok(next.run(Request::from_parts(parts, body)).await)
}

pub async fn authenticate<Authentication>(
    Extension(state): Extension<Arc<ComhairleState>>,
    request: Request,
    next: Next,
) -> Result<Response, ComhairleError>
where
    Authentication: FromRequestParts<Arc<ComhairleState>, Rejection = ComhairleError>,
{
    let (mut parts, body) = request.into_parts();
    Authentication::from_request_parts(&mut parts, &state).await?;
    Ok(next.run(Request::from_parts(parts, body)).await)
}

#[cfg(test)]
mod tests {
    use super::*;

    use aide::axum::ApiRouter;
    use aide::axum::routing::get_with;
    use axum::middleware::from_fn_with_state;
    use axum::{extract::Query, http::StatusCode};

    use crate::models::permissions::{SystemResource, system};
    use crate::routes::permissions::ListPermissionsByActionQuery;

    #[test]
    fn routes_can_be_built_before_application_state_is_supplied() {
        async fn handler(State(_state): State<Arc<ComhairleState>>) -> StatusCode {
            StatusCode::OK
        }

        let _: ApiRouter<Arc<ComhairleState>> = ApiRouter::new()
            .api_route("/", get_with(handler, |operation| operation))
            .route_layer(from_fn_with_state(
                PermissionRequirement::<SystemResource>::new(system::Action::ListPermission),
                authorize::<SystemResource>,
            ))
            .api_route(
                "/{resource_type}/{resource_id}",
                get_with(handler, |operation| operation).route_layer(from_fn_with_state(
                    PermissionRequirement::<SystemResource>::new(system::Action::ListPermission),
                    authorize::<SystemResource>,
                )),
            )
            .api_route(
                "/by-action/{action}",
                get_with(handler, |operation| operation).route_layer(from_fn_with_state(
                    PermissionRequirement::<SystemResource>::new(system::Action::ListPermission),
                    authorize_or_self::<SystemResource, Query<ListPermissionsByActionQuery>>,
                )),
            );
    }
}
