//! Thinking space tool configuration.

use comhairle_macros::TranslatableJson;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::error::ModelError;
use crate::models::tools::ToolConfigSanitize;
use crate::models::translations::{BuildTextTranslation, TextContentId, TextFormat};

// ======================
//
// LEGACY TYPES
//
// Keep for migration binary
//
// ======================

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, PartialEq)]
pub struct LegacyThinkingSpaceQuestion {
    pub id: Uuid,
    pub text: String,
    /// Admin-authored description of *why* this question is being asked — fed to
    /// the AI as `question_intent` so it can generate sharper follow-ups. Never
    /// shown to participants.
    pub intent: String,
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, PartialEq)]
pub struct LegacyThinkingSpaceToolConfig {
    pub topic: String,
    pub root_questions: Vec<LegacyThinkingSpaceQuestion>,
    pub follow_up_rounds_count: u8,
}

// ======================
//
// END LEGACY TYPES
//
// ======================

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, PartialEq, TranslatableJson)]
pub struct ThinkingSpaceQuestion {
    pub id: Uuid,
    #[translatable]
    pub text: TextContentId,
    /// Admin-authored description of *why* this question is being asked — fed to
    /// the AI as `question_intent` so it can generate sharper follow-ups. Never
    /// shown to participants.
    #[translatable]
    pub intent: TextContentId,
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, PartialEq, TranslatableJson)]
pub struct ThinkingSpaceToolConfig {
    #[translatable]
    pub topic: TextContentId,
    #[translatable]
    pub root_questions: Vec<ThinkingSpaceQuestion>,
    pub follow_up_rounds_count: u8,
}

impl ToolConfigSanitize for ThinkingSpaceToolConfig {
    fn sanitize(&self) -> Self {
        self.clone()
    }
}

// =================
// Setup structs
// =================

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, PartialEq)]
pub struct ThinkingSpaceSetupQuestion {
    pub text: String,
    pub intent: String,
}

impl ThinkingSpaceSetupQuestion {
    async fn build_with_translations(
        self,
        db: &PgPool,
        locale: &str,
    ) -> Result<ThinkingSpaceQuestion, ModelError> {
        Ok(ThinkingSpaceQuestion {
            id: Uuid::new_v4(),
            text: self
                .text
                .build_text_translation(db, locale, TextFormat::Plain)
                .await?,
            intent: self
                .intent
                .build_text_translation(db, locale, TextFormat::Plain)
                .await?,
        })
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct ThinkingSpaceToolSetup {
    pub topic: String,
    pub root_questions: Vec<ThinkingSpaceSetupQuestion>,
    pub follow_up_rounds_count: u8,
}

impl ThinkingSpaceToolSetup {
    pub async fn build_with_translations(
        self,
        db: &PgPool,
        locale: &str,
    ) -> Result<ThinkingSpaceToolConfig, ModelError> {
        Ok(ThinkingSpaceToolConfig {
            topic: self
                .topic
                .build_text_translation(db, locale, TextFormat::Plain)
                .await?,
            root_questions: {
                let mut built = Vec::with_capacity(self.root_questions.len());
                for question in self.root_questions {
                    built.push(question.build_with_translations(db, locale).await?);
                }
                built
            },
            follow_up_rounds_count: self.follow_up_rounds_count,
        })
    }
}

#[derive(PartialEq, Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct ThinkingSpaceReport;
