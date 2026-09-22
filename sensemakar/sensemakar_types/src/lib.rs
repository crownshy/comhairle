use std::fmt::Display;

use jiff::Timestamp;
use language_tags::LanguageTag;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;
pub mod thinking_space;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[schemars(
    description = "A theme that represents a grouping of statements or utterances. For example 'animals' or 'grevances'"
)]
pub struct Theme {
    /// A descriptive name for the theme
    pub name: String,
    /// A short description of what the theme captures
    pub description: String,
    /// A unique id for the theme
    pub id: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[schemars(description = "The themes extracted from the provided statements")]
pub struct ThemeList {
    /// The extracted themes
    pub themes: Vec<Theme>,
}

impl Display for ThemeList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_model_embed().unwrap())
    }
}

impl ThemeList {
    /// Serializes the statement list to JSON for the model
    pub fn to_model_embed(&self) -> Result<String, ThemeError> {
        let results: Result<Vec<String>, serde_json::Error> = self
            .themes
            .iter()
            .map(|s| serde_json::to_string(s))
            .collect();
        let strings = results.map_err(ThemeError::from)?;
        Ok(strings.join("\n"))
    }
}

impl From<Vec<Theme>> for ThemeList {
    fn from(value: Vec<Theme>) -> Self {
        ThemeList { themes: value }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
/// Represents the assignment of a statement to a theme  
pub struct ThemeAssignment {
    // The id of the statement
    pub statement_id: String,
    // The id of the theme
    pub theme_id: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[schemars(
    description = "Pairs of theme and statement ids that represent the assignment of a statement to a theme"
)]
pub struct ThemeAssignmentResult {
    pub statement_assignments: Vec<ThemeAssignment>,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
/// The broad semantic type of statement
pub enum StatementType {
    /// A claim that is being made.
    Claim,
    /// A fact that is being expressed. This should be a concrete fact about the world.
    Fact,
    /// A belief that is being expressed
    Belief,
    /// A question that is being asked
    Question,
    /// An expressed disagreement about a Belief, Claim or fact
    Refutaion,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
/// A statement that is being made by someone.
pub struct Statement {
    pub id: String,
    /// An optional speaker id that can be used to uniquely identify the user
    pub speaker_id: Option<String>,
    /// The ontological type os statement
    pub statement_type: Option<StatementType>,
    /// The text of the statement itself
    pub text: String,
    /// An optional language tag
    #[schemars(with = "String")]
    pub lang: LanguageTag,
}

impl Statement {
    /// Builds a statement with no speaker and no classified statement type.
    pub fn new(id: impl Into<String>, text: impl Into<String>, lang: LanguageTag) -> Self {
        Statement {
            id: id.into(),
            speaker_id: None,
            statement_type: None,
            text: text.into(),
            lang,
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct StatementList {
    pub statements: Vec<Statement>,
}

impl From<Vec<Statement>> for StatementList {
    fn from(value: Vec<Statement>) -> Self {
        StatementList { statements: value }
    }
}

#[derive(Error, Debug)]
pub enum StatementError {
    #[error("Failed to serialize statement {0}")]
    SerializationError(#[from] serde_json::Error),
}

#[derive(Error, Debug)]
pub enum ThemeError {
    #[error("Failed to serialize theme {0}")]
    SerializationError(#[from] serde_json::Error),
}

impl StatementList {
    /// Serializes the statement list to JSON for the model
    pub fn to_model_embed(&self) -> Result<String, StatementError> {
        let results: Result<Vec<String>, serde_json::Error> = self
            .statements
            .iter()
            .map(|s| serde_json::to_string(s))
            .collect();
        let strings = results.map_err(StatementError::from)?; // or StatementError::Serde(e), etc.
        Ok(strings.join("\n"))
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Transcript {
    pub id: String,
    pub parts: Vec<Utterance>,
    pub started_at: Timestamp,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Utterance {
    speaker_id: String,
    text: String,
    started_offset: u32,
    ended_offset: u32,
}
