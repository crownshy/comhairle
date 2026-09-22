use askama::Template;
use bon::Builder;
use rig::{
    completion::CompletionModel,
    extractor::{ExtractionError, ExtractorBuilder},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct WikiPollStatementWithGroupVotes {
    pub statement: String,
    pub total_votes: Votes,
    pub group_votes: HashMap<String, Votes>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct Votes {
    agree: usize,
    disagree: usize,
    pass: usize,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
/// A description of a WikiPoll Group
pub struct WikiPollGroupDescription {
    /// An appropriate human readable name for the group
    name: String,
    /// An longer description of the group and how it
    /// differs from others
    description: String,
    /// What makes this cluster unique
    unique_statement: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct WikiPollReportResult {
    /// A description of each group with the keys being the group
    group_descriptions: HashMap<String, WikiPollGroupDescription>,
    /// A description of the censensus statements
    consensus_description: String,
    /// Disagreement description
    disagrement_description: String,
}

#[derive(Error, Debug)]
pub enum WikiPollGroupDescriberError {
    #[error("model error (0)")]
    RigError(#[from] rig::ProviderResponseError),
    #[error("Template Error ()")]
    TemplateError(#[from] askama::Error),
    #[error("extractor error (0)")]
    RigExtractorError(#[from] ExtractionError),
    #[error("json error error (0)")]
    StatementFormatError(#[from] serde_json::Error),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WikiPollData {
    pub title: String,
    pub statements: Vec<WikiPollStatement>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WikiPollStatement {
    pub statement: String,
    pub total_votes: WikiPollVoteSummary,
    pub group_votes: HashMap<String, WikiPollVoteSummary>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WikiPollVoteSummary {
    pub agree: u32,
    pub disagree: u32,
    pub pass: u32,
}

#[derive(Builder, Template)]
#[template(path = "wikipoll/describer.md")]
pub struct WikiPollGroupDescriber {
    context: Option<String>,
    additional_instructions: Option<String>,
}

impl WikiPollGroupDescriber {
    pub async fn run_with_model<M>(
        &self,
        wiki_poll_report: &WikiPollData,
        model: M,
    ) -> Result<WikiPollReportResult, WikiPollGroupDescriberError>
    where
        M: CompletionModel + 'static,
    {
        let template = self.render()?;
        let extractor = ExtractorBuilder::<WikiPollReportResult>::new(model)
            .preamble(&template)
            .build();

        let statement_json: String = serde_json::to_string(wiki_poll_report)?;

        let result = extractor.extract(statement_json).await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{
        test_helpers::test_model,
        wikipoll_describer::{WikiPollData, WikiPollGroupDescriber, WikiPollStatement},
    };

    #[tokio::test]
    async fn test_with_data() {
        let model = test_model();
        let data = fs::read_to_string("data/wiki_poll_results.json").unwrap();
        let data: Vec<WikiPollStatement> = serde_json::from_str(&data).unwrap();
        println!("total statements {}", data.len());
        let data: Vec<_> = data.into_iter().take(100).collect();
        let data = WikiPollData {
            title: "What is the future of south staffishire".into(),
            statements: data,
        };
        let describer = WikiPollGroupDescriber::builder()
            .context("Thesse are statements from a polis conv with the title 'What should be the futrue of south staffisher".into())
            .build();
        for i in 0..10 {
            let result = describer.run_with_model(&data, model.clone()).await;
            println!("{result:#?}");
            assert!(result.is_ok(), "The extractor should run ");
            let result = result.unwrap();
            fs::write(
                format!("run_{i}.json",),
                serde_json::to_string_pretty(&result).unwrap(),
            )
            .unwrap();
        }
    }
}
