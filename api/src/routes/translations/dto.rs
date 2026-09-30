//! Re-export; the DTOs moved to the model layer because
//! `TextContentWithTranslations` embeds them. See [`crate::models::dto`].

pub use crate::models::dto::translations::{TextContentDto, TextTranslationDto};
