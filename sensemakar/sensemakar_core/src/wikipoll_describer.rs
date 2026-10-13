use askama::Template;
use bon::Builder;
use rig_agent::completion::StructuredOutputError;
use rig_agent::extractor::ExtractorBuilder;
use rig_core::DynModel;
use rig_core::operation::Completion;
use sensemakar_types::{WikiPollData, WikiPollReportResult};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WikiPollGroupDescriberError {
    #[error("Template Error {0}")]
    TemplateError(#[from] askama::Error),
    #[error("extractor error {0}")]
    RigExtractorError(#[from] Box<StructuredOutputError>),
    #[error("json error error {0}")]
    StatementFormatError(#[from] serde_json::Error),
}

#[derive(Builder, Template)]
#[template(path = "wikipoll/describer.md")]
pub struct WikiPollGroupDescriber {
    pub context: Option<String>,
    pub additional_instructions: Option<String>,
}

impl WikiPollGroupDescriber {
    pub async fn run_with_model(
        &self,
        wiki_poll_report: &WikiPollData,
        model: impl Into<DynModel<Completion>>,
    ) -> Result<WikiPollReportResult, WikiPollGroupDescriberError> {
        let template = self.render()?;
        let extractor = ExtractorBuilder::<WikiPollReportResult>::new(model)
            .append_preamble(&template)
            .retries(2)
            .build();

        let statement_json: String = serde_json::to_string(wiki_poll_report)?;

        let result = extractor.extract(statement_json).await.map_err(Box::new)?.output;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use sensemakar_types::{WikiPollData, WikiPollStatement};

    use crate::{test_helpers::test_model, wikipoll_describer::WikiPollGroupDescriber};

    #[tokio::test]
    async fn test_with_data() {
        let model = test_model();
        let data = fs::read_to_string("data/wiki_poll_results.json").unwrap();
        let data: Vec<WikiPollStatement> = serde_json::from_str(&data).unwrap();
        let data: Vec<_> = data.into_iter().take(100).collect();
        let data = WikiPollData {
            title: "What is the future of south staffishire".into(),
            statements: data,
        };
        let describer = WikiPollGroupDescriber::builder()
            .context("Thesse are statements from a polis conv with the title 'What should be the futrue of south staffisher".into())
            .build();
        for _ in 0..10 {
            let result = describer.run_with_model(&data, model.clone()).await;
            assert!(result.is_ok(), "The extractor should run ");
            result.unwrap();
        }
    }
}
