//! Binary to migrate legacy thinking_space workflow steps, which contain simple
//! text fields in the tool_config / preview_tool_config, to tool_config's which
//! support translations.
//!
//! # Usage
//!
//! ```bash
//! cargo run --bin migrate_ts_trans
//! ```
//!
//! # Args
//!
//! * `--dry-run` - run programme to test impact without executing migration transaction

use std::error::Error;

use comhairle::db::setup_db;
use comhairle::models::translations::{
    TextContent, TextContentIden, TextFormat, TextTranslation, TextTranslationIden,
};

use sea_query::{PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::Deserialize;
use serde_json::json;
use sqlx::Acquire;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct LegacyThinkingSpaceQuestion {
    pub id: Uuid,
    pub text: String,
    pub intent: String,
}

#[derive(Deserialize, Debug)]
pub struct LegacyThinkingSpaceToolConfig {
    pub topic: String,
    pub root_questions: Vec<LegacyThinkingSpaceQuestion>,
    pub follow_up_rounds_count: u8,
}

#[derive(Deserialize, Debug, FromRow)]
struct MatchingTsStep {
    id: Uuid,
    tool_config: Option<serde_json::Value>,
    preview_tool_config: serde_json::Value,
    primary_locale: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();

    let dry_run = std::env::args().any(|arg| arg == "--dry-run");
    let db_url = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL");

    let db = setup_db(&db_url).await?;

    let ts_steps = sqlx::query_as!(
        MatchingTsStep,
        r#"
        SELECT ws.id, ws.tool_config, ws.preview_tool_config, c.primary_locale
        FROM workflow_step ws
        JOIN workflow w ON w.id = ws.workflow_id
        JOIN conversation c ON c.id = w.conversation_id
        WHERE (ws.tool_config ->> 'type' = 'thinkingspace')
            OR (ws.preview_tool_config ->> 'type' = 'thinkingspace')
    "#
    )
    .fetch_all(&db)
    .await?;

    println!("Found {} thinking_space rows", ts_steps.len());

    for step in ts_steps {
        let locale = step.primary_locale;
        let mut tx = db.begin().await?;

        if already_migrated(&step.preview_tool_config)
            && step.tool_config.as_ref().map_or(true, already_migrated)
        {
            println!();
            println!("Step {} already migrated", step.id);

            continue;
        }

        let new_tool_config = match step.tool_config {
            Some(tc) if !already_migrated(&tc) => Some(migrate_config(&mut tx, tc, &locale).await?),
            other => other,
        };

        let new_preview_tool_config = if !already_migrated(&step.preview_tool_config) {
            migrate_config(&mut tx, step.preview_tool_config, &locale).await?
        } else {
            step.preview_tool_config
        };

        println!();
        println!(
            "Step {}: tool_config migrated={} preview_tool_config_migrated=true",
            step.id,
            new_tool_config.is_some(),
        );

        if !dry_run {
            sqlx::query!(
                r#"
            UPDATE workflow_step
            SET tool_config = $1, preview_tool_config = $2
            WHERE id = $3
            "#,
                new_tool_config,
                new_preview_tool_config,
                step.id
            )
            .execute(&mut *tx)
            .await?;

            tx.commit().await?;
        } else {
            tx.rollback().await?;
        }
    }

    Ok(())
}

async fn migrate_config(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    tool_config: serde_json::Value,
    locale: &str,
) -> Result<serde_json::Value, Box<dyn Error>> {
    let legacy_config: LegacyThinkingSpaceToolConfig =
        serde_json::from_value(tool_config.to_owned())?;

    let topic_id = new_translation_tx(tx, locale, &legacy_config.topic, TextFormat::Plain)
        .await?
        .id;

    let mut root_questions = Vec::with_capacity(legacy_config.root_questions.len());
    for question in legacy_config.root_questions {
        let text_id = new_translation_tx(tx, locale, &question.text, TextFormat::Plain)
            .await?
            .id;
        let intent_id = new_translation_tx(tx, locale, &question.intent, TextFormat::Plain)
            .await?
            .id;

        root_questions.push(json!({
            "id": question.id,
            "text": text_id,
            "intent": intent_id
        }));
    }

    Ok(json!({
        "type": "thinkingspace",
        "topic": topic_id,
        "root_questions": root_questions,
        "follow_up_rounds_count": legacy_config.follow_up_rounds_count
    }))
}

fn already_migrated(tool_config: &serde_json::Value) -> bool {
    tool_config
        .get("topic")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok())
        .is_some()
}

/// Transaction-scoped re-implementation of
/// `comhairle::models::transaltions::new_translation`.
///
/// Exists to batch each insert into one atomic transaction that can be rolled
/// back on error or deliberately by caller in `--dry-run` mode.
async fn new_translation_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    locale: &str,
    content: &str,
    format: TextFormat,
) -> Result<TextContent, Box<dyn Error>> {
    let conn = tx.acquire().await?;

    let tc_columns = vec![TextContentIden::PrimaryLocale, TextContentIden::Format];
    let tc_values = vec![locale.to_owned().into(), format.to_string().into()];

    let (sql, values) = Query::insert()
        .into_table(TextContentIden::Table)
        .columns(tc_columns)
        .values(tc_values)?
        .returning(Query::returning().columns([
            TextContentIden::Id,
            TextContentIden::PrimaryLocale,
            TextContentIden::Format,
            TextContentIden::CreatedAt,
            TextContentIden::UpdatedAt,
        ]))
        .build_sqlx(PostgresQueryBuilder);

    let text_content: TextContent = sqlx::query_as_with(&sql, values)
        .fetch_one(&mut *conn)
        .await?;

    let tt_columns = vec![
        TextTranslationIden::ContentId,
        TextTranslationIden::Locale,
        TextTranslationIden::Content,
        TextTranslationIden::AiGenerated,
        TextTranslationIden::RequiresValidation,
    ];
    let tt_values = vec![
        text_content.id.into(),
        locale.to_owned().into(),
        content.to_owned().into(),
        false.into(),
        false.into(),
    ];

    let (sql, values) = Query::insert()
        .into_table(TextTranslationIden::Table)
        .columns(tt_columns)
        .values(tt_values)?
        .returning(Query::returning().columns([
            TextTranslationIden::Id,
            TextTranslationIden::ContentId,
            TextTranslationIden::Locale,
            TextTranslationIden::Content,
            TextTranslationIden::AiGenerated,
            TextTranslationIden::RequiresValidation,
            TextTranslationIden::CreatedAt,
            TextTranslationIden::UpdatedAt,
        ]))
        .build_sqlx(PostgresQueryBuilder);

    let _text_translation: TextTranslation =
        sqlx::query_as_with(&sql, values).fetch_one(conn).await?;

    Ok(text_content)
}
