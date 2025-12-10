use aide::OperationIo;
use async_trait::async_trait;
use mockall::{automock, predicate::*};
use reqwest::Client;
use serde::Deserialize;

use thiserror::Error;
#[derive(Error, Debug, OperationIo)]
pub enum TranslationError {
    #[error("Translation Failed")]
    TranslationFailed(String),
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait TranslationService: Send + Sync {
    async fn translate_from_to(
        &self,
        content: &str,
        from_locale: &str,
        to_locale: &str,
    ) -> Result<String, TranslationError>;
}

pub struct GoogleTranslateService {
    api_key: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GoogleTranslation {
    translated_text: String,
}

#[derive(Deserialize)]
struct GoogleData {
    translations: Vec<GoogleTranslation>,
}

#[derive(Deserialize)]
struct GoogleTranslateResponse {
    data: GoogleData,
}

impl GoogleTranslateService {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[async_trait]
impl TranslationService for GoogleTranslateService {
    async fn translate_from_to(
        &self,
        content: &str,
        from_locale: &str,
        to_locale: &str,
    ) -> Result<String, TranslationError> {
        let url = format!(
            "https://translation.googleapis.com/language/translate/v2?key={}",
            self.api_key
        );

        let client = Client::new();

        let res: GoogleTranslateResponse = client
            .post(&url)
            .json(&serde_json::json!({
                "q": content,
                "source": from_locale,
                "target": to_locale,
                "format": "text"
            }))
            .send()
            .await
            .map_err(|e| TranslationError::TranslationFailed(e.to_string()))?
            .json()
            .await
            .map_err(|e| TranslationError::TranslationFailed(e.to_string()))?;

        Ok(res.data.translations[0].translated_text.clone())
    }
}
