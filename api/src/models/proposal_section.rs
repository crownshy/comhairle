use chrono::{DateTime, Utc};
use comhairle_macros::Translatable;
use schemars::JsonSchema;
use sea_query::{Expr, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow, query_as_with};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::{
        SqlxResultExt,
        translations::{TextContentId, TextFormat, new_translation},
    },
};

/// A single translatable body section of a [`crate::models::proposal::Proposal`].
///
/// Proposals are made up of an ordered list of sections (by `position`). Each
/// section owns its own translatable text content, so it can be edited,
/// translated and reordered independently of the others.
#[derive(Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema, Translatable)]
#[enum_def(table_name = "proposal_evaluation_proposal_section")]
pub struct ProposalSection {
    pub id: Uuid,
    pub proposal_id: Uuid,
    pub position: i32,
    pub body: TextContentId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [ProposalSectionIden; 6] = [
    ProposalSectionIden::Id,
    ProposalSectionIden::ProposalId,
    ProposalSectionIden::Position,
    ProposalSectionIden::Body,
    ProposalSectionIden::CreatedAt,
    ProposalSectionIden::UpdatedAt,
];

/// Creates a new section for a proposal, storing `body` as translatable rich content.
#[instrument(err(Debug))]
pub async fn create(
    db: &PgPool,
    proposal_id: &Uuid,
    position: i32,
    body: &str,
    locale: &str,
) -> Result<ProposalSection, ComhairleError> {
    let content = new_translation(db, locale, body, TextFormat::Rich).await?;

    let (sql, values) = Query::insert()
        .into_table(ProposalSectionIden::Table)
        .columns([
            ProposalSectionIden::ProposalId,
            ProposalSectionIden::Position,
            ProposalSectionIden::Body,
        ])
        .values([(*proposal_id).into(), position.into(), content.id.into()])?
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let section = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(section)
}

/// Returns the next `position` value for a proposal (max existing + 1, or 0).
#[instrument(err(Debug))]
pub async fn next_position(db: &PgPool, proposal_id: &Uuid) -> Result<i32, ComhairleError> {
    let sections = list(db, proposal_id).await?;
    Ok(sections
        .iter()
        .map(|s| s.position)
        .max()
        .map_or(0, |m| m + 1))
}

/// Lists the raw sections for a proposal, ordered by `position`.
#[instrument(err(Debug))]
pub async fn list(db: &PgPool, proposal_id: &Uuid) -> Result<Vec<ProposalSection>, ComhairleError> {
    let (sql, values) = Query::select()
        .from(ProposalSectionIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(Expr::col(ProposalSectionIden::ProposalId).eq(proposal_id.to_owned()))
        .order_by(ProposalSectionIden::Position, sea_query::Order::Asc)
        .build_sqlx(PostgresQueryBuilder);

    let sections = query_as_with(&sql, values).fetch_all(db).await?;

    Ok(sections)
}

/// Lists sections for a proposal with `body` resolved to the requested locale.
#[instrument(err(Debug))]
pub async fn list_localized(
    db: &PgPool,
    proposal_id: &Uuid,
    locale: &str,
) -> Result<Vec<LocalizedProposalSection>, ComhairleError> {
    let query = Query::select()
        .from(ProposalSectionIden::Table)
        .columns(DEFAULT_COLUMNS.map(|col| (ProposalSectionIden::Table, col)))
        .and_where(
            Expr::col((ProposalSectionIden::Table, ProposalSectionIden::ProposalId))
                .eq(proposal_id.to_owned()),
        )
        .order_by(
            (ProposalSectionIden::Table, ProposalSectionIden::Position),
            sea_query::Order::Asc,
        )
        .to_owned();

    let query = LocalizedProposalSection::query_to_localisation(query, locale);

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let sections = query_as_with(&sql, values).fetch_all(db).await?;

    Ok(sections)
}

/// Lists sections for a proposal together with their full translation metadata.
#[instrument(err(Debug))]
pub async fn list_with_translations(
    db: &PgPool,
    proposal_id: &Uuid,
    locale: &str,
) -> Result<Vec<ProposalSectionWithTranslations>, ComhairleError> {
    let sections = list(db, proposal_id).await?;
    let mut out = Vec::with_capacity(sections.len());
    for section in sections {
        out.push(ProposalSectionWithTranslations::from_original(db, section, locale).await?);
    }
    Ok(out)
}

/// Deletes a single section.
#[instrument(err(Debug))]
pub async fn delete(db: &PgPool, id: &Uuid) -> Result<ProposalSection, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(ProposalSectionIden::Table)
        .and_where(Expr::col(ProposalSectionIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let section = query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("ProposalSection")?;

    Ok(section)
}
