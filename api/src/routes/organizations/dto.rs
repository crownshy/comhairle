use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::{
        organization::{LocalizedOrganization, Organization, OrganizationType},
        pagination::PaginatedResults,
        translations::TextContentId,
    },
    schema_helpers::{example_localized_text, example_uuid},
};

/// Data transfer object (public API representation) for an Organization.
///
/// This DTO is returned by organization related endpoints and is safe to expose
/// to clients. It intentionally omits fields such as:
///
/// * `updated_at`
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationDto {
    #[schemars(example = "example_uuid")]
    pub id: Uuid,
    pub name: String,
    #[schemars(example = "example_uuid")]
    pub description: TextContentId,
    #[schemars(example = "example_uuid")]
    pub mission: TextContentId,
    pub org_type: OrganizationType,
    pub contact_email: Option<String>,
    pub external_url: Option<String>,
    pub regions: Vec<Uuid>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

impl From<Organization> for OrganizationDto {
    fn from(o: Organization) -> Self {
        Self {
            id: o.id,
            name: o.name,
            description: o.description,
            mission: o.mission,
            org_type: o.org_type,
            contact_email: o.contact_email,
            external_url: o.external_url,
            regions: o.regions,
            metadata: o.metadata,
            created_at: o.created_at,
        }
    }
}

/// Data transfer object (public API representation) for a LocalizedOrganization.
///
/// This DTO is returned by organization related endpoints and is safe to expose
/// to clients. It intentionally omits fields such as:
///
/// * `updated_at`
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LocalizedOrganizationDto {
    #[schemars(example = "example_uuid")]
    pub id: Uuid,
    pub name: String,
    #[schemars(example = "example_localized_text")]
    pub description: String,
    #[schemars(example = "example_localized_text")]
    pub mission: String,
    pub org_type: OrganizationType,
    pub contact_email: Option<String>,
    pub external_url: Option<String>,
    pub regions: Vec<Uuid>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

impl From<LocalizedOrganization> for LocalizedOrganizationDto {
    fn from(o: LocalizedOrganization) -> Self {
        Self {
            id: o.id,
            name: o.name,
            description: o.description,
            mission: o.mission,
            org_type: o.org_type,
            contact_email: o.contact_email,
            external_url: o.external_url,
            regions: o.regions,
            metadata: o.metadata,
            created_at: o.created_at,
        }
    }
}

impl From<PaginatedResults<LocalizedOrganization>> for PaginatedResults<LocalizedOrganizationDto> {
    fn from(r: PaginatedResults<LocalizedOrganization>) -> Self {
        Self {
            total: r.total,
            records: r.records.into_iter().map(Into::into).collect(),
        }
    }
}
