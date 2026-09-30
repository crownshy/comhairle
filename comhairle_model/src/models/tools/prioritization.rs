//! Prioritization tool configuration.

use comhairle_macros::TranslatableJson;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::error::ModelError;
use crate::models::tools::ToolConfigSanitize;
use crate::models::translations::{BuildTextTranslation, TextContentId, TextFormat};

#[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq, Clone, TranslatableJson)]
pub struct PrioritizationToolConfig {
    /// Questions asked once about the proposal as a whole.
    #[translatable]
    pub questions: Vec<Question>,
    /// Questions asked about each section individually. The same set is used
    /// for every section; participants answer them once per section.
    #[serde(default)]
    #[translatable]
    pub section_questions: Vec<Question>,
    pub randomize_order: bool,
    pub alignment_question_id: Option<Uuid>,
    /// Minimum proposals a participant must review before they can continue to
    /// the next step. `None` means every proposal must be reviewed, which is the
    /// default; an admin sets a number only to loosen that. Non-positive values
    /// are normalised back to `None` by `ToolConfigSanitize` on save, not on
    /// every read, and the participant UI clamps the value to the proposal
    /// count, so the gate is always satisfiable.
    #[serde(default)]
    pub required_reviews: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq, Clone, TranslatableJson)]
pub struct Question {
    pub id: Uuid,
    #[translatable]
    pub text: TextContentId,
    #[translatable]
    pub r#type: QuestionType,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq, Clone, TranslatableJson)]
#[serde(rename_all = "snake_case")]
pub enum QuestionType {
    Text,
    LikertScale {
        #[translatable]
        categories: Vec<Category>,
    },
    Continuous {
        #[serde(default = "default_sub_steps")]
        sub_steps: i32,
        #[serde(default)]
        min_value: f64,
        #[serde(default = "default_max_value")]
        max_value: f64,
        #[translatable]
        min_label: TextContentId,
        #[translatable]
        max_label: TextContentId,
    },
}

fn default_sub_steps() -> i32 {
    10
}

fn default_max_value() -> f64 {
    10.0
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq, Clone, TranslatableJson)]
pub struct Category {
    value: f64,
    #[translatable]
    label: TextContentId,
}

// =================
// Setup structs
// =================

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub struct SetupQuestion {
    pub text: String,
    pub r#type: SetupQuestionType,
}

impl SetupQuestion {
    pub async fn build_with_translations(
        self,
        db: &PgPool,
        locale: &str,
    ) -> Result<Question, ModelError> {
        Ok(Question {
            id: Uuid::new_v4(),
            text: self
                .text
                .build_text_translation(db, locale, TextFormat::Plain)
                .await?,
            r#type: self.r#type.build_with_translations(db, locale).await?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SetupQuestionType {
    Text,
    LikertScale {
        categories: Vec<SetupCategory>,
    },
    Continuous {
        sub_steps: i32,
        min_value: f64,
        max_value: f64,
        min_label: String,
        max_label: String,
    },
}

impl SetupQuestionType {
    async fn build_with_translations(
        self,
        db: &PgPool,
        locale: &str,
    ) -> Result<QuestionType, ModelError> {
        Ok(match self {
            SetupQuestionType::Text => QuestionType::Text,
            SetupQuestionType::LikertScale { categories } => {
                let mut built = Vec::with_capacity(categories.len());
                for category in categories {
                    built.push(category.build_with_translations(db, locale).await?);
                }
                QuestionType::LikertScale { categories: built }
            }
            SetupQuestionType::Continuous {
                sub_steps,
                min_value,
                max_value,
                min_label,
                max_label,
            } => QuestionType::Continuous {
                sub_steps,
                min_value,
                max_value,
                min_label: min_label
                    .build_text_translation(db, locale, TextFormat::Plain)
                    .await?,
                max_label: max_label
                    .build_text_translation(db, locale, TextFormat::Plain)
                    .await?,
            },
        })
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub struct SetupCategory {
    value: f64,
    label: String,
}

impl SetupCategory {
    async fn build_with_translations(
        self,
        db: &PgPool,
        locale: &str,
    ) -> Result<Category, ModelError> {
        Ok(Category {
            value: self.value,
            label: self
                .label
                .build_text_translation(db, locale, TextFormat::Plain)
                .await?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub struct PrioritizationToolSetup {
    pub questions: Vec<SetupQuestion>,
}

impl ToolConfigSanitize for PrioritizationToolConfig {
    fn sanitize(&self) -> Self {
        Self {
            questions: self.questions.clone(),
            section_questions: self.section_questions.clone(),
            randomize_order: self.randomize_order,
            alignment_question_id: self.alignment_question_id,
            // A stored zero or negative would make the gate meaningless, so treat
            // it as unset (every proposal must be reviewed).
            required_reviews: self.required_reviews.filter(|v| *v >= 1),
        }
    }
}

#[derive(PartialEq, Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub struct PrioritizationReport;
