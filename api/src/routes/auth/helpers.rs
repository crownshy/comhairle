use std::sync::Arc;

use aide::{OperationIo, axum::routing::ApiMethodRouter};
use axum::{RequestPartsExt, extract::FromRequestParts};
use axum_extra::extract::CookieJar;
use axum_keycloak_auth::{
    NonEmpty, PassthroughMode, extract::TokenExtractor, instance::KeycloakAuthInstance,
    layer::KeycloakAuthLayer,
};

use crate::{
    ComhairleState,
    error::ComhairleError,
    routes::auth::{KC_ACCESS_KEY, KcAccessTokenCookieExtractor},
};

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
) -> KeycloakAuthLayer<String> {
    let mut expected_audiences = vec!["account".to_string()];
    if let Some(audiences) = audiences {
        for audience in audiences {
            expected_audiences.push(audience);
        }
    }

    KeycloakAuthLayer::<String>::builder()
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

#[derive(OperationIo, Debug)]
pub struct OptionalRawAccessToken(pub Option<String>);

impl FromRequestParts<Arc<ComhairleState>> for OptionalRawAccessToken {
    type Rejection = ComhairleError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &Arc<ComhairleState>,
    ) -> Result<Self, Self::Rejection> {
        let jar = parts
            .extract::<CookieJar>()
            .await
            .map_err(|e| ComhairleError::AuthJWTError(e.to_string()))?;

        // TODO: possibly decode and validate access token before returning
        Ok(OptionalRawAccessToken(
            jar.get(KC_ACCESS_KEY)
                .map(|cookie| cookie.value().to_string()),
        ))
    }
}
