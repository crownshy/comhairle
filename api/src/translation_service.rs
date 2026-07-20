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

/// Maps an application locale code to the closest locale supported by the
/// Google Translate API.
///
/// Google Translate has no distinct Dari (`prs`) model, so Dari is translated
/// as Persian (`fa`), with which it shares a script and is largely mutually
/// intelligible in written form. All other locales are passed through unchanged.
fn to_google_locale(locale: &str) -> &str {
    match locale {
        "prs" => "fa",
        other => other,
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
                "source": to_google_locale(from_locale),
                "target": to_google_locale(to_locale),
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

#[cfg(test)]
mod tests {
    use super::to_google_locale;

    #[test]
    fn maps_dari_to_persian() {
        assert_eq!(to_google_locale("prs"), "fa");
    }

    #[test]
    fn passes_other_locales_through() {
        assert_eq!(to_google_locale("ps"), "ps");
        assert_eq!(to_google_locale("en"), "en");
        assert_eq!(to_google_locale("fa"), "fa");
    }
}
