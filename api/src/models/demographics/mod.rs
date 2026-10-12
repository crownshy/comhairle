//! Demographic questions and forms.
//!
//! Questions and forms are versioned by UUID. Published versions are immutable and
//! responses point at a question version, so an answer always records exactly what was
//! asked. See CONTEXT.md (Demographics) and ADRs 0051-0054.

pub mod defaults;
pub mod forms;
pub mod questions;
pub mod report;
pub mod responses;
pub mod steps;
pub mod validation;

#[cfg(test)]
mod tests;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum QuestionType {
    Number,
    SingleSelect,
    MultiSelect,
    Date,
    AnyString,
    Location,
    Custom,
}

impl QuestionType {
    pub fn is_select(self) -> bool {
        matches!(self, Self::SingleSelect | Self::MultiSelect)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DatePrecision {
    Day,
    Month,
    Year,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum LocationFormat {
    Postcode,
    LatLong,
    Country,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum VersionStatus {
    Draft,
    Published,
}

/// Lightweight view of a version, used in version lists.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct VersionSummary {
    pub id: uuid::Uuid,
    pub version_number: i32,
    pub status: VersionStatus,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
}
