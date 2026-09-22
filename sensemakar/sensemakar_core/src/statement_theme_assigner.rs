use std::collections::HashSet;

use askama::Template;
use bon::Builder;
use rig::{
    completion::CompletionModel,
    extractor::{ExtractionError, ExtractorBuilder},
};
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
    #[error("model error (0)")]
    RigError(#[from] rig::ProviderResponseError),
    #[error("Template Error ()")]
    TemplateError(#[from] askama::Error),
    #[error("extractor error (0)")]
    RigExtractorError(#[from] ExtractionError),
    #[error("Statement serialization error(0)")]
    StatementSeializationError(#[from] StatementError),
    #[error("Theme serialization error(0)")]
    ThemeSeializationError(#[from] ThemeError),
}

impl StatementThemeAssigner {
    pub async fn run_with_model<M, S>(
        &self,
        statement_list: S,
        model: M,
    ) -> Result<ThemeAssignmentResult, StatementThemeAssignerError>
    where
        M: CompletionModel + 'static,
        S: Into<StatementList>,
    {
        let mut all_results: Vec<ThemeAssignment> = vec![];

        let template = self.render()?;
        println!("Template is {template}");
        let extractor = ExtractorBuilder::<ThemeAssignmentResult>::new(model)
            .preamble(&template)
            .build();

        let list: StatementList = statement_list.into();
        let mut pending_ids: HashSet<String> =
            list.statements.iter().map(|s| s.id.clone()).collect();

        while pending_ids.len() > 0 {
            let filtered_statement_list: Vec<Statement> = list
                .statements
                .iter()
                .filter(|statement| pending_ids.contains(&statement.id))
                .map(|s| s.clone())
                .collect();

            let statement_text = StatementList {
                statements: filtered_statement_list,
            }
            .to_model_embed()?;

            let result = extractor.extract(statement_text).await?;

            for entry in &result.statement_assignments {
                pending_ids.remove(&entry.statement_id);
            }
            all_results.extend(result.statement_assignments);
        }

        Ok(ThemeAssignmentResult {
            statement_assignments: all_results,
        })
    }

    // pub async fn run_in_batches_with_model<M, S>(
    //     &self,
    //     statement_list: S,
    //     model: M,
    //     batch_size: usize,
    // ) -> Result<Vec<Statement>, StatementTypeClassifierError>
    // where
    //     M: CompletionModel + 'static,
    //     S: Into<StatementList>,
    // {
    //     let template = self.render()?;
    //
    //     let extractor = ExtractorBuilder::<StatementList>::new(model)
    //         .preamble(&template)
    //         .build();
    //
    //     let list: StatementList = statement_list.into();
    //     let mut results = vec![];
    //
    //     for chunk in list.statements.chunks(batch_size) {
    //         let statement_text = StatementList::from(chunk.to_vec()).to_model_embed()?;
    //         println!("statement text {statement_text}");
    //
    //         let result = extractor.extract(statement_text).await?;
    //         results.extend(result.statements);
    //     }
    //     Ok(results)
    // }
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
        println!("{statements_with_themes:#?}");
        assert!(statements_with_themes.is_ok(), "Should run ok");

        let statements_with_themes = statements_with_themes.unwrap();
        assert_eq!(
            no_statements,
            statements_with_themes.statement_assignments.len(),
            "Should get back as many as we sent"
        );
        println!("statements {statements_with_themes:#?}");
    }
}
