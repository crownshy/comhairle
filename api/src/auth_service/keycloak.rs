use crate::{config::AuthServiceConfig, error::ComhairleError};

use super::{AuthService, error::AuthServiceError};

use async_trait::async_trait;
use keycloak::{KeycloakAdmin, KeycloakAdminToken, KeycloakRealmAdmin, prelude::reqwest};
use serde::Serialize;

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
    async fn get_users(&self) -> () {
        let realm = self.realm();

        let _results = realm.users_get().await.unwrap();
    }

    async fn get_authorization_tokens(
        &self,
        code: &str,
        redirect_uri: &str,
    ) -> Result<serde_json::Value, ComhairleError> {
        // TODO:
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
            .map_err(|_| ComhairleError::BadRequest("Not a bad request".to_string()))?;

        let status = response.status();

        if !status.is_success() {
            // let text = response.text().await?;
            println!();
            println!("    >>>>    Do some error handling cause token request has failed: {status}");
            println!();
        }

        // TODO:
        let json: serde_json::Value = response.json().await.expect("Json not parsable");

        println!();
        println!("    >>>>    Success json: {json:#?}");
        println!();

        Ok(json)
    }
}
