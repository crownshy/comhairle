use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::{
        conversation::{Conversation, LocalizedConversation},
        pagination::PaginatedResults,
        translations::TextContentId,
    },
    schema_helpers::{example_bot_service_id, example_localized_text, example_uuid},
};

/// Data transfer object (public API representation) for a Conversation.
///
/// This DTO is returned by conversation related endpoints and is safe to expose
/// to clients. It intentionally omits fields such as:
///
/// * `owner_id`
/// * `default_workflow_id`
/// * `created_at`
/// * `updated_at`
///
/// It includes raw `uuid` values for translatable fields:
///
/// * `title`
/// * `short_description`
/// * `description`
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
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
    pub organization_id: Option<Uuid>,
    #[schemars(example = "example_uuid")]
    pub privacy_policy: Option<TextContentId>,
}

/// Data transfer object (public API representation) for a LocalizedConversation.
///
/// This DTO is returned by conversation related endpoints and is safe to expose
/// to clients. It intentionally omits fields such as:
///
/// * `owner_id`
/// * `default_workflow_id`
/// * `created_at`
/// * `updated_at`
///
/// It includes localized `String` values for translatable fields:
///
/// * `title`
/// * `short_description`
/// * `description`
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
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
    pub organization_id: Option<Uuid>,
    #[schemars(example = "example_localized_text")]
    pub privacy_policy: Option<String>,
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
            organization_id: c.organization_id,
            privacy_policy: c.privacy_policy,
        }
    }
}

impl From<LocalizedConversation> for LocalizedConversationDto {
    fn from(c: LocalizedConversation) -> Self {
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
            organization_id: c.organization_id,
            privacy_policy: c.privacy_policy,
        }
    }
}

impl From<PaginatedResults<LocalizedConversation>> for PaginatedResults<LocalizedConversationDto> {
    fn from(r: PaginatedResults<LocalizedConversation>) -> Self {
        Self {
            total: r.total,
            records: r.records.into_iter().map(Into::into).collect(),
        }
    }
}
