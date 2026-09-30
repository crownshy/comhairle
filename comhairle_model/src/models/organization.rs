use crate::models::error::ModelError;
use chrono::{DateTime, Utc};
use comhairle_macros::Translatable;
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{Expr, PostgresQueryBuilder, Query, SelectStatement, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow, query_as_with};
use tracing::instrument;
use uuid::Uuid;

#[cfg(any(test, feature = "test-util"))]
use fake::Dummy;

use crate::models::error::ValidationError;
use crate::models::{
    SqlxResultExt,
    pagination::{Order, PageOptions, PaginatedResults},
    translations::{TextContentId, TextFormat, new_translation},
    users,
};

#[derive(Partial, Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema, Translatable)]
#[enum_def(table_name = "organization")]
#[partially(derive(Serialize, Deserialize, Debug, JsonSchema, Default))]
pub struct Organization {
    #[partially(omit)]
    pub id: Uuid,
    pub name: String,
    #[partially(omit)]
    pub description: TextContentId,
    #[partially(omit)]
    pub mission: TextContentId,
    pub org_type: OrganizationType,
    pub contact_email: Option<String>,
    pub external_url: Option<String>,
    pub regions: Vec<Uuid>,
    pub metadata: Option<serde_json::Value>,
    #[partially(omit)]
    pub created_at: DateTime<Utc>,
    #[partially(omit)]
    pub updated_at: DateTime<Utc>,
}

#[derive(
    Debug, Default, Deserialize, Serialize, PartialEq, PartialOrd, sqlx::Type, Clone, JsonSchema,
)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(any(test, feature = "test-util"), derive(Dummy))]
pub enum OrganizationType {
    #[sqlx(rename = "non_profit")]
    NonProfit,
    #[sqlx(rename = "governmental")]
    Governmental,
    #[default]
    #[sqlx(rename = "other")]
    Other,
}

impl From<OrganizationType> for sea_query::Value {
    fn from(val: OrganizationType) -> Self {
        sea_query::Value::String(Some(Box::new(val.to_string())))
    }
}

impl std::fmt::Display for OrganizationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            OrganizationType::NonProfit => "non_profit",
            OrganizationType::Governmental => "governmental",
            OrganizationType::Other => "other",
        };
        write!(f, "{}", value)
    }
}

const DEFAULT_COLUMNS: [OrganizationIden; 11] = [
    OrganizationIden::Id,
    OrganizationIden::Name,
    OrganizationIden::Description,
    OrganizationIden::Mission,
    OrganizationIden::OrgType,
    OrganizationIden::ContactEmail,
    OrganizationIden::ExternalUrl,
    OrganizationIden::Regions,
    OrganizationIden::Metadata,
    OrganizationIden::CreatedAt,
    OrganizationIden::UpdatedAt,
];

#[derive(Serialize, Deserialize, JsonSchema, Debug, Default)]
pub struct CreateOrganization {
    pub name: String,
    pub description: String,
    pub mission: String,
    pub org_type: OrganizationType,
    pub contact_email: Option<String>,
    pub external_url: Option<String>,
    pub regions: Option<Vec<Uuid>>,
}

impl CreateOrganization {
    fn columns(&self) -> Vec<OrganizationIden> {
        let mut columns = vec![OrganizationIden::Name, OrganizationIden::OrgType];

        if self.contact_email.is_some() {
            columns.push(OrganizationIden::ContactEmail);
        }
        if self.external_url.is_some() {
            columns.push(OrganizationIden::ExternalUrl);
        }
        if self.regions.is_some() {
            columns.push(OrganizationIden::Regions);
        }

        columns
    }

    fn values(&self) -> Vec<sea_query::SimpleExpr> {
        let mut values = vec![(*self.name).into(), self.org_type.clone().into()];

        if let Some(value) = &self.contact_email {
            values.push(value.clone().into());
        }
        if let Some(value) = &self.external_url {
            values.push(value.clone().into());
        }
        if let Some(value) = &self.regions {
            values.push(value.clone().into());
        }

        values
    }
}

#[instrument(err(Debug), skip(db))]
pub async fn create(
    db: &PgPool,
    new_org: &CreateOrganization,
    locale: &str,
) -> Result<Organization, ModelError> {
    let mut columns = new_org.columns();
    let mut values = new_org.values();

    let description_translation =
        new_translation(db, locale, &new_org.description, TextFormat::Plain).await?;

    columns.push(OrganizationIden::Description);
    values.push(description_translation.id.into());

    let mission_translation =
        new_translation(db, locale, &new_org.mission, TextFormat::Plain).await?;
    columns.push(OrganizationIden::Mission);
    values.push(mission_translation.id.into());

    let (sql, values) = Query::insert()
        .into_table(OrganizationIden::Table)
        .columns(columns)
        .values(values)?
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let organization = sqlx::query_as_with(&sql, values).fetch_one(db).await?;

    Ok(organization)
}

#[instrument(err(Debug), skip(db))]
pub async fn add_member_emails(
    db: &PgPool,
    organization_id: &Uuid,
    user_emails: &[String],
) -> Result<(), ModelError> {
    for user_email in user_emails {
        let trimmed = user_email.trim();
        if trimmed.is_empty() {
            continue;
        }

        match users::get_user_by_email(trimmed, db).await {
            Ok(user) => {
                let update_request = users::UpdateUserRequest {
                    organization_id: Some(*organization_id),
                    ..Default::default()
                };
                if let Err(error) = users::update_user(&user.id, &update_request, db).await {
                    tracing::warn!(
                        "Failed to add user {} to organization {}: {:?}",
                        user.id,
                        organization_id,
                        error
                    );
                }
            }
            Err(error) => {
                tracing::warn!(
                    "Failed to resolve user email {} for organization {}: {:?}",
                    trimmed,
                    organization_id,
                    error
                );
            }
        }
    }

    Ok(())
}

impl PartialOrganization {
    pub fn to_values(&self) -> Vec<(OrganizationIden, sea_query::SimpleExpr)> {
        let mut values = vec![];
        if let Some(value) = &self.name {
            values.push((OrganizationIden::Name, value.into()));
        }
        if let Some(value) = &self.org_type {
            values.push((OrganizationIden::OrgType, value.clone().into()));
        }
        if let Some(value) = &self.contact_email {
            values.push((OrganizationIden::ContactEmail, value.clone().into()));
        }
        if let Some(value) = &self.external_url {
            values.push((OrganizationIden::ExternalUrl, value.clone().into()));
        }
        // TODO: think about how to handle pushing into array of removing from array
        // instead of simply overrding the array
        if let Some(value) = &self.regions {
            values.push((OrganizationIden::Regions, value.clone().into()));
        }
        if let Some(value) = &self.metadata {
            values.push((OrganizationIden::Metadata, value.clone().into()));
        }

        values
    }
}

#[instrument(err(Debug), skip(db))]
pub async fn update(
    db: &PgPool,
    id: &Uuid,
    update_org: &PartialOrganization,
) -> Result<Organization, ModelError> {
    let values = update_org.to_values();

    if values.is_empty() {
        return Err(ValidationError::NoValidUpdates.into());
    }

    let (sql, values) = Query::update()
        .table(OrganizationIden::Table)
        .values(values)
        .and_where(Expr::col(OrganizationIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let organization = sqlx::query_as_with(&sql, values).fetch_one(db).await?;

    Ok(organization)
}

/// Get the organization's `metadata` jsonb column
#[instrument(err(Debug), skip(db))]
pub async fn get_metadata(db: &PgPool, id: &Uuid) -> Result<Option<serde_json::Value>, ModelError> {
    let (sql, values) = Query::select()
        .columns([OrganizationIden::Metadata])
        .from(OrganizationIden::Table)
        .and_where(Expr::col((OrganizationIden::Table, OrganizationIden::Id)).eq(id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let (metadata,) = query_as_with::<_, (Option<serde_json::Value>,), _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(metadata)
}

/// Merge the supplied object into the organization's `metadata` jsonb column
/// at the top level. Existing keys are overwritten by the patch.
#[instrument(err(Debug), skip(db))]
pub async fn patch_metadata(
    db: &PgPool,
    id: &Uuid,
    patch: serde_json::Value,
) -> Result<Organization, ModelError> {
    if !patch.is_object() {
        return Err(
            ValidationError::BadRequest("metadata patch must be a JSON object".into()).into(),
        );
    }

    let organization = sqlx::query_as::<_, Organization>(
        "UPDATE organization
            SET metadata = COALESCE(metadata, '{}'::jsonb) || $1::jsonb,
                updated_at = NOW()
            WHERE id = $2
            RETURNING *",
    )
    .bind(patch)
    .bind(id)
    .fetch_one(db)
    .await
    .resolve_db_err("Organization")?;

    Ok(organization)
}

#[derive(Deserialize, Debug, JsonSchema, Default)]
pub struct OrganizationOrderOptions {
    pub name: Option<Order>,
    pub created_at: Option<Order>,
}

impl OrganizationOrderOptions {
    fn apply(&self, mut query: SelectStatement) -> SelectStatement {
        if let Some(order) = &self.name {
            query = query
                .order_by(
                    (OrganizationIden::Table, OrganizationIden::Name),
                    order.into(),
                )
                .to_owned();
        }
        if let Some(order) = &self.created_at {
            query = query
                .order_by(
                    (OrganizationIden::Table, OrganizationIden::CreatedAt),
                    order.into(),
                )
                .to_owned();
        }
        query
    }
}

#[derive(Deserialize, Debug, JsonSchema, Default)]
pub struct OrganizationFilterOptions {
    pub region_id: Option<Uuid>,
}

impl OrganizationFilterOptions {
    fn apply(&self, mut query: SelectStatement) -> SelectStatement {
        if let Some(value) = self.region_id {
            query = query
                .and_where(Expr::cust_with_values(
                    "organization.regions @> $1::uuid[]",
                    [vec![value]],
                ))
                .to_owned();
        }

        query
    }
}

#[instrument(err(Debug), skip(db))]
pub async fn list(
    db: &PgPool,
    page_options: PageOptions,
    filter_options: OrganizationFilterOptions,
    order_options: OrganizationOrderOptions,
    locale: &str,
) -> Result<PaginatedResults<LocalizedOrganization>, ModelError> {
    let query = Query::select()
        .from(OrganizationIden::Table)
        .columns(DEFAULT_COLUMNS.map(|col| (OrganizationIden::Table, col)))
        .to_owned();

    let query = LocalizedOrganization::query_to_localisation(query, locale);

    let query = filter_options.apply(query);
    let query = order_options.apply(query);
    let organizations = page_options.fetch_paginated_results(db, query).await?;

    Ok(organizations)
}

#[instrument(err(Debug), skip(db))]
pub async fn get_localized_by_id(
    db: &PgPool,
    id: &Uuid,
    locale: &str,
) -> Result<LocalizedOrganization, ModelError> {
    let query = Query::select()
        .columns(DEFAULT_COLUMNS.map(|col| (OrganizationIden::Table, col)))
        .from(OrganizationIden::Table)
        .and_where(Expr::col((OrganizationIden::Table, OrganizationIden::Id)).eq(id.to_owned()))
        .to_owned();

    let query = LocalizedOrganization::query_to_localisation(query, locale);

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let organization = query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Organization")?;

    Ok(organization)
}

#[instrument(err(Debug), skip(db))]
pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<Organization, ModelError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS.map(|col| (OrganizationIden::Table, col)))
        .from(OrganizationIden::Table)
        .and_where(Expr::col((OrganizationIden::Table, OrganizationIden::Id)).eq(id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let organization = query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Organization")?;

    Ok(organization)
}

#[instrument(err(Debug), skip(db))]
pub async fn delete(db: &PgPool, id: &Uuid) -> Result<Organization, ModelError> {
    let mut tx = db.begin().await?;

    sqlx::query("UPDATE comhairle_user SET organization_id = NULL WHERE organization_id = $1")
        .bind(id)
        .execute(&mut *tx)
        .await?;

    sqlx::query("UPDATE conversation SET organization_id = NULL WHERE organization_id = $1")
        .bind(id)
        .execute(&mut *tx)
        .await?;

    sqlx::query(
        "UPDATE email_template_config SET organization_id = NULL WHERE organization_id = $1",
    )
    .bind(id)
    .execute(&mut *tx)
    .await?;

    let (sql, values) = Query::delete()
        .from_table(OrganizationIden::Table)
        .and_where(Expr::col(OrganizationIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let organization = sqlx::query_as_with(&sql, values)
        .fetch_one(&mut *tx)
        .await
        .resolve_db_err("Organization")?;

    tx.commit().await?;

    Ok(organization)
}
