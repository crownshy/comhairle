use std::borrow::Cow;
use std::sync::Arc;

use aide::OperationIo;
use axum::http::request::Parts;
use axum::{
    RequestPartsExt,
    extract::{Extension, FromRequestParts, Request},
};
use axum_extra::extract::CookieJar;
use axum_keycloak_auth::KeycloakAuthStatus;
use axum_keycloak_auth::decode::{KeycloakToken, ProfileAndEmail};
use axum_keycloak_auth::error::AuthError;
use axum_keycloak_auth::extract::{ExtractedToken, TokenExtractor};
use serde::Deserialize;
use uuid::Uuid;

use super::KC_ACCESS_KEY;

use crate::models::users::UserAuthType;
use crate::routes::auth::is_user_admin;
use crate::routes::user::dto::UserDto;
use crate::{ComhairleError, ComhairleState};

#[derive(Deserialize, Debug, Clone)]
pub struct ComhairleExtAttrs {
    #[serde(flatten)]
    pub profile: ProfileAndEmail,
    pub comhairle_auth_type: UserAuthType,
    pub avatar_url: Option<String>,
    pub organization_id: Option<Uuid>,
    pub guest_code: Option<String>,
}

/// Custom token extractor for `axum_keycloak_auth`, which extracts access token
/// from request cookies. Allows better integration with `@crown-shy/api-client`
/// than `axum_keycloak_auth` default behaviour, which looks for token in
/// `Authorization` header.
#[derive(OperationIo, Debug, Clone, Default)]
pub struct KcAccessTokenCookieExtractor {}

impl KcAccessTokenCookieExtractor {
    /// Extracts an access token from cookies, reporting an absent header as `None`.
    fn try_extract<'a>(request: &Request) -> Option<ExtractedToken<'a>> {
        let jar = CookieJar::from_headers(request.headers());
        let token = jar.get(KC_ACCESS_KEY)?.value();

        if token.trim().is_empty() {
            return None;
        }

        Some(Cow::Owned(token.to_string()))
    }
}

impl TokenExtractor for KcAccessTokenCookieExtractor {
    fn extract<'a>(&self, request: &'a Request) -> Result<ExtractedToken<'a>, AuthError> {
        Self::try_extract(request).ok_or(AuthError::MissingToken)
    }
}

// ===============
//
// USER EXTRACTORS
//
// ===============

/// An extractor to get a required current user. If no user is logged in then
/// this will fail and return a Not Found response.
///
/// To be used on endpoints authenticated with [`crate::auth::layer::required_auth`].
#[derive(OperationIo)]
pub struct RequiredAdminUser(pub UserDto);

impl FromRequestParts<Arc<ComhairleState>> for RequiredAdminUser {
    type Rejection = ComhairleError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<ComhairleState>,
    ) -> Result<Self, Self::Rejection> {
        let Extension(access_token) =
            Extension::<KeycloakToken<String, ComhairleExtAttrs>>::from_request_parts(parts, state)
                .await
                .map_err(|e| {
                    tracing::warn!("{e}");
                    ComhairleError::NoLoggedInUser
                })?;

        let user = UserDto::try_from(access_token).map_err(|e| {
            ComhairleError::CorruptedData(format!(
                "Valid AuthService token had unparsable subject: {e}"
            ))
        })?;

        if is_user_admin(state, &user).await {
            Ok(RequiredAdminUser(user))
        } else {
            Err(ComhairleError::RequiresAuthUser)
        }
    }
}

/// An extractor to get a required current user. If no user is logged in then
/// this will fail and return a NotFound response.
///
/// To be used on endpoints authenticated with [`crate::auth::layer::required_auth`].
#[derive(OperationIo)]
pub struct RequiredUser(pub UserDto);

impl FromRequestParts<Arc<ComhairleState>> for RequiredUser {
    type Rejection = ComhairleError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<ComhairleState>,
    ) -> Result<Self, Self::Rejection> {
        // TODO: handle API key authorization, see resolve_user_from_request

        let Extension(access_token) =
            Extension::<KeycloakToken<String, ComhairleExtAttrs>>::from_request_parts(parts, state)
                .await
                .map_err(|e| {
                    tracing::warn!("{e}");
                    ComhairleError::NoLoggedInUser
                })?;

        let user = UserDto::try_from(access_token).map_err(|e| {
            ComhairleError::CorruptedData(format!(
                "Valid AuthService token had unparsable subject: {e}"
            ))
        })?;

        Ok(RequiredUser(user))
    }
}

/// An extractor to get the current user if they exist and return None if no user
/// is logged in.
///
/// To be used on endpoints authenticated with [`crate::auth::layer::optional_auth`].
#[derive(OperationIo, Debug)]
pub struct OptionalUser(pub Option<UserDto>);

impl FromRequestParts<Arc<ComhairleState>> for OptionalUser {
    type Rejection = ComhairleError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<ComhairleState>,
    ) -> Result<Self, Self::Rejection> {
        let auth_status =
            Extension::<KeycloakAuthStatus<String, ComhairleExtAttrs>>::from_request_parts(
                parts, state,
            )
            .await
            .map(|Extension(status)| status)
            .inspect_err(|e| tracing::error!("{e}"))
            .ok();

        let user = auth_status.and_then(|status| match status {
            KeycloakAuthStatus::Success(token) => UserDto::try_from(token)
                .inspect_err(|e| tracing::error!("{e}"))
                .ok(),
            KeycloakAuthStatus::Failure(e) => {
                tracing::debug!("{e}");
                None
            }
        });

        Ok(OptionalUser(user))
    }
}

/// An extractor to get raw JWT access token, used to authenticate external
/// auth requests via Authorization header. Returns None if no access token
/// is found.
///
/// To be used on endpoints authenticated with [`crate::auth::layer::optional_auth`].
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
