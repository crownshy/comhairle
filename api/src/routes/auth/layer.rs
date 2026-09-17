use std::sync::Arc;

use aide::axum::routing::ApiMethodRouter;
use axum_keycloak_auth::{
    NonEmpty, PassthroughMode, extract::TokenExtractor, instance::KeycloakAuthInstance,
    layer::KeycloakAuthLayer,
};

use super::extract::{ComhairleExtAttrs, KcAccessTokenCookieExtractor};

use crate::ComhairleState;

pub fn optional_auth(
    mr: ApiMethodRouter<Arc<ComhairleState>>,
    instance: Arc<KeycloakAuthInstance>,
) -> ApiMethodRouter<Arc<ComhairleState>> {
    mr.layer(keycloak_layer(instance, PassthroughMode::Pass, None))
}

pub fn required_auth(
    mr: ApiMethodRouter<Arc<ComhairleState>>,
    instance: Arc<KeycloakAuthInstance>,
    audiences: Option<Vec<String>>,
) -> ApiMethodRouter<Arc<ComhairleState>> {
    mr.layer(keycloak_layer(instance, PassthroughMode::Block, audiences))
}

pub fn keycloak_layer(
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
