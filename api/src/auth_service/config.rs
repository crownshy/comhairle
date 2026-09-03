use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct AuthServiceConfig {
    pub url: String,
    pub admin_user: String,
    pub admin_password: String,
    pub realm: String,
}
