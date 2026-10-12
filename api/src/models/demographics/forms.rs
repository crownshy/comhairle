use std::collections::HashMap;

use chrono::{DateTime, Utc};
use sea_query::{
    Alias, Expr, Func, JoinType, LockType, Order, PostgresQueryBuilder, Query, SimpleExpr,
};
use sea_query_binder::SqlxBinder;
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

use crate::error::ComhairleError;
use crate::models::translations::{TextContentId, localize_translations};

use super::VersionStatus;
use super::questions::usable_published_version;
use super::types::*;
use super::validation::invalid;

const FORM_COLUMNS: [DemographicFormIden; 4] = [
    DemographicFormIden::Id,
    DemographicFormIden::OrganizationId,
    DemographicFormIden::ArchivedAt,
    DemographicFormIden::CreatedAt,
];

/// Generates a sub-query for the next version number of a form.
fn next_version_number(form_id: Uuid) -> SimpleExpr {
    let next = Query::select()
        .expr(
            Expr::expr(Func::coalesce([
                Func::max(Expr::col(DemographicFormVersionIden::VersionNumber)).into(),
                Expr::val(0).into(),
            ]))
            .add(1),
        )
        .from(DemographicFormVersionIden::Table)
        .and_where(Expr::col(DemographicFormVersionIden::FormId).eq(form_id))
        .to_owned();
    SimpleExpr::SubQuery(None, Box::new(next.into_sub_query_statement()))
}

/// This function loads the entries for the specified demographic form version IDs, including the localized question text for each entry.
///
/// The returned entries are grouped by form version ID in a `HashMap`.
///
/// See [`DemographicFormEntryDto`] for the structure of the returned entries.
pub async fn load_entries(
    db: &PgPool,
    form_version_ids: &[Uuid],
    locale: &str,
) -> Result<HashMap<Uuid, Vec<DemographicFormEntryDto>>, ComhairleError> {
    let text_version = Alias::new("qv");
    let display_text = Query::select()
        .column(DemographicQuestionVersionIden::TextContentId)
        .from(DemographicQuestionVersionIden::Table)
        .and_where(
            Expr::col((
                DemographicQuestionVersionIden::Table,
                DemographicQuestionVersionIden::QuestionId,
            ))
            .equals((
                DemographicFormVersionQuestionIden::Table,
                DemographicFormVersionQuestionIden::QuestionId,
            )),
        )
        .and_where(
            Expr::col((
                DemographicQuestionVersionIden::Table,
                DemographicQuestionVersionIden::Status,
            ))
            .eq(VersionStatus::Published),
        )
        .and_where(
            Expr::col((
                DemographicFormVersionQuestionIden::Table,
                DemographicFormVersionQuestionIden::QuestionVersionId,
            ))
            .is_null()
            .or(Expr::col((
                DemographicQuestionVersionIden::Table,
                DemographicQuestionVersionIden::Id,
            ))
            .equals((
                DemographicFormVersionQuestionIden::Table,
                DemographicFormVersionQuestionIden::QuestionVersionId,
            ))),
        )
        .order_by(
            (
                DemographicQuestionVersionIden::Table,
                DemographicQuestionVersionIden::VersionNumber,
            ),
            Order::Desc,
        )
        .limit(1)
        .to_owned();

    let query = Query::select()
        .columns([
            (
                DemographicFormVersionQuestionIden::Table,
                DemographicFormVersionQuestionIden::FormVersionId,
            ),
            (
                DemographicFormVersionQuestionIden::Table,
                DemographicFormVersionQuestionIden::FormId,
            ),
            (
                DemographicFormVersionQuestionIden::Table,
                DemographicFormVersionQuestionIden::QuestionId,
            ),
            (
                DemographicFormVersionQuestionIden::Table,
                DemographicFormVersionQuestionIden::QuestionVersionId,
            ),
            (
                DemographicFormVersionQuestionIden::Table,
                DemographicFormVersionQuestionIden::Position,
            ),
            (
                DemographicFormVersionQuestionIden::Table,
                DemographicFormVersionQuestionIden::Required,
            ),
        ])
        .expr_as(
            Expr::col((
                text_version.clone(),
                DemographicQuestionVersionIden::TextContentId,
            )),
            Alias::new("display_text_content_id"),
        )
        .from(DemographicFormVersionQuestionIden::Table)
        .join_lateral(
            JoinType::LeftJoin,
            display_text,
            text_version,
            SimpleExpr::from(Expr::val(true)),
        )
        .and_where(
            Expr::col((
                DemographicFormVersionQuestionIden::Table,
                DemographicFormVersionQuestionIden::FormVersionId,
            ))
            .is_in(form_version_ids.to_vec()),
        )
        .order_by(
            (
                DemographicFormVersionQuestionIden::Table,
                DemographicFormVersionQuestionIden::FormVersionId,
            ),
            Order::Asc,
        )
        .order_by(
            (
                DemographicFormVersionQuestionIden::Table,
                DemographicFormVersionQuestionIden::Position,
            ),
            Order::Asc,
        )
        .to_owned();
    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let rows = sqlx::query_as_with::<_, DemographicFormEntryWithText, _>(&sql, values)
        .fetch_all(db)
        .await?;
    let text_ids: Vec<TextContentId> = rows
        .iter()
        .filter_map(|row| row.display_text_content_id)
        .collect();
    let texts = localize_translations(db, &text_ids, locale).await?;

    let mut entries: HashMap<Uuid, Vec<DemographicFormEntryDto>> = HashMap::new();
    for row in rows {
        let question_text = row
            .display_text_content_id
            .and_then(|id| texts.get(&id).cloned())
            .unwrap_or_default();
        entries
            .entry(row.entry.form_version_id)
            .or_default()
            .push(DemographicFormEntryDto {
                entry: row.entry,
                question_text,
            });
    }
    Ok(entries)
}

/// This function takes a list of demographic forms and a locale, and returns the forms
/// with their versions and entries hydrated, including localized question text.
///
/// See [`DemographicFormDto`] for the structure of the returned forms.
async fn hydrate_forms(
    db: &PgPool,
    forms: Vec<DemographicForm>,
    locale: &str,
) -> Result<Vec<DemographicFormDto>, ComhairleError> {
    if forms.is_empty() {
        return Ok(Vec::new());
    }
    let form_ids: Vec<Uuid> = forms.iter().map(|form| form.id).collect();
    let (sql, values) = Query::select()
        .columns([
            DemographicFormVersionIden::Id,
            DemographicFormVersionIden::FormId,
            DemographicFormVersionIden::VersionNumber,
            DemographicFormVersionIden::Status,
            DemographicFormVersionIden::Name,
            DemographicFormVersionIden::PublishedAt,
            DemographicFormVersionIden::UpdatedAt,
        ])
        .from(DemographicFormVersionIden::Table)
        .and_where(Expr::col(DemographicFormVersionIden::FormId).is_in(form_ids))
        .order_by(DemographicFormVersionIden::FormId, Order::Asc)
        .order_by(DemographicFormVersionIden::VersionNumber, Order::Asc)
        .build_sqlx(PostgresQueryBuilder);
    let versions = sqlx::query_as_with::<_, DemographicFormVersion, _>(&sql, values)
        .fetch_all(db)
        .await?;
    let version_ids: Vec<Uuid> = versions.iter().map(|version| version.id).collect();
    let mut entries = load_entries(db, &version_ids, locale).await?;

    let mut versions_by_form: HashMap<Uuid, Vec<DemographicFormVersionDto>> = HashMap::new();
    for version in versions {
        let entries = entries.remove(&version.id).unwrap_or_default();
        versions_by_form
            .entry(version.form_id)
            .or_default()
            .push(DemographicFormVersionDto { version, entries });
    }

    Ok(forms
        .into_iter()
        .map(|form| {
            let versions = versions_by_form.remove(&form.id).unwrap_or_default();
            DemographicFormDto {
                versions: versions
                    .iter()
                    .map(|dto| VersionSummary::from(&dto.version))
                    .collect(),
                latest_published: versions
                    .iter()
                    .filter(|dto| dto.version.status == VersionStatus::Published)
                    .max_by_key(|dto| dto.version.version_number)
                    .cloned(),
                draft: versions
                    .iter()
                    .find(|dto| dto.version.status == VersionStatus::Draft)
                    .cloned(),
                form,
            }
        })
        .collect())
}

/// This function lists all demographic forms for the specified organization, including their versions and entries, localized to the specified locale.
///
/// See [`DemographicFormDto`] for the structure of the returned forms.
pub async fn list_forms(
    db: &PgPool,
    organization_id: Uuid,
    locale: &str,
) -> Result<Vec<DemographicFormDto>, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(FORM_COLUMNS)
        .from(DemographicFormIden::Table)
        .and_where(Expr::col(DemographicFormIden::OrganizationId).eq(organization_id))
        .order_by(DemographicFormIden::CreatedAt, Order::Asc)
        .order_by(DemographicFormIden::Id, Order::Asc)
        .build_sqlx(PostgresQueryBuilder);
    let rows = sqlx::query_as_with::<_, DemographicForm, _>(&sql, values)
        .fetch_all(db)
        .await?;
    hydrate_forms(db, rows, locale).await
}

/// This function retrieves a specific demographic form for the specified organization, including its versions and entries, localized to the specified locale.
///
/// See [`DemographicFormDto`] for the structure of the returned form.
pub async fn get_form(
    db: &PgPool,
    organization_id: Uuid,
    form_id: Uuid,
    locale: &str,
) -> Result<DemographicFormDto, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(FORM_COLUMNS)
        .from(DemographicFormIden::Table)
        .and_where(Expr::col(DemographicFormIden::Id).eq(form_id))
        .and_where(Expr::col(DemographicFormIden::OrganizationId).eq(organization_id))
        .build_sqlx(PostgresQueryBuilder);
    let rows = sqlx::query_as_with::<_, DemographicForm, _>(&sql, values)
        .fetch_all(db)
        .await?;
    hydrate_forms(db, rows, locale)
        .await?
        .pop()
        .ok_or_else(|| ComhairleError::ResourceNotFound("demographic form".into()))
}

/// This function cleans and validates a form name, ensuring it is not blank.
fn clean_name(raw: &str) -> Result<&str, ComhairleError> {
    let name = raw.trim();
    if name.is_empty() {
        return Err(invalid("form name must not be blank"));
    }
    Ok(name)
}

/// This function locks a demographic form for the specified organization, preventing concurrent edits, and returns its archived_at timestamp if it exists.
async fn lock_form(
    conn: &mut PgConnection,
    organization_id: Uuid,
    form_id: Uuid,
) -> Result<Option<DateTime<Utc>>, ComhairleError> {
    let (sql, values) = Query::select()
        .column(DemographicFormIden::ArchivedAt)
        .from(DemographicFormIden::Table)
        .and_where(Expr::col(DemographicFormIden::Id).eq(form_id))
        .and_where(Expr::col(DemographicFormIden::OrganizationId).eq(organization_id))
        .lock(LockType::Update)
        .build_sqlx(PostgresQueryBuilder);
    let archived: Option<Option<DateTime<Utc>>> = sqlx::query_scalar_with(&sql, values)
        .fetch_optional(&mut *conn)
        .await?;
    archived.ok_or_else(|| ComhairleError::ResourceNotFound("demographic form".into()))
}

/// This function attempts to lock a demographic form for editing. If the form is archived, it returns a conflict error, preventing edits.
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

/// This function retrieves the ID of the draft version of a demographic form, if it exists.
async fn draft_form_version_id(
    conn: &mut PgConnection,
    form_id: Uuid,
) -> Result<Option<Uuid>, ComhairleError> {
    let (sql, values) = Query::select()
        .column(DemographicFormVersionIden::Id)
        .from(DemographicFormVersionIden::Table)
        .and_where(Expr::col(DemographicFormVersionIden::FormId).eq(form_id))
        .and_where(Expr::col(DemographicFormVersionIden::Status).eq(VersionStatus::Draft))
        .build_sqlx(PostgresQueryBuilder);
    Ok(sqlx::query_scalar_with(&sql, values)
        .fetch_optional(&mut *conn)
        .await?)
}

/// This function requires that a draft version of a demographic form exists and returns its ID.
/// If no draft exists, it returns a resource not found error.
async fn require_form_draft(
    conn: &mut PgConnection,
    form_id: Uuid,
) -> Result<Uuid, ComhairleError> {
    draft_form_version_id(conn, form_id)
        .await?
        .ok_or_else(|| ComhairleError::ResourceNotFound("draft for demographic form".into()))
}

/// This function retrieves the ID of the latest published version of a demographic form, if it exists.
pub async fn latest_published_form_version_id(
    conn: &mut PgConnection,
    form_id: Uuid,
) -> Result<Option<Uuid>, ComhairleError> {
    let (sql, values) = Query::select()
        .column(DemographicFormVersionIden::Id)
        .from(DemographicFormVersionIden::Table)
        .and_where(Expr::col(DemographicFormVersionIden::FormId).eq(form_id))
        .and_where(Expr::col(DemographicFormVersionIden::Status).eq(VersionStatus::Published))
        .order_by(DemographicFormVersionIden::VersionNumber, Order::Desc)
        .limit(1)
        .build_sqlx(PostgresQueryBuilder);
    Ok(sqlx::query_scalar_with(&sql, values)
        .fetch_optional(&mut *conn)
        .await?)
}

/// This function checks if a specific version of a demographic form is published and belongs to the given form.
/// If it does, it returns the version ID; otherwise, it returns an error.
pub async fn published_form_version_of(
    conn: &mut PgConnection,
    form_id: Uuid,
    version_id: Uuid,
) -> Result<Uuid, ComhairleError> {
    let (sql, values) = Query::select()
        .column(DemographicFormVersionIden::Id)
        .from(DemographicFormVersionIden::Table)
        .and_where(Expr::col(DemographicFormVersionIden::Id).eq(version_id))
        .and_where(Expr::col(DemographicFormVersionIden::FormId).eq(form_id))
        .and_where(Expr::col(DemographicFormVersionIden::Status).eq(VersionStatus::Published))
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_scalar_with(&sql, values)
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(|| invalid("pinned form version is not a published version of this form"))
}

/// This function validates the entries of a demographic form.
/// It ensures that each entry has a non-negative position, that question IDs and positions are unique within the form, and that each question has a usable published version for the given organization.
async fn validate_entries(
    conn: &mut PgConnection,
    organization_id: Uuid,
    entries: &[DemographicFormEntryInput],
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

/// This function inserts a new draft version of a demographic form with the given name and returns the ID of the newly created version.
async fn insert_draft_version(
    conn: &mut PgConnection,
    form_id: Uuid,
    name: &str,
) -> Result<Uuid, ComhairleError> {
    let version_id = Uuid::new_v4();
    let (sql, values) = Query::insert()
        .into_table(DemographicFormVersionIden::Table)
        .columns([
            DemographicFormVersionIden::Id,
            DemographicFormVersionIden::FormId,
            DemographicFormVersionIden::VersionNumber,
            DemographicFormVersionIden::Status,
            DemographicFormVersionIden::Name,
        ])
        .values_panic([
            version_id.into(),
            form_id.into(),
            next_version_number(form_id),
            VersionStatus::Draft.into(),
            name.into(),
        ])
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(&mut *conn).await?;
    Ok(version_id)
}

/// This function replaces the entries of a specific version of a demographic form.
/// It replaces all of the existing entries for the given form version with the new entries provided.
async fn replace_entries(
    conn: &mut PgConnection,
    form_id: Uuid,
    version_id: Uuid,
    entries: &[DemographicFormEntryInput],
) -> Result<(), ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(DemographicFormVersionQuestionIden::Table)
        .and_where(Expr::col(DemographicFormVersionQuestionIden::FormVersionId).eq(version_id))
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(&mut *conn).await?;
    if entries.is_empty() {
        return Ok(());
    }
    let mut insert = Query::insert();
    insert
        .into_table(DemographicFormVersionQuestionIden::Table)
        .columns([
            DemographicFormVersionQuestionIden::FormVersionId,
            DemographicFormVersionQuestionIden::FormId,
            DemographicFormVersionQuestionIden::QuestionId,
            DemographicFormVersionQuestionIden::Position,
            DemographicFormVersionQuestionIden::Required,
        ]);
    for entry in entries {
        insert.values_panic([
            version_id.into(),
            form_id.into(),
            entry.question_id.into(),
            entry.position.into(),
            entry.required.into(),
        ]);
    }
    let (sql, values) = insert.build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(&mut *conn).await?;
    Ok(())
}

/// This function creates a new demographic form for the specified organization.
/// It validates the entries, inserts the form into the database, creates a draft version with the provided name, replaces the entries for the draft version and then retrieves the newly created form.
pub async fn create_form(
    db: &PgPool,
    organization_id: Uuid,
    input: &DemographicFormDraftInput,
    locale: &str,
) -> Result<DemographicFormDto, ComhairleError> {
    let name = clean_name(&input.name)?;
    let form_id = Uuid::new_v4();
    let mut tx = db.begin().await?;
    let conn = &mut *tx;
    validate_entries(conn, organization_id, &input.entries).await?;
    let (sql, values) = Query::insert()
        .into_table(DemographicFormIden::Table)
        .columns([DemographicFormIden::Id, DemographicFormIden::OrganizationId])
        .values_panic([form_id.into(), organization_id.into()])
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(&mut *conn).await?;
    let version_id = insert_draft_version(conn, form_id, name).await?;
    replace_entries(conn, form_id, version_id, &input.entries).await?;
    tx.commit().await?;
    get_form(db, organization_id, form_id, locale).await
}

/// This function starts a new draft for a demographic form by copying the latest published version's name and entries.
/// It ensures that the form is editable, checks that no draft already exists, creates a new draft version and duplicates the entries from the latest published version into the new draft.
pub async fn start_form_draft(
    db: &PgPool,
    organization_id: Uuid,
    form_id: Uuid,
    locale: &str,
) -> Result<DemographicFormDto, ComhairleError> {
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
    let copied_version = Query::select()
        .expr(Expr::val(draft_id))
        .column(DemographicFormVersionIden::FormId)
        .expr(next_version_number(form_id))
        .expr(Expr::val(VersionStatus::Draft))
        .column(DemographicFormVersionIden::Name)
        .from(DemographicFormVersionIden::Table)
        .and_where(Expr::col(DemographicFormVersionIden::Id).eq(source))
        .to_owned();
    let (sql, values) = Query::insert()
        .into_table(DemographicFormVersionIden::Table)
        .columns([
            DemographicFormVersionIden::Id,
            DemographicFormVersionIden::FormId,
            DemographicFormVersionIden::VersionNumber,
            DemographicFormVersionIden::Status,
            DemographicFormVersionIden::Name,
        ])
        .select_from(copied_version)?
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(&mut *conn).await?;

    let copied_entries = Query::select()
        .expr(Expr::val(draft_id))
        .columns([
            DemographicFormVersionQuestionIden::FormId,
            DemographicFormVersionQuestionIden::QuestionId,
            DemographicFormVersionQuestionIden::Position,
            DemographicFormVersionQuestionIden::Required,
        ])
        .from(DemographicFormVersionQuestionIden::Table)
        .and_where(Expr::col(DemographicFormVersionQuestionIden::FormVersionId).eq(source))
        .to_owned();
    let (sql, values) = Query::insert()
        .into_table(DemographicFormVersionQuestionIden::Table)
        .columns([
            DemographicFormVersionQuestionIden::FormVersionId,
            DemographicFormVersionQuestionIden::FormId,
            DemographicFormVersionQuestionIden::QuestionId,
            DemographicFormVersionQuestionIden::Position,
            DemographicFormVersionQuestionIden::Required,
        ])
        .select_from(copied_entries)?
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(&mut *conn).await?;
    tx.commit().await?;
    get_form(db, organization_id, form_id, locale).await
}

/// This function updates the draft version of a demographic form.
/// It validates the input entries, updates the draft's name and timestamp, replaces the entries for the draft version, and then retrieves the updated form.
pub async fn update_form_draft(
    db: &PgPool,
    organization_id: Uuid,
    form_id: Uuid,
    input: &DemographicFormDraftInput,
    locale: &str,
) -> Result<DemographicFormDto, ComhairleError> {
    let name = clean_name(&input.name)?;
    let mut tx = db.begin().await?;
    let conn = &mut *tx;
    lock_editable_form(conn, organization_id, form_id).await?;
    let draft_id = require_form_draft(conn, form_id).await?;
    validate_entries(conn, organization_id, &input.entries).await?;
    let (sql, values) = Query::update()
        .table(DemographicFormVersionIden::Table)
        .values([
            (DemographicFormVersionIden::Name, name.into()),
            (
                DemographicFormVersionIden::UpdatedAt,
                Expr::current_timestamp().into(),
            ),
        ])
        .and_where(Expr::col(DemographicFormVersionIden::Id).eq(draft_id))
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(&mut *conn).await?;
    replace_entries(conn, form_id, draft_id, &input.entries).await?;
    tx.commit().await?;
    get_form(db, organization_id, form_id, locale).await
}

/// This function discards the draft version of a demographic form.
/// If the form was never published, it removes the form entirely.
pub async fn discard_form_draft(
    db: &PgPool,
    organization_id: Uuid,
    form_id: Uuid,
) -> Result<(), ComhairleError> {
    let mut tx = db.begin().await?;
    let conn = &mut *tx;
    lock_form(conn, organization_id, form_id).await?;
    let (sql, values) = Query::delete()
        .from_table(DemographicFormVersionIden::Table)
        .and_where(Expr::col(DemographicFormVersionIden::FormId).eq(form_id))
        .and_where(Expr::col(DemographicFormVersionIden::Status).eq(VersionStatus::Draft))
        .build_sqlx(PostgresQueryBuilder);
    let deleted = sqlx::query_with(&sql, values)
        .execute(&mut *conn)
        .await?
        .rows_affected();
    if deleted == 0 {
        return Err(ComhairleError::ResourceNotFound(
            "draft for demographic form".into(),
        ));
    }
    let remaining_versions = Query::select()
        .expr(Expr::val(1))
        .from(DemographicFormVersionIden::Table)
        .and_where(
            Expr::col((
                DemographicFormVersionIden::Table,
                DemographicFormVersionIden::FormId,
            ))
            .equals((DemographicFormIden::Table, DemographicFormIden::Id)),
        )
        .to_owned();
    let (sql, values) = Query::delete()
        .from_table(DemographicFormIden::Table)
        .and_where(Expr::col(DemographicFormIden::Id).eq(form_id))
        .and_where(Expr::exists(remaining_versions).not())
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(&mut *conn).await?;
    tx.commit().await?;
    Ok(())
}

/// It publishes a draft version of a demographic form by ensuring it has at least one question,
/// pinning each question to its latest published version, and updating the form version accordingly.
pub async fn publish_form(
    db: &PgPool,
    organization_id: Uuid,
    form_id: Uuid,
    locale: &str,
) -> Result<DemographicFormDto, ComhairleError> {
    let mut tx = db.begin().await?;
    let conn = &mut *tx;
    lock_editable_form(conn, organization_id, form_id).await?;
    let draft_id = require_form_draft(conn, form_id).await?;
    let (sql, values) = Query::select()
        .column(DemographicFormVersionQuestionIden::QuestionId)
        .from(DemographicFormVersionQuestionIden::Table)
        .and_where(Expr::col(DemographicFormVersionQuestionIden::FormVersionId).eq(draft_id))
        .build_sqlx(PostgresQueryBuilder);
    let question_ids: Vec<Uuid> = sqlx::query_scalar_with(&sql, values)
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
    let latest_published = Query::select()
        .column(DemographicQuestionVersionIden::Id)
        .from(DemographicQuestionVersionIden::Table)
        .and_where(
            Expr::col((
                DemographicQuestionVersionIden::Table,
                DemographicQuestionVersionIden::QuestionId,
            ))
            .equals((
                DemographicFormVersionQuestionIden::Table,
                DemographicFormVersionQuestionIden::QuestionId,
            )),
        )
        .and_where(Expr::col(DemographicQuestionVersionIden::Status).eq(VersionStatus::Published))
        .order_by(DemographicQuestionVersionIden::VersionNumber, Order::Desc)
        .limit(1)
        .to_owned();
    let (sql, values) = Query::update()
        .table(DemographicFormVersionQuestionIden::Table)
        .value(
            DemographicFormVersionQuestionIden::QuestionVersionId,
            SimpleExpr::SubQuery(None, Box::new(latest_published.into_sub_query_statement())),
        )
        .and_where(Expr::col(DemographicFormVersionQuestionIden::FormVersionId).eq(draft_id))
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(&mut *conn).await?;

    let (sql, values) = Query::update()
        .table(DemographicFormVersionIden::Table)
        .values([
            (
                DemographicFormVersionIden::Status,
                VersionStatus::Published.into(),
            ),
            (
                DemographicFormVersionIden::PublishedAt,
                Expr::current_timestamp().into(),
            ),
            (
                DemographicFormVersionIden::UpdatedAt,
                Expr::current_timestamp().into(),
            ),
        ])
        .and_where(Expr::col(DemographicFormVersionIden::Id).eq(draft_id))
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(&mut *conn).await?;
    tx.commit().await?;
    get_form(db, organization_id, form_id, locale).await
}

/// It archives a demographic form by setting its archived timestamp if it hasn't been set already.
pub async fn archive_form(
    db: &PgPool,
    organization_id: Uuid,
    form_id: Uuid,
    locale: &str,
) -> Result<DemographicFormDto, ComhairleError> {
    let (sql, values) = Query::update()
        .table(DemographicFormIden::Table)
        .value(
            DemographicFormIden::ArchivedAt,
            Func::coalesce([
                Expr::col(DemographicFormIden::ArchivedAt).into(),
                Expr::current_timestamp().into(),
            ]),
        )
        .and_where(Expr::col(DemographicFormIden::Id).eq(form_id))
        .and_where(Expr::col(DemographicFormIden::OrganizationId).eq(organization_id))
        .build_sqlx(PostgresQueryBuilder);
    let result = sqlx::query_with(&sql, values).execute(db).await?;
    if result.rows_affected() == 0 {
        return Err(ComhairleError::ResourceNotFound("demographic form".into()));
    }
    get_form(db, organization_id, form_id, locale).await
}
