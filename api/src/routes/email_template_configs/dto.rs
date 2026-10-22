use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::email_template_config::{
    EmailTemplateConfig, EmailTemplateConfigWithScope, EmailTemplateSlots, EmailType,
};

/// Data transfer object (public API representation) for an EmailTemplateConfig.
///
/// This DTO is returned by email_template_config related endpoints and is safe to expose
/// to clients. It intentionally omits fields such as:
///
/// * `updated_at`
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EmailTemplateConfigDto {
    pub id: Uuid,
    pub email_type: EmailType,
    pub owner_id: Uuid,
    pub organization_id: Option<Uuid>,
    pub slots: EmailTemplateSlots,
    pub subject: Option<String>,
    pub conversation_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

impl From<EmailTemplateConfig> for EmailTemplateConfigDto {
    fn from(c: EmailTemplateConfig) -> Self {
        Self {
            id: c.id,
            email_type: c.email_type,
            owner_id: c.owner_id,
            organization_id: c.organization_id,
            slots: c.slots,
            subject: c.subject,
            conversation_id: c.conversation_id,
            created_at: c.created_at,
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EmailTemplateConfigWithScopeDto {
    pub id: Uuid,
    pub email_type: EmailType,
    pub owner_id: Uuid,
    pub organization_id: Option<Uuid>,
    pub slots: EmailTemplateSlots,
    pub subject: Option<String>,
    pub conversation_id: Option<Uuid>,
    pub conversation_title: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<EmailTemplateConfigWithScope> for EmailTemplateConfigWithScopeDto {
    fn from(value: EmailTemplateConfigWithScope) -> Self {
        Self {
            id: value.config.id,
            email_type: value.config.email_type,
            owner_id: value.config.owner_id,
            organization_id: value.config.organization_id,
            slots: value.config.slots,
            subject: value.config.subject,
            conversation_id: value.config.conversation_id,
            conversation_title: value.conversation_title,
            created_at: value.config.created_at,
        }
    }
}
