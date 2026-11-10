//! Public report data for the Prioritization section blocks (ADR-0061).
//!
//! Everything here is counted or summed across reviewers. A reviewer's own answers, and in
//! particular anything they typed, never leave through these endpoints.

use std::collections::HashSet;
use std::sync::Arc;

use aide::axum::ApiRouter;
use aide::axum::routing::get_with;
use axum::extract::{Json, Query, State};
use schemars::JsonSchema;
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
use crate::models::users::User;
use crate::routes::auth::RequiredUser;
use crate::routes::translations::LocaleExtractor;
use crate::tools::public_report_data::anonymous::{
    Count, FacilitatorText, PublicReportData, Score, public_report_data,
};
use crate::tools::public_report_data::themes::{
    PublicTheme, latest_theme_collection, public_theme,
};
use crate::tools::public_report_data::{
    preview_tool_config, serve_public_report_data, serve_report_preview_data,
};

/// Section block types, as the report stores them (`lib/reports/embeds.ts`).
const KEY_STATS: &str = "prioritization-key-stats";
const RANKING: &str = "prioritization-ranking";
const PROPOSAL: &str = "prioritization-proposal";
const THEMES: &str = "prioritization-themes";

/// Where the theme collections of a Prioritization step come from (`sensemaking::harvest`).
const THEME_ORIGIN: &str = "prioritization";

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
        pub alignment: Score,
        pub reviews: Count,
    }
}

public_report_data! {
    /// Proposals ranked by alignment, highest first.
    pub struct PrioritizationPublicRanking {
        pub question: FacilitatorText,
        /// One answer's lowest and highest possible value, so the bars share a scale.
        pub min: Score,
        pub max: Score,
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
    /// for continuous ones. Free-text questions are never served.
    pub struct PrioritizationPublicQuestion {
        pub question: FacilitatorText,
        pub answered: Count,
        pub choices: Vec<PrioritizationPublicChoice>,
        pub scale: Option<PrioritizationPublicScale>,
    }
}

public_report_data! {
    pub struct PrioritizationPublicSection {
        pub position: Count,
        pub body: FacilitatorText,
        pub questions: Vec<PrioritizationPublicQuestion>,
    }
}

public_report_data! {
    /// One proposal's results: the questions asked about it as a whole, then about each section.
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

#[derive(Serialize, Deserialize, JsonSchema)]
struct StepQuery {
    workflow_step_id: Uuid,
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct ProposalQuery {
    workflow_step_id: Uuid,
    proposal_id: Uuid,
}

pub fn routes() -> ApiRouter<Arc<ComhairleState>> {
    ApiRouter::new()
        .api_route(
            "/prioritization/public_report/key_stats",
            get_with(get_key_stats, |op| {
                op.id("PrioritizationGetPublicKeyStats")
                    .tag("Tools")
                    .summary("Key stats for a published report")
                    .description(
                        "Reviewer, proposal and review counts for a Prioritization step. 404 \
                         unless the conversation's published report embeds this section block \
                         for the step.",
                    )
                    .response::<200, Json<PrioritizationPublicKeyStats>>()
            }),
        )
        .api_route(
            "/prioritization/report_preview/key_stats",
            get_with(preview_key_stats, |op| {
                op.id("PrioritizationPreviewKeyStats")
                    .tag("Tools")
                    .summary("Key stats as the published report will show them")
                    .description(
                        "The same data as PrioritizationGetPublicKeyStats, for facilitators \
                         who can update the conversation, before the report is published.",
                    )
                    .response::<200, Json<PrioritizationPublicKeyStats>>()
            }),
        )
        .api_route(
            "/prioritization/public_report/ranking",
            get_with(get_ranking, |op| {
                op.id("PrioritizationGetPublicRanking")
                    .tag("Tools")
                    .summary("Proposal ranking for a published report")
                    .description(
                        "Proposals ranked by their summed alignment answers. 404 unless the \
                         conversation's published report embeds this section block for the \
                         step; 422 when the step has no alignment question.",
                    )
                    .response::<200, Json<PrioritizationPublicRanking>>()
            }),
        )
        .api_route(
            "/prioritization/report_preview/ranking",
            get_with(preview_ranking, |op| {
                op.id("PrioritizationPreviewRanking")
                    .tag("Tools")
                    .summary("Proposal ranking as the published report will show it")
                    .description(
                        "The same data as PrioritizationGetPublicRanking, for facilitators who \
                         can update the conversation, before the report is published.",
                    )
                    .response::<200, Json<PrioritizationPublicRanking>>()
            }),
        )
        .api_route(
            "/prioritization/public_report/proposal",
            get_with(get_proposal, |op| {
                op.id("PrioritizationGetPublicProposal")
                    .tag("Tools")
                    .summary("One proposal's results for a published report")
                    .description(
                        "Counts per Likert choice or scale bucket for every rated question \
                         about the proposal and its sections. 404 unless the conversation's \
                         published report embeds this proposal.",
                    )
                    .response::<200, Json<PrioritizationPublicProposal>>()
            }),
        )
        .api_route(
            "/prioritization/report_preview/proposal",
            get_with(preview_proposal, |op| {
                op.id("PrioritizationPreviewProposal")
                    .tag("Tools")
                    .summary("One proposal's results as the published report will show them")
                    .description(
                        "The same data as PrioritizationGetPublicProposal, for facilitators who \
                         can update the conversation, before the report is published.",
                    )
                    .response::<200, Json<PrioritizationPublicProposal>>()
            }),
        )
        .api_route(
            "/prioritization/public_report/themes",
            get_with(get_themes, |op| {
                op.id("PrioritizationGetPublicThemes")
                    .tag("Tools")
                    .summary("Themes for a published report")
                    .description(
                        "The step's latest themes with how many written answers each groups. \
                         404 unless the conversation's published report embeds this section \
                         block for the step.",
                    )
                    .response::<200, Json<PrioritizationPublicThemes>>()
            }),
        )
        .api_route(
            "/prioritization/report_preview/themes",
            get_with(preview_themes, |op| {
                op.id("PrioritizationPreviewThemes")
                    .tag("Tools")
                    .summary("Themes as the published report will show them")
                    .description(
                        "The same data as PrioritizationGetPublicThemes, for facilitators who \
                         can update the conversation, before the report is published.",
                    )
                    .response::<200, Json<PrioritizationPublicThemes>>()
            }),
        )
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

#[instrument(err(Debug), skip(state))]
async fn get_key_stats(
    State(state): State<Arc<ComhairleState>>,
    LocaleExtractor(locale): LocaleExtractor,
    Query(StepQuery { workflow_step_id }): Query<StepQuery>,
) -> Result<Json<PrioritizationPublicKeyStats>, ComhairleError> {
    serve_public(&state, workflow_step_id, KEY_STATS, &locale, async |step| {
        Ok(key_stats(&step))
    })
    .await
}

#[instrument(err(Debug), skip(state))]
async fn preview_key_stats(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    LocaleExtractor(locale): LocaleExtractor,
    Query(StepQuery { workflow_step_id }): Query<StepQuery>,
) -> Result<Json<PrioritizationPublicKeyStats>, ComhairleError> {
    serve_preview(&state, &user, workflow_step_id, &locale, async |step| {
        Ok(key_stats(&step))
    })
    .await
}

#[instrument(err(Debug), skip(state))]
async fn get_ranking(
    State(state): State<Arc<ComhairleState>>,
    LocaleExtractor(locale): LocaleExtractor,
    Query(StepQuery { workflow_step_id }): Query<StepQuery>,
) -> Result<Json<PrioritizationPublicRanking>, ComhairleError> {
    serve_public(&state, workflow_step_id, RANKING, &locale, async |step| {
        ranking(&state, &step, &locale).await
    })
    .await
}

#[instrument(err(Debug), skip(state))]
async fn preview_ranking(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    LocaleExtractor(locale): LocaleExtractor,
    Query(StepQuery { workflow_step_id }): Query<StepQuery>,
) -> Result<Json<PrioritizationPublicRanking>, ComhairleError> {
    serve_preview(&state, &user, workflow_step_id, &locale, async |step| {
        ranking(&state, &step, &locale).await
    })
    .await
}

#[instrument(err(Debug), skip(state))]
async fn get_proposal(
    State(state): State<Arc<ComhairleState>>,
    LocaleExtractor(locale): LocaleExtractor,
    Query(ProposalQuery {
        workflow_step_id,
        proposal_id,
    }): Query<ProposalQuery>,
) -> Result<Json<PrioritizationPublicProposal>, ComhairleError> {
    let data = serve_public_report_data::<PrioritizationTool, _>(
        &state.db,
        &workflow_step_id,
        PROPOSAL,
        Some(&proposal_id.to_string()),
        async |config| {
            let step = load_step(&state, &workflow_step_id, config, &locale).await?;
            proposal_results(&state, &step, &proposal_id, &locale).await
        },
    )
    .await?;
    Ok(Json(data))
}

#[instrument(err(Debug), skip(state))]
async fn preview_proposal(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    LocaleExtractor(locale): LocaleExtractor,
    Query(ProposalQuery {
        workflow_step_id,
        proposal_id,
    }): Query<ProposalQuery>,
) -> Result<Json<PrioritizationPublicProposal>, ComhairleError> {
    serve_preview(&state, &user, workflow_step_id, &locale, async |step| {
        proposal_results(&state, &step, &proposal_id, &locale).await
    })
    .await
}

#[instrument(err(Debug), skip(state))]
async fn get_themes(
    State(state): State<Arc<ComhairleState>>,
    Query(StepQuery { workflow_step_id }): Query<StepQuery>,
) -> Result<Json<PrioritizationPublicThemes>, ComhairleError> {
    let data = serve_public_report_data::<PrioritizationTool, _>(
        &state.db,
        &workflow_step_id,
        THEMES,
        None,
        async |_| themes(&state, &workflow_step_id).await,
    )
    .await?;
    Ok(Json(data))
}

#[instrument(err(Debug), skip(state))]
async fn preview_themes(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Query(StepQuery { workflow_step_id }): Query<StepQuery>,
) -> Result<Json<PrioritizationPublicThemes>, ComhairleError> {
    let data = serve_report_preview_data::<PrioritizationTool, _>(
        &state,
        &user,
        &workflow_step_id,
        async |_| themes(&state, &workflow_step_id).await,
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

async fn serve_public<Data: PublicReportData>(
    state: &ComhairleState,
    workflow_step_id: Uuid,
    section_block: &str,
    locale: &str,
    build: impl AsyncFnOnce(Step) -> Result<Data, ComhairleError>,
) -> Result<Json<Data>, ComhairleError> {
    let data = serve_public_report_data::<PrioritizationTool, _>(
        &state.db,
        &workflow_step_id,
        section_block,
        None,
        async |config| build(load_step(state, &workflow_step_id, config, locale).await?).await,
    )
    .await?;
    Ok(Json(data))
}

async fn serve_preview<Data: PublicReportData>(
    state: &Arc<ComhairleState>,
    user: &User,
    workflow_step_id: Uuid,
    locale: &str,
    build: impl AsyncFnOnce(Step) -> Result<Data, ComhairleError>,
) -> Result<Json<Data>, ComhairleError> {
    let data = serve_report_preview_data::<PrioritizationTool, _>(
        state,
        user,
        &workflow_step_id,
        async |config| build(load_step(state, &workflow_step_id, config, locale).await?).await,
    )
    .await?;
    Ok(Json(data))
}

/// A proposal with everything reviewers handed in about it.
struct ReviewedProposal {
    proposal: LocalizedProposal,
    sections: Vec<LocalizedProposalSection>,
    reviews: Vec<ProposalResponse>,
}

/// The step's config and proposals, in the reader's language.
struct Step {
    config: PrioritizationToolConfig,
    proposals: Vec<ReviewedProposal>,
}

async fn load_step(
    state: &ComhairleState,
    workflow_step_id: &Uuid,
    config: PrioritizationToolConfig,
    locale: &str,
) -> Result<Step, ComhairleError> {
    let mut proposals = vec![];
    for proposal in proposal::list_localized(&state.db, workflow_step_id, locale).await? {
        let sections = proposal_section::list_localized(&state.db, &proposal.id, locale).await?;
        let reviews = proposal_response::list(
            &state.db,
            &proposal.id,
            ProposalResponseFilterOptions,
            ProposalResponseOrderOptions,
        )
        .await?;
        proposals.push(ReviewedProposal {
            proposal,
            sections,
            reviews,
        });
    }
    Ok(Step { config, proposals })
}

fn key_stats(step: &Step) -> PrioritizationPublicKeyStats {
    let reviewers: HashSet<Uuid> = step
        .proposals
        .iter()
        .flat_map(|p| p.reviews.iter().map(|r| r.user_id))
        .collect();
    PrioritizationPublicKeyStats {
        participants: Count(reviewers.len() as u64),
        proposals: Count(step.proposals.len() as u64),
        reviews: Count(step.proposals.iter().map(|p| p.reviews.len() as u64).sum()),
    }
}

async fn ranking(
    state: &ComhairleState,
    step: &Step,
    locale: &str,
) -> Result<PrioritizationPublicRanking, ComhairleError> {
    let alignment_question = step
        .config
        .alignment_question_id
        .and_then(|id| step.config.questions.iter().find(|q| q.id == id))
        .ok_or_else(|| {
            ComhairleError::Unprocessable("The step has no alignment question".to_string())
        })?;
    let texts = Texts::load(state, &step.config, locale).await?;
    let (min, max) = answer_range(alignment_question);

    let mut proposals: Vec<PrioritizationPublicRankedProposal> = step
        .proposals
        .iter()
        .map(|p| PrioritizationPublicRankedProposal {
            title: FacilitatorText::from_facilitator(p.proposal.title.clone()),
            alignment: Score(
                p.reviews
                    .iter()
                    .flat_map(|r| r.response.0.iter())
                    .filter(|a| a.question_id == alignment_question.id)
                    .filter_map(|a| number(&a.value))
                    .sum(),
            ),
            reviews: Count(p.reviews.len() as u64),
        })
        .collect();
    proposals.sort_by(|a, b| b.alignment.0.total_cmp(&a.alignment.0));

    Ok(PrioritizationPublicRanking {
        question: texts.get(&alignment_question.text),
        min: Score(min),
        max: Score(max),
        proposals,
    })
}

async fn proposal_results(
    state: &ComhairleState,
    step: &Step,
    proposal_id: &Uuid,
    locale: &str,
) -> Result<PrioritizationPublicProposal, ComhairleError> {
    let reviewed = step
        .proposals
        .iter()
        .find(|p| p.proposal.id == *proposal_id)
        .ok_or_else(|| ComhairleError::ResourceNotFound(format!("Proposal {proposal_id}")))?;
    let texts = Texts::load(state, &step.config, locale).await?;
    let answers: Vec<&Response> = reviewed
        .reviews
        .iter()
        .flat_map(|r| r.response.0.iter())
        .collect();

    let questions = step
        .config
        .questions
        .iter()
        .filter_map(|question| {
            question_results(
                question,
                answers
                    .iter()
                    .filter(|a| a.question_id == question.id && a.section_id.is_none())
                    .map(|a| &a.value),
                &texts,
            )
        })
        .collect();

    let sections = reviewed
        .sections
        .iter()
        .map(|section| PrioritizationPublicSection {
            position: Count(u64::try_from(section.position).unwrap_or(0)),
            body: FacilitatorText::from_facilitator(section.body.clone()),
            questions: step
                .config
                .section_questions
                .iter()
                .filter_map(|question| {
                    question_results(
                        question,
                        answers
                            .iter()
                            .filter(|a| {
                                a.question_id == question.id && a.section_id == Some(section.id)
                            })
                            .map(|a| &a.value),
                        &texts,
                    )
                })
                .collect(),
        })
        .collect();

    Ok(PrioritizationPublicProposal {
        title: FacilitatorText::from_facilitator(reviewed.proposal.title.clone()),
        reviews: Count(reviewed.reviews.len() as u64),
        questions,
        sections,
    })
}

async fn themes(
    state: &ComhairleState,
    workflow_step_id: &Uuid,
) -> Result<PrioritizationPublicThemes, ComhairleError> {
    let collection = latest_theme_collection(&state.db, workflow_step_id, THEME_ORIGIN)
        .await?
        .unwrap_or_default();
    Ok(PrioritizationPublicThemes {
        themes: collection.iter().map(public_theme).collect(),
    })
}

/// One question's answers as counts. `None` for free-text questions, which can't go public.
fn question_results<'a>(
    question: &Question,
    answers: impl Iterator<Item = &'a ResponseValue>,
    texts: &Texts,
) -> Option<PrioritizationPublicQuestion> {
    let numbers: Vec<f64> = answers.filter_map(number).collect();
    match &question.r#type {
        QuestionType::Text => None,
        QuestionType::LikertScale { categories } => Some(PrioritizationPublicQuestion {
            question: texts.get(&question.text),
            answered: Count(numbers.len() as u64),
            choices: categories
                .iter()
                .map(|category| PrioritizationPublicChoice {
                    label: texts.get(&category.label),
                    count: Count(numbers.iter().filter(|v| **v == category.value).count() as u64),
                })
                .collect(),
            scale: None,
        }),
        QuestionType::Continuous {
            sub_steps,
            min_value,
            max_value,
            min_label,
            max_label,
        } => {
            let steps = usize::try_from(*sub_steps).unwrap_or(0).max(1);
            let width = (max_value - min_value) / steps as f64;
            let mut counts = vec![0u64; steps];
            for value in &numbers {
                let bucket = if width > 0.0 {
                    ((value - min_value) / width).floor()
                } else {
                    0.0
                };
                let bucket = bucket.clamp(0.0, (steps - 1) as f64) as usize;
                counts[bucket] += 1;
            }
            Some(PrioritizationPublicQuestion {
                question: texts.get(&question.text),
                answered: Count(numbers.len() as u64),
                choices: vec![],
                scale: Some(PrioritizationPublicScale {
                    min: Score(*min_value),
                    max: Score(*max_value),
                    min_label: texts.get(min_label),
                    max_label: texts.get(max_label),
                    buckets: counts
                        .into_iter()
                        .enumerate()
                        .map(|(i, count)| PrioritizationPublicBucket {
                            from: Score(min_value + i as f64 * width),
                            count: Count(count),
                        })
                        .collect(),
                }),
            })
        }
    }
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

fn number(value: &ResponseValue) -> Option<f64> {
    match value {
        ResponseValue::Number(v) => Some(*v),
        ResponseValue::Text(_) => None,
    }
}

/// The step's question texts and labels in one language.
struct Texts(std::collections::HashMap<TextContentId, String>);

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
