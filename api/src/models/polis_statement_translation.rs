use std::collections::HashSet;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{Expr, OnConflict, PostgresQueryBuilder, Query, SimpleExpr, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::{PgPool, query_as_with};
use tracing::{instrument, warn};
use uuid::Uuid;

use crate::error::ComhairleError;
use crate::models::polis_statement_aux::PolisStatementAuxIden;
use crate::translation_service::TranslationService;

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "polis_statement_translation")]
pub struct PolisStatementTranslation {
    pub id: Uuid,
    pub polis_statement_aux_id: Uuid,
    pub locale: String,
    pub content: String,
    /// Whether this translation was produced by the machine translation service.
    pub ai_generated: bool,
    /// Whether this translation still needs a human to validate it before it is
    /// treated as trustworthy for display.
    pub requires_validation: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [PolisStatementTranslationIden; 8] = [
    PolisStatementTranslationIden::Id,
    PolisStatementTranslationIden::PolisStatementAuxId,
    PolisStatementTranslationIden::Locale,
    PolisStatementTranslationIden::Content,
    PolisStatementTranslationIden::AiGenerated,
    PolisStatementTranslationIden::RequiresValidation,
    PolisStatementTranslationIden::CreatedAt,
    PolisStatementTranslationIden::UpdatedAt,
];

/// Insert a translation for a statement, or refresh its `content` if one already
/// exists for that `(polis_statement_aux_id, locale)` pair. Re-translating (e.g.
/// after a statement edit) overwrites the stored text and re-flags it for
/// validation.
#[instrument(err(Debug), skip(db))]
pub async fn upsert(
    db: &PgPool,
    polis_statement_aux_id: Uuid,
    locale: &str,
    content: &str,
    ai_generated: bool,
    requires_validation: bool,
) -> Result<PolisStatementTranslation, ComhairleError> {
    let columns = [
        PolisStatementTranslationIden::PolisStatementAuxId,
        PolisStatementTranslationIden::Locale,
        PolisStatementTranslationIden::Content,
        PolisStatementTranslationIden::AiGenerated,
        PolisStatementTranslationIden::RequiresValidation,
    ];
    let values: Vec<SimpleExpr> = vec![
        polis_statement_aux_id.into(),
        locale.into(),
        content.into(),
        ai_generated.into(),
        requires_validation.into(),
    ];

    let (sql, values) = Query::insert()
        .into_table(PolisStatementTranslationIden::Table)
        .columns(columns)
        .values(values)?
        .on_conflict(
            OnConflict::columns([
                PolisStatementTranslationIden::PolisStatementAuxId,
                PolisStatementTranslationIden::Locale,
            ])
            .update_columns([
                PolisStatementTranslationIden::Content,
                PolisStatementTranslationIden::AiGenerated,
                PolisStatementTranslationIden::RequiresValidation,
            ])
            .value(
                PolisStatementTranslationIden::UpdatedAt,
                Expr::current_timestamp(),
            )
            .to_owned(),
        )
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let translation = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(translation)
}

/// List all stored translations for a statement.
#[instrument(err(Debug))]
pub async fn list_by_statement_aux_id(
    db: &PgPool,
    polis_statement_aux_id: &Uuid,
) -> Result<Vec<PolisStatementTranslation>, ComhairleError> {
    let (sql, values) = Query::select()
        .from(PolisStatementTranslationIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(
            Expr::col(PolisStatementTranslationIden::PolisStatementAuxId)
                .eq(*polis_statement_aux_id),
        )
        .build_sqlx(PostgresQueryBuilder);

    let translations = query_as_with(&sql, values).fetch_all(db).await?;

    Ok(translations)
}

/// List every stored translation for the statements of one workflow step, so the
/// moderation queue can show them without a request per statement.
#[instrument(err(Debug))]
pub async fn list_by_workflow_step_id(
    db: &PgPool,
    workflow_step_id: &Uuid,
) -> Result<Vec<PolisStatementTranslation>, ComhairleError> {
    let statements_in_step = Query::select()
        .column(PolisStatementAuxIden::Id)
        .from(PolisStatementAuxIden::Table)
        .and_where(Expr::col(PolisStatementAuxIden::WorkflowStepId).eq(*workflow_step_id))
        .to_owned();

    let (sql, values) = Query::select()
        .from(PolisStatementTranslationIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(
            Expr::col(PolisStatementTranslationIden::PolisStatementAuxId)
                .in_subquery(statements_in_step),
        )
        .build_sqlx(PostgresQueryBuilder);

    let translations = query_as_with(&sql, values).fetch_all(db).await?;

    Ok(translations)
}

/// Fetch a single translation by id.
#[instrument(err(Debug))]
pub async fn get_by_id(
    db: &PgPool,
    id: &Uuid,
) -> Result<PolisStatementTranslation, ComhairleError> {
    let (sql, values) = Query::select()
        .from(PolisStatementTranslationIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(Expr::col(PolisStatementTranslationIden::Id).eq(*id))
        .build_sqlx(PostgresQueryBuilder);

    let translation = query_as_with(&sql, values).fetch_optional(db).await?;

    translation
        .ok_or_else(|| ComhairleError::ResourceNotFound("No translation found with this id".into()))
}

/// Mark a translation as human-verified without changing its `content` or
/// `ai_generated` flag: an admin read the machine translation and confirmed it's
/// correct as-is.
#[instrument(err(Debug), skip(db))]
pub async fn verify(db: &PgPool, id: Uuid) -> Result<PolisStatementTranslation, ComhairleError> {
    let (sql, values) = Query::update()
        .table(PolisStatementTranslationIden::Table)
        .values([(
            PolisStatementTranslationIden::RequiresValidation,
            false.into(),
        )])
        .value(
            PolisStatementTranslationIden::UpdatedAt,
            Expr::current_timestamp(),
        )
        .and_where(Expr::col(PolisStatementTranslationIden::Id).eq(id))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let translation = query_as_with(&sql, values).fetch_optional(db).await?;

    translation
        .ok_or_else(|| ComhairleError::ResourceNotFound("No translation found with this id".into()))
}

/// Overwrite a translation's `content` with an admin-provided correction. Clears
/// `ai_generated` and `requires_validation`: the text is no longer the raw
/// machine-translation output, and a human has just reviewed it.
#[instrument(err(Debug), skip(db))]
pub async fn update_content(
    db: &PgPool,
    id: Uuid,
    content: &str,
) -> Result<PolisStatementTranslation, ComhairleError> {
    let (sql, values) = Query::update()
        .table(PolisStatementTranslationIden::Table)
        .values([
            (PolisStatementTranslationIden::Content, content.into()),
            (PolisStatementTranslationIden::AiGenerated, false.into()),
            (
                PolisStatementTranslationIden::RequiresValidation,
                false.into(),
            ),
        ])
        .value(
            PolisStatementTranslationIden::UpdatedAt,
            Expr::current_timestamp(),
        )
        .and_where(Expr::col(PolisStatementTranslationIden::Id).eq(id))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let translation = query_as_with(&sql, values).fetch_optional(db).await?;

    translation
        .ok_or_else(|| ComhairleError::ResourceNotFound("No translation found with this id".into()))
}

/// Machine-translate `statement_text` into every `supported_language` other than
/// `source_locale` that has no translation yet. Each generated row is flagged
/// `ai_generated` and `requires_validation`.
///
/// Existing translations are left alone: Polis statement text never changes, and a
/// moderator may have corrected or verified one. Every statement sync calls this, so
/// regenerating would wipe those corrections and re-pay for every translation.
///
/// Individual target failures are logged and skipped rather than aborting the
/// whole batch, so a single unsupported locale does not lose the others. This is
/// intended to be spawned as a background task off the submission request.
#[instrument(err(Debug), skip(db, translator, statement_text))]
pub async fn generate_for_statement(
    db: &PgPool,
    translator: &Arc<dyn TranslationService>,
    polis_statement_aux_id: Uuid,
    statement_text: &str,
    source_locale: &str,
    supported_languages: &[String],
) -> Result<Vec<PolisStatementTranslation>, ComhairleError> {
    let existing_locales: HashSet<String> = list_by_statement_aux_id(db, &polis_statement_aux_id)
        .await?
        .into_iter()
        .map(|translation| translation.locale)
        .collect();

    let mut out = vec![];

    for locale in supported_languages.iter().filter(|locale| {
        locale.as_str() != source_locale && !existing_locales.contains(locale.as_str())
    }) {
        let translated = match translator
            .translate_from_to(statement_text, source_locale, locale)
            .await
        {
            Ok(text) => text,
            Err(err) => {
                warn!(
                    ?err,
                    locale, %polis_statement_aux_id,
                    "failed to translate statement into locale, skipping"
                );
                continue;
            }
        };

        let row = upsert(db, polis_statement_aux_id, locale, &translated, true, true).await?;
        out.push(row);
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use serde_json::json;
    use sqlx::PgPool;

    use super::*;
    use crate::models::model_test_helpers::setup_default_app_and_session;
    use crate::models::polis_statement_aux::{self, UpsertFromPolis};
    use crate::test_helpers::{extract, polis_tool_config};
    use crate::translation_service::{MockTranslationService, TranslationService};
    use crate::wiki_poll_service::ModerationStatus;

    async fn seed_aux(pool: &PgPool) -> Result<Uuid, Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(pool).await?;

        let (_, conversation, _) = session.create_random_conversation(&app).await?;
        let conversation_id: String = extract("id", &conversation);

        let (_, workflow, _) = session
            .create_random_workflow(&app, &conversation_id)
            .await?;
        let workflow_id: String = extract("id", &workflow);

        let (_, workflow_step, _) = session
            .post(
                &app,
                &format!("/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step"),
                json!({
                    "name": "Polis step",
                    "step_order": 1,
                    "activation_rule": "manual",
                    "description": "polis step",
                    "is_offline": false,
                    "required": true,
                    "tool_setup": polis_tool_config(),
                })
                .to_string()
                .into(),
            )
            .await?;
        let workflow_step_id: String = extract("id", &workflow_step);
        let workflow_step_id = Uuid::parse_str(&workflow_step_id)?;

        let aux = polis_statement_aux::upsert_from_polis(
            pool,
            &UpsertFromPolis {
                workflow_step_id,
                user_id: None,
                zid: 7,
                polis_conversation_id: "test-poll".into(),
                polis_statement_id: 42,
                statement_text: "hello".into(),
                source_locale: Some("en".into()),
                is_seed: false,
                moderation_status: ModerationStatus::Pending,
            },
        )
        .await?;

        Ok(aux.id)
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn generates_one_translation_per_non_source_language(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let aux_id = seed_aux(&pool).await?;

        let translator: Arc<dyn TranslationService> = Arc::new(MockTranslationService::base());
        let supported = vec!["en".to_string(), "es".to_string(), "fr".to_string()];

        let generated =
            generate_for_statement(&pool, &translator, aux_id, "hello", "en", &supported).await?;

        // Source locale ("en") is skipped; "es" and "fr" are produced.
        assert_eq!(generated.len(), 2);
        assert!(
            generated
                .iter()
                .all(|t| t.ai_generated && t.requires_validation)
        );

        let stored = list_by_statement_aux_id(&pool, &aux_id).await?;
        let mut locales: Vec<_> = stored.iter().map(|t| t.locale.clone()).collect();
        locales.sort();
        assert_eq!(locales, vec!["es".to_string(), "fr".to_string()]);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn verify_clears_requires_validation_only(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let aux_id = seed_aux(&pool).await?;

        let row = upsert(&pool, aux_id, "es", "hola generado", true, true).await?;

        let verified = verify(&pool, row.id).await?;

        assert_eq!(verified.content, "hola generado");
        assert!(verified.ai_generated);
        assert!(!verified.requires_validation);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn update_content_overwrites_and_clears_flags(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let aux_id = seed_aux(&pool).await?;

        let row = upsert(&pool, aux_id, "es", "hola generado", true, true).await?;

        let updated = update_content(&pool, row.id, "hola corregido").await?;

        assert_eq!(updated.content, "hola corregido");
        assert!(!updated.ai_generated);
        assert!(!updated.requires_validation);

        let fetched = get_by_id(&pool, &row.id).await?;
        assert_eq!(fetched.content, "hola corregido");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn re_generating_only_adds_missing_languages(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let aux_id = seed_aux(&pool).await?;

        let translator: Arc<dyn TranslationService> = Arc::new(MockTranslationService::base());
        let before = vec!["en".to_string(), "es".to_string()];
        let after = vec!["en".to_string(), "es".to_string(), "fr".to_string()];

        generate_for_statement(&pool, &translator, aux_id, "hello", "en", &before).await?;
        let added =
            generate_for_statement(&pool, &translator, aux_id, "hello", "en", &after).await?;

        assert_eq!(added.len(), 1);
        assert_eq!(added[0].locale, "fr");

        let mut stored: Vec<String> = list_by_statement_aux_id(&pool, &aux_id)
            .await?
            .into_iter()
            .map(|translation| translation.locale)
            .collect();
        stored.sort();
        assert_eq!(
            stored,
            vec!["es", "fr"],
            "one row per language, no duplicates"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn re_generating_keeps_a_moderator_correction(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let aux_id = seed_aux(&pool).await?;

        let translator: Arc<dyn TranslationService> = Arc::new(MockTranslationService::base());
        let supported = vec!["en".to_string(), "es".to_string()];

        let generated =
            generate_for_statement(&pool, &translator, aux_id, "hello", "en", &supported).await?;
        update_content(&pool, generated[0].id, "hola corregido").await?;

        let regenerated =
            generate_for_statement(&pool, &translator, aux_id, "hello", "en", &supported).await?;
        assert!(
            regenerated.is_empty(),
            "nothing missing, so nothing to translate"
        );

        let kept = get_by_id(&pool, &generated[0].id).await?;
        assert_eq!(kept.content, "hola corregido");
        assert!(!kept.ai_generated);
        assert!(!kept.requires_validation);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn lists_translations_for_a_whole_step(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let aux_id = seed_aux(&pool).await?;
        let aux = polis_statement_aux::get_by_id(&pool, &aux_id).await?;

        let translator: Arc<dyn TranslationService> = Arc::new(MockTranslationService::base());
        let supported = vec!["en".to_string(), "es".to_string()];
        generate_for_statement(&pool, &translator, aux_id, "hello", "en", &supported).await?;

        let in_step = list_by_workflow_step_id(&pool, &aux.workflow_step_id).await?;
        assert_eq!(in_step.len(), 1);
        assert_eq!(in_step[0].polis_statement_aux_id, aux_id);

        let other_step = list_by_workflow_step_id(&pool, &Uuid::new_v4()).await?;
        assert!(other_step.is_empty());

        Ok(())
    }
}
