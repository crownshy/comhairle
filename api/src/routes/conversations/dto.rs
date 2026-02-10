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
    #[schemars(example = "example_uuid")]
    pub id: Uuid,
    #[schemars(example = "example_uuid")]
    pub title: TextContentId,
    #[schemars(example = "example_uuid")]
    pub short_description: TextContentId,
    #[schemars(example = "example_uuid")]
    pub description: TextContentId,
    pub video_url: Option<String>,
    pub image_url: String,
    pub tags: Vec<String>,
    pub is_public: bool,
    pub is_live: bool,
    pub is_complete: bool,
    pub is_invite_only: bool,
    pub slug: Option<String>,
    pub primary_locale: String,
    #[schemars(example = "example_bot_service_id")]
    pub knowledge_base_id: Option<String>,
    #[schemars(example = "example_bot_service_id")]
    pub chat_bot_id: Option<String>,
    pub enable_qa_chat_bot: bool,
    pub supported_languages: Vec<String>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocalizedConversationDto {
    #[schemars(example = "example_uuid")]
    pub id: Uuid,
    #[schemars(example = "example_localized_text")]
    pub title: String,
    #[schemars(example = "example_localized_text")]
    pub short_description: String,
    #[schemars(example = "example_localized_text")]
    pub description: String,
    pub video_url: Option<String>,
    pub image_url: String,
    pub tags: Vec<String>,
    pub is_public: bool,
    pub is_live: bool,
    pub is_complete: bool,
    pub is_invite_only: bool,
    pub slug: Option<String>,
    pub primary_locale: String,
    #[schemars(example = "example_bot_service_id")]
    pub knowledge_base_id: Option<String>,
    #[schemars(example = "example_bot_service_id")]
    pub chat_bot_id: Option<String>,
    pub enable_qa_chat_bot: bool,
    pub supported_languages: Vec<String>,
}

impl From<Conversation> for ConversationDto {
    fn from(c: Conversation) -> Self {
        Self {
            id: c.id,
            title: c.title,
            short_description: c.short_description,
            description: c.description,
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

impl From<LocalisedConversation> for LocalizedConversationDto {
    fn from(c: LocalisedConversation) -> Self {
        Self {
            id: c.id,
            title: c.title,
            short_description: c.short_description,
            description: c.description,
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

fn example_localized_text() -> String {
    "Example localized text".to_string()
}

fn example_uuid() -> Uuid {
    Uuid::nil()
}

fn example_bot_service_id() -> String {
    "asdqweasdqweasdqwe".to_string()
}
