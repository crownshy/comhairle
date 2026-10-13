use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{PgConnection, PgExecutor, PgPool};
use uuid::Uuid;

use super::validation::{invalid, validate_question_draft};
use super::{DatePrecision, LocationFormat, QuestionType, VersionStatus, VersionSummary};
use crate::error::ComhairleError;
use crate::models::SqlxResultExt;
use crate::models::translations::{
    TextContentId, TextFormat, localize_translations, new_translation_in_connection,
};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, sqlx::FromRow)]
pub struct TagDto {
    pub id: Uuid,
    pub organization_id: Option<Uuid>,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct OptionDto {
    pub id: Uuid,
    pub position: i32,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct QuestionVersionDto {
    pub id: Uuid,
    pub question_id: Uuid,
    pub version_number: i32,
    pub status: VersionStatus,
    pub published_at: Option<DateTime<Utc>>,
    pub text: String,
    pub question_type: QuestionType,
    pub date_precision: Option<DatePrecision>,
    pub location_format: Option<LocationFormat>,
    pub location_country: Option<String>,
    pub max_selections: Option<i32>,
    pub allow_other: bool,
    pub prefer_not_to_say: bool,
    pub options: Vec<OptionDto>,
    pub json_schema: Option<Value>,
    pub special_category: bool,
    pub consent_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct QuestionDto {
    pub id: Uuid,
    pub organization_id: Option<Uuid>,
    pub forked_from_id: Option<Uuid>,
    pub archived_at: Option<DateTime<Utc>>,
    pub tags: Vec<TagDto>,
    pub versions: Vec<VersionSummary>,
    pub latest_published: Option<QuestionVersionDto>,
    pub draft: Option<QuestionVersionDto>,
}

/// Body for creating a question or replacing its draft.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QuestionDraftInput {
    pub text: String,
    pub question_type: QuestionType,
    #[serde(default)]
    pub date_precision: Option<DatePrecision>,
    #[serde(default)]
    pub location_format: Option<LocationFormat>,
    #[serde(default)]
    pub location_country: Option<String>,
    #[serde(default)]
    pub max_selections: Option<i32>,
    #[serde(default)]
    pub allow_other: bool,
    #[serde(default)]
    pub prefer_not_to_say: bool,
    /// Option labels in display order. Option ids are regenerated on every save.
    #[serde(default)]
    pub options: Vec<String>,
    #[serde(default)]
    pub json_schema: Option<Value>,
    #[serde(default)]
    pub special_category: bool,
    #[serde(default)]
    pub consent_text: Option<String>,
    #[serde(default)]
    pub tag_ids: Vec<Uuid>,
}

#[derive(sqlx::FromRow)]
struct QuestionRow {
    id: Uuid,
    organization_id: Option<Uuid>,
    forked_from_id: Option<Uuid>,
    archived_at: Option<DateTime<Utc>>,
}

#[derive(sqlx::FromRow)]
struct VersionRow {
    id: Uuid,
    question_id: Uuid,
    version_number: i32,
    status: VersionStatus,
    published_at: Option<DateTime<Utc>>,
    text_content_id: TextContentId,
    question_type: QuestionType,
    date_precision: Option<DatePrecision>,
    location_format: Option<LocationFormat>,
    location_country: Option<String>,
    max_selections: Option<i32>,
    allow_other: bool,
    prefer_not_to_say: bool,
    json_schema: Option<Value>,
    special_category: bool,
    consent_text_content_id: Option<TextContentId>,
}

#[derive(sqlx::FromRow)]
struct OptionRow {
    id: Uuid,
    question_version_id: Uuid,
    position: i32,
    label_text_content_id: TextContentId,
}

#[derive(sqlx::FromRow)]
struct QuestionTagRow {
    question_id: Uuid,
    #[sqlx(flatten)]
    tag: TagDto,
}

const VERSION_COLUMNS: &str = "id, question_id, version_number, status, published_at, \
    text_content_id, question_type, date_precision, location_format, location_country, \
    max_selections, allow_other, prefer_not_to_say, json_schema, special_category, \
    consent_text_content_id";

/// Questions visible to an owner: their own rows, plus system defaults that the owner
/// has not forked. `owner` is `None` for super admins, who see only system rows.
const VISIBLE_QUESTIONS_SQL: &str = "
    SELECT q.id, q.organization_id, q.forked_from_id, q.archived_at
    FROM demographics_question q
    WHERE ($2::uuid IS NULL OR q.id = $2::uuid)
      AND (
        q.organization_id IS NOT DISTINCT FROM $1::uuid
        OR (
          $1::uuid IS NOT NULL
          AND q.organization_id IS NULL
          AND NOT EXISTS (
            SELECT 1 FROM demographics_question f
            WHERE f.forked_from_id = q.id AND f.organization_id = $1::uuid
          )
        )
      )
    ORDER BY q.created_at, q.id";

async fn visible_question_rows(
    executor: impl PgExecutor<'_>,
    owner: Option<Uuid>,
    only: Option<Uuid>,
) -> Result<Vec<QuestionRow>, ComhairleError> {
    Ok(sqlx::query_as::<_, QuestionRow>(VISIBLE_QUESTIONS_SQL)
        .bind(owner)
        .bind(only)
        .fetch_all(executor)
        .await?)
}

/// Whether `owner` (or the system defaults when None) can see the question, archived or not.
pub async fn is_question_visible(
    executor: impl PgExecutor<'_>,
    owner: Option<Uuid>,
    question_id: Uuid,
) -> Result<bool, ComhairleError> {
    Ok(!visible_question_rows(executor, owner, Some(question_id))
        .await?
        .is_empty())
}

pub async fn list_questions(
    db: &PgPool,
    owner: Option<Uuid>,
    locale: &str,
) -> Result<Vec<QuestionDto>, ComhairleError> {
    let rows = visible_question_rows(db, owner, None).await?;
    question_dtos(db, rows, locale).await
}

pub async fn get_question(
    db: &PgPool,
    owner: Option<Uuid>,
    question_id: Uuid,
    locale: &str,
) -> Result<QuestionDto, ComhairleError> {
    let rows = visible_question_rows(db, owner, Some(question_id)).await?;
    question_dtos(db, rows, locale)
        .await?
        .pop()
        .ok_or_else(|| ComhairleError::ResourceNotFound("demographic question".into()))
}

async fn question_dtos(
    db: &PgPool,
    rows: Vec<QuestionRow>,
    locale: &str,
) -> Result<Vec<QuestionDto>, ComhairleError> {
    if rows.is_empty() {
        return Ok(Vec::new());
    }
    let ids: Vec<Uuid> = rows.iter().map(|row| row.id).collect();

    let tag_rows = sqlx::query_as::<_, QuestionTagRow>(
        "SELECT qt.question_id, t.id, t.organization_id, t.name
         FROM demographics_question_tag qt
         JOIN demographics_tag t ON t.id = qt.tag_id
         WHERE qt.question_id = ANY($1::uuid[])
         ORDER BY t.name",
    )
    .bind(&ids)
    .fetch_all(db)
    .await?;
    let mut tags_by_question: HashMap<Uuid, Vec<TagDto>> = HashMap::new();
    for row in tag_rows {
        tags_by_question
            .entry(row.question_id)
            .or_default()
            .push(row.tag);
    }

    let version_rows = sqlx::query_as::<_, VersionRow>(&format!(
        "SELECT {VERSION_COLUMNS} FROM demographics_question_version
         WHERE question_id = ANY($1::uuid[])
         ORDER BY question_id, version_number"
    ))
    .bind(&ids)
    .fetch_all(db)
    .await?;
    let mut versions_by_question: HashMap<Uuid, Vec<QuestionVersionDto>> = HashMap::new();
    for version in hydrate_versions(db, version_rows, locale).await? {
        versions_by_question
            .entry(version.question_id)
            .or_default()
            .push(version);
    }

    Ok(rows
        .into_iter()
        .map(|row| {
            let versions = versions_by_question.remove(&row.id).unwrap_or_default();
            let latest_published = versions
                .iter()
                .filter(|version| version.status == VersionStatus::Published)
                .max_by_key(|version| version.version_number)
                .cloned();
            let draft = versions
                .iter()
                .find(|version| version.status == VersionStatus::Draft)
                .cloned();
            QuestionDto {
                id: row.id,
                organization_id: row.organization_id,
                forked_from_id: row.forked_from_id,
                archived_at: row.archived_at,
                tags: tags_by_question.remove(&row.id).unwrap_or_default(),
                versions: versions.iter().map(version_summary).collect(),
                latest_published,
                draft,
            }
        })
        .collect())
}

fn version_summary(version: &QuestionVersionDto) -> VersionSummary {
    VersionSummary {
        id: version.id,
        version_number: version.version_number,
        status: version.status,
        published_at: version.published_at,
    }
}

async fn hydrate_versions(
    db: &PgPool,
    rows: Vec<VersionRow>,
    locale: &str,
) -> Result<Vec<QuestionVersionDto>, ComhairleError> {
    if rows.is_empty() {
        return Ok(Vec::new());
    }
    let version_ids: Vec<Uuid> = rows.iter().map(|row| row.id).collect();
    let option_rows = sqlx::query_as::<_, OptionRow>(
        "SELECT id, question_version_id, position, label_text_content_id
         FROM demographics_question_option
         WHERE question_version_id = ANY($1::uuid[])
         ORDER BY question_version_id, position",
    )
    .bind(&version_ids)
    .fetch_all(db)
    .await?;

    let mut text_ids: Vec<TextContentId> = rows.iter().map(|row| row.text_content_id).collect();
    text_ids.extend(rows.iter().filter_map(|row| row.consent_text_content_id));
    text_ids.extend(
        option_rows
            .iter()
            .map(|option| option.label_text_content_id),
    );
    let texts = localize_translations(db, &text_ids, locale).await?;

    let mut options_by_version: HashMap<Uuid, Vec<OptionDto>> = HashMap::new();
    for option in option_rows {
        options_by_version
            .entry(option.question_version_id)
            .or_default()
            .push(OptionDto {
                id: option.id,
                position: option.position,
                label: texts
                    .get(&option.label_text_content_id)
                    .cloned()
                    .unwrap_or_default(),
            });
    }

    Ok(rows
        .into_iter()
        .map(|row| QuestionVersionDto {
            id: row.id,
            question_id: row.question_id,
            version_number: row.version_number,
            status: row.status,
            published_at: row.published_at,
            text: texts.get(&row.text_content_id).cloned().unwrap_or_default(),
            question_type: row.question_type,
            date_precision: row.date_precision,
            location_format: row.location_format,
            location_country: row.location_country,
            max_selections: row.max_selections,
            allow_other: row.allow_other,
            prefer_not_to_say: row.prefer_not_to_say,
            options: options_by_version.remove(&row.id).unwrap_or_default(),
            json_schema: row.json_schema,
            special_category: row.special_category,
            consent_text: row
                .consent_text_content_id
                .and_then(|id| texts.get(&id).cloned()),
        })
        .collect())
}

/// Loads specific versions by id, in the order requested. Missing ids are skipped.
pub async fn load_versions(
    db: &PgPool,
    version_ids: &[Uuid],
    locale: &str,
) -> Result<Vec<QuestionVersionDto>, ComhairleError> {
    let rows = sqlx::query_as::<_, VersionRow>(&format!(
        "SELECT {VERSION_COLUMNS} FROM demographics_question_version WHERE id = ANY($1::uuid[])"
    ))
    .bind(version_ids)
    .fetch_all(db)
    .await?;
    let mut by_id: HashMap<Uuid, QuestionVersionDto> = hydrate_versions(db, rows, locale)
        .await?
        .into_iter()
        .map(|version| (version.id, version))
        .collect();
    Ok(version_ids
        .iter()
        .filter_map(|id| by_id.remove(id))
        .collect())
}

pub async fn load_version(
    db: &PgPool,
    version_id: Uuid,
    locale: &str,
) -> Result<QuestionVersionDto, ComhairleError> {
    load_versions(db, &[version_id], locale)
        .await?
        .pop()
        .ok_or_else(|| ComhairleError::ResourceNotFound("demographic question version".into()))
}

pub async fn load_latest_published(
    db: &PgPool,
    question_id: Uuid,
    locale: &str,
) -> Result<Option<QuestionVersionDto>, ComhairleError> {
    let Some(version_id) = latest_published_version_id(db, question_id).await? else {
        return Ok(None);
    };
    Ok(Some(load_version(db, version_id, locale).await?))
}

pub async fn latest_published_version_id(
    executor: impl PgExecutor<'_>,
    question_id: Uuid,
) -> Result<Option<Uuid>, ComhairleError> {
    Ok(sqlx::query_scalar(
        "SELECT id FROM demographics_question_version
         WHERE question_id = $1 AND status = 'published'
         ORDER BY version_number DESC LIMIT 1",
    )
    .bind(question_id)
    .fetch_optional(executor)
    .await?)
}

async fn draft_version_id(
    conn: &mut PgConnection,
    question_id: Uuid,
) -> Result<Option<Uuid>, ComhairleError> {
    Ok(sqlx::query_scalar(
        "SELECT id FROM demographics_question_version WHERE question_id = $1 AND status = 'draft'",
    )
    .bind(question_id)
    .fetch_optional(&mut *conn)
    .await?)
}

/// Returns the published version of a question that a form or step may reference.
pub async fn usable_published_version(
    conn: &mut PgConnection,
    owner: Option<Uuid>,
    question_id: Uuid,
) -> Result<Uuid, ComhairleError> {
    let visible = visible_question_rows(&mut *conn, owner, Some(question_id)).await?;
    let question = visible
        .first()
        .ok_or_else(|| invalid("demographic question is not available to this organization"))?;
    if question.archived_at.is_some() {
        return Err(invalid("demographic question is archived"));
    }
    latest_published_version_id(&mut *conn, question_id)
        .await?
        .ok_or_else(|| invalid("demographic question has no published version"))
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Resolve {
    /// Writing to a system default forks it for the owner when no fork exists.
    ForkDefault,
    /// Writing to a system default only reaches the owner's existing fork.
    ExistingOnly,
}

fn not_found() -> ComhairleError {
    ComhairleError::ResourceNotFound("demographic question".into())
}

/// Maps the requested question to the row this owner may write, which is the
/// owner's own question or their fork of a system default.
async fn resolve_writable(
    conn: &mut PgConnection,
    owner: Option<Uuid>,
    question_id: Uuid,
    resolve: Resolve,
) -> Result<Uuid, ComhairleError> {
    // Locking the row serializes concurrent forks of the same default.
    let organization_id: Option<Option<Uuid>> = sqlx::query_scalar(
        "SELECT organization_id FROM demographics_question WHERE id = $1 FOR UPDATE",
    )
    .bind(question_id)
    .fetch_optional(&mut *conn)
    .await?;
    let organization_id = organization_id.ok_or_else(not_found)?;

    if organization_id == owner {
        return Ok(question_id);
    }
    match (organization_id, owner, resolve) {
        (None, Some(org), Resolve::ForkDefault) => fork_default(conn, org, question_id).await,
        (None, Some(org), Resolve::ExistingOnly) => find_fork(conn, org, question_id)
            .await?
            .ok_or_else(not_found),
        _ => Err(not_found()),
    }
}

async fn find_fork(
    conn: &mut PgConnection,
    organization_id: Uuid,
    default_id: Uuid,
) -> Result<Option<Uuid>, ComhairleError> {
    Ok(sqlx::query_scalar(
        "SELECT id FROM demographics_question WHERE organization_id = $1 AND forked_from_id = $2",
    )
    .bind(organization_id)
    .bind(default_id)
    .fetch_optional(&mut *conn)
    .await?)
}

/// Creates the organization's copy of a system default, with the default's latest
/// published content as a new draft. Returns the existing fork if there is one.
async fn fork_default(
    conn: &mut PgConnection,
    organization_id: Uuid,
    default_id: Uuid,
) -> Result<Uuid, ComhairleError> {
    if let Some(existing) = find_fork(&mut *conn, organization_id, default_id).await? {
        return Ok(existing);
    }
    let source = latest_published_version_id(&mut *conn, default_id)
        .await?
        .ok_or_else(|| invalid("system default question has no published version"))?;

    let fork_id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO demographics_question (id, organization_id, forked_from_id)
         VALUES ($1, $2, $3)",
    )
    .bind(fork_id)
    .bind(organization_id)
    .bind(default_id)
    .execute(&mut *conn)
    .await?;
    sqlx::query(
        "INSERT INTO demographics_question_tag (question_id, tag_id)
         SELECT $1::uuid, tag_id FROM demographics_question_tag WHERE question_id = $2::uuid",
    )
    .bind(fork_id)
    .bind(default_id)
    .execute(&mut *conn)
    .await?;
    copy_version_to_draft(&mut *conn, source, fork_id).await?;
    Ok(fork_id)
}

/// Copies a version (and its options) into a new draft of `question_id`, numbered
/// after every existing version so numbers never repeat.
async fn copy_version_to_draft(
    conn: &mut PgConnection,
    source_version_id: Uuid,
    question_id: Uuid,
) -> Result<Uuid, ComhairleError> {
    let draft_id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO demographics_question_version (
            id, question_id, version_number, status, text_content_id, question_type,
            date_precision, location_format, location_country, max_selections, allow_other,
            prefer_not_to_say, json_schema, special_category, consent_text_content_id
        )
        SELECT $1::uuid, $2::uuid,
            (SELECT COALESCE(MAX(version_number), 0) + 1
             FROM demographics_question_version WHERE question_id = $2::uuid),
            'draft', text_content_id, question_type, date_precision, location_format,
            location_country, max_selections, allow_other, prefer_not_to_say, json_schema,
            special_category, consent_text_content_id
        FROM demographics_question_version WHERE id = $3::uuid",
    )
    .bind(draft_id)
    .bind(question_id)
    .bind(source_version_id)
    .execute(&mut *conn)
    .await?;
    sqlx::query(
        "INSERT INTO demographics_question_option (id, question_version_id, position, label_text_content_id)
         SELECT uuid_generate_v4(), $1::uuid, position, label_text_content_id
         FROM demographics_question_option WHERE question_version_id = $2::uuid",
    )
    .bind(draft_id)
    .bind(source_version_id)
    .execute(&mut *conn)
    .await?;
    Ok(draft_id)
}

async fn reject_if_archived(
    conn: &mut PgConnection,
    question_id: Uuid,
) -> Result<(), ComhairleError> {
    let archived: bool = sqlx::query_scalar(
        "SELECT archived_at IS NOT NULL FROM demographics_question WHERE id = $1",
    )
    .bind(question_id)
    .fetch_one(&mut *conn)
    .await?;
    if archived {
        return Err(ComhairleError::Conflict(
            "archived demographic questions cannot be edited".into(),
        ));
    }
    Ok(())
}

async fn require_draft(conn: &mut PgConnection, question_id: Uuid) -> Result<Uuid, ComhairleError> {
    draft_version_id(conn, question_id)
        .await?
        .ok_or_else(|| ComhairleError::ResourceNotFound("draft for demographic question".into()))
}

fn unique_ids(ids: &[Uuid]) -> Vec<Uuid> {
    let mut seen = HashSet::new();
    ids.iter().copied().filter(|id| seen.insert(*id)).collect()
}

/// Tags must be system tags or belong to the owner's organization.
async fn validate_tag_ids(
    conn: &mut PgConnection,
    owner: Option<Uuid>,
    tag_ids: &[Uuid],
) -> Result<(), ComhairleError> {
    let ids = unique_ids(tag_ids);
    if ids.is_empty() {
        return Ok(());
    }
    let usable: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM demographics_tag
         WHERE id = ANY($1::uuid[]) AND (organization_id IS NULL OR organization_id = $2::uuid)",
    )
    .bind(&ids)
    .bind(owner)
    .fetch_one(&mut *conn)
    .await?;
    if usable != ids.len() as i64 {
        return Err(invalid(
            "one or more tags do not exist for this organization",
        ));
    }
    Ok(())
}

async fn replace_tags(
    conn: &mut PgConnection,
    question_id: Uuid,
    tag_ids: &[Uuid],
) -> Result<(), ComhairleError> {
    sqlx::query("DELETE FROM demographics_question_tag WHERE question_id = $1")
        .bind(question_id)
        .execute(&mut *conn)
        .await?;
    let ids = unique_ids(tag_ids);
    if ids.is_empty() {
        return Ok(());
    }
    sqlx::query(
        "INSERT INTO demographics_question_tag (question_id, tag_id)
         SELECT $1::uuid, UNNEST($2::uuid[])",
    )
    .bind(question_id)
    .bind(&ids)
    .execute(&mut *conn)
    .await?;
    Ok(())
}

/// Text content is immutable, so every save writes new rows for text fields.
struct DraftText {
    text: TextContentId,
    consent: Option<TextContentId>,
    options: Vec<TextContentId>,
}

async fn write_text(
    conn: &mut PgConnection,
    locale: &str,
    content: &str,
) -> Result<TextContentId, ComhairleError> {
    Ok(
        new_translation_in_connection(conn, locale, content, TextFormat::Plain)
            .await?
            .id,
    )
}

async fn write_draft_text(
    conn: &mut PgConnection,
    locale: &str,
    input: &QuestionDraftInput,
) -> Result<DraftText, ComhairleError> {
    let text = write_text(&mut *conn, locale, input.text.trim()).await?;
    let consent = match (input.special_category, input.consent_text.as_deref()) {
        (true, Some(consent)) => Some(write_text(&mut *conn, locale, consent.trim()).await?),
        _ => None,
    };
    let mut options = Vec::with_capacity(input.options.len());
    for label in &input.options {
        options.push(write_text(&mut *conn, locale, label.trim()).await?);
    }
    Ok(DraftText {
        text,
        consent,
        options,
    })
}

async fn insert_draft_version(
    conn: &mut PgConnection,
    question_id: Uuid,
    version_number: i32,
    input: &QuestionDraftInput,
    texts: &DraftText,
) -> Result<Uuid, ComhairleError> {
    let version_id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO demographics_question_version (
            id, question_id, version_number, status, text_content_id, question_type,
            date_precision, location_format, location_country, max_selections, allow_other,
            prefer_not_to_say, json_schema, special_category, consent_text_content_id
        ) VALUES ($1, $2, $3, 'draft', $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)",
    )
    .bind(version_id)
    .bind(question_id)
    .bind(version_number)
    .bind(texts.text)
    .bind(input.question_type)
    .bind(input.date_precision)
    .bind(input.location_format)
    .bind(input.location_country.as_deref().map(str::trim))
    .bind(input.max_selections)
    .bind(input.allow_other)
    .bind(input.prefer_not_to_say)
    .bind(input.json_schema.as_ref())
    .bind(input.special_category)
    .bind(texts.consent)
    .execute(&mut *conn)
    .await?;
    replace_options(&mut *conn, version_id, &texts.options).await?;
    Ok(version_id)
}

async fn save_draft_fields(
    conn: &mut PgConnection,
    version_id: Uuid,
    input: &QuestionDraftInput,
    texts: &DraftText,
) -> Result<(), ComhairleError> {
    sqlx::query(
        "UPDATE demographics_question_version SET
            text_content_id = $2,
            consent_text_content_id = $3,
            question_type = $4,
            date_precision = $5,
            location_format = $6,
            location_country = $7,
            max_selections = $8,
            allow_other = $9,
            prefer_not_to_say = $10,
            json_schema = $11,
            special_category = $12,
            updated_at = NOW()
         WHERE id = $1",
    )
    .bind(version_id)
    .bind(texts.text)
    .bind(texts.consent)
    .bind(input.question_type)
    .bind(input.date_precision)
    .bind(input.location_format)
    .bind(input.location_country.as_deref().map(str::trim))
    .bind(input.max_selections)
    .bind(input.allow_other)
    .bind(input.prefer_not_to_say)
    .bind(input.json_schema.as_ref())
    .bind(input.special_category)
    .execute(&mut *conn)
    .await?;
    replace_options(&mut *conn, version_id, &texts.options).await
}

async fn replace_options(
    conn: &mut PgConnection,
    version_id: Uuid,
    label_ids: &[TextContentId],
) -> Result<(), ComhairleError> {
    sqlx::query("DELETE FROM demographics_question_option WHERE question_version_id = $1")
        .bind(version_id)
        .execute(&mut *conn)
        .await?;
    for (position, label_id) in label_ids.iter().enumerate() {
        sqlx::query(
            "INSERT INTO demographics_question_option (id, question_version_id, position, label_text_content_id)
             VALUES ($1, $2, $3, $4)",
        )
        .bind(Uuid::new_v4())
        .bind(version_id)
        .bind(position as i32)
        .bind(label_id)
        .execute(&mut *conn)
        .await?;
    }
    Ok(())
}

pub async fn create_question(
    db: &PgPool,
    owner: Option<Uuid>,
    input: &QuestionDraftInput,
    locale: &str,
) -> Result<QuestionDto, ComhairleError> {
    validate_question_draft(input)?;
    let question_id = Uuid::new_v4();
    let mut tx = db.begin().await?;
    let conn = &mut *tx;
    validate_tag_ids(conn, owner, &input.tag_ids).await?;
    sqlx::query("INSERT INTO demographics_question (id, organization_id) VALUES ($1, $2)")
        .bind(question_id)
        .bind(owner)
        .execute(&mut *conn)
        .await?;
    let texts = write_draft_text(conn, locale, input).await?;
    insert_draft_version(conn, question_id, 1, input, &texts).await?;
    replace_tags(conn, question_id, &input.tag_ids).await?;
    tx.commit().await?;
    get_question(db, owner, question_id, locale).await
}

/// Starts a draft from the latest published version. Starting a draft on a system
/// default forks it for the owner, and the response is the fork.
pub async fn start_draft(
    db: &PgPool,
    owner: Option<Uuid>,
    question_id: Uuid,
    locale: &str,
) -> Result<QuestionDto, ComhairleError> {
    let mut tx = db.begin().await?;
    let conn = &mut *tx;
    let target = resolve_writable(conn, owner, question_id, Resolve::ForkDefault).await?;
    reject_if_archived(conn, target).await?;
    if draft_version_id(conn, target).await?.is_some() {
        if target == question_id {
            return Err(ComhairleError::Conflict(
                "demographic question already has a draft".into(),
            ));
        }
    } else {
        let source = latest_published_version_id(&mut *conn, target)
            .await?
            .ok_or_else(|| invalid("demographic question has no published version to edit"))?;
        copy_version_to_draft(conn, source, target).await?;
    }
    tx.commit().await?;
    get_question(db, owner, target, locale).await
}

/// Replaces the draft's content. Editing a system default forks it for the owner.
pub async fn update_draft(
    db: &PgPool,
    owner: Option<Uuid>,
    question_id: Uuid,
    input: &QuestionDraftInput,
    locale: &str,
) -> Result<QuestionDto, ComhairleError> {
    validate_question_draft(input)?;
    let mut tx = db.begin().await?;
    let conn = &mut *tx;
    let target = resolve_writable(conn, owner, question_id, Resolve::ForkDefault).await?;
    reject_if_archived(conn, target).await?;
    let draft_id = require_draft(conn, target).await?;
    validate_tag_ids(conn, owner, &input.tag_ids).await?;
    let texts = write_draft_text(conn, locale, input).await?;
    save_draft_fields(conn, draft_id, input, &texts).await?;
    replace_tags(conn, target, &input.tag_ids).await?;
    tx.commit().await?;
    get_question(db, owner, target, locale).await
}

/// Deletes the draft row. Published versions are untouched, and a question that was
/// never published is removed, which makes a discarded fork visible as the default again.
pub async fn discard_draft(
    db: &PgPool,
    owner: Option<Uuid>,
    question_id: Uuid,
) -> Result<(), ComhairleError> {
    let mut tx = db.begin().await?;
    let conn = &mut *tx;
    let target = resolve_writable(conn, owner, question_id, Resolve::ExistingOnly).await?;
    let deleted = sqlx::query(
        "DELETE FROM demographics_question_version WHERE question_id = $1 AND status = 'draft'",
    )
    .bind(target)
    .execute(&mut *conn)
    .await?
    .rows_affected();
    if deleted == 0 {
        return Err(ComhairleError::ResourceNotFound(
            "draft for demographic question".into(),
        ));
    }
    sqlx::query(
        "DELETE FROM demographics_question q
         WHERE q.id = $1
           AND NOT EXISTS (SELECT 1 FROM demographics_question_version v WHERE v.question_id = q.id)",
    )
    .bind(target)
    .execute(&mut *conn)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn publish_question(
    db: &PgPool,
    owner: Option<Uuid>,
    question_id: Uuid,
    locale: &str,
) -> Result<QuestionDto, ComhairleError> {
    let mut tx = db.begin().await?;
    let conn = &mut *tx;
    let target = resolve_writable(conn, owner, question_id, Resolve::ExistingOnly).await?;
    reject_if_archived(conn, target).await?;
    let draft_id = require_draft(conn, target).await?;
    let tag_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM demographics_question_tag WHERE question_id = $1")
            .bind(target)
            .fetch_one(&mut *conn)
            .await?;
    if tag_count == 0 {
        return Err(invalid(
            "a demographic question needs at least one tag before publishing",
        ));
    }
    sqlx::query(
        "UPDATE demographics_question_version
         SET status = 'published', published_at = NOW(), updated_at = NOW()
         WHERE id = $1",
    )
    .bind(draft_id)
    .execute(&mut *conn)
    .await?;
    tx.commit().await?;
    get_question(db, owner, target, locale).await
}

pub async fn archive_question(
    db: &PgPool,
    owner: Option<Uuid>,
    question_id: Uuid,
    locale: &str,
) -> Result<QuestionDto, ComhairleError> {
    let mut tx = db.begin().await?;
    let conn = &mut *tx;
    let target = resolve_writable(conn, owner, question_id, Resolve::ExistingOnly).await?;
    sqlx::query(
        "UPDATE demographics_question SET archived_at = COALESCE(archived_at, NOW()) WHERE id = $1",
    )
    .bind(target)
    .execute(&mut *conn)
    .await?;
    tx.commit().await?;
    get_question(db, owner, target, locale).await
}

pub async fn list_tags(db: &PgPool, owner: Option<Uuid>) -> Result<Vec<TagDto>, ComhairleError> {
    Ok(sqlx::query_as::<_, TagDto>(
        "SELECT id, organization_id, name FROM demographics_tag
         WHERE organization_id IS NULL OR organization_id = $1
         ORDER BY name",
    )
    .bind(owner)
    .fetch_all(db)
    .await?)
}

pub async fn create_tag(
    db: &PgPool,
    owner: Option<Uuid>,
    name: &str,
) -> Result<TagDto, ComhairleError> {
    let name = name.trim();
    if name.is_empty() {
        return Err(invalid("tag name must not be blank"));
    }
    sqlx::query_as::<_, TagDto>(
        "INSERT INTO demographics_tag (organization_id, name) VALUES ($1, $2)
         RETURNING id, organization_id, name",
    )
    .bind(owner)
    .bind(name)
    .fetch_one(db)
    .await
    .resolve_db_err("demographics tag")
}
