use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct AuthServiceConfig {
    pub url: String,
    pub root_user: String,
    pub root_password: String,
    pub admin_realm: String,
    pub admin_client_id: String,
    pub admin_client_secret: String,
    pub public_realm: String,
    pub public_client_id: String,
    pub public_client_secret: String,
}
