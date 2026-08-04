use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::{
        organization::{
            LocalizedOrganization, Organization, OrganizationAdminBootstrapResult, OrganizationType,
        },
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
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationAdminBootstrapFailureDto {
    pub email: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationAdminBootstrapSummaryDto {
    pub attempted: usize,
    pub assigned: usize,
    pub created_accounts: usize,
    pub emailed: usize,
    pub failures: Vec<OrganizationAdminBootstrapFailureDto>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrganizationResponseDto {
    #[serde(flatten)]
    pub organization: OrganizationDto,
    pub admin_bootstrap_summary: OrganizationAdminBootstrapSummaryDto,
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
            created_at: o.created_at,
        }
    }
}

impl OrganizationAdminBootstrapSummaryDto {
    pub fn from_results(results: &[OrganizationAdminBootstrapResult]) -> Self {
        let attempted = results.len();
        let assigned = results.iter().filter(|result| result.assigned).count();
        let created_accounts = results
            .iter()
            .filter(|result| result.created_account)
            .count();
        let emailed = results.iter().filter(|result| result.emailed).count();
        let failures = results
            .iter()
            .filter_map(|result| {
                result
                    .error
                    .as_ref()
                    .map(|message| OrganizationAdminBootstrapFailureDto {
                        email: result.email.clone(),
                        message: message.clone(),
                    })
            })
            .collect();

        Self {
            attempted,
            assigned,
            created_accounts,
            emailed,
            failures,
        }
    }
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
