use std::collections::HashMap;

use crate::auth_service::{GetAuthorizationTokensResponse, GetUserInfoResponse};
use crate::config::AuthServiceConfig;
use crate::models::users::User;
use crate::routes::user::dto::UserDto;

use super::{AuthService, error::AuthServiceError};

use argon2::password_hash::{PasswordHash, Salt};
use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use keycloak::{
    KeycloakAdmin, KeycloakAdminToken, KeycloakRealmAdmin,
    prelude::reqwest,
    types::{CredentialRepresentation, UserRepresentation},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::warn;
use uuid::Uuid;

#[allow(dead_code)]
pub struct KeycloakClient {
    domain: String,
    admin_client: KeycloakAdmin,
    realm_name: String,
    auth_client: reqwest::Client,
    admin_user: String,
    admin_password: String,
    client_id: String,
    client_secret: String,
}

impl KeycloakClient {
    pub async fn new(config: &AuthServiceConfig) -> Result<Self, AuthServiceError> {
        let client = reqwest::Client::new();
        let admin_token = KeycloakAdminToken::acquire(
            &config.url,
            &config.admin_user,
            &config.admin_password,
            &client,
        )
        .await?;

        let admin = KeycloakAdmin::new(&config.url, admin_token, client.clone());

        let auth_client = reqwest::Client::new();

        Ok(Self {
            domain: config.url.to_owned(),
            admin_client: admin,
            realm_name: config.realm.to_string(),
            auth_client,
            admin_user: config.admin_user.to_owned(),
            admin_password: config.admin_password.to_owned(),
            client_id: config.client_id.to_owned(),
            client_secret: config.client_secret.to_owned(),
        })
    }

    fn realm(&self) -> KeycloakRealmAdmin<'_, KeycloakAdminToken> {
        self.admin_client.realm(&self.realm_name)
    }

    /// Custom helper to authenticate with Keycloak Admin Rest API. Required to
    /// implement user imports via realm partialImport endpoint, currently not
    /// supported by the [`keycloak`] crate.
    async fn custom_authenticate(&self) -> Result<MasterAuthResponse, AuthServiceError> {
        let client = reqwest::Client::new();

        let url = format!(
            "{}/realms/master/protocol/openid-connect/token",
            self.domain
        );

        let body = json!({
            "username": &self.admin_user,
            "password": &self.admin_password,
            // TODO: see if there is a better way of authenticating as
            // this grant_type is no longer recommended
            "grant_type": "password",
            "client_id": "admin-cli"
        });

        let response = client
            .post(&url)
            .form(&body)
            .send()
            .await
            .map_err(|e| AuthServiceError::AccessTokenFailure(e.to_string()))?;
        let status = response.status();

        if !status.is_success() {
            let text = response.text().await.map_err(|_| {
                AuthServiceError::AccessTokenFailure(format!("Failed with status code {}", status))
            })?;
            return Err(AuthServiceError::AccessTokenFailure(
                json!({ "status": status.to_string(), "message": text }).to_string(),
            ));
        }

        let json: MasterAuthResponse = response
            .json()
            .await
            .map_err(|e| AuthServiceError::AccessTokenFailure(e.to_string()))?;

        Ok(json)
    }

    /// Centralizes requests to Keycloak token endpoint, which may contain
    /// differing body params, ie authentication request vs refresh request.
    async fn auth_tokens<T: Serialize>(
        &self,
        form_body: &T,
    ) -> Result<GetAuthorizationTokensResponse, AuthServiceError> {
        let url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.domain, self.realm_name
        );

        let response = self
            .auth_client
            .post(url)
            .form(form_body)
            .send()
            .await
            .map_err(|e| AuthServiceError::AccessTokenFailure(e.to_string()))?;

        let status = response.status();

        if !status.is_success() {
            let text = response.text().await.map_err(|_| {
                AuthServiceError::AccessTokenFailure(format!("Failed with status code {}", status))
            })?;
            return Err(AuthServiceError::AccessTokenFailure(
                json!({ "status": status.to_string(), "message": text }).to_string(),
            ));
        }

        let json: GetAuthorizationTokensResponse = response
            .json()
            .await
            .map_err(|e| AuthServiceError::AccessTokenFailure(e.to_string()))?;

        Ok(json)
    }
}

#[derive(Deserialize, Debug)]
struct MasterAuthResponse {
    access_token: String,
}

#[derive(Serialize, Debug)]
struct OpenidTokenRequest<'a> {
    grant_type: &'a str,
    client_id: &'a str,
    client_secret: &'a str,
    code: &'a str,
    redirect_uri: &'a str,
}

#[derive(Serialize, Debug)]
struct OpenidRefreshRequest<'a> {
    grant_type: &'a str,
    refresh_token: &'a str,
    client_id: &'a str,
    client_secret: &'a str,
}

#[async_trait]
impl AuthService for KeycloakClient {
    async fn import_user(
        &self,
        comhairle_user: &User,
    ) -> Result<serde_json::Value, AuthServiceError> {
        let token_res = self.custom_authenticate().await?;

        // Use custom request instead of [`keycloak::KeycloakAdmin`] as crate
        // doesn't support this endpoint.
        //
        // Needs to use realm `partialImport` endpoint instead of `users_post`
        // endpoint as former maintains comhairle user ids, the latter does not.
        let url = format!(
            "{}/admin/realms/{}/partialImport",
            self.domain, self.realm_name,
        );

        let mut additional_attributes = HashMap::from([(
            "comhairle_auth_type".to_string(),
            vec![comhairle_user.auth_type.to_string()],
        )]);
        if let Some(ref avatar_url) = comhairle_user.avatar_url {
            additional_attributes.insert("avatar_url".to_string(), vec![avatar_url.to_owned()]);
        }
        if let Some(ref organization_id) = comhairle_user.organization_id {
            additional_attributes.insert(
                "organization_id".to_string(),
                vec![organization_id.to_string()],
            );
        }

        let response = self
            .auth_client
            .post(&url)
            .header(
                "Authorization",
                format!("Bearer {}", token_res.access_token),
            )
            .json(&json!({
                "ifResourceExists": "FAIL",
                "users": [
                    UserRepresentation {
                        id: Some(comhairle_user.id.to_string()),
                        email: comhairle_user.email.clone(),
                        email_verified: Some(comhairle_user.email_verified),
                        username: comhairle_user.username.clone(),
                        enabled: Some(true),
                        attributes: Some(additional_attributes),
                        credentials: comhairle_user
                            .password
                            .as_ref()
                            .map(|pw_hash| phc_to_keycloak_cred(pw_hash))
                            .transpose()?
                            .map(|cred| vec![cred]),
                        ..Default::default()
                    }
                ]
            }))
            .send()
            .await
            .inspect_err(|e| {
                warn!("{e:#?}");
            })
            .map_err(|e| AuthServiceError::SyncUserError(e.to_string()))?;

        let status = response.status();

        if !status.is_success() {
            return Err(AuthServiceError::SyncUserError(status.to_string()));
        }

        Ok(serde_json::json!({ "status": status.to_string() }))
    }

    async fn get_user_info(&self, token: &str) -> Result<GetUserInfoResponse, AuthServiceError> {
        let url = format!(
            "{}/realms/{}/protocol/openid-connect/userinfo",
            self.domain, self.realm_name
        );

        let result = self
            .auth_client
            .get(&url)
            .header("Authorization", format!("Bearer {token}"))
            .send()
            .await?
            .json()
            .await?;

        Ok(result)
    }

    async fn get_user_by_id(&self, id: Uuid) -> Result<UserDto, AuthServiceError> {
        let realm = self.realm();

        let result = realm.users_with_user_id_get(&id.to_string()).await?;

        let user =
            UserDto::try_from(result).map_err(|e| AuthServiceError::InvalidData(e.to_string()))?;

        Ok(user)
    }

    async fn get_user_by_email(&self, email: &str) -> Result<UserDto, AuthServiceError> {
        let realm = self.realm();

        let mut results = realm
            .users_get()
            .email(email.to_string())
            .exact(true)
            .await?;

        if results.is_empty() {
            return Err(AuthServiceError::ResourceNotFound(format!(
                "User not found with email {email}"
            )));
        }

        if results.len() > 1 {
            return Err(AuthServiceError::Conflict(format!(
                "Multiple users found with email {email}"
            )));
        }

        let user = UserDto::try_from(results.remove(0))
            .map_err(|e| AuthServiceError::InvalidData(e.to_string()))?;

        Ok(user)
    }

    async fn get_authorization_tokens(
        &self,
        code: &str,
        redirect_uri: &str,
    ) -> Result<GetAuthorizationTokensResponse, AuthServiceError> {
        let form_body = OpenidTokenRequest {
            grant_type: "authorization_code",
            client_id: &self.client_id,
            client_secret: &self.client_secret,
            redirect_uri,
            code,
        };

        self.auth_tokens(&form_body).await
    }

    async fn refresh_session(
        &self,
        refresh_token: &str,
    ) -> Result<GetAuthorizationTokensResponse, AuthServiceError> {
        let form_body = OpenidRefreshRequest {
            grant_type: "refresh_token",
            refresh_token,
            client_id: &self.client_id,
            client_secret: &self.client_secret,
        };

        self.auth_tokens(&form_body).await
    }
}

/// Converts a raw Argon2 PHC (Password Hashing Competition) string into a
/// Keycloak [`CredentialRepresentation`] suitable for importing an
/// already-hashed password via the `secretData`/`credentialData` fields.
///
/// This does **not** re-hash the password. It re-encodes the existing
/// Argon2 parameters, salt, and hash into the JSON shape Keycloak's
/// `argon2` password-hash provider expects, e.g.:
///
/// ```json
/// {
///   "hashIterations": 5,
///   "algorithm": "argon2",
///   "additionalParameters": {
///     "hashLength": ["32"],
///     "memory": ["7168"],
///     "type": ["id"],
///     "version": ["1.3"],
///     "parallelism": ["1"]
///   }
/// }
/// ```
///
/// # Arguments
///
/// * `phc_str` - An Argon2 hash in PHC string format, as produced by the
///   `argon2` crate, e.g.
///   `$argon2id$v=19$m=19456,t=2,p=1$<salt>$<hash>`.
///
/// # Returns
///
/// A [`CredentialRepresentation`] with `type_`, `credential_data`, and
/// `secret_data` populated, ready to attach to a [`UserRepresentation`]
/// for `users_post`.
///
/// # Errors
///
/// Returns [`AuthServiceError`] if `phc_str` is not a valid PHC string, is
/// missing an expected field (version, params, salt, or hash), or uses an
/// Argon2 version this function doesn't recognise (currently only
/// `v=19` and `v=16` are mapped).
fn phc_to_keycloak_cred(phc_str: &str) -> Result<CredentialRepresentation, AuthServiceError> {
    let phc = PasswordHash::new(phc_str).map_err(|e| {
        AuthServiceError::CredentialExtractionFailure(format!("Unable to parse PHC string: {e}"))
    })?;

    let type_param = phc.algorithm.as_str().trim_start_matches("argon2");

    // 19 decimal bytes => 1.3, 16 decimal bytes => 1.0
    let version_label = match phc.version {
        Some(19) => "1.3",
        Some(16) => "1.0",
        _ => {
            return Err(AuthServiceError::CredentialExtractionFailure(
                "Unrecognised argon2 version".to_string(),
            ));
        }
    };

    let memory = phc
        .params
        .get_str("m")
        .ok_or(AuthServiceError::CredentialExtractionFailure(
            "Missing memory param".to_string(),
        ))?;
    let iterations =
        phc.params
            .get_str("t")
            .ok_or(AuthServiceError::CredentialExtractionFailure(
                "Missing iterations param".to_string(),
            ))?;
    let parallelism =
        phc.params
            .get_str("p")
            .ok_or(AuthServiceError::CredentialExtractionFailure(
                "Missing parallelism param".to_string(),
            ))?;

    let salt: Salt = phc
        .salt
        .ok_or(AuthServiceError::CredentialExtractionFailure(
            "Unable to extract salt".to_string(),
        ))?;
    let salt_raw = salt.to_string();

    let hash_raw = phc
        .hash
        .ok_or(AuthServiceError::CredentialExtractionFailure(
            "Unable to extract hash".to_string(),
        ))?;

    let secret_data = json!({
        "value": STANDARD.encode(hash_raw),
        "salt": &salt_raw,
    })
    .to_string();

    let credentials_data = json!({
        "hashIterations": iterations.parse::<i32>().unwrap_or(0),
        "algorithm": "argon2",
        "additionalParameters": {
            "hashLength": [hash_raw.as_bytes().len().to_string()],
            "memory": [memory],
            "type": [type_param],
            "version": [version_label],
            "parallelism": [parallelism]
        }
    })
    .to_string();

    Ok(CredentialRepresentation {
        type_: Some("password".to_string()),
        credential_data: Some(credentials_data),
        secret_data: Some(secret_data),
        temporary: Some(false),
        ..Default::default()
    })
}
