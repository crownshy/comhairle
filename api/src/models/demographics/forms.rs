use std::collections::HashMap;

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

use super::questions::usable_published_version;
use super::validation::invalid;
use super::{VersionStatus, VersionSummary};
use crate::error::ComhairleError;
use crate::models::translations::{TextContentId, localize_translations};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FormEntryInput {
    pub question_id: Uuid,
    pub position: i32,
    #[serde(default = "required_by_default")]
    pub required: bool,
}

/// Body for creating a form or replacing its draft.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct FormDraftInput {
    pub name: String,
    #[serde(default)]
    pub entries: Vec<FormEntryInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FormEntryDto {
    pub question_id: Uuid,
    /// The pinned question version. Empty on drafts.
    pub question_version_id: Option<Uuid>,
    pub position: i32,
    pub required: bool,
    /// Text of the pinned version, or of the latest published version on drafts.
    pub question_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FormVersionDto {
    pub id: Uuid,
    pub form_id: Uuid,
    pub version_number: i32,
    pub status: VersionStatus,
    pub published_at: Option<DateTime<Utc>>,
    pub name: String,
    pub entries: Vec<FormEntryDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FormDto {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub archived_at: Option<DateTime<Utc>>,
    pub versions: Vec<VersionSummary>,
    pub latest_published: Option<FormVersionDto>,
    pub draft: Option<FormVersionDto>,
}

fn required_by_default() -> bool {
    true
}

#[derive(sqlx::FromRow)]
struct FormRow {
    id: Uuid,
    organization_id: Uuid,
    archived_at: Option<DateTime<Utc>>,
}

#[derive(sqlx::FromRow)]
struct FormVersionRow {
    id: Uuid,
    form_id: Uuid,
    version_number: i32,
    status: VersionStatus,
    name: String,
    published_at: Option<DateTime<Utc>>,
}

#[derive(sqlx::FromRow)]
struct FormEntryRow {
    form_version_id: Uuid,
    question_id: Uuid,
    question_version_id: Option<Uuid>,
    position: i32,
    required: bool,
    display_text_content_id: Option<TextContentId>,
}

const FORM_COLUMNS: &str = "id, organization_id, archived_at";

/// Entry text comes from the pinned version, or from the latest published version
/// while the form has no pinned version (drafts).
const ENTRY_SQL: &str = "
    SELECT e.form_version_id, e.question_id, e.question_version_id, e.position, e.required,
           qv.text_content_id AS display_text_content_id
    FROM demographics_form_version_question e
    LEFT JOIN LATERAL (
        SELECT v.text_content_id
        FROM demographics_question_version v
        WHERE v.question_id = e.question_id
          AND v.status = 'published'
          AND (e.question_version_id IS NULL OR v.id = e.question_version_id)
        ORDER BY v.version_number DESC
        LIMIT 1
    ) qv ON TRUE
    WHERE e.form_version_id = ANY($1::uuid[])
    ORDER BY e.form_version_id, e.position";

/// Entries of the given form versions, grouped by version id, with localized question text.
pub async fn load_entries(
    db: &PgPool,
    form_version_ids: &[Uuid],
    locale: &str,
) -> Result<HashMap<Uuid, Vec<FormEntryDto>>, ComhairleError> {
    let rows = sqlx::query_as::<_, FormEntryRow>(ENTRY_SQL)
        .bind(form_version_ids)
        .fetch_all(db)
        .await?;
    let text_ids: Vec<TextContentId> = rows
        .iter()
        .filter_map(|row| row.display_text_content_id)
        .collect();
    let texts = localize_translations(db, &text_ids, locale).await?;

    let mut entries: HashMap<Uuid, Vec<FormEntryDto>> = HashMap::new();
    for row in rows {
        let question_text = row
            .display_text_content_id
            .and_then(|id| texts.get(&id).cloned())
            .unwrap_or_default();
        entries
            .entry(row.form_version_id)
            .or_default()
            .push(FormEntryDto {
                question_id: row.question_id,
                question_version_id: row.question_version_id,
                position: row.position,
                required: row.required,
                question_text,
            });
    }
    Ok(entries)
}

async fn hydrate_forms(
    db: &PgPool,
    rows: Vec<FormRow>,
    locale: &str,
) -> Result<Vec<FormDto>, ComhairleError> {
    if rows.is_empty() {
        return Ok(Vec::new());
    }
    let form_ids: Vec<Uuid> = rows.iter().map(|row| row.id).collect();
    let version_rows = sqlx::query_as::<_, FormVersionRow>(
        "SELECT id, form_id, version_number, status, name, published_at
         FROM demographics_form_version
         WHERE form_id = ANY($1::uuid[])
         ORDER BY form_id, version_number",
    )
    .bind(&form_ids)
    .fetch_all(db)
    .await?;
    let version_ids: Vec<Uuid> = version_rows.iter().map(|row| row.id).collect();
    let mut entries = load_entries(db, &version_ids, locale).await?;

    let mut versions_by_form: HashMap<Uuid, Vec<FormVersionDto>> = HashMap::new();
    for row in version_rows {
        versions_by_form
            .entry(row.form_id)
            .or_default()
            .push(FormVersionDto {
                id: row.id,
                form_id: row.form_id,
                version_number: row.version_number,
                status: row.status,
                published_at: row.published_at,
                name: row.name,
                entries: entries.remove(&row.id).unwrap_or_default(),
            });
    }

    Ok(rows
        .into_iter()
        .map(|row| {
            let versions = versions_by_form.remove(&row.id).unwrap_or_default();
            FormDto {
                id: row.id,
                organization_id: row.organization_id,
                archived_at: row.archived_at,
                versions: versions
                    .iter()
                    .map(|version| VersionSummary {
                        id: version.id,
                        version_number: version.version_number,
                        status: version.status,
                        published_at: version.published_at,
                    })
                    .collect(),
                latest_published: versions
                    .iter()
                    .filter(|version| version.status == VersionStatus::Published)
                    .max_by_key(|version| version.version_number)
                    .cloned(),
                draft: versions
                    .iter()
                    .find(|version| version.status == VersionStatus::Draft)
                    .cloned(),
            }
        })
        .collect())
}

pub async fn list_forms(
    db: &PgPool,
    organization_id: Uuid,
    locale: &str,
) -> Result<Vec<FormDto>, ComhairleError> {
    let rows = sqlx::query_as::<_, FormRow>(&format!(
        "SELECT {FORM_COLUMNS} FROM demographics_form
         WHERE organization_id = $1
         ORDER BY created_at, id"
    ))
    .bind(organization_id)
    .fetch_all(db)
    .await?;
    hydrate_forms(db, rows, locale).await
}

pub async fn get_form(
    db: &PgPool,
    organization_id: Uuid,
    form_id: Uuid,
    locale: &str,
) -> Result<FormDto, ComhairleError> {
    let rows = sqlx::query_as::<_, FormRow>(&format!(
        "SELECT {FORM_COLUMNS} FROM demographics_form WHERE id = $1 AND organization_id = $2"
    ))
    .bind(form_id)
    .bind(organization_id)
    .fetch_all(db)
    .await?;
    hydrate_forms(db, rows, locale)
        .await?
        .pop()
        .ok_or_else(|| ComhairleError::ResourceNotFound("demographic form".into()))
}

fn clean_name(raw: &str) -> Result<&str, ComhairleError> {
    let name = raw.trim();
    if name.is_empty() {
        return Err(invalid("form name must not be blank"));
    }
    Ok(name)
}

/// Locks the form for the organization and returns its archived_at.
async fn lock_form(
    conn: &mut PgConnection,
    organization_id: Uuid,
    form_id: Uuid,
) -> Result<Option<DateTime<Utc>>, ComhairleError> {
    let archived: Option<Option<DateTime<Utc>>> = sqlx::query_scalar(
        "SELECT archived_at FROM demographics_form
         WHERE id = $1 AND organization_id = $2
         FOR UPDATE",
    )
    .bind(form_id)
    .bind(organization_id)
    .fetch_optional(&mut *conn)
    .await?;
    archived.ok_or_else(|| ComhairleError::ResourceNotFound("demographic form".into()))
}

async fn lock_editable_form(
    conn: &mut PgConnection,
    organization_id: Uuid,
    form_id: Uuid,
) -> Result<(), ComhairleError> {
    if lock_form(conn, organization_id, form_id).await?.is_some() {
        return Err(ComhairleError::Conflict(
            "archived demographic forms cannot be edited".into(),
        ));
    }
    Ok(())
}

async fn draft_form_version_id(
    conn: &mut PgConnection,
    form_id: Uuid,
) -> Result<Option<Uuid>, ComhairleError> {
    Ok(sqlx::query_scalar(
        "SELECT id FROM demographics_form_version WHERE form_id = $1 AND status = 'draft'",
    )
    .bind(form_id)
    .fetch_optional(&mut *conn)
    .await?)
}

async fn require_form_draft(
    conn: &mut PgConnection,
    form_id: Uuid,
) -> Result<Uuid, ComhairleError> {
    draft_form_version_id(conn, form_id)
        .await?
        .ok_or_else(|| ComhairleError::ResourceNotFound("draft for demographic form".into()))
}

/// Returns the latest published version of a form, which is what steps use by default.
pub async fn latest_published_form_version_id(
    conn: &mut PgConnection,
    form_id: Uuid,
) -> Result<Option<Uuid>, ComhairleError> {
    Ok(sqlx::query_scalar(
        "SELECT id FROM demographics_form_version
         WHERE form_id = $1 AND status = 'published'
         ORDER BY version_number DESC LIMIT 1",
    )
    .bind(form_id)
    .fetch_optional(&mut *conn)
    .await?)
}

/// Checks that a specific form version is published and belongs to the form.
pub async fn published_form_version_of(
    conn: &mut PgConnection,
    form_id: Uuid,
    version_id: Uuid,
) -> Result<Uuid, ComhairleError> {
    sqlx::query_scalar(
        "SELECT id FROM demographics_form_version
         WHERE id = $1 AND form_id = $2 AND status = 'published'",
    )
    .bind(version_id)
    .bind(form_id)
    .fetch_optional(&mut *conn)
    .await?
    .ok_or_else(|| invalid("pinned form version is not a published version of this form"))
}

/// Every entry must reference a question the organization can use and that has a
/// published version. Positions and questions must be unique within the form.
async fn validate_entries(
    conn: &mut PgConnection,
    organization_id: Uuid,
    entries: &[FormEntryInput],
) -> Result<(), ComhairleError> {
    let mut question_ids = std::collections::HashSet::new();
    let mut positions = std::collections::HashSet::new();
    for entry in entries {
        if entry.position < 0 {
            return Err(invalid("form entry positions must not be negative"));
        }
        if !question_ids.insert(entry.question_id) {
            return Err(invalid(
                "a demographic question can appear only once in a form",
            ));
        }
        if !positions.insert(entry.position) {
            return Err(invalid("form entry positions must be unique"));
        }
        usable_published_version(&mut *conn, Some(organization_id), entry.question_id).await?;
    }
    Ok(())
}

async fn insert_draft_version(
    conn: &mut PgConnection,
    form_id: Uuid,
    name: &str,
) -> Result<Uuid, ComhairleError> {
    let version_id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO demographics_form_version (id, form_id, version_number, status, name)
         VALUES ($1, $2,
            (SELECT COALESCE(MAX(version_number), 0) + 1
             FROM demographics_form_version WHERE form_id = $2),
            'draft', $3)",
    )
    .bind(version_id)
    .bind(form_id)
    .bind(name)
    .execute(&mut *conn)
    .await?;
    Ok(version_id)
}

async fn replace_entries(
    conn: &mut PgConnection,
    form_id: Uuid,
    version_id: Uuid,
    entries: &[FormEntryInput],
) -> Result<(), ComhairleError> {
    sqlx::query("DELETE FROM demographics_form_version_question WHERE form_version_id = $1")
        .bind(version_id)
        .execute(&mut *conn)
        .await?;
    for entry in entries {
        sqlx::query(
            "INSERT INTO demographics_form_version_question
                (form_version_id, form_id, question_id, position, required)
             VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(version_id)
        .bind(form_id)
        .bind(entry.question_id)
        .bind(entry.position)
        .bind(entry.required)
        .execute(&mut *conn)
        .await?;
    }
    Ok(())
}

pub async fn create_form(
    db: &PgPool,
    organization_id: Uuid,
    input: &FormDraftInput,
    locale: &str,
) -> Result<FormDto, ComhairleError> {
    let name = clean_name(&input.name)?;
    let form_id = Uuid::new_v4();
    let mut tx = db.begin().await?;
    let conn = &mut *tx;
    validate_entries(conn, organization_id, &input.entries).await?;
    sqlx::query("INSERT INTO demographics_form (id, organization_id) VALUES ($1, $2)")
        .bind(form_id)
        .bind(organization_id)
        .execute(&mut *conn)
        .await?;
    let version_id = insert_draft_version(conn, form_id, name).await?;
    replace_entries(conn, form_id, version_id, &input.entries).await?;
    tx.commit().await?;
    get_form(db, organization_id, form_id, locale).await
}

/// Starts a draft by copying the latest published version's name and entries.
pub async fn start_form_draft(
    db: &PgPool,
    organization_id: Uuid,
    form_id: Uuid,
    locale: &str,
) -> Result<FormDto, ComhairleError> {
    let mut tx = db.begin().await?;
    let conn = &mut *tx;
    lock_editable_form(conn, organization_id, form_id).await?;
    if draft_form_version_id(conn, form_id).await?.is_some() {
        return Err(ComhairleError::Conflict(
            "demographic form already has a draft".into(),
        ));
    }
    let source = latest_published_form_version_id(conn, form_id)
        .await?
        .ok_or_else(|| invalid("demographic form has no published version to edit"))?;
    let draft_id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO demographics_form_version (id, form_id, version_number, status, name)
         SELECT $1, form_id,
            (SELECT COALESCE(MAX(version_number), 0) + 1
             FROM demographics_form_version WHERE form_id = $2),
            'draft', name
         FROM demographics_form_version WHERE id = $3",
    )
    .bind(draft_id)
    .bind(form_id)
    .bind(source)
    .execute(&mut *conn)
    .await?;
    sqlx::query(
        "INSERT INTO demographics_form_version_question
            (form_version_id, form_id, question_id, position, required)
         SELECT $1, form_id, question_id, position, required
         FROM demographics_form_version_question WHERE form_version_id = $2",
    )
    .bind(draft_id)
    .bind(source)
    .execute(&mut *conn)
    .await?;
    tx.commit().await?;
    get_form(db, organization_id, form_id, locale).await
}

pub async fn update_form_draft(
    db: &PgPool,
    organization_id: Uuid,
    form_id: Uuid,
    input: &FormDraftInput,
    locale: &str,
) -> Result<FormDto, ComhairleError> {
    let name = clean_name(&input.name)?;
    let mut tx = db.begin().await?;
    let conn = &mut *tx;
    lock_editable_form(conn, organization_id, form_id).await?;
    let draft_id = require_form_draft(conn, form_id).await?;
    validate_entries(conn, organization_id, &input.entries).await?;
    sqlx::query("UPDATE demographics_form_version SET name = $2, updated_at = NOW() WHERE id = $1")
        .bind(draft_id)
        .bind(name)
        .execute(&mut *conn)
        .await?;
    replace_entries(conn, form_id, draft_id, &input.entries).await?;
    tx.commit().await?;
    get_form(db, organization_id, form_id, locale).await
}

/// Deletes the draft. A form that was never published is removed with it.
pub async fn discard_form_draft(
    db: &PgPool,
    organization_id: Uuid,
    form_id: Uuid,
) -> Result<(), ComhairleError> {
    let mut tx = db.begin().await?;
    let conn = &mut *tx;
    lock_form(conn, organization_id, form_id).await?;
    let deleted = sqlx::query(
        "DELETE FROM demographics_form_version WHERE form_id = $1 AND status = 'draft'",
    )
    .bind(form_id)
    .execute(&mut *conn)
    .await?
    .rows_affected();
    if deleted == 0 {
        return Err(ComhairleError::ResourceNotFound(
            "draft for demographic form".into(),
        ));
    }
    sqlx::query(
        "DELETE FROM demographics_form f
         WHERE f.id = $1
           AND NOT EXISTS (SELECT 1 FROM demographics_form_version v WHERE v.form_id = f.id)",
    )
    .bind(form_id)
    .execute(&mut *conn)
    .await?;
    tx.commit().await?;
    Ok(())
}

/// Publishing pins every entry to its question's latest published version. Question
/// drafts are never pinned.
pub async fn publish_form(
    db: &PgPool,
    organization_id: Uuid,
    form_id: Uuid,
    locale: &str,
) -> Result<FormDto, ComhairleError> {
    let mut tx = db.begin().await?;
    let conn = &mut *tx;
    lock_editable_form(conn, organization_id, form_id).await?;
    let draft_id = require_form_draft(conn, form_id).await?;
    let question_ids: Vec<Uuid> = sqlx::query_scalar(
        "SELECT question_id FROM demographics_form_version_question WHERE form_version_id = $1",
    )
    .bind(draft_id)
    .fetch_all(&mut *conn)
    .await?;
    if question_ids.is_empty() {
        return Err(invalid(
            "a demographic form needs at least one question before publishing",
        ));
    }
    for question_id in question_ids {
        usable_published_version(&mut *conn, Some(organization_id), question_id).await?;
    }
    sqlx::query(
        "UPDATE demographics_form_version_question e
         SET question_version_id = (
            SELECT v.id FROM demographics_question_version v
            WHERE v.question_id = e.question_id AND v.status = 'published'
            ORDER BY v.version_number DESC LIMIT 1
         )
         WHERE e.form_version_id = $1",
    )
    .bind(draft_id)
    .execute(&mut *conn)
    .await?;
    sqlx::query(
        "UPDATE demographics_form_version
         SET status = 'published', published_at = NOW(), updated_at = NOW()
         WHERE id = $1",
    )
    .bind(draft_id)
    .execute(&mut *conn)
    .await?;
    tx.commit().await?;
    get_form(db, organization_id, form_id, locale).await
}

pub async fn archive_form(
    db: &PgPool,
    organization_id: Uuid,
    form_id: Uuid,
    locale: &str,
) -> Result<FormDto, ComhairleError> {
    let result = sqlx::query(
        "UPDATE demographics_form SET archived_at = COALESCE(archived_at, NOW())
         WHERE id = $1 AND organization_id = $2",
    )
    .bind(form_id)
    .bind(organization_id)
    .execute(db)
    .await?;
    if result.rows_affected() == 0 {
        return Err(ComhairleError::ResourceNotFound("demographic form".into()));
    }
    get_form(db, organization_id, form_id, locale).await
}
