use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{
    conversation::{Conversation, LocalisedConversation},
    translations::TextContentId,
};

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConversationDto {
    pub id: Uuid,
    pub title: TranslatableField,
    pub short_description: TranslatableField,
    pub description: TranslatableField,
    pub video_url: Option<String>,
    pub image_url: String,
    pub tags: Vec<String>,
    pub is_public: bool,
    pub is_live: bool,
    pub is_complete: bool,
    pub is_invite_only: bool,
    pub slug: Option<String>,
    pub primary_locale: String,
    pub knowledge_base_id: Option<String>,
    pub chat_bot_id: Option<String>,
    pub enable_qa_chat_bot: bool,
    pub supported_languages: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq)]
#[serde(untagged)]
pub enum TranslatableField {
    TextContentId(TextContentId),
    Localized(String),
}

impl From<Conversation> for ConversationDto {
    fn from(c: Conversation) -> Self {
        Self {
            id: c.id,
            title: TranslatableField::TextContentId(c.title),
            short_description: TranslatableField::TextContentId(c.short_description),
            description: TranslatableField::TextContentId(c.description),
            video_url: c.video_url,
            image_url: c.image_url,
            tags: c.tags,
            is_public: c.is_public,
            is_live: c.is_live,
            is_complete: c.is_complete,
            is_invite_only: c.is_invite_only,
            slug: c.slug,
            primary_locale: c.primary_locale,
            knowledge_base_id: c.knowledge_base_id,
            chat_bot_id: c.chat_bot_id,
            enable_qa_chat_bot: c.enable_qa_chat_bot,
            supported_languages: c.supported_languages,
        }
    }
}

impl From<LocalisedConversation> for ConversationDto {
    fn from(c: LocalisedConversation) -> Self {
        Self {
            id: c.id,
            title: TranslatableField::Localized(c.title),
            short_description: TranslatableField::Localized(c.short_description),
            description: TranslatableField::Localized(c.description),
            video_url: c.video_url,
            image_url: c.image_url,
            tags: c.tags,
            is_public: c.is_public,
            is_live: c.is_live,
            is_complete: c.is_complete,
            is_invite_only: c.is_invite_only,
            slug: c.slug,
            primary_locale: c.primary_locale,
            knowledge_base_id: c.knowledge_base_id,
            chat_bot_id: c.chat_bot_id,
            enable_qa_chat_bot: c.enable_qa_chat_bot,
            supported_languages: c.supported_languages,
        }
    }
}
