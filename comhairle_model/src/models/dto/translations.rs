use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::models::translations::{TextContent, TextContentId, TextFormat, TextTranslation};

/// Data transfer object (public API representation) for a translation's TextContent.
///
/// This DTO is returned by translations routes as well as GetConversation if
/// `withTranslations` query param is included the the request.
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TextContentDto {
    pub id: TextContentId,
    pub primary_locale: String,
    pub format: TextFormat,
}

impl From<TextContent> for TextContentDto {
    fn from(t: TextContent) -> Self {
        Self {
            id: t.id,
            primary_locale: t.primary_locale,
            format: t.format,
        }
    }
}

/// Data transfer object (public API representation) for a translation's TextTranslation.
///
/// This DTO is returned by translations routes as well as GetConversation if
/// `withTranslations` query param is included the the request.
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct TextTranslationDto {
    pub id: Uuid,
    pub content_id: TextContentId,
    pub locale: String,
    pub content: String,
    pub ai_generated: bool,
    pub requires_validation: bool,
}

impl From<TextTranslation> for TextTranslationDto {
    fn from(t: TextTranslation) -> Self {
        Self {
            id: t.id,
            content_id: t.content_id,
            locale: t.locale,
            content: t.content,
            ai_generated: t.ai_generated,
            requires_validation: t.requires_validation,
        }
    }
}
