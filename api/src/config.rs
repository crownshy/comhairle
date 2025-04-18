use config::{Config, Environment, File};
use serde::Deserialize;

use crate::error::ComhairleError;

pub fn load() -> Result<ComhairleConfig, ComhairleError> {
    let config = Config::builder()
        .set_default(
            "jwt_secret",
            "ababa039cc54b5df83e8899c3c5839e096379d507263c732eb54c52477bf8087",
        )?
        .add_source(
            Environment::default()
                .list_separator(",")
                .with_list_parse_key("admin_users")
                .try_parsing(true),
        )
        .add_source(File::with_name("config.toml").required(false))
        .build()?;

    let config: ComhairleConfig = config.try_deserialize()?;
    Ok(config)
}

#[derive(Clone, Debug, Deserialize)]
pub struct ComhairleConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub resource_bucket: String,
    pub admin_users: Option<Vec<String>>,
}
