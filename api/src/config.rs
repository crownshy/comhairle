use config::{Config, Environment, File};
use serde::Deserialize;

use crate::error::ComhairleError;

pub fn load() -> Result<ComhairleConfig, ComhairleError> {
    let config = Config::builder()
        .set_default(
            "jwt_secret",
            "ababa039cc54b5df83e8899c3c5839e096379d507263c732eb54c52477bf8087",
        )?
        .set_default("domain", "http://localhost:5173")?
        .set_default("enable_rate_limiting", true)?
        .set_default("mailer.host", "")?
        .set_default("mailer.user", "")?
        .set_default("mailer.password", "")?
        .set_default("mailer.from_email", "invites@comhairle.scot")?
        .set_default("heyform_url", "forms.comhairle.scot")?
        .set_default("polis_url", "polis.comhairle.scot")?
        .add_source(
            Environment::default()
                .list_separator(",")
                .separator("__")
                .with_list_parse_key("admin_users")
                .with_list_parse_key("whitelisted_domains")
                .try_parsing(true),
        )
        .add_source(File::with_name("config.toml").required(false))
        .build()?;

    let config: ComhairleConfig = config.try_deserialize()?;
    Ok(config)
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct MailerConfig {
    pub host: String,
    pub user: String,
    pub password: String,
    pub from_email: String,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct GoogleTranslateConfig {
    pub api_key: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum TranslatorConfig {
    Google(GoogleTranslateConfig),
}

#[derive(Clone, Debug, Deserialize)]
pub struct VideoCallConfig {
    pub jwt_app_id: String,
    pub jwt_app_secret: String,
    pub jwt_sub: String,
    pub jwt_accepted_issuers: String,
    pub jwt_accepted_audiences: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WorkerConfig {
    pub redis_url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ComhairleConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub resource_bucket: String,
    pub admin_users: Option<Vec<String>>,
    pub mailer: MailerConfig,
    pub domain: String,
    pub translator: Option<TranslatorConfig>,
    pub bot_service_host: Option<String>,
    pub bot_service_api_key: Option<String>,
    pub default_knowledge_base_id: Option<String>,
    pub elicitation_bot_agent_id: Option<String>,
    pub whitelisted_domains: Option<Vec<String>>,
    pub enable_rate_limiting: bool,
    pub heyform_url: String,
    pub polis_url: String,
    pub video_call_service: Option<VideoCallConfig>,
    pub worker_service: Option<WorkerConfig>,
}
