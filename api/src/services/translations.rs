//! Machine translation.
//!
//! These call the translation service, so they sit above the model layer. The
//! `TextContent` / `TextTranslation` rows and their queries stay in
//! [`crate::models::translations`].

use std::sync::Arc;

use sqlx::PgPool;
use tracing::instrument;

use crate::error::ComhairleError;
use crate::models::translations::{
    TextContentId, TextTranslation, UpdateTextTranslation, get_text_content_by_id,
    get_text_translation_by_content_and_locale, get_text_translations_by_content_id,
    update_text_translation,
};
use crate::translation_service::TranslationService;

/// text content. Will use the primary_locale as the
/// base line and use the translator service to generate
/// all of the others
// TODO This can be improved, it currently looks up
// the text content for each translation. We can
// make this smoother
#[instrument(err(Debug), skip(translator))]
pub async fn auto_generate_all_translations(
    db: &PgPool,
    translator: &Arc<dyn TranslationService>,
    text_content_id: &TextContentId,
) -> Result<Vec<TextTranslation>, ComhairleError> {
    let text_content = get_text_content_by_id(db, text_content_id).await?;
    let translations = get_text_translations_by_content_id(db, text_content_id).await?;
    let mut result: Vec<TextTranslation> = vec![];
    for translation in translations.iter() {
        if translation.locale != text_content.primary_locale {
            let new_translation =
                auto_generate_translation(db, translator, text_content_id, &translation.locale)
                    .await?;
            result.push(new_translation);
        }
    }
    return Ok(result);
}

/// Update this translation using the primary local
/// as a reference
#[instrument(err(Debug), skip(translator))]
pub async fn auto_generate_translation(
    db: &PgPool,
    translator: &Arc<dyn TranslationService>,
    text_content_id: &TextContentId,
    locale: &str,
) -> Result<TextTranslation, ComhairleError> {
    let text_content = get_text_content_by_id(db, text_content_id).await?;

    let translation =
        get_text_translation_by_content_and_locale(db, text_content_id, locale).await?;

    let reference_text = get_text_translation_by_content_and_locale(
        db,
        &text_content.id,
        &text_content.primary_locale,
    )
    .await?;

    let translated_text = translator
        .translate_from_to(
            &reference_text.content,
            &reference_text.locale,
            &translation.locale,
        )
        .await?;

    let updated_translation = update_text_translation(
        db,
        &translation.id,
        &UpdateTextTranslation {
            content: Some(translated_text),
            ai_generated: Some(true),
            requires_validation: Some(true),
            ..Default::default()
        },
    )
    .await?;
    Ok(updated_translation)
}
