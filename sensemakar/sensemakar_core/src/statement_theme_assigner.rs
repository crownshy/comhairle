use std::collections::HashSet;

use askama::Template;
use bon::Builder;
use rig_agent::completion::StructuredOutputError;
use rig_agent::extractor::ExtractorBuilder;
use rig_core::DynModel;
use rig_core::operation::Completion;
use sensemakar_types::{Statement, ThemeAssignment, ThemeAssignmentResult, ThemeError};
use sensemakar_types::{StatementError, StatementList, ThemeList};
use thiserror::Error;

#[derive(Builder, Template)]
#[template(path = "statements/assign_themes.md")]
pub struct StatementThemeAssigner {
    context: Option<String>,
    additional_instructions: Option<String>,
    themes: ThemeList,
}

#[derive(Error, Debug)]
pub enum StatementThemeAssignerError {
    #[error("Template Error {0}")]
    TemplateError(#[from] askama::Error),
    #[error("extractor error {0}")]
    RigExtractorError(#[from] Box<StructuredOutputError>),
    #[error("Statement serialization error {0}")]
    StatementSeializationError(#[from] StatementError),
    #[error("Theme serialization error {0}")]
    ThemeSeializationError(#[from] ThemeError),
}

impl StatementThemeAssigner {
    pub async fn run_with_model<S>(
        &self,
        statement_list: S,
        model: impl Into<DynModel<Completion>>,
    ) -> Result<ThemeAssignmentResult, StatementThemeAssignerError>
    where
        S: Into<StatementList>,
    {
        let mut all_results: Vec<ThemeAssignment> = vec![];

        let template = self.render()?;
        let extractor = ExtractorBuilder::<ThemeAssignmentResult>::new(model)
            .append_preamble(&template)
            .retries(2)
            .build();

        let list: StatementList = statement_list.into();
        let mut pending_ids: HashSet<String> =
            list.statements.iter().map(|s| s.id.clone()).collect();

        while !pending_ids.is_empty() {
            let filtered_statement_list: Vec<Statement> = list
                .statements
                .iter()
                .filter(|statement| pending_ids.contains(&statement.id))
                .cloned()
                .collect();

            let statement_text = StatementList {
                statements: filtered_statement_list,
            }
            .to_model_embed()?;

            let result = extractor.extract(statement_text).await.map_err(Box::new)?.output;

            for entry in &result.statement_assignments {
                pending_ids.remove(&entry.statement_id);
            }
            all_results.extend(result.statement_assignments);
        }

        Ok(ThemeAssignmentResult {
            statement_assignments: all_results,
        })
    }
}

#[cfg(test)]
mod tests {
    use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};

    use crate::{
        statement_theme_assigner::StatementThemeAssigner,
        test_helpers::{test_model, test_statements, test_themes},
    };

    #[tokio::test]
    async fn test_statement_assignment() {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::new("rig=debug")) // Focuses strictly on Rig internals
            .with_test_writer()
            .with_span_events(FmtSpan::CLOSE) // CRUCIAL: Emits logs when a span completes, including its fields
            .try_init();

        let model = test_model();
        let statements = test_statements();
        let themes = test_themes();

        let no_statements = statements.len();

        let classifier = StatementThemeAssigner::builder()
            .context("These statements are part of a survey about issues in a council area".into())
            .themes(themes.into())
            .build();

        let statements_with_themes = classifier.run_with_model(statements, model).await;
        assert!(statements_with_themes.is_ok(), "Should run ok");

        let statements_with_themes = statements_with_themes.unwrap();
        assert_eq!(
            no_statements,
            statements_with_themes.statement_assignments.len(),
            "Should get back as many as we sent"
        );
    }
}
