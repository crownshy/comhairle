use async_trait::async_trait;
use error::{Result, TranslationError};
use reqwest::Client;
use serde::Deserialize;
pub mod config;
pub mod error;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait TranslationService: Send + Sync {
    async fn translate_from_to(
        &self,
        content: &str,
        from_locale: &str,
        to_locale: &str,
    ) -> Result<String>;
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

#[cfg(test)]
impl MockTranslationService {
    pub fn base() -> MockTranslationService {
        let mut translator = MockTranslationService::new();

        translator
            .expect_translate_from_to()
            .returning(|_, _, _| Ok("Translated String".into()));

        translator
    }
}

#[async_trait]
impl TranslationService for GoogleTranslateService {
    async fn translate_from_to(
        &self,
        content: &str,
        from_locale: &str,
        to_locale: &str,
    ) -> Result<String> {
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
