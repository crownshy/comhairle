use askama::Template;
use bon::Builder;
use rig_agent::completion::StructuredOutputError;
use rig_agent::extractor::ExtractorBuilder;
use rig_core::DynModel;
use rig_core::operation::Completion;
use sensemakar_types::{Theme, ThemeError, ThemeList, ThemeMergeResult};
use thiserror::Error;

#[derive(Builder, Template)]
#[template(path = "themes/merge_themes.md")]
pub struct ThemeMerger {
    context: Option<String>,
}

#[derive(Error, Debug)]
pub enum ThemeMergerError {
    #[error("Template Error {0}")]
    TemplateError(#[from] askama::Error),
    #[error("extractor error {0}")]
    RigExtractorError(#[from] Box<StructuredOutputError>),
    #[error("Theme serialization error {0}")]
    ThemeSerializationError(#[from] ThemeError),
}

impl ThemeMerger {
    /// Collapses a candidate list down to one canonical theme per group of
    /// near-duplicates. The caller assigns `candidates[n].id` before calling
    /// this - the merge call only needs those ids to be unique so it can
    /// report which candidates each returned group absorbed.
    pub async fn merge_with_model(
        &self,
        candidates: Vec<Theme>,
        model: impl Into<DynModel<Completion>>,
    ) -> Result<ThemeMergeResult, ThemeMergerError> {
        let template = self.render()?;
        let extractor = ExtractorBuilder::<ThemeMergeResult>::new(model)
            .append_preamble(&template)
            .retries(2)
            .build();

        let candidate_text = ThemeList { themes: candidates }.to_model_embed()?;
        let result = extractor
            .extract(candidate_text)
            .await
            .map_err(Box::new)?
            .output;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use sensemakar_types::Theme;

    use crate::{test_helpers::test_model, theme_merger::ThemeMerger};

    #[tokio::test]
    async fn merges_near_duplicate_candidates_into_one_group() {
        let merger = ThemeMerger::builder()
            .context("These are candidate themes from a council consultation".into())
            .build();

        let candidates = vec![
            Theme {
                id: "a".into(),
                name: "Bus reliability".into(),
                description: "Buses running late or not turning up".into(),
            },
            Theme {
                id: "b".into(),
                name: "Late buses".into(),
                description: "Buses are frequently late, making people miss work".into(),
            },
            Theme {
                id: "c".into(),
                name: "Bike storage".into(),
                description: "Nowhere safe to lock up bikes near the station".into(),
            },
        ];

        let result = merger
            .merge_with_model(candidates, test_model())
            .await
            .unwrap();

        let all_ids: Vec<&str> = result
            .groups
            .iter()
            .flat_map(|g| g.merged_ids.iter().map(String::as_str))
            .collect();
        assert_eq!(all_ids.len(), 3, "every candidate id should appear exactly once: {all_ids:?}");

        let bus_group = result
            .groups
            .iter()
            .find(|g| g.merged_ids.contains(&"a".to_string()))
            .expect("candidate 'a' should be in some group");
        assert!(
            bus_group.merged_ids.contains(&"b".to_string()),
            "the two bus-lateness candidates should be merged into the same group: {:?}",
            result.groups
        );
    }
}
