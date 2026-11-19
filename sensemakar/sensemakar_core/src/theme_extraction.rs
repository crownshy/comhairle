use std::collections::HashSet;

use askama::Template;
use bon::Builder;
use sensemakar_types::{StatementError, StatementList, Theme, ThemeList};

use rig_agent::completion::StructuredOutputError;
use rig_agent::extractor::ExtractorBuilder;
use rig_core::DynModel;
use rig_core::operation::Completion;
use thiserror::Error;

use crate::theme_merger::{ThemeMerger, ThemeMergerError};

#[derive(Builder, Template)]
#[template(path = "themes/extract_themes.md")]
pub struct ThemeExtractor {
    existing_themes: Option<Vec<Theme>>,
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
    #[error("theme merge error {0}")]
    ThemeMergeError(#[from] ThemeMergerError),
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

    /// Returns only the themes newly discovered during this call, not the
    /// ones it was seeded with via `existing_themes` - those are assumed
    /// already persisted by the caller, so echoing them back would make the
    /// caller re-create them as if they were new.
    ///
    /// Each batch only sees the themes found by earlier batches (via the
    /// growing `existing_themes` prompt context), which doesn't reliably
    /// stop it minting a near-duplicate of something an earlier batch
    /// already found. A merge pass runs once at the end, over every
    /// candidate from every batch plus the seed, to collapse those
    /// duplicates - including ones that just reword a seed theme - down to
    /// one canonical entry per real topic.
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
        let seed_themes = self.existing_themes.clone().unwrap_or_default();
        let mut newly_found_themes: Vec<Theme> = vec![];

        for group in list.statements.chunks(batch_size) {
            let new_themes = self.run_with_model(group.to_vec(), model.clone()).await?;
            if !new_themes.is_empty() {
                if let Some(existing_themes) = &mut self.existing_themes {
                    existing_themes.extend(new_themes.clone());
                } else {
                    self.existing_themes = Some(new_themes.clone());
                }
                newly_found_themes.extend(new_themes);
            }
        }

        if newly_found_themes.is_empty() {
            return Ok(newly_found_themes);
        }

        merge_newly_found_themes(&seed_themes, newly_found_themes, model, self.context.clone())
            .await
    }
}

/// Runs the deduplication merge pass described on `run_in_batches_with_model`.
/// Returns only the canonical themes that are genuinely new - any merge
/// group that absorbed a seed theme is dropped, since that just confirms an
/// existing theme already covers it and the caller has that seed theme
/// already.
async fn merge_newly_found_themes(
    seed_themes: &[Theme],
    newly_found_themes: Vec<Theme>,
    model: DynModel<Completion>,
    context: Option<String>,
) -> Result<Vec<Theme>, ThemeExtractorError> {
    if seed_themes.len() + newly_found_themes.len() < 2 {
        return Ok(newly_found_themes);
    }

    // Each batch's model call invents its own candidate ids independently,
    // so ids commonly collide across batches - assign fresh ones that are
    // guaranteed unique for this merge call, tagged so we can tell seed
    // candidates apart from newly-found ones in the result.
    let seed_ids: HashSet<String> = (0..seed_themes.len()).map(|i| format!("seed-{i}")).collect();

    let mut candidates: Vec<Theme> = Vec::with_capacity(seed_themes.len() + newly_found_themes.len());
    for (i, theme) in seed_themes.iter().enumerate() {
        candidates.push(Theme {
            id: format!("seed-{i}"),
            name: theme.name.clone(),
            description: theme.description.clone(),
        });
    }
    for (i, theme) in newly_found_themes.into_iter().enumerate() {
        candidates.push(Theme {
            id: format!("new-{i}"),
            name: theme.name,
            description: theme.description,
        });
    }

    let merger = ThemeMerger::builder().maybe_context(context).build();
    let merge_result = merger.merge_with_model(candidates, model).await?;

    let merged_new_themes = merge_result
        .groups
        .into_iter()
        .filter(|group| !group.merged_ids.iter().any(|id| seed_ids.contains(id)))
        .map(|group| Theme {
            id: String::new(),
            name: group.name,
            description: group.description,
        })
        .collect();

    Ok(merged_new_themes)
}

#[cfg(test)]
mod tests {

    use askama::Template;
    use sensemakar_types::Theme;

    use crate::{
        test_helpers::{test_model, test_statements},
        theme_extraction::ThemeExtractor,
    };

    #[test]
    fn prompt_tells_model_to_keep_finding_themes_when_additional_themes_allowed() {
        let extractor = ThemeExtractor::builder()
            .existing_themes(vec![Theme {
                id: "1".into(),
                name: "Buses".into(),
                description: "More buses".into(),
            }])
            .allow_additional_themes(true)
            .build();

        let rendered = extractor.render().unwrap();

        assert!(
            rendered.contains("should still be grouped into"),
            "prompt should tell the model to keep extracting new themes: {rendered}"
        );
    }

    #[test]
    fn prompt_restricts_to_existing_themes_when_additional_themes_disallowed() {
        let extractor = ThemeExtractor::builder()
            .existing_themes(vec![Theme {
                id: "1".into(),
                name: "Buses".into(),
                description: "More buses".into(),
            }])
            .allow_additional_themes(false)
            .build();

        let rendered = extractor.render().unwrap();

        assert!(
            rendered.contains("Only use the themes listed above"),
            "prompt should restrict the model to the existing themes: {rendered}"
        );
    }

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

    #[tokio::test]
    async fn test_batches_do_not_echo_back_seeded_existing_themes() {
        let seed_theme_name = "Buses";
        let mut extractor = ThemeExtractor::builder()
            .context("These statements are from a survey form".into())
            .existing_themes(vec![Theme {
                id: "1".into(),
                name: seed_theme_name.into(),
                description: "More buses".into(),
            }])
            .allow_additional_themes(true)
            .build();

        let statement_list = test_statements();
        let model = test_model();

        let themes = extractor
            .run_in_batches_with_model(statement_list, model, 2)
            .await
            .unwrap();

        assert!(
            themes.iter().all(|theme| theme.name != seed_theme_name),
            "seeded existing theme should not be echoed back as newly discovered: {themes:?}"
        );
    }
}
