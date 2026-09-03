use super::{AuthService, error::AuthServiceError};

use async_trait::async_trait;
use keycloak::{KeycloakAdmin, KeycloakAdminToken, KeycloakRealmAdmin, prelude::reqwest};

pub struct KeycloakClient {
    admin: KeycloakAdmin,
    realm_name: String,
}

impl KeycloakClient {
    pub async fn new(
        url: &str,
        admin_user: &str,
        admin_password: &str,
        realm_name: &str,
    ) -> Result<Self, AuthServiceError> {
        let client = reqwest::Client::new();
        let admin_token =
            KeycloakAdminToken::acquire(url, admin_user, admin_password, &client).await?;

        let admin = KeycloakAdmin::new(url, admin_token, client.clone());

        Ok(Self {
            admin,
            realm_name: realm_name.to_string(),
        })
    }

    fn realm(&self) -> KeycloakRealmAdmin<'_, KeycloakAdminToken> {
        self.admin.realm(&self.realm_name)
    }
}

#[async_trait]
impl AuthService for KeycloakClient {
    async fn get_users(&self) -> () {
        let realm = self.realm();

        let results = realm.users_get().await.unwrap();
        println!();
        println!("    >>>>    Users: {results:#?}");
        println!();
    }
}
