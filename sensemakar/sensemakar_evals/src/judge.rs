use askama::Template;
use rig_agent::completion::StructuredOutputError;
use rig_agent::extractor::ExtractorBuilder;
use rig_core::DynModel;
use rig_core::operation::Completion;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::rubric::RubricCase;

#[derive(Error, Debug)]
pub enum JudgeError {
    #[error("template error {0}")]
    TemplateError(#[from] askama::Error),
    #[error("judge extractor error {0}")]
    RigExtractorError(#[from] Box<StructuredOutputError>),
    #[error("failed to serialize value for judging {0}")]
    SerializationError(#[from] serde_json::Error),
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct PointVerdict {
    /// The criterion text being judged, copied verbatim from the rubric's should/should_not list
    pub criterion: String,
    /// Whether the criterion was satisfied. For a should_not criterion, true means the
    /// described bad thing is correctly absent from the output.
    pub passed: bool,
    /// A short rationale grounded in the actual output
    pub rationale: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Default)]
#[schemars(description = "One verdict per rubric criterion that was judged")]
pub struct JudgeVerdict {
    pub points: Vec<PointVerdict>,
}

impl JudgeVerdict {
    /// Fraction of criteria that passed, in [0, 1]. A case with no criteria
    /// at all trivially scores 1.0 - there's nothing to fail.
    pub fn score(&self) -> f64 {
        if self.points.is_empty() {
            return 1.0;
        }
        let passed = self.points.iter().filter(|point| point.passed).count();
        passed as f64 / self.points.len() as f64
    }
}

#[derive(Template)]
#[template(path = "judge/llm_as_judge.md")]
struct JudgePrompt<'a> {
    task_description: &'a str,
    input_rendered: &'a str,
    output_rendered: &'a str,
    should: &'a [String],
    should_not: &'a [String],
}

/// Grades a single task's output against a rubric case's `should`/`should_not`
/// criteria, using a model as the judge - the LLM-as-judge pattern used
/// throughout the eval suite.
pub struct LlmAsJudge {
    model: DynModel<Completion>,
}

impl LlmAsJudge {
    pub fn new(model: impl Into<DynModel<Completion>>) -> Self {
        Self { model: model.into() }
    }

    pub async fn evaluate(
        &self,
        task_description: &str,
        input_rendered: &str,
        output_rendered: &str,
        case: &RubricCase,
    ) -> Result<JudgeVerdict, JudgeError> {
        let template = JudgePrompt {
            task_description,
            input_rendered,
            output_rendered,
            should: &case.should,
            should_not: &case.should_not,
        }
        .render()?;

        let extractor = ExtractorBuilder::<JudgeVerdict>::new(self.model.clone())
            .append_preamble(&template)
            .retries(2)
            .build();

        let verdict = extractor
            .extract("Evaluate the response above against every criterion listed and return a verdict for each.".to_string())
            .await
            .map_err(Box::new)?
            .output;

        Ok(verdict)
    }
}
