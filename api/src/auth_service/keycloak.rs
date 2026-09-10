use crate::{config::AuthServiceConfig, models::users::User};

use super::{AuthService, error::AuthServiceError};

use argon2::password_hash::{PasswordHash, Salt};
use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use keycloak::{
    KeycloakAdmin, KeycloakAdminToken, KeycloakRealmAdmin,
    prelude::reqwest,
    types::{CredentialRepresentation, UserRepresentation},
};
use serde::Serialize;
use serde_json::json;
use tracing::warn;

pub struct KeycloakClient {
    domain: String,
    admin_client: KeycloakAdmin,
    realm_name: String,
    auth_client: reqwest::Client,
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
            client_id: config.client_id.to_owned(),
            client_secret: config.client_secret.to_owned(),
        })
    }

    fn realm(&self) -> KeycloakRealmAdmin<'_, KeycloakAdminToken> {
        self.admin_client.realm(&self.realm_name)
    }
}

#[derive(Serialize, Debug)]
struct OpenidTokenRequest {
    grant_type: String,
    client_id: String,
    client_secret: String,
    code: String,
    redirect_uri: String,
}

#[async_trait]
impl AuthService for KeycloakClient {
    async fn create_user(
        &self,
        comhairle_user: &User,
    ) -> Result<serde_json::Value, AuthServiceError> {
        let realm = self.realm();

        let result = realm
            .users_post(UserRepresentation {
                id: Some(comhairle_user.id.to_string()),
                email: comhairle_user.email.clone(),
                email_verified: Some(comhairle_user.email_verified),
                username: comhairle_user.username.clone(),
                enabled: Some(true),
                credentials: comhairle_user
                    .password
                    .as_ref()
                    .map(|pw_hash| phc_to_keycloak_cred(pw_hash))
                    .transpose()?
                    .map(|cred| vec![cred]),
                ..Default::default()
            })
            .await
            .inspect_err(|e| {
                warn!("{e:#?}");
            })
            .map_err(|e| AuthServiceError::SyncUserError(e.to_string()))?;

        let response = result.into_response();
        let status = response.status();

        if !status.is_success() {
            return Err(AuthServiceError::SyncUserError(status.to_string()));
        }

        Ok(serde_json::json!({ "status": status.to_string() }))
    }

    async fn get_authorization_tokens(
        &self,
        code: &str,
        redirect_uri: &str,
    ) -> Result<serde_json::Value, AuthServiceError> {
        let url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.domain, self.realm_name
        );

        let response = self
            .auth_client
            .post(url)
            .form(&OpenidTokenRequest {
                grant_type: "authorization_code".to_string(),
                client_id: self.client_id.clone(),
                client_secret: self.client_secret.clone(),
                redirect_uri: redirect_uri.to_string(),
                code: code.to_owned(),
            })
            .send()
            .await
            .map_err(|e| AuthServiceError::AccessTokenFailure(e.to_string()))?;

        let status = response.status();

        if !status.is_success() {
            println!();
            println!("    >>>>    Do some error handling cause token request has failed: {status}");
            println!();
        }

        let json: serde_json::Value = response.json().await.expect("Json not parsable");

        Ok(json)
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
