use std::{marker::PhantomData, sync::Arc};

use axum::{
    Extension,
    extract::{FromRequestParts, Request, State},
    middleware::Next,
    response::Response,
};

use crate::{
    ComhairleState,
    error::ComhairleError,
    models::permissions::{
        PermissionResource, PermissionTargetResource, can_perform_action,
        can_perform_target_action, system,
    },
    routes::auth::RequiredUser,
};

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

pub async fn authorize<Resource: PermissionResource>(
    State(requirement): State<PermissionRequirement<Resource>>,
    Extension(state): Extension<Arc<ComhairleState>>,
    request: Request,
    next: Next,
) -> Result<Response, ComhairleError> {
    let (mut parts, body) = request.into_parts();
    let RequiredUser(caller) = RequiredUser::from_request_parts(&mut parts, &state).await?;
    let resource = Resource::from_request_parts(&mut parts, &state).await?;
    if !can_perform_action(
        &state,
        &resource.resource_id(),
        requirement.action,
        &caller.id,
        resource.owner_id().as_ref(),
    )
    .await?
    {
        return Err(ComhairleError::UserNotAuthorized);
    }
    Ok(next.run(Request::from_parts(parts, body)).await)
}

#[derive(Clone)]
pub struct TargetPermissionRequirement {
    action: system::Action,
}

impl TargetPermissionRequirement {
    pub fn new(action: system::Action) -> Self {
        Self { action }
    }
}

pub async fn authorize_target(
    State(requirement): State<TargetPermissionRequirement>,
    Extension(state): Extension<Arc<ComhairleState>>,
    request: Request,
    next: Next,
) -> Result<Response, ComhairleError> {
    let (mut parts, body) = request.into_parts();
    let RequiredUser(caller) = RequiredUser::from_request_parts(&mut parts, &state).await?;
    let target = PermissionTargetResource::from_request_parts(&mut parts, &state).await?;
    if !can_perform_target_action(&state, &target, requirement.action.as_ref(), &caller.id).await? {
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
    use crate::models::permissions::SystemResource;
    use aide::axum::{ApiRouter, routing::get_with};
    use axum::{http::StatusCode, middleware::from_fn_with_state};

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
                    TargetPermissionRequirement::new(system::Action::ListPermission),
                    authorize_target,
                )),
            );
    }
}
