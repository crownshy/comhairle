use std::sync::Arc;

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{Expr, OnConflict, PostgresQueryBuilder, Query, SimpleExpr, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow, query_as_with};
use tracing::{instrument, warn};
use uuid::Uuid;

use crate::{error::ComhairleError, translation_service::TranslationService};

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

/// Machine-translate `statement_text` into every `supported_language` other than
/// `source_locale`, upserting a row per target. Each generated row is flagged
/// `ai_generated` and `requires_validation`.
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
    let mut out = vec![];

    for locale in supported_languages
        .iter()
        .filter(|locale| locale.as_str() != source_locale)
    {
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

    use crate::{
        models::model_test_helpers::setup_default_app_and_session,
        models::polis_statement_aux::{self, UpsertFromPolis},
        test_helpers::{extract, polis_tool_config},
        translation_service::{MockTranslationService, TranslationService},
        wiki_poll_service::ModerationStatus,
    };

    use super::*;

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
    async fn re_generating_upserts_rather_than_duplicating(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let aux_id = seed_aux(&pool).await?;

        let translator: Arc<dyn TranslationService> = Arc::new(MockTranslationService::base());
        let supported = vec!["en".to_string(), "es".to_string()];

        generate_for_statement(&pool, &translator, aux_id, "hello", "en", &supported).await?;
        generate_for_statement(&pool, &translator, aux_id, "hello again", "en", &supported).await?;

        let stored = list_by_statement_aux_id(&pool, &aux_id).await?;
        assert_eq!(
            stored.len(),
            1,
            "should upsert on (aux_id, locale), not duplicate"
        );

        Ok(())
    }
}
