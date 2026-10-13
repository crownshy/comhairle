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
    allow_additional_themes: bool,
    context: Option<String>,
    additional_instructions: Option<String>,
    max_themes: Option<u32>,
    min_themes: Option<u32>,
}

#[derive(Error, Debug)]
pub enum ThemeExtractorError {
    #[error("Template Error {0}")]
    TemplateError(#[from] askama::Error),
    #[error("extractor error {0}")]
    RigExtractorError(#[from] StructuredOutputError),
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
        println!("statement text {statement_text}");

        let result = extractor.extract(statement_text).await?.output;

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
            match !new_themes.is_empty() {
                true => {
                    if let Some(existing_themes) = &mut self.existing_themes {
                        existing_themes.extend(new_themes);
                    } else {
                        self.existing_themes = Some(new_themes);
                    }
                }
                false => (),
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
        println!("{:#?}", themes);
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
        println!("{:#?}", themes);
        let themes = themes.unwrap();

        assert!(!themes.is_empty(), "expected at least one extracted theme");
    }
}
