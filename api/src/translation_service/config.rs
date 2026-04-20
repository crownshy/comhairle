use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, Default)]
pub struct GoogleTranslateConfig {
    pub api_key: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum TranslatorConfig {
    Google(GoogleTranslateConfig),
}
