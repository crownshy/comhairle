use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use serde::de::DeserializeOwned;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RubricError {
    #[error("failed to read blueprint {path}: {source}")]
    Read { path: PathBuf, source: std::io::Error },
    #[error("failed to parse blueprint {path}: {source}")]
    Parse { path: PathBuf, source: serde_norway::Error },
    #[error("no case with id '{0}' in this blueprint")]
    UnknownCase(String),
    #[error("failed to read fixture {path}: {source}")]
    ReadFixture { path: PathBuf, source: std::io::Error },
    #[error("failed to parse fixture {path}: {source}")]
    ParseFixture { path: PathBuf, source: serde_json::Error },
}

/// A weval-style rubric blueprint for one sensemakar task: a description of
/// what the task does, plus a handful of sketched cases. Each case points at
/// a local fixture file rather than embedding a prompt string, since our
/// "prompt" is a typed Rust call (`Vec<Statement>`, `WikiPollData`, ...) and
/// not a chat message.
#[derive(Debug, Deserialize)]
pub struct RubricBlueprint {
    pub task: String,
    pub description: String,
    pub cases: Vec<RubricCase>,
}

#[derive(Debug, Deserialize)]
pub struct RubricCase {
    pub id: String,
    fixture: String,
    #[serde(default)]
    pub should: Vec<String>,
    #[serde(default)]
    pub should_not: Vec<String>,
    #[serde(default = "default_weight")]
    pub weight: f64,
    #[serde(skip)]
    fixture_path: PathBuf,
}

fn default_weight() -> f64 {
    1.0
}

impl RubricBlueprint {
    /// Loads and parses a blueprint, resolving every case's `fixture` path
    /// relative to the blueprint file's own directory.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, RubricError> {
        let path = path.as_ref();
        let text = fs::read_to_string(path).map_err(|source| RubricError::Read {
            path: path.to_path_buf(),
            source,
        })?;
        let mut blueprint: RubricBlueprint =
            serde_norway::from_str(&text).map_err(|source| RubricError::Parse {
                path: path.to_path_buf(),
                source,
            })?;

        let base_dir = path.parent().unwrap_or_else(|| Path::new("."));
        for case in &mut blueprint.cases {
            case.fixture_path = base_dir.join(&case.fixture);
        }

        Ok(blueprint)
    }

    pub fn case(&self, id: &str) -> Result<&RubricCase, RubricError> {
        self.cases
            .iter()
            .find(|case| case.id == id)
            .ok_or_else(|| RubricError::UnknownCase(id.to_string()))
    }
}

impl RubricCase {
    /// Loads this case's fixture file, deserializing it as the type the
    /// caller's task expects (`Vec<Statement>`, `WikiPollData`, ...).
    pub fn load_fixture<T: DeserializeOwned>(&self) -> Result<T, RubricError> {
        let text =
            fs::read_to_string(&self.fixture_path).map_err(|source| RubricError::ReadFixture {
                path: self.fixture_path.clone(),
                source,
            })?;
        serde_json::from_str(&text).map_err(|source| RubricError::ParseFixture {
            path: self.fixture_path.clone(),
            source,
        })
    }
}
