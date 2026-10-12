use std::collections::{HashMap, HashSet};

use sea_query::{
    Alias, Asterisk, Condition, Expr, Func, LockType, Order, PostgresQueryBuilder, Query,
    SelectStatement, SimpleExpr,
};
use sea_query_binder::SqlxBinder;
use sqlx::{PgConnection, PgExecutor, PgPool};
use uuid::Uuid;

use crate::error::ComhairleError;
use crate::models::SqlxResultExt;
use crate::models::translations::{
    TextContentId, TextFormat, localize_translations, new_translation_in_connection,
};

use super::VersionStatus;
use super::types::*;
use super::validation::{invalid, validate_question_draft};

const VERSION_COLUMNS: [DemographicQuestionVersionIden; 17] = [
    DemographicQuestionVersionIden::Id,
    DemographicQuestionVersionIden::QuestionId,
    DemographicQuestionVersionIden::VersionNumber,
    DemographicQuestionVersionIden::Status,
    DemographicQuestionVersionIden::PublishedAt,
    DemographicQuestionVersionIden::TextContentId,
    DemographicQuestionVersionIden::QuestionType,
    DemographicQuestionVersionIden::DatePrecision,
    DemographicQuestionVersionIden::LocationFormat,
    DemographicQuestionVersionIden::LocationCountry,
    DemographicQuestionVersionIden::MaxSelections,
    DemographicQuestionVersionIden::AllowOther,
    DemographicQuestionVersionIden::PreferNotToSay,
    DemographicQuestionVersionIden::JsonSchema,
    DemographicQuestionVersionIden::SpecialCategory,
    DemographicQuestionVersionIden::ConsentTextContentId,
    DemographicQuestionVersionIden::UpdatedAt,
];

/// Returns a condition that filters questions visible to the specified owner.
fn visible_to_owner(owner: Option<Uuid>) -> Condition {
    let Some(owner_id) = owner else {
        return Condition::all().add(Expr::col(QuestionRowIden::OrganizationId).is_null());
    };
    Condition::any()
        .add(Expr::col(QuestionRowIden::OrganizationId).eq(owner_id))
        .add(
            Condition::all()
                .add(Expr::col(QuestionRowIden::OrganizationId).is_null())
                .add(Expr::exists(forked_by_owner(owner_id)).not()),
        )
}

/// Returns a SQL select statement that finds questions forked by the specified owner.
fn forked_by_owner(owner_id: Uuid) -> SelectStatement {
    let fork = Alias::new("f");
    Query::select()
        .expr(Expr::val(1))
        .from_as(QuestionRowIden::Table, fork.clone())
        .and_where(
            Expr::col((fork.clone(), QuestionRowIden::ForkedFromId))
                .equals((QuestionRowIden::Table, QuestionRowIden::Id)),
        )
        .and_where(Expr::col((fork, QuestionRowIden::OrganizationId)).eq(owner_id))
        .to_owned()
}

/// Returns all question rows visible to the specified owner, optionally filtered by a specific question ID.
async fn visible_question_rows(
    executor: impl PgExecutor<'_>,
    owner: Option<Uuid>,
    only: Option<Uuid>,
) -> Result<Vec<QuestionRow>, ComhairleError> {
    let mut query = Query::select();
    query
        .columns([
            QuestionRowIden::Id,
            QuestionRowIden::OrganizationId,
            QuestionRowIden::ForkedFromId,
            QuestionRowIden::ArchivedAt,
            QuestionRowIden::CreatedAt,
        ])
        .from(QuestionRowIden::Table)
        .cond_where(visible_to_owner(owner));
    if let Some(question_id) = only {
        query.and_where(Expr::col(QuestionRowIden::Id).eq(question_id));
    }
    query
        .order_by(QuestionRowIden::CreatedAt, Order::Asc)
        .order_by(QuestionRowIden::Id, Order::Asc);

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    Ok(sqlx::query_as_with::<_, QuestionRow, _>(&sql, values)
        .fetch_all(executor)
        .await?)
}

/// Returns whether the specified question is visible to the given owner, considering both ownership and system defaults, regardless of whether it is archived.
pub async fn is_question_visible(
    executor: impl PgExecutor<'_>,
    owner: Option<Uuid>,
    question_id: Uuid,
) -> Result<bool, ComhairleError> {
    Ok(!visible_question_rows(executor, owner, Some(question_id))
        .await?
        .is_empty())
}

/// Returns a list of questions visible to the specified owner, localized to the given locale.
pub async fn list_questions(
    db: &PgPool,
    owner: Option<Uuid>,
    locale: &str,
) -> Result<Vec<QuestionDto>, ComhairleError> {
    let rows = visible_question_rows(db, owner, None).await?;
    question_dtos(db, rows, locale).await
}

/// Returns the specified question if it is visible to the given owner, localized to the given locale.
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

/// Converts a list of question rows into their corresponding DTOs, including tags and localized versions.
async fn question_dtos(
    db: &PgPool,
    rows: Vec<QuestionRow>,
    locale: &str,
) -> Result<Vec<QuestionDto>, ComhairleError> {
    if rows.is_empty() {
        return Ok(Vec::new());
    }
    let ids: Vec<Uuid> = rows.iter().map(|row| row.id).collect();

    let (sql, values) = Query::select()
        .column((QuestionTagIden::Table, QuestionTagIden::QuestionId))
        .columns([
            (DemographicTagIden::Table, DemographicTagIden::Id),
            (
                DemographicTagIden::Table,
                DemographicTagIden::OrganizationId,
            ),
            (DemographicTagIden::Table, DemographicTagIden::Name),
        ])
        .from(QuestionTagIden::Table)
        .inner_join(
            DemographicTagIden::Table,
            Expr::col((DemographicTagIden::Table, DemographicTagIden::Id))
                .equals((QuestionTagIden::Table, QuestionTagIden::TagId)),
        )
        .and_where(
            Expr::col((QuestionTagIden::Table, QuestionTagIden::QuestionId))
                .is_in(ids.iter().copied()),
        )
        .order_by(
            (DemographicTagIden::Table, DemographicTagIden::Name),
            Order::Asc,
        )
        .build_sqlx(PostgresQueryBuilder);
    let tag_rows = sqlx::query_as_with::<_, QuestionTagRow, _>(&sql, values)
        .fetch_all(db)
        .await?;
    let mut tags_by_question: HashMap<Uuid, Vec<DemographicTag>> = HashMap::new();
    for row in tag_rows {
        tags_by_question
            .entry(row.question_id)
            .or_default()
            .push(row.tag);
    }

    let (sql, values) = Query::select()
        .columns(VERSION_COLUMNS)
        .from(DemographicQuestionVersionIden::Table)
        .and_where(Expr::col(DemographicQuestionVersionIden::QuestionId).is_in(ids.iter().copied()))
        .order_by(DemographicQuestionVersionIden::QuestionId, Order::Asc)
        .order_by(DemographicQuestionVersionIden::VersionNumber, Order::Asc)
        .build_sqlx(PostgresQueryBuilder);
    let version_rows = sqlx::query_as_with::<_, DemographicQuestionVersion, _>(&sql, values)
        .fetch_all(db)
        .await?;
    let mut versions_by_question: HashMap<Uuid, Vec<QuestionVersionDto>> = HashMap::new();
    for version in hydrate_versions(db, version_rows, locale).await? {
        versions_by_question
            .entry(version.version.question_id)
            .or_default()
            .push(version);
    }

    Ok(rows
        .into_iter()
        .map(|row| {
            let versions = versions_by_question.remove(&row.id).unwrap_or_default();
            let latest_published = versions
                .iter()
                .filter(|dto| dto.version.status == VersionStatus::Published)
                .max_by_key(|dto| dto.version.version_number)
                .cloned();
            let draft = versions
                .iter()
                .find(|dto| dto.version.status == VersionStatus::Draft)
                .cloned();
            QuestionDto {
                tags: tags_by_question.remove(&row.id).unwrap_or_default(),
                versions: versions
                    .iter()
                    .map(|dto| VersionSummary::from(&dto.version))
                    .collect(),
                question: row,
                latest_published,
                draft,
            }
        })
        .collect())
}

/// Takes a list of demographic question version rows and a locale, and returns a list of hydrated question version DTOs with localized text and options.
async fn hydrate_versions(
    db: &PgPool,
    rows: Vec<DemographicQuestionVersion>,
    locale: &str,
) -> Result<Vec<QuestionVersionDto>, ComhairleError> {
    if rows.is_empty() {
        return Ok(Vec::new());
    }
    let version_ids: Vec<Uuid> = rows.iter().map(|row| row.id).collect();
    let (sql, values) = Query::select()
        .columns([
            OptionRowIden::Id,
            OptionRowIden::QuestionVersionId,
            OptionRowIden::Position,
            OptionRowIden::LabelTextContentId,
        ])
        .from(OptionRowIden::Table)
        .and_where(Expr::col(OptionRowIden::QuestionVersionId).is_in(version_ids))
        .order_by(OptionRowIden::QuestionVersionId, Order::Asc)
        .order_by(OptionRowIden::Position, Order::Asc)
        .build_sqlx(PostgresQueryBuilder);
    let option_rows = sqlx::query_as_with::<_, OptionRow, _>(&sql, values)
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
        .map(|row| {
            let text = texts.get(&row.text_content_id).cloned().unwrap_or_default();
            let consent_text = row
                .consent_text_content_id
                .and_then(|id| texts.get(&id).cloned());
            let options = options_by_version.remove(&row.id).unwrap_or_default();
            QuestionVersionDto {
                version: row,
                text,
                options,
                consent_text,
            }
        })
        .collect())
}

/// Loads specific versions by id, in the order requested. Missing ids are skipped.
pub async fn load_versions(
    db: &PgPool,
    version_ids: &[Uuid],
    locale: &str,
) -> Result<Vec<QuestionVersionDto>, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(VERSION_COLUMNS)
        .from(DemographicQuestionVersionIden::Table)
        .and_where(Expr::col(DemographicQuestionVersionIden::Id).is_in(version_ids.iter().copied()))
        .build_sqlx(PostgresQueryBuilder);
    let rows = sqlx::query_as_with::<_, DemographicQuestionVersion, _>(&sql, values)
        .fetch_all(db)
        .await?;
    let mut by_id: HashMap<Uuid, QuestionVersionDto> = hydrate_versions(db, rows, locale)
        .await?
        .into_iter()
        .map(|dto| (dto.version.id, dto))
        .collect();
    Ok(version_ids
        .iter()
        .filter_map(|id| by_id.remove(id))
        .collect())
}

/// Loads a specific demographic question version by its ID, localized to the given locale.
/// Returns an error if the version is not found.
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

/// Loads the latest published version of a demographic question, localized to the given locale.
/// Returns `None` if no published version exists.
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

/// Retrieves the ID of the latest published version of a demographic question, if it exists.
pub async fn latest_published_version_id(
    executor: impl PgExecutor<'_>,
    question_id: Uuid,
) -> Result<Option<Uuid>, ComhairleError> {
    let (sql, values) = Query::select()
        .column(DemographicQuestionVersionIden::Id)
        .from(DemographicQuestionVersionIden::Table)
        .and_where(Expr::col(DemographicQuestionVersionIden::QuestionId).eq(question_id))
        .and_where(Expr::col(DemographicQuestionVersionIden::Status).eq(VersionStatus::Published))
        .order_by(DemographicQuestionVersionIden::VersionNumber, Order::Desc)
        .limit(1)
        .build_sqlx(PostgresQueryBuilder);
    Ok(sqlx::query_scalar_with(&sql, values)
        .fetch_optional(executor)
        .await?)
}

/// Retrieves the ID of the draft version of a demographic question, if it exists.
async fn draft_version_id(
    conn: &mut PgConnection,
    question_id: Uuid,
) -> Result<Option<Uuid>, ComhairleError> {
    let (sql, values) = Query::select()
        .column(DemographicQuestionVersionIden::Id)
        .from(DemographicQuestionVersionIden::Table)
        .and_where(Expr::col(DemographicQuestionVersionIden::QuestionId).eq(question_id))
        .and_where(Expr::col(DemographicQuestionVersionIden::Status).eq(VersionStatus::Draft))
        .build_sqlx(PostgresQueryBuilder);
    Ok(sqlx::query_scalar_with(&sql, values)
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

fn not_found() -> ComhairleError {
    ComhairleError::ResourceNotFound("demographic question".into())
}

/// Locks a question the owner may write to. Questions owned by another organization,
/// and system defaults when `owner` is an organization, are reported as not found.
async fn lock_owned_question(
    conn: &mut PgConnection,
    owner: Option<Uuid>,
    question_id: Uuid,
) -> Result<(), ComhairleError> {
    let (sql, values) = Query::select()
        .column(QuestionRowIden::OrganizationId)
        .from(QuestionRowIden::Table)
        .and_where(Expr::col(QuestionRowIden::Id).eq(question_id))
        .lock(LockType::Update)
        .build_sqlx(PostgresQueryBuilder);
    let organization_id: Option<Option<Uuid>> = sqlx::query_scalar_with(&sql, values)
        .fetch_optional(&mut *conn)
        .await?;
    match organization_id {
        Some(organization_id) if organization_id == owner => Ok(()),
        _ => Err(not_found()),
    }
}

/// Copies a version (and its options) into a new draft of `question_id`, numbered
/// after every existing version so numbers never repeat.
async fn copy_version_to_draft(
    conn: &mut PgConnection,
    source_version_id: Uuid,
    question_id: Uuid,
) -> Result<Uuid, ComhairleError> {
    let draft_id = Uuid::new_v4();
    let copied_version = Query::select()
        .expr(Expr::val(draft_id))
        .expr(Expr::val(question_id))
        .expr(next_version_number(question_id))
        .expr(Expr::val(VersionStatus::Draft))
        .columns(CONTENT_COLUMNS)
        .from(DemographicQuestionVersionIden::Table)
        .and_where(Expr::col(DemographicQuestionVersionIden::Id).eq(source_version_id))
        .to_owned();
    let (sql, values) = Query::insert()
        .into_table(DemographicQuestionVersionIden::Table)
        .columns(
            [
                DemographicQuestionVersionIden::Id,
                DemographicQuestionVersionIden::QuestionId,
                DemographicQuestionVersionIden::VersionNumber,
                DemographicQuestionVersionIden::Status,
            ]
            .into_iter()
            .chain(CONTENT_COLUMNS),
        )
        .select_from(copied_version)?
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(&mut *conn).await?;

    let copied_options = Query::select()
        .expr(Func::cust(Alias::new("uuid_generate_v4")))
        .expr(Expr::val(draft_id))
        .columns([OptionRowIden::Position, OptionRowIden::LabelTextContentId])
        .from(OptionRowIden::Table)
        .and_where(Expr::col(OptionRowIden::QuestionVersionId).eq(source_version_id))
        .to_owned();
    let (sql, values) = Query::insert()
        .into_table(OptionRowIden::Table)
        .columns([
            OptionRowIden::Id,
            OptionRowIden::QuestionVersionId,
            OptionRowIden::Position,
            OptionRowIden::LabelTextContentId,
        ])
        .select_from(copied_options)?
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(&mut *conn).await?;
    Ok(draft_id)
}

/// Returns a sub-query for the next version number of a question, so numbers never repeat.
fn next_version_number(question_id: Uuid) -> SimpleExpr {
    let next = Query::select()
        .expr(
            Expr::expr(Func::coalesce([
                Func::max(Expr::col(DemographicQuestionVersionIden::VersionNumber)).into(),
                Expr::val(0).into(),
            ]))
            .add(1),
        )
        .from(DemographicQuestionVersionIden::Table)
        .and_where(Expr::col(DemographicQuestionVersionIden::QuestionId).eq(question_id))
        .to_owned();
    SimpleExpr::SubQuery(None, Box::new(next.into_sub_query_statement()))
}

async fn reject_if_archived(
    conn: &mut PgConnection,
    question_id: Uuid,
) -> Result<(), ComhairleError> {
    let (sql, values) = Query::select()
        .expr(Expr::col(QuestionRowIden::ArchivedAt).is_not_null())
        .from(QuestionRowIden::Table)
        .and_where(Expr::col(QuestionRowIden::Id).eq(question_id))
        .build_sqlx(PostgresQueryBuilder);
    let archived: bool = sqlx::query_scalar_with(&sql, values)
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
    let (sql, values) = Query::select()
        .expr(Func::count(Expr::col(Asterisk)))
        .from(DemographicTagIden::Table)
        .and_where(Expr::col(DemographicTagIden::Id).is_in(ids.iter().copied()))
        .cond_where(tag_visible_to(owner))
        .build_sqlx(PostgresQueryBuilder);
    let usable: i64 = sqlx::query_scalar_with(&sql, values)
        .fetch_one(&mut *conn)
        .await?;
    if usable != ids.len() as i64 {
        return Err(invalid(
            "one or more tags do not exist for this organization",
        ));
    }
    Ok(())
}

/// Tags visible to an owner are the system tags plus the organization's own.
fn tag_visible_to(owner: Option<Uuid>) -> Condition {
    Condition::any()
        .add(Expr::col(DemographicTagIden::OrganizationId).is_null())
        .add_option(
            owner.map(|owner_id| Expr::col(DemographicTagIden::OrganizationId).eq(owner_id)),
        )
}

async fn replace_tags(
    conn: &mut PgConnection,
    question_id: Uuid,
    tag_ids: &[Uuid],
) -> Result<(), ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(QuestionTagIden::Table)
        .and_where(Expr::col(QuestionTagIden::QuestionId).eq(question_id))
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(&mut *conn).await?;
    let ids = unique_ids(tag_ids);
    if ids.is_empty() {
        return Ok(());
    }
    let mut insert = Query::insert();
    insert
        .into_table(QuestionTagIden::Table)
        .columns([QuestionTagIden::QuestionId, QuestionTagIden::TagId]);
    for tag_id in ids {
        insert.values([question_id.into(), tag_id.into()])?;
    }
    let (sql, values) = insert.build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(&mut *conn).await?;
    Ok(())
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

/// The version columns that hold editable content.
const CONTENT_COLUMNS: [DemographicQuestionVersionIden; 11] = [
    DemographicQuestionVersionIden::TextContentId,
    DemographicQuestionVersionIden::QuestionType,
    DemographicQuestionVersionIden::DatePrecision,
    DemographicQuestionVersionIden::LocationFormat,
    DemographicQuestionVersionIden::LocationCountry,
    DemographicQuestionVersionIden::MaxSelections,
    DemographicQuestionVersionIden::AllowOther,
    DemographicQuestionVersionIden::PreferNotToSay,
    DemographicQuestionVersionIden::JsonSchema,
    DemographicQuestionVersionIden::SpecialCategory,
    DemographicQuestionVersionIden::ConsentTextContentId,
];

/// Pairs each of the `CONTENT_COLUMNS` with the value taken from the draft input.
fn draft_content(
    input: &QuestionDraftInput,
    texts: &DraftText,
) -> [(DemographicQuestionVersionIden, SimpleExpr); 11] {
    use DemographicQuestionVersionIden as Col;
    let location_country = input
        .location_country
        .as_deref()
        .map(|country| country.trim().to_owned());
    [
        (Col::TextContentId, Expr::val(texts.text.into_uuid()).into()),
        (Col::QuestionType, Expr::val(input.question_type).into()),
        (Col::DatePrecision, Expr::val(input.date_precision).into()),
        (Col::LocationFormat, Expr::val(input.location_format).into()),
        (Col::LocationCountry, Expr::val(location_country).into()),
        (Col::MaxSelections, Expr::val(input.max_selections).into()),
        (Col::AllowOther, Expr::val(input.allow_other).into()),
        (
            Col::PreferNotToSay,
            Expr::val(input.prefer_not_to_say).into(),
        ),
        (Col::JsonSchema, Expr::val(input.json_schema.clone()).into()),
        (
            Col::SpecialCategory,
            Expr::val(input.special_category).into(),
        ),
        (
            Col::ConsentTextContentId,
            Expr::val(texts.consent.map(TextContentId::into_uuid)).into(),
        ),
    ]
}

async fn insert_draft_version(
    conn: &mut PgConnection,
    question_id: Uuid,
    version_number: i32,
    input: &QuestionDraftInput,
    texts: &DraftText,
) -> Result<Uuid, ComhairleError> {
    let version_id = Uuid::new_v4();
    let identity = [
        (DemographicQuestionVersionIden::Id, version_id.into()),
        (
            DemographicQuestionVersionIden::QuestionId,
            question_id.into(),
        ),
        (
            DemographicQuestionVersionIden::VersionNumber,
            version_number.into(),
        ),
        (
            DemographicQuestionVersionIden::Status,
            Expr::val(VersionStatus::Draft).into(),
        ),
    ];
    let (columns, values): (Vec<_>, Vec<_>) = identity
        .into_iter()
        .chain(draft_content(input, texts))
        .unzip();
    let (sql, values) = Query::insert()
        .into_table(DemographicQuestionVersionIden::Table)
        .columns(columns)
        .values(values)?
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(&mut *conn).await?;
    replace_options(&mut *conn, version_id, &texts.options).await?;
    Ok(version_id)
}

async fn save_draft_fields(
    conn: &mut PgConnection,
    version_id: Uuid,
    input: &QuestionDraftInput,
    texts: &DraftText,
) -> Result<(), ComhairleError> {
    let (sql, values) = Query::update()
        .table(DemographicQuestionVersionIden::Table)
        .values(draft_content(input, texts))
        .value(
            DemographicQuestionVersionIden::UpdatedAt,
            Expr::current_timestamp(),
        )
        .and_where(Expr::col(DemographicQuestionVersionIden::Id).eq(version_id))
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(&mut *conn).await?;
    replace_options(&mut *conn, version_id, &texts.options).await
}

async fn replace_options(
    conn: &mut PgConnection,
    version_id: Uuid,
    label_ids: &[TextContentId],
) -> Result<(), ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(OptionRowIden::Table)
        .and_where(Expr::col(OptionRowIden::QuestionVersionId).eq(version_id))
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(&mut *conn).await?;
    if label_ids.is_empty() {
        return Ok(());
    }
    let mut insert = Query::insert();
    insert.into_table(OptionRowIden::Table).columns([
        OptionRowIden::Id,
        OptionRowIden::QuestionVersionId,
        OptionRowIden::Position,
        OptionRowIden::LabelTextContentId,
    ]);
    for (position, label_id) in label_ids.iter().enumerate() {
        insert.values([
            Uuid::new_v4().into(),
            version_id.into(),
            (position as i32).into(),
            label_id.into_uuid().into(),
        ])?;
    }
    let (sql, values) = insert.build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(&mut *conn).await?;
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
    let (sql, values) = Query::insert()
        .into_table(QuestionRowIden::Table)
        .columns([QuestionRowIden::Id, QuestionRowIden::OrganizationId])
        .values([question_id.into(), owner.into()])?
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(&mut *conn).await?;
    let texts = write_draft_text(conn, locale, input).await?;
    insert_draft_version(conn, question_id, 1, input, &texts).await?;
    replace_tags(conn, question_id, &input.tag_ids).await?;
    tx.commit().await?;
    get_question(db, owner, question_id, locale).await
}

/// Starts a draft from the latest published version.
pub async fn start_draft(
    db: &PgPool,
    owner: Option<Uuid>,
    question_id: Uuid,
    locale: &str,
) -> Result<QuestionDto, ComhairleError> {
    let mut tx = db.begin().await?;
    let conn = &mut *tx;
    lock_owned_question(conn, owner, question_id).await?;
    reject_if_archived(conn, question_id).await?;
    if draft_version_id(conn, question_id).await?.is_some() {
        return Err(ComhairleError::Conflict(
            "demographic question already has a draft".into(),
        ));
    }
    let source = latest_published_version_id(&mut *conn, question_id)
        .await?
        .ok_or_else(|| invalid("demographic question has no published version to edit"))?;
    copy_version_to_draft(conn, source, question_id).await?;
    tx.commit().await?;
    get_question(db, owner, question_id, locale).await
}

/// Replaces the draft's content.
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
    lock_owned_question(conn, owner, question_id).await?;
    reject_if_archived(conn, question_id).await?;
    let draft_id = require_draft(conn, question_id).await?;
    validate_tag_ids(conn, owner, &input.tag_ids).await?;
    let texts = write_draft_text(conn, locale, input).await?;
    save_draft_fields(conn, draft_id, input, &texts).await?;
    replace_tags(conn, question_id, &input.tag_ids).await?;
    tx.commit().await?;
    get_question(db, owner, question_id, locale).await
}

/// Deletes the draft row. Published versions are untouched, and a question that was
/// never published is removed.
pub async fn discard_draft(
    db: &PgPool,
    owner: Option<Uuid>,
    question_id: Uuid,
) -> Result<(), ComhairleError> {
    let mut tx = db.begin().await?;
    let conn = &mut *tx;
    lock_owned_question(conn, owner, question_id).await?;
    let (sql, values) = Query::delete()
        .from_table(DemographicQuestionVersionIden::Table)
        .and_where(Expr::col(DemographicQuestionVersionIden::QuestionId).eq(question_id))
        .and_where(Expr::col(DemographicQuestionVersionIden::Status).eq(VersionStatus::Draft))
        .build_sqlx(PostgresQueryBuilder);
    let deleted = sqlx::query_with(&sql, values)
        .execute(&mut *conn)
        .await?
        .rows_affected();
    if deleted == 0 {
        return Err(ComhairleError::ResourceNotFound(
            "draft for demographic question".into(),
        ));
    }
    let has_versions = Query::select()
        .expr(Expr::val(1))
        .from(DemographicQuestionVersionIden::Table)
        .and_where(
            Expr::col((
                DemographicQuestionVersionIden::Table,
                DemographicQuestionVersionIden::QuestionId,
            ))
            .equals((QuestionRowIden::Table, QuestionRowIden::Id)),
        )
        .to_owned();
    let (sql, values) = Query::delete()
        .from_table(QuestionRowIden::Table)
        .and_where(Expr::col(QuestionRowIden::Id).eq(question_id))
        .and_where(Expr::exists(has_versions).not())
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(&mut *conn).await?;
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
    lock_owned_question(conn, owner, question_id).await?;
    reject_if_archived(conn, question_id).await?;
    let draft_id = require_draft(conn, question_id).await?;
    let (sql, values) = Query::select()
        .expr(Func::count(Expr::col(Asterisk)))
        .from(QuestionTagIden::Table)
        .and_where(Expr::col(QuestionTagIden::QuestionId).eq(question_id))
        .build_sqlx(PostgresQueryBuilder);
    let tag_count: i64 = sqlx::query_scalar_with(&sql, values)
        .fetch_one(&mut *conn)
        .await?;
    if tag_count == 0 {
        return Err(invalid(
            "a demographic question needs at least one tag before publishing",
        ));
    }
    let (sql, values) = Query::update()
        .table(DemographicQuestionVersionIden::Table)
        .value(
            DemographicQuestionVersionIden::Status,
            Expr::val(VersionStatus::Published),
        )
        .value(
            DemographicQuestionVersionIden::PublishedAt,
            Expr::current_timestamp(),
        )
        .value(
            DemographicQuestionVersionIden::UpdatedAt,
            Expr::current_timestamp(),
        )
        .and_where(Expr::col(DemographicQuestionVersionIden::Id).eq(draft_id))
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(&mut *conn).await?;
    tx.commit().await?;
    get_question(db, owner, question_id, locale).await
}

pub async fn archive_question(
    db: &PgPool,
    owner: Option<Uuid>,
    question_id: Uuid,
    locale: &str,
) -> Result<QuestionDto, ComhairleError> {
    let mut tx = db.begin().await?;
    let conn = &mut *tx;
    lock_owned_question(conn, owner, question_id).await?;
    let (sql, values) = Query::update()
        .table(QuestionRowIden::Table)
        .value(
            QuestionRowIden::ArchivedAt,
            Func::coalesce([
                Expr::col(QuestionRowIden::ArchivedAt).into(),
                Expr::current_timestamp().into(),
            ]),
        )
        .and_where(Expr::col(QuestionRowIden::Id).eq(question_id))
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(&mut *conn).await?;
    tx.commit().await?;
    get_question(db, owner, question_id, locale).await
}

pub async fn list_tags(
    db: &PgPool,
    owner: Option<Uuid>,
) -> Result<Vec<DemographicTag>, ComhairleError> {
    let (sql, values) = Query::select()
        .columns([
            DemographicTagIden::Id,
            DemographicTagIden::OrganizationId,
            DemographicTagIden::Name,
        ])
        .from(DemographicTagIden::Table)
        .cond_where(tag_visible_to(owner))
        .order_by(DemographicTagIden::Name, Order::Asc)
        .build_sqlx(PostgresQueryBuilder);
    Ok(sqlx::query_as_with::<_, DemographicTag, _>(&sql, values)
        .fetch_all(db)
        .await?)
}

pub async fn create_tag(
    db: &PgPool,
    owner: Option<Uuid>,
    name: &str,
) -> Result<DemographicTag, ComhairleError> {
    let name = name.trim();
    if name.is_empty() {
        return Err(invalid("tag name must not be blank"));
    }
    let (sql, values) = Query::insert()
        .into_table(DemographicTagIden::Table)
        .columns([DemographicTagIden::OrganizationId, DemographicTagIden::Name])
        .values([owner.into(), name.into()])?
        .returning(Query::returning().columns([
            DemographicTagIden::Id,
            DemographicTagIden::OrganizationId,
            DemographicTagIden::Name,
        ]))
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_as_with::<_, DemographicTag, _>(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("demographics tag")
}
