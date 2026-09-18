use std::sync::Arc;

use aide::axum::routing::ApiMethodRouter;
use axum_keycloak_auth::{
    NonEmpty, PassthroughMode, extract::TokenExtractor, instance::KeycloakAuthInstance,
    layer::KeycloakAuthLayer,
};

use super::extract::{ComhairleExtAttrs, KcAccessTokenCookieExtractor};

use crate::ComhairleState;

/// Applies Keycloak authentication to a route without requiring a logged-in user.
///
/// Wraps `method_router` with [`PassthroughMode::Pass`]: requests without a valid access token
/// cookie still reach the handler, but with no `KeycloakAuthStatus`/`KeycloakToken`
/// extension inserted (or a `KeycloakAuthStatus::Failure` if using [`KeycloakAuthStatus`]).
/// Pair this with the [`OptionalUser`] extractor, not [`RequiredUser`], in the handler.
pub fn optional_auth(
    method_router: ApiMethodRouter<Arc<ComhairleState>>,
    instance: Arc<KeycloakAuthInstance>,
) -> ApiMethodRouter<Arc<ComhairleState>> {
    method_router.layer(keycloak_layer(instance, PassthroughMode::Pass, None))
}

/// Applies Keycloak authentication to a route, rejecting unauthenticated requests.
///
/// Wraps `method_router` with [`PassthroughMode::Block`]: requests without a valid access token
/// cookie are rejected before reaching the handler. On success, a `KeycloakToken`
/// extension is guaranteed to be present, so pair this with the [`RequiredUser`]
/// extractor in the handler.
///
/// `audiences` are appended to the default `"account"` audience; pass `None` if the
/// route has no additional audience requirements beyond the default.
pub fn required_auth(
    method_router: ApiMethodRouter<Arc<ComhairleState>>,
    instance: Arc<KeycloakAuthInstance>,
    audiences: Option<Vec<String>>,
) -> ApiMethodRouter<Arc<ComhairleState>> {
    method_router.layer(keycloak_layer(instance, PassthroughMode::Block, audiences))
}

/// Builds a [`KeycloakAuthLayer`] configured for this application: access tokens are
/// read from the Keycloak access-token cookie (see [`KcAccessTokenCookieExtractor`]),
/// decoded into `KeycloakToken<String, ComhairleExtAttrs>`.
fn keycloak_layer(
    instance: Arc<KeycloakAuthInstance>,
    mode: PassthroughMode,
    audiences: Option<Vec<String>>,
) -> KeycloakAuthLayer<String, ComhairleExtAttrs> {
    let mut expected_audiences = vec!["account".to_string()];
    if let Some(audiences) = audiences {
        for audience in audiences {
            expected_audiences.push(audience);
        }
    }

    KeycloakAuthLayer::<String, ComhairleExtAttrs>::builder()
        .instance(instance)
        .passthrough_mode(mode)
        .persist_raw_claims(false)
        .token_extractors(NonEmpty::<Arc<dyn TokenExtractor>> {
            head: Arc::new(KcAccessTokenCookieExtractor::default()),
            tail: vec![],
        })
        .expected_audiences(expected_audiences)
        .build()
}
