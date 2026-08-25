use std::error::Error;

use comhairle::db::setup_db;
use comhairle::models::translations::{
    TextContent, TextContentIden, TextFormat, TextTranslation, TextTranslationIden,
};
use comhairle::tools::prioritization::PrioritizationToolConfig;

use sea_query::{PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::Deserialize;
use serde_json::json;
use sqlx::{Acquire, prelude::FromRow};
use uuid::Uuid;

#[derive(Deserialize, Debug, FromRow)]
struct MatchingStep {
    id: Uuid,
    tool_config: Option<serde_json::Value>,
    preview_tool_config: serde_json::Value,
    primary_locale: String,
}

#[derive(Deserialize, Debug)]
pub struct LegacyPrioritizationToolConfig {
    pub questions: Vec<LegacyQuestion>,
    pub section_questions: Vec<LegacyQuestion>,
    pub randomize_order: bool,
    pub alignment_question_id: Option<Uuid>,
    pub required_reviews: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct LegacyQuestion {
    pub id: Uuid,
    pub text: String,
    pub r#type: LegacyQuestionType,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum LegacyQuestionType {
    Text,
    LikertScale {
        categories: Vec<LegacyCategory>,
    },
    Continuous {
        sub_steps: i32,
        min_value: f64,
        max_value: f64,
        min_label: String,
        max_label: String,
    },
}

#[derive(Deserialize, Debug)]
pub struct LegacyCategory {
    value: f64,
    label: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();

    let dry_run = std::env::args().any(|arg| arg == "--dry-run");
    let verbose = std::env::args().any(|arg| arg == "--verbose");
    let db_url = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL");

    let db = setup_db(&db_url).await?;

    let prioritization_steps = sqlx::query_as!(
        MatchingStep,
        r#"
        SELECT ws.id, ws.tool_config, ws.preview_tool_config, c.primary_locale
        FROM workflow_step ws
        JOIN workflow w ON w.id = ws.workflow_id
        JOIN conversation c ON c.id = w.conversation_id
        WHERE (ws.tool_config ->> 'type' = 'prioritization')
            OR (ws.preview_tool_config ->> 'type' = 'prioritization')
    "#
    )
    .fetch_all(&db)
    .await?;

    println!("Found {} prioritization rows", prioritization_steps.len());

    for step in prioritization_steps {
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

        if let Some(Err(error)) = new_tool_config
            .clone()
            .map(serde_json::from_value::<PrioritizationToolConfig>)
        {
            println!(
                "Unable to deserialize transformed tool_config into PrioritizationToolConfig. Error: {error:#?}"
            );
            continue;
        }

        if let Err(error) =
            serde_json::from_value::<PrioritizationToolConfig>(new_preview_tool_config.clone())
        {
            println!(
                "Unable to deserialize transformed preview_tool_config into PrioritizationToolConfig. Error: {error:#?}"
            );
            continue;
        }

        println!();
        println!(
            "Step {}: tool_config migrated={} preview_tool_config_migrated=true",
            step.id,
            new_tool_config.is_some(),
        );

        if verbose {
            println!("Transformed tool_config: {new_tool_config:#?}");
            println!("Transformed preview_tool_config: {new_preview_tool_config:#?}");
        }

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

fn already_migrated(raw: &serde_json::Value) -> bool {
    let tc = serde_json::from_value::<PrioritizationToolConfig>(raw.to_owned());

    tc.is_ok()
}

async fn migrate_config(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    tool_config: serde_json::Value,
    locale: &str,
) -> Result<serde_json::Value, Box<dyn Error>> {
    let legacy_config: LegacyPrioritizationToolConfig = serde_json::from_value(tool_config)?;

    let mut trans_questions = Vec::with_capacity(legacy_config.questions.len());
    for question in legacy_config.questions {
        trans_questions.push(migrate_question(tx, question, locale).await?);
    }

    let mut trans_section_questions = Vec::with_capacity(legacy_config.section_questions.len());
    for section_question in legacy_config.section_questions {
        trans_section_questions.push(migrate_question(tx, section_question, locale).await?);
    }

    Ok(json!({
        "type": "prioritization",
        "questions": trans_questions,
        "section_questions": trans_section_questions,
        "randomize_order": legacy_config.randomize_order,
        "alignment_question_id": legacy_config.alignment_question_id,
        "required_reviews": legacy_config.required_reviews
    }))
}

async fn migrate_question(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    question: LegacyQuestion,
    locale: &str,
) -> Result<serde_json::Value, Box<dyn Error>> {
    let text_id = new_translation_tx(tx, locale, &question.text, TextFormat::Plain)
        .await?
        .id;

    let type_with_trans = match question.r#type {
        LegacyQuestionType::Continuous {
            sub_steps,
            min_value,
            max_value,
            min_label,
            max_label,
        } => {
            let min_id = new_translation_tx(tx, locale, &min_label, TextFormat::Plain)
                .await?
                .id;
            let max_id = new_translation_tx(tx, locale, &max_label, TextFormat::Plain)
                .await?
                .id;

            json!({
                "continuous": {
                    "sub_steps": sub_steps,
                    "min_value": min_value,
                    "max_value": max_value,
                    "min_label": min_id,
                    "max_label": max_id
                }
            })
        }
        LegacyQuestionType::LikertScale { categories } => {
            let mut trans_cats = Vec::with_capacity(categories.len());
            for cat in categories {
                let label_id = new_translation_tx(tx, locale, &cat.label, TextFormat::Plain)
                    .await?
                    .id;
                trans_cats.push(json!({ "value": cat.value, "label": label_id }))
            }
            json!({ "likert_scale": { "categories": trans_cats } })
        }
        LegacyQuestionType::Text => json!("text"),
    };

    Ok(json!({
        "id": question.id,
        "text": text_id,
        "type": type_with_trans
    }))
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
