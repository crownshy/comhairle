use askama::Template;
use bon::Builder;
use sensemakar_types::{StatementError, StatementList, Theme, ThemeList};

use rig_agent::completion::StructuredOutputError;
use rig_agent::extractor::ExtractorBuilder;
use rig_core::DynModel;
use rig_core::operation::Completion;
use thiserror::Error;

#[derive(Builder, Template)]
#[template(path = "themes/extract_themes.md")]
pub struct ThemeExtractor {
    existing_themes: Option<Vec<Theme>>,
    // Not yet read by the template; plumbed through from CLI/API for a future constraint.
    #[allow(dead_code)]
    allow_additional_themes: bool,
    context: Option<String>,
    additional_instructions: Option<String>,
    #[allow(dead_code)]
    max_themes: Option<u32>,
    #[allow(dead_code)]
    min_themes: Option<u32>,
}

#[derive(Error, Debug)]
pub enum ThemeExtractorError {
    #[error("Template Error {0}")]
    TemplateError(#[from] askama::Error),
    #[error("extractor error {0}")]
    RigExtractorError(#[from] Box<StructuredOutputError>),
    #[error("Statement serialization error {0}")]
    StatementSeializationError(#[from] StatementError),
}

impl ThemeExtractor {
    pub async fn run_with_model<S>(
        &self,
        statement_list: S,
        model: impl Into<DynModel<Completion>>,
    ) -> Result<Vec<Theme>, ThemeExtractorError>
    where
        S: Into<StatementList>,
    {
        let template = self.render()?;
        let extractor = ExtractorBuilder::<ThemeList>::new(model)
            .append_preamble(&template)
            .retries(2)
            .build();

        let list: StatementList = statement_list.into();
        let statement_text = list.to_model_embed()?;

        let result = extractor.extract(statement_text).await.map_err(Box::new)?.output;

        Ok(result.themes)
    }

    pub async fn run_in_batches_with_model<S>(
        &mut self,
        statement_list: S,
        model: DynModel<Completion>,
        batch_size: usize,
    ) -> Result<Vec<Theme>, ThemeExtractorError>
    where
        S: Into<StatementList>,
    {
        let list: StatementList = statement_list.into();

        for group in list.statements.chunks(batch_size) {
            let new_themes = self.run_with_model(group.to_vec(), model.clone()).await?;
            if !new_themes.is_empty() {
                if let Some(existing_themes) = &mut self.existing_themes {
                    existing_themes.extend(new_themes);
                } else {
                    self.existing_themes = Some(new_themes);
                }
            }
        }

        Ok(self.existing_themes.clone().unwrap_or(vec![]))
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        test_helpers::{test_model, test_statements},
        theme_extraction::ThemeExtractor,
    };

    #[tokio::test]
    async fn test_theme_assignment() {
        let extractor = ThemeExtractor::builder()
            .context("These statements are from a survey form".into())
            .allow_additional_themes(true)
            .build();

        let statement_list = test_statements();
        let model = test_model();

        let themes = extractor.run_with_model(statement_list, model).await;
        let themes = themes.unwrap();

        assert!(!themes.is_empty(), "expected at least one extracted theme");
    }

    #[tokio::test]
    async fn test_theme_assignment_batches() {
        let mut extractor = ThemeExtractor::builder()
            .context("These statements are from a survey form".into())
            .allow_additional_themes(true)
            .build();

        let statement_list = test_statements();
        let model = test_model();

        let themes = extractor
            .run_in_batches_with_model(statement_list, model, 2)
            .await;
        let themes = themes.unwrap();

        assert!(!themes.is_empty(), "expected at least one extracted theme");
    }
}
