//! Public report data for the Prioritization section blocks (ADR-0061).
//!
//! Everything here is counted or summed across reviewers, and a breakdown built from fewer
//! than `MIN_ANSWERS_SHOWN` answers is withheld. Anything a reviewer typed never leaves.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, LazyLock};
use std::time::Duration;

use aide::axum::ApiRouter;
use aide::axum::routing::get_with;
use axum::extract::{Json, Query, State};
use moka::future::Cache;
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use super::{PrioritizationTool, PrioritizationToolConfig, Question, QuestionType};
use crate::ComhairleState;
use crate::error::ComhairleError;
use crate::models::proposal::{self, LocalizedProposal};
use crate::models::proposal_response::{
    self, ProposalResponse, ProposalResponseFilterOptions, ProposalResponseOrderOptions, Response,
    ResponseValue,
};
use crate::models::proposal_section::{self, LocalizedProposalSection};
use crate::models::translations::{TextContentId, localize_translations};
use crate::routes::auth::RequiredUser;
use crate::routes::translations::LocaleExtractor;
use crate::tools::public_report_data::anonymous::{
    Count, FacilitatorText, MIN_ANSWERS_SHOWN, PublicReportData, Score, public_report_data,
};
use crate::tools::public_report_data::themes::{
    PublicTheme, latest_theme_collection, public_theme,
};
use crate::tools::public_report_data::{
    preview_tool_config, serve_public_report_data, serve_report_preview_data,
};

/// Where the theme collections of a Prioritization step come from (`sensemaking::harvest`).
const THEME_ORIGIN: &str = "prioritization";

/// How long a step's proposals, reviews and themes are served before reading them again, so
/// a new review can take up to a minute to reach the report.
const CONTEXT_FRESH_FOR: Duration = Duration::from_secs(60);

/// Upper bound on cached steps and languages; well above anything one deployment hosts.
const CONTEXT_CACHE_CAPACITY: u64 = 1_000;

/// Keyed by step and language, since proposal and section text is stored per language.
static CONTEXT_CACHE: LazyLock<Cache<(Uuid, String), Arc<StepContext>>> = LazyLock::new(|| {
    Cache::builder()
        .max_capacity(CONTEXT_CACHE_CAPACITY)
        .time_to_live(CONTEXT_FRESH_FOR)
        .build()
});

public_report_data! {
    pub struct PrioritizationPublicKeyStats {
        pub participants: Count,
        pub proposals: Count,
        /// Proposal reviews handed in, across every proposal.
        pub reviews: Count,
    }
}

public_report_data! {
    pub struct PrioritizationPublicRankedProposal {
        pub title: FacilitatorText,
        /// The alignment question's answers summed, which is how the Insights page ranks.
        /// `None` when too few reviewers answered it to show.
        pub alignment: Option<Score>,
        pub reviews: Count,
    }
}

public_report_data! {
    pub struct PrioritizationPublicAlignmentQuestion {
        pub text: FacilitatorText,
        /// One answer's lowest and highest possible value, so the bars share a scale.
        pub min: Score,
        pub max: Score,
    }
}

public_report_data! {
    /// Proposals ranked by alignment, highest first, then the ones with too few answers.
    pub struct PrioritizationPublicRanking {
        /// `None` when the step has no alignment question, and `proposals` is then empty.
        pub alignment_question: Option<PrioritizationPublicAlignmentQuestion>,
        pub proposals: Vec<PrioritizationPublicRankedProposal>,
    }
}

public_report_data! {
    pub struct PrioritizationPublicChoice {
        pub label: FacilitatorText,
        pub count: Count,
    }
}

public_report_data! {
    /// One slice of a continuous scale, from `from` up to the next bucket's `from`.
    pub struct PrioritizationPublicBucket {
        pub from: Score,
        pub count: Count,
    }
}

public_report_data! {
    pub struct PrioritizationPublicScale {
        pub min: Score,
        pub max: Score,
        pub min_label: FacilitatorText,
        pub max_label: FacilitatorText,
        /// One bucket per sub-step of the scale, empty ones included.
        pub buckets: Vec<PrioritizationPublicBucket>,
    }
}

public_report_data! {
    /// One question's answers as counts. `choices` is filled for Likert questions and `scale`
    /// for continuous ones, unless `too_few_answers` is set. Free-text questions are never
    /// served.
    pub struct PrioritizationPublicQuestion {
        pub question: FacilitatorText,
        pub answered: Count,
        pub too_few_answers: bool,
        pub choices: Vec<PrioritizationPublicChoice>,
        pub scale: Option<PrioritizationPublicScale>,
    }
}

public_report_data! {
    pub struct PrioritizationPublicSection {
        pub body: FacilitatorText,
        pub questions: Vec<PrioritizationPublicQuestion>,
    }
}

public_report_data! {
    /// One proposal's results: the questions asked about it as a whole, then about each
    /// section, in the order the facilitator set them out.
    pub struct PrioritizationPublicProposal {
        pub title: FacilitatorText,
        pub reviews: Count,
        pub questions: Vec<PrioritizationPublicQuestion>,
        pub sections: Vec<PrioritizationPublicSection>,
    }
}

public_report_data! {
    pub struct PrioritizationPublicThemes {
        pub themes: Vec<PublicTheme>,
    }
}

/// A proposal the report could embed, for the add-chart dialog.
#[derive(Serialize, Debug, JsonSchema)]
pub struct PrioritizationEmbeddableProposal {
    pub id: Uuid,
    pub title: String,
}

/// What a section block's endpoints are called with.
trait SectionBlockQuery: DeserializeOwned + JsonSchema + Send + Sync + 'static {
    fn workflow_step_id(&self) -> Uuid;
    /// The embed's `config.item`, for section blocks embedded once per item.
    fn item(&self) -> Option<String>;
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct StepQuery {
    workflow_step_id: Uuid,
}

impl SectionBlockQuery for StepQuery {
    fn workflow_step_id(&self) -> Uuid {
        self.workflow_step_id
    }

    fn item(&self) -> Option<String> {
        None
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct ProposalQuery {
    workflow_step_id: Uuid,
    proposal_id: Uuid,
}

impl SectionBlockQuery for ProposalQuery {
    fn workflow_step_id(&self) -> Uuid {
        self.workflow_step_id
    }

    fn item(&self) -> Option<String> {
        Some(self.proposal_id.to_string())
    }
}

/// One embeddable section block: its public endpoint, preview twin and the data both serve.
trait SectionBlock: Send + Sync + 'static {
    /// The section block type the report stores (`lib/reports/embeds.ts`).
    const TYPE: &'static str;
    /// The last segment of both endpoint paths.
    const PATH: &'static str;
    /// Completes the operation ids `PrioritizationGetPublic…` and `PrioritizationPreview…`.
    const OPERATION: &'static str;
    const SUMMARY: &'static str;
    const DESCRIPTION: &'static str;

    type Query: SectionBlockQuery;
    type Data: PublicReportData + JsonSchema + Send;

    fn build(
        source: &ReportSource<'_>,
        query: &Self::Query,
    ) -> impl Future<Output = Result<Self::Data, ComhairleError>> + Send;
}

struct KeyStats;
struct Ranking;
struct ProposalResults;
struct Themes;

impl SectionBlock for KeyStats {
    const TYPE: &'static str = "prioritization-key-stats";
    const PATH: &'static str = "key_stats";
    const OPERATION: &'static str = "KeyStats";
    const SUMMARY: &'static str = "Key stats";
    const DESCRIPTION: &'static str = "Reviewer, proposal and review counts.";
    type Query = StepQuery;
    type Data = PrioritizationPublicKeyStats;

    async fn build(source: &ReportSource<'_>, _: &StepQuery) -> Result<Self::Data, ComhairleError> {
        Ok(key_stats(&source.context().await?.proposals))
    }
}

impl SectionBlock for Ranking {
    const TYPE: &'static str = "prioritization-ranking";
    const PATH: &'static str = "ranking";
    const OPERATION: &'static str = "Ranking";
    const SUMMARY: &'static str = "Proposal ranking";
    const DESCRIPTION: &'static str = "Proposals ranked by their summed alignment answers, \
        with no proposals when the step has no alignment question.";
    type Query = StepQuery;
    type Data = PrioritizationPublicRanking;

    async fn build(source: &ReportSource<'_>, _: &StepQuery) -> Result<Self::Data, ComhairleError> {
        let context = source.context().await?;
        Ok(ranking(
            &source.config,
            &context.proposals,
            &source.texts().await?,
        ))
    }
}

impl SectionBlock for ProposalResults {
    const TYPE: &'static str = "prioritization-proposal";
    const PATH: &'static str = "proposal";
    const OPERATION: &'static str = "Proposal";
    const SUMMARY: &'static str = "One proposal's results";
    const DESCRIPTION: &'static str = "Counts per Likert choice or scale bucket for every rated \
        question about the proposal and its sections. The embed must be for this proposal.";
    type Query = ProposalQuery;
    type Data = PrioritizationPublicProposal;

    async fn build(
        source: &ReportSource<'_>,
        query: &ProposalQuery,
    ) -> Result<Self::Data, ComhairleError> {
        let context = source.context().await?;
        let reviewed = context
            .proposals
            .iter()
            .find(|p| p.proposal.id == query.proposal_id)
            .ok_or_else(|| {
                ComhairleError::ResourceNotFound(format!("Proposal {}", query.proposal_id))
            })?;
        Ok(proposal_results(
            &source.config,
            reviewed,
            &source.texts().await?,
        ))
    }
}

impl SectionBlock for Themes {
    const TYPE: &'static str = "prioritization-themes";
    const PATH: &'static str = "themes";
    const OPERATION: &'static str = "Themes";
    const SUMMARY: &'static str = "Themes";
    const DESCRIPTION: &'static str =
        "The step's latest themes with how many written answers each groups.";
    type Query = StepQuery;
    type Data = PrioritizationPublicThemes;

    async fn build(source: &ReportSource<'_>, _: &StepQuery) -> Result<Self::Data, ComhairleError> {
        Ok(PrioritizationPublicThemes {
            themes: source.context().await?.themes.clone(),
        })
    }
}

pub fn routes() -> ApiRouter<Arc<ComhairleState>> {
    ApiRouter::new()
        .merge(section_block_routes::<KeyStats>())
        .merge(section_block_routes::<Ranking>())
        .merge(section_block_routes::<ProposalResults>())
        .merge(section_block_routes::<Themes>())
        .api_route(
            "/prioritization/report_preview/proposals",
            get_with(embeddable_proposals, |op| {
                op.id("PrioritizationListEmbeddableProposals")
                    .tag("Tools")
                    .summary("Proposals the report can embed")
                    .description(
                        "The step's proposals, for facilitators who can update the \
                         conversation.",
                    )
                    .response::<200, Json<Vec<PrioritizationEmbeddableProposal>>>()
            }),
        )
}

/// The public endpoint and its preview twin for one section block.
fn section_block_routes<Block: SectionBlock>() -> ApiRouter<Arc<ComhairleState>> {
    ApiRouter::new()
        .api_route(
            &format!("/prioritization/public_report/{}", Block::PATH),
            get_with(get_public::<Block>, |op| {
                op.id(&format!("PrioritizationGetPublic{}", Block::OPERATION))
                    .tag("Tools")
                    .summary(&format!("{} for a published report", Block::SUMMARY))
                    .description(&format!(
                        "{} 404 unless the conversation's published report embeds this \
                         section block for the step.",
                        Block::DESCRIPTION
                    ))
                    .response::<200, Json<Block::Data>>()
            }),
        )
        .api_route(
            &format!("/prioritization/report_preview/{}", Block::PATH),
            get_with(get_preview::<Block>, |op| {
                op.id(&format!("PrioritizationPreview{}", Block::OPERATION))
                    .tag("Tools")
                    .summary(&format!(
                        "{} as the published report will show them",
                        Block::SUMMARY
                    ))
                    .description(&format!(
                        "The same data as PrioritizationGetPublic{}, for facilitators who can \
                         update the conversation, before the report is published.",
                        Block::OPERATION
                    ))
                    .response::<200, Json<Block::Data>>()
            }),
        )
}

#[instrument(err(Debug), skip_all, fields(section_block = Block::TYPE))]
async fn get_public<Block: SectionBlock>(
    State(state): State<Arc<ComhairleState>>,
    LocaleExtractor(locale): LocaleExtractor,
    Query(query): Query<Block::Query>,
) -> Result<Json<Block::Data>, ComhairleError> {
    let workflow_step_id = query.workflow_step_id();
    let data = serve_public_report_data::<PrioritizationTool, _>(
        &state.db,
        &workflow_step_id,
        Block::TYPE,
        query.item().as_deref(),
        async |config| {
            let source = ReportSource::new(&state, workflow_step_id, config, &locale);
            Block::build(&source, &query).await
        },
    )
    .await?;
    Ok(Json(data))
}

#[instrument(err(Debug), skip_all, fields(section_block = Block::TYPE))]
async fn get_preview<Block: SectionBlock>(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    LocaleExtractor(locale): LocaleExtractor,
    Query(query): Query<Block::Query>,
) -> Result<Json<Block::Data>, ComhairleError> {
    let workflow_step_id = query.workflow_step_id();
    let data = serve_report_preview_data::<PrioritizationTool, _>(
        &state,
        &user,
        &workflow_step_id,
        async |config| {
            let source = ReportSource::new(&state, workflow_step_id, config, &locale);
            Block::build(&source, &query).await
        },
    )
    .await?;
    Ok(Json(data))
}

#[instrument(err(Debug), skip(state))]
async fn embeddable_proposals(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    LocaleExtractor(locale): LocaleExtractor,
    Query(StepQuery { workflow_step_id }): Query<StepQuery>,
) -> Result<Json<Vec<PrioritizationEmbeddableProposal>>, ComhairleError> {
    preview_tool_config::<PrioritizationTool>(&state, &user, &workflow_step_id).await?;
    let proposals = proposal::list_localized(&state.db, &workflow_step_id, &locale).await?;
    Ok(Json(
        proposals
            .into_iter()
            .map(|proposal| PrioritizationEmbeddableProposal {
                id: proposal.id,
                title: proposal.title,
            })
            .collect(),
    ))
}

/// A proposal with everything reviewers handed in about it.
struct ReviewedProposal {
    proposal: LocalizedProposal,
    sections: Vec<LocalizedProposalSection>,
    reviews: Vec<ProposalResponse>,
}

impl ReviewedProposal {
    fn answers(&self) -> impl Iterator<Item = &Response> {
        self.reviews
            .iter()
            .flat_map(|review| review.response.0.iter())
    }
}

/// What every section block of a step reads, in one language. Shared through `CONTEXT_CACHE`.
struct StepContext {
    proposals: Vec<ReviewedProposal>,
    themes: Vec<PublicTheme>,
}

/// One Prioritization step's stored data and config, in the reader's language.
struct ReportSource<'a> {
    state: &'a ComhairleState,
    workflow_step_id: Uuid,
    config: PrioritizationToolConfig,
    locale: &'a str,
}

impl<'a> ReportSource<'a> {
    fn new(
        state: &'a ComhairleState,
        workflow_step_id: Uuid,
        config: PrioritizationToolConfig,
        locale: &'a str,
    ) -> Self {
        Self {
            state,
            workflow_step_id,
            config,
            locale,
        }
    }

    /// Requests that miss at the same time wait on one load.
    async fn context(&self) -> Result<Arc<StepContext>, ComhairleError> {
        let key = (self.workflow_step_id, self.locale.to_string());
        CONTEXT_CACHE
            .try_get_with(key, async { self.load_context().await.map(Arc::new) })
            .await
            .map_err(|error: Arc<ComhairleError>| {
                ComhairleError::ReportDataUnavailable(error.to_string())
            })
    }

    async fn load_context(&self) -> Result<StepContext, ComhairleError> {
        Ok(StepContext {
            proposals: self.reviewed_proposals().await?,
            themes: self.themes().await?,
        })
    }

    async fn reviewed_proposals(&self) -> Result<Vec<ReviewedProposal>, ComhairleError> {
        let db = &self.state.db;
        let mut reviewed = vec![];
        for proposal in proposal::list_localized(db, &self.workflow_step_id, self.locale).await? {
            let sections = proposal_section::list_localized(db, &proposal.id, self.locale).await?;
            let reviews = proposal_response::list(
                db,
                &proposal.id,
                ProposalResponseFilterOptions::default(),
                ProposalResponseOrderOptions,
            )
            .await?;
            reviewed.push(ReviewedProposal {
                proposal,
                sections,
                reviews,
            });
        }
        Ok(reviewed)
    }

    async fn themes(&self) -> Result<Vec<PublicTheme>, ComhairleError> {
        let collection =
            latest_theme_collection(&self.state.db, &self.workflow_step_id, THEME_ORIGIN)
                .await?
                .unwrap_or_default();
        Ok(collection.iter().map(public_theme).collect())
    }

    /// Not cached with the context: the preview and live configs can word questions
    /// differently.
    async fn texts(&self) -> Result<Texts, ComhairleError> {
        Texts::load(self.state, &self.config, self.locale).await
    }
}

fn key_stats(proposals: &[ReviewedProposal]) -> PrioritizationPublicKeyStats {
    let reviewers: HashSet<Uuid> = proposals
        .iter()
        .flat_map(|p| p.reviews.iter().map(|r| r.user_id))
        .collect();
    PrioritizationPublicKeyStats {
        participants: Count(reviewers.len() as u64),
        proposals: Count(proposals.len() as u64),
        reviews: Count(proposals.iter().map(|p| p.reviews.len() as u64).sum()),
    }
}

fn ranking(
    config: &PrioritizationToolConfig,
    proposals: &[ReviewedProposal],
    texts: &Texts,
) -> PrioritizationPublicRanking {
    let Some(alignment_question) = config
        .alignment_question_id
        .and_then(|id| config.questions.iter().find(|q| q.id == id))
    else {
        return PrioritizationPublicRanking {
            alignment_question: None,
            proposals: vec![],
        };
    };
    let (min, max) = answer_range(alignment_question);

    let mut ranked: Vec<PrioritizationPublicRankedProposal> = proposals
        .iter()
        .map(|p| {
            let alignment_answers: Vec<f64> = p
                .answers()
                .filter(|a| a.question_id == alignment_question.id && a.section_id.is_none())
                .filter_map(|a| numeric_answer(&a.value))
                .collect();
            PrioritizationPublicRankedProposal {
                title: FacilitatorText::from_facilitator(p.proposal.title.clone()),
                alignment: (alignment_answers.len() >= MIN_ANSWERS_SHOWN)
                    .then(|| Score(alignment_answers.iter().sum())),
                reviews: Count(p.reviews.len() as u64),
            }
        })
        .collect();
    ranked.sort_by(|a, b| match (a.alignment, b.alignment) {
        (Some(a), Some(b)) => b.0.total_cmp(&a.0),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => std::cmp::Ordering::Equal,
    });

    PrioritizationPublicRanking {
        alignment_question: Some(PrioritizationPublicAlignmentQuestion {
            text: texts.get(&alignment_question.text),
            min: Score(min),
            max: Score(max),
        }),
        proposals: ranked,
    }
}

fn proposal_results(
    config: &PrioritizationToolConfig,
    reviewed: &ReviewedProposal,
    texts: &Texts,
) -> PrioritizationPublicProposal {
    let answers_to = |question: &Question, section_id: Option<Uuid>| -> Vec<&ResponseValue> {
        reviewed
            .answers()
            .filter(|a| a.question_id == question.id && a.section_id == section_id)
            .map(|a| &a.value)
            .collect()
    };

    let questions = config
        .questions
        .iter()
        .filter_map(|question| question_results(question, &answers_to(question, None), texts))
        .collect();

    let sections = reviewed
        .sections
        .iter()
        .map(|section| PrioritizationPublicSection {
            body: FacilitatorText::from_facilitator(section.body.clone()),
            questions: config
                .section_questions
                .iter()
                .filter_map(|question| {
                    question_results(question, &answers_to(question, Some(section.id)), texts)
                })
                .collect(),
        })
        .collect();

    PrioritizationPublicProposal {
        title: FacilitatorText::from_facilitator(reviewed.proposal.title.clone()),
        reviews: Count(reviewed.reviews.len() as u64),
        questions,
        sections,
    }
}

/// One question's answers as counts. `None` for free-text questions, which can't go public.
fn question_results(
    question: &Question,
    answers: &[&ResponseValue],
    texts: &Texts,
) -> Option<PrioritizationPublicQuestion> {
    if matches!(question.r#type, QuestionType::Text) {
        return None;
    }
    let numbers: Vec<f64> = answers.iter().filter_map(|a| numeric_answer(a)).collect();
    let mut results = PrioritizationPublicQuestion {
        question: texts.get(&question.text),
        answered: Count(numbers.len() as u64),
        too_few_answers: numbers.len() < MIN_ANSWERS_SHOWN,
        choices: vec![],
        scale: None,
    };
    if results.too_few_answers {
        return Some(results);
    }
    match &question.r#type {
        QuestionType::Text => {}
        QuestionType::LikertScale { categories } => {
            results.choices = categories
                .iter()
                .map(|category| PrioritizationPublicChoice {
                    label: texts.get(&category.label),
                    count: Count(numbers.iter().filter(|v| **v == category.value).count() as u64),
                })
                .collect();
        }
        QuestionType::Continuous {
            sub_steps,
            min_value,
            max_value,
            min_label,
            max_label,
        } => {
            results.scale = Some(PrioritizationPublicScale {
                min: Score(*min_value),
                max: Score(*max_value),
                min_label: texts.get(min_label),
                max_label: texts.get(max_label),
                buckets: buckets(&numbers, *sub_steps, *min_value, *max_value),
            });
        }
    }
    Some(results)
}

/// Counts the answers in each of `sub_steps` equal slices of the scale, empty ones included.
fn buckets(
    numbers: &[f64],
    sub_steps: i32,
    min_value: f64,
    max_value: f64,
) -> Vec<PrioritizationPublicBucket> {
    let steps = usize::try_from(sub_steps).unwrap_or(0).max(1);
    let width = (max_value - min_value) / steps as f64;
    let mut counts = vec![0u64; steps];
    for value in numbers {
        let bucket = if width > 0.0 {
            ((value - min_value) / width).floor()
        } else {
            0.0
        };
        counts[bucket.clamp(0.0, (steps - 1) as f64) as usize] += 1;
    }
    counts
        .into_iter()
        .enumerate()
        .map(|(index, count)| PrioritizationPublicBucket {
            from: Score(min_value + index as f64 * width),
            count: Count(count),
        })
        .collect()
}

/// The lowest and highest value one answer to the question can take.
fn answer_range(question: &Question) -> (f64, f64) {
    match &question.r#type {
        QuestionType::Text => (0.0, 0.0),
        QuestionType::LikertScale { categories } => categories
            .iter()
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), c| {
                (min.min(c.value), max.max(c.value))
            }),
        QuestionType::Continuous {
            min_value,
            max_value,
            ..
        } => (*min_value, *max_value),
    }
}

fn numeric_answer(value: &ResponseValue) -> Option<f64> {
    match value {
        ResponseValue::Number(number) => Some(*number),
        ResponseValue::Text(_) => None,
    }
}

/// The step's question texts and labels in one language.
struct Texts(HashMap<TextContentId, String>);

impl Texts {
    async fn load(
        state: &ComhairleState,
        config: &PrioritizationToolConfig,
        locale: &str,
    ) -> Result<Self, ComhairleError> {
        let ids: Vec<TextContentId> = config
            .questions
            .iter()
            .chain(&config.section_questions)
            .flat_map(|question| {
                let mut ids = vec![question.text];
                match &question.r#type {
                    QuestionType::Text => {}
                    QuestionType::LikertScale { categories } => {
                        ids.extend(categories.iter().map(|c| c.label));
                    }
                    QuestionType::Continuous {
                        min_label,
                        max_label,
                        ..
                    } => ids.extend([*min_label, *max_label]),
                }
                ids
            })
            .collect();
        Ok(Self(localize_translations(&state.db, &ids, locale).await?))
    }

    fn get(&self, id: &TextContentId) -> FacilitatorText {
        FacilitatorText::from_facilitator(self.0.get(id).cloned().unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::prioritization::Category;

    fn likert_question() -> Question {
        Question {
            id: Uuid::new_v4(),
            text: TextContentId(Uuid::new_v4()),
            r#type: QuestionType::LikertScale {
                categories: [1.0, 2.0, 3.0]
                    .map(|value| Category {
                        value,
                        label: TextContentId(Uuid::new_v4()),
                    })
                    .to_vec(),
            },
            required: true,
        }
    }

    fn answers(values: &[f64]) -> Vec<ResponseValue> {
        values.iter().map(|v| ResponseValue::Number(*v)).collect()
    }

    fn results(question: &Question, values: &[ResponseValue]) -> PrioritizationPublicQuestion {
        let values: Vec<&ResponseValue> = values.iter().collect();
        question_results(question, &values, &Texts(HashMap::new())).unwrap()
    }

    #[test]
    fn should_withhold_a_breakdown_of_too_few_answers() {
        let withheld = results(&likert_question(), &answers(&[1.0, 3.0]));

        assert!(withheld.too_few_answers);
        assert_eq!(withheld.answered, Count(2));
        assert!(withheld.choices.is_empty());
    }

    #[test]
    fn should_count_each_likert_choice_once_enough_answered() {
        let shown = results(&likert_question(), &answers(&[1.0, 3.0, 3.0]));

        assert!(!shown.too_few_answers);
        let counts: Vec<Count> = shown.choices.iter().map(|c| c.count).collect();
        assert_eq!(counts, [Count(1), Count(0), Count(2)]);
    }

    #[test]
    fn should_never_serve_free_text_questions() {
        let question = Question {
            r#type: QuestionType::Text,
            ..likert_question()
        };
        let typed = vec![ResponseValue::Text("My answer".into()); 3];
        let typed: Vec<&ResponseValue> = typed.iter().collect();

        assert!(question_results(&question, &typed, &Texts(HashMap::new())).is_none());
    }

    #[test]
    fn should_bucket_continuous_answers_including_the_top_edge() {
        let counts: Vec<Count> = buckets(&[0.0, 4.9, 5.0, 10.0], 2, 0.0, 10.0)
            .iter()
            .map(|bucket| bucket.count)
            .collect();

        assert_eq!(counts, [Count(2), Count(2)]);
    }
}
