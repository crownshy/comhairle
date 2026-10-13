use askama::Template;
use bon::Builder;
use rig_agent::completion::StructuredOutputError;
use rig_agent::extractor::ExtractorBuilder;
use rig_core::DynModel;
use rig_core::operation::Completion;
use sensemakar_types::Statement;
use sensemakar_types::{StatementError, StatementList};
use thiserror::Error;

#[derive(Builder, Template)]
#[template(path = "statements/classify_statement_type.md")]
pub struct StatementTypeClassifier {
    context: Option<String>,
    additional_instructions: Option<String>,
}

#[derive(Error, Debug)]
pub enum StatementTypeClassifierError {
    #[error("Template Error {0}")]
    TemplateError(#[from] askama::Error),
    #[error("extractor error {0}")]
    RigExtractorError(#[from] StructuredOutputError),
    #[error("Statement serialization error {0}")]
    StatementSeializationError(#[from] StatementError),
}

impl StatementTypeClassifier {
    pub async fn run_with_model<S>(
        &self,
        statement_list: S,
        model: impl Into<DynModel<Completion>>,
    ) -> Result<Vec<Statement>, StatementTypeClassifierError>
    where
        S: Into<StatementList>,
    {
        let template = self.render()?;
        let extractor = ExtractorBuilder::<StatementList>::new(model)
            .append_preamble(&template)
            .retries(2)
            .build();

        let list: StatementList = statement_list.into();
        let statement_text = list.to_model_embed()?;

        let result = extractor.extract(statement_text).await?.output;

        Ok(result.statements)
    }

    pub async fn run_in_batches_with_model<S>(
        &self,
        statement_list: S,
        model: impl Into<DynModel<Completion>>,
        batch_size: usize,
    ) -> Result<Vec<Statement>, StatementTypeClassifierError>
    where
        S: Into<StatementList>,
    {
        let template = self.render()?;

        let extractor = ExtractorBuilder::<StatementList>::new(model)
            .append_preamble(&template)
            .retries(2)
            .build();

        let list: StatementList = statement_list.into();
        let mut results = vec![];

        for chunk in list.statements.chunks(batch_size) {
            let statement_text = StatementList::from(chunk.to_vec()).to_model_embed()?;

            let result = extractor.extract(statement_text).await?.output;
            results.extend(result.statements);
        }
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        statement_classifier::StatementTypeClassifier,
        test_helpers::{test_model, test_statements},
    };

    #[tokio::test]
    async fn test_statement_classification() {
        let model = test_model();
        let statements = test_statements();
        let no_statements = statements.len();
        let classifier = StatementTypeClassifier::builder()
            .context("These statements are part of a survey".into())
            .build();
        let statements_with_classification = classifier.run_with_model(statements, model).await;
        assert!(statements_with_classification.is_ok(), "Should run ok");

        let statements_with_classification = statements_with_classification.unwrap();
        assert_eq!(
            no_statements,
            statements_with_classification.len(),
            "Should get back as many as we sent"
        );
    }

    #[tokio::test]
    async fn test_statement_classification_batch() {
        let model = test_model();
        let statements = test_statements();
        let no_statements = statements.len();
        let classifier = StatementTypeClassifier::builder()
            .context("These statements are part of a survey".into())
            .build();
        let statements_with_classification = classifier
            .run_in_batches_with_model(statements, model, 2)
            .await;
        assert!(statements_with_classification.is_ok(), "Should run ok");

        let statements_with_classification = statements_with_classification.unwrap();
        assert_eq!(
            no_statements,
            statements_with_classification.len(),
            "Should get back as many as we sent"
        );
    }
}
