use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::email_template_config::{EmailTemplateConfig, EmailTemplateSlots};

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
    pub email_type: String,
    pub owner_id: Uuid,
    pub organization_id: Option<Uuid>,
    pub slots: EmailTemplateSlots,
    pub subject: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<EmailTemplateConfig> for EmailTemplateConfigDto {
    fn from(c: EmailTemplateConfig) -> Self {
        Self {
            id: c.id,
            email_type: c.slots.to_string(),
            owner_id: c.owner_id,
            organization_id: c.organization_id,
            slots: c.slots,
            subject: c.subject,
            created_at: c.created_at,
        }
    }
}
