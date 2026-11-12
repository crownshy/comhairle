//! Public report data for the Thinking Space section blocks (ADR-0061).
//!
//! Only aggregates leave here. A participant's answers and summary are shared with the
//! organisers under the step's consent switch, never with the report's readers.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, LazyLock};
use std::time::Duration;

use aide::axum::ApiRouter;
use aide::axum::routing::get_with;
use axum::extract::{Json, Query, State};
use moka::future::Cache;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use super::{ThinkingSpaceTool, ThinkingSpaceToolConfig};
use crate::ComhairleState;
use crate::error::ComhairleError;
use crate::models::thinking_space_answer::{
    self, ThinkingSpaceAnswer, ThinkingSpaceAnswerFilterOptions,
};
use crate::models::thinking_space_summary::{self, ThinkingSpaceSummaryFilterOptions};
use crate::models::translations::{get_text_translations_by_content_id, localize_translations};
use crate::models::users::User;
use crate::routes::auth::RequiredUser;
use crate::routes::translations::LocaleExtractor;
use crate::tools::public_report_data::anonymous::{
    Count, FacilitatorText, MIN_ANSWERS_SHOWN, PublicReportData, Score, public_report_data,
};
use crate::tools::public_report_data::themes::{
    PublicTheme, latest_theme_collection, public_theme,
};
use crate::tools::public_report_data::{serve_public_report_data, serve_report_preview_data};

/// Section block types, as the report stores them (`lib/reports/embeds.ts`).
const KEY_STATS: &str = "thinkingspace-key-stats";
const QUESTIONS: &str = "thinkingspace-questions";
const THEMES: &str = "thinkingspace-themes";

/// Where the theme collections of a Thinking Space step come from (`sensemaking::harvest`).
const THEME_ORIGIN: &str = "thinking_space";

/// How long a step's shared answers and themes are served before reading them again, so a
/// newly shared summary can take up to a minute to reach the report.
const CONTEXT_FRESH_FOR: Duration = Duration::from_secs(60);

/// Upper bound on cached steps; well above anything one deployment hosts.
const CONTEXT_CACHE_CAPACITY: u64 = 1_000;

static CONTEXT_CACHE: LazyLock<Cache<Uuid, Arc<StepContext>>> = LazyLock::new(|| {
    Cache::builder()
        .max_capacity(CONTEXT_CACHE_CAPACITY)
        .time_to_live(CONTEXT_FRESH_FOR)
        .build()
});

public_report_data! {
    pub struct ThinkingSpacePublicKeyStats {
        /// People who finished and agreed to share their summary with the organisers.
        pub participants: Count,
        /// `None` when fewer than `MIN_ANSWERS_SHOWN` people shared, so one person's count
        /// can't be read back.
        pub follow_ups_per_participant: Option<Score>,
    }
}

public_report_data! {
    /// One of the facilitator's opening questions and how far people took it.
    pub struct ThinkingSpacePublicQuestion {
        pub question: FacilitatorText,
        pub participants: Count,
        pub follow_ups: Count,
    }
}

public_report_data! {
    pub struct ThinkingSpacePublicQuestions {
        pub questions: Vec<ThinkingSpacePublicQuestion>,
    }
}

public_report_data! {
    pub struct ThinkingSpacePublicThemes {
        pub themes: Vec<PublicTheme>,
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct PublicReportQuery {
    pub workflow_step_id: Uuid,
}

pub fn routes() -> ApiRouter<Arc<ComhairleState>> {
    ApiRouter::new()
        .api_route(
            "/thinking_space/public_report/key_stats",
            get_with(get_key_stats, |op| {
                op.id("ThinkingSpaceGetPublicKeyStats")
                    .tag("Tools")
                    .summary("Key stats for a published report")
                    .description(
                        "How many people shared a Thinking Space summary and how many follow-up \
                         questions they took on average. 404 unless the conversation's \
                         published report embeds this section block for the step.",
                    )
                    .response::<200, Json<ThinkingSpacePublicKeyStats>>()
            }),
        )
        .api_route(
            "/thinking_space/report_preview/key_stats",
            get_with(preview_key_stats, |op| {
                op.id("ThinkingSpacePreviewKeyStats")
                    .tag("Tools")
                    .summary("Key stats as the published report will show them")
                    .description(
                        "The same data as ThinkingSpaceGetPublicKeyStats, for facilitators who \
                         can update the conversation, before the report is published.",
                    )
                    .response::<200, Json<ThinkingSpacePublicKeyStats>>()
            }),
        )
        .api_route(
            "/thinking_space/public_report/questions",
            get_with(get_questions, |op| {
                op.id("ThinkingSpaceGetPublicQuestions")
                    .tag("Tools")
                    .summary("Opening questions for a published report")
                    .description(
                        "Each opening question with how many people answered it and how many \
                         follow-ups they took. 404 unless the conversation's published report \
                         embeds this section block for the step.",
                    )
                    .response::<200, Json<ThinkingSpacePublicQuestions>>()
            }),
        )
        .api_route(
            "/thinking_space/report_preview/questions",
            get_with(preview_questions, |op| {
                op.id("ThinkingSpacePreviewQuestions")
                    .tag("Tools")
                    .summary("Opening questions as the published report will show them")
                    .description(
                        "The same data as ThinkingSpaceGetPublicQuestions, for facilitators \
                         who can update the conversation, before the report is published.",
                    )
                    .response::<200, Json<ThinkingSpacePublicQuestions>>()
            }),
        )
        .api_route(
            "/thinking_space/public_report/themes",
            get_with(get_themes, |op| {
                op.id("ThinkingSpaceGetPublicThemes")
                    .tag("Tools")
                    .summary("Themes for a published report")
                    .description(
                        "The step's latest themes with how many shared summaries each groups. \
                         404 unless the conversation's published report embeds this section \
                         block for the step.",
                    )
                    .response::<200, Json<ThinkingSpacePublicThemes>>()
            }),
        )
        .api_route(
            "/thinking_space/report_preview/themes",
            get_with(preview_themes, |op| {
                op.id("ThinkingSpacePreviewThemes")
                    .tag("Tools")
                    .summary("Themes as the published report will show them")
                    .description(
                        "The same data as ThinkingSpaceGetPublicThemes, for facilitators who \
                         can update the conversation, before the report is published.",
                    )
                    .response::<200, Json<ThinkingSpacePublicThemes>>()
            }),
        )
}

#[instrument(err(Debug), skip(state))]
async fn get_key_stats(
    State(state): State<Arc<ComhairleState>>,
    Query(PublicReportQuery { workflow_step_id }): Query<PublicReportQuery>,
) -> Result<Json<ThinkingSpacePublicKeyStats>, ComhairleError> {
    serve_public(&state, workflow_step_id, KEY_STATS, build_key_stats).await
}

#[instrument(err(Debug), skip(state))]
async fn preview_key_stats(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Query(PublicReportQuery { workflow_step_id }): Query<PublicReportQuery>,
) -> Result<Json<ThinkingSpacePublicKeyStats>, ComhairleError> {
    serve_preview(&state, &user, workflow_step_id, build_key_stats).await
}

#[instrument(err(Debug), skip(state))]
async fn get_questions(
    State(state): State<Arc<ComhairleState>>,
    LocaleExtractor(locale): LocaleExtractor,
    Query(PublicReportQuery { workflow_step_id }): Query<PublicReportQuery>,
) -> Result<Json<ThinkingSpacePublicQuestions>, ComhairleError> {
    serve_public(
        &state,
        workflow_step_id,
        QUESTIONS,
        async |state, id, config| build_questions(state, id, config, &locale).await,
    )
    .await
}

#[instrument(err(Debug), skip(state))]
async fn preview_questions(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    LocaleExtractor(locale): LocaleExtractor,
    Query(PublicReportQuery { workflow_step_id }): Query<PublicReportQuery>,
) -> Result<Json<ThinkingSpacePublicQuestions>, ComhairleError> {
    serve_preview(
        &state,
        &user,
        workflow_step_id,
        async |state, id, config| build_questions(state, id, config, &locale).await,
    )
    .await
}

#[instrument(err(Debug), skip(state))]
async fn get_themes(
    State(state): State<Arc<ComhairleState>>,
    Query(PublicReportQuery { workflow_step_id }): Query<PublicReportQuery>,
) -> Result<Json<ThinkingSpacePublicThemes>, ComhairleError> {
    serve_public(&state, workflow_step_id, THEMES, build_themes).await
}

#[instrument(err(Debug), skip(state))]
async fn preview_themes(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Query(PublicReportQuery { workflow_step_id }): Query<PublicReportQuery>,
) -> Result<Json<ThinkingSpacePublicThemes>, ComhairleError> {
    serve_preview(&state, &user, workflow_step_id, build_themes).await
}

async fn serve_public<Data: PublicReportData>(
    state: &ComhairleState,
    workflow_step_id: Uuid,
    section_block: &str,
    build: impl AsyncFnOnce(
        &ComhairleState,
        &Uuid,
        ThinkingSpaceToolConfig,
    ) -> Result<Data, ComhairleError>,
) -> Result<Json<Data>, ComhairleError> {
    let data = serve_public_report_data::<ThinkingSpaceTool, _>(
        &state.db,
        &workflow_step_id,
        section_block,
        None,
        async |config: ThinkingSpaceToolConfig| build(state, &workflow_step_id, config).await,
    )
    .await?;
    Ok(Json(data))
}

async fn serve_preview<Data: PublicReportData>(
    state: &Arc<ComhairleState>,
    user: &User,
    workflow_step_id: Uuid,
    build: impl AsyncFnOnce(
        &ComhairleState,
        &Uuid,
        ThinkingSpaceToolConfig,
    ) -> Result<Data, ComhairleError>,
) -> Result<Json<Data>, ComhairleError> {
    let data = serve_report_preview_data::<ThinkingSpaceTool, _>(
        state,
        user,
        &workflow_step_id,
        async |config: ThinkingSpaceToolConfig| build(state, &workflow_step_id, config).await,
    )
    .await?;
    Ok(Json(data))
}

async fn build_key_stats(
    state: &ComhairleState,
    workflow_step_id: &Uuid,
    _: ThinkingSpaceToolConfig,
) -> Result<ThinkingSpacePublicKeyStats, ComhairleError> {
    Ok(key_stats(
        &step_context(state, workflow_step_id).await?.answers,
    ))
}

async fn build_questions(
    state: &ComhairleState,
    workflow_step_id: &Uuid,
    config: ThinkingSpaceToolConfig,
    locale: &str,
) -> Result<ThinkingSpacePublicQuestions, ComhairleError> {
    let context = step_context(state, workflow_step_id).await?;
    let opening_questions = OpeningQuestion::load(state, &config, locale).await?;
    Ok(questions(&context.answers, &opening_questions))
}

async fn build_themes(
    state: &ComhairleState,
    workflow_step_id: &Uuid,
    _: ThinkingSpaceToolConfig,
) -> Result<ThinkingSpacePublicThemes, ComhairleError> {
    Ok(ThinkingSpacePublicThemes {
        themes: step_context(state, workflow_step_id).await?.themes.clone(),
    })
}

/// What every section block of a step reads. Shared through `CONTEXT_CACHE`.
struct StepContext {
    answers: SharedAnswers,
    themes: Vec<PublicTheme>,
}

/// Requests that miss at the same time wait on one load.
async fn step_context(
    state: &ComhairleState,
    workflow_step_id: &Uuid,
) -> Result<Arc<StepContext>, ComhairleError> {
    CONTEXT_CACHE
        .try_get_with(*workflow_step_id, async {
            let answers = load_shared_answers(state, workflow_step_id).await?;
            let themes = latest_theme_collection(&state.db, workflow_step_id, THEME_ORIGIN)
                .await?
                .unwrap_or_default()
                .iter()
                .map(public_theme)
                .collect();
            Ok::<_, ComhairleError>(Arc::new(StepContext { answers, themes }))
        })
        .await
        .map_err(|error: Arc<ComhairleError>| {
            ComhairleError::ReportDataUnavailable(error.to_string())
        })
}

/// The answers of everyone who agreed to share, grouped the way the admin Insights page
/// groups them: opening answers per person, follow-ups per opening answer.
struct SharedAnswers {
    participants: usize,
    opening_answers: Vec<ThinkingSpaceAnswer>,
    follow_ups_by_opening_answer: HashMap<Uuid, usize>,
}

impl SharedAnswers {
    fn follow_ups_for(&self, opening_answer: &ThinkingSpaceAnswer) -> usize {
        self.follow_ups_by_opening_answer
            .get(&opening_answer.id)
            .copied()
            .unwrap_or(0)
    }
}

async fn load_shared_answers(
    state: &ComhairleState,
    workflow_step_id: &Uuid,
) -> Result<SharedAnswers, ComhairleError> {
    let summaries = thinking_space_summary::list(
        &state.db,
        workflow_step_id,
        ThinkingSpaceSummaryFilterOptions {
            is_shared_with_organizer: Some(true),
            ..Default::default()
        },
    )
    .await?;
    let sharing: HashSet<Uuid> = summaries.into_iter().map(|s| s.user_id).collect();

    let answers = thinking_space_answer::list(
        &state.db,
        workflow_step_id,
        ThinkingSpaceAnswerFilterOptions::default(),
    )
    .await?;
    let mut opening_answers: Vec<ThinkingSpaceAnswer> = vec![];
    let mut follow_ups_by_opening_answer: HashMap<Uuid, usize> = HashMap::new();
    for answer in answers {
        if !sharing.contains(&answer.user_id) {
            continue;
        }
        match answer.root_question_id {
            Some(root) => *follow_ups_by_opening_answer.entry(root).or_default() += 1,
            None => opening_answers.push(answer),
        }
    }

    Ok(SharedAnswers {
        participants: sharing.len(),
        opening_answers,
        follow_ups_by_opening_answer,
    })
}

fn key_stats(answers: &SharedAnswers) -> ThinkingSpacePublicKeyStats {
    let follow_ups: usize = answers
        .opening_answers
        .iter()
        .map(|opening_answer| answers.follow_ups_for(opening_answer))
        .sum();
    let enough_participants = answers.participants >= MIN_ANSWERS_SHOWN;
    ThinkingSpacePublicKeyStats {
        participants: Count(answers.participants as u64),
        follow_ups_per_participant: enough_participants
            .then(|| Score(follow_ups as f64 / answers.participants as f64)),
    }
}

/// One of the step's configured opening questions. Answers store the question as the
/// participant saw it, in their language, so `wordings` holds every translation to match on.
struct OpeningQuestion {
    text: String,
    wordings: HashSet<String>,
}

impl OpeningQuestion {
    async fn load(
        state: &ComhairleState,
        config: &ThinkingSpaceToolConfig,
        locale: &str,
    ) -> Result<Vec<Self>, ComhairleError> {
        let ids: Vec<_> = config.root_questions.iter().map(|q| q.text).collect();
        let localized = localize_translations(&state.db, &ids, locale).await?;
        let mut questions = vec![];
        for id in ids {
            let wordings = get_text_translations_by_content_id(&state.db, &id)
                .await?
                .into_iter()
                .map(|translation| translation.content)
                .collect();
            questions.push(Self {
                text: localized.get(&id).cloned().unwrap_or_default(),
                wordings,
            });
        }
        Ok(questions)
    }
}

/// Opening questions in the order the step asks them. Only the configured wording is
/// published: an answer's stored question comes from the participant's request, so one that
/// matches no configured question is left out rather than shown.
fn questions(
    answers: &SharedAnswers,
    opening_questions: &[OpeningQuestion],
) -> ThinkingSpacePublicQuestions {
    ThinkingSpacePublicQuestions {
        questions: opening_questions
            .iter()
            .map(|opening_question| {
                let matching: Vec<&ThinkingSpaceAnswer> = answers
                    .opening_answers
                    .iter()
                    .filter(|answer| opening_question.wordings.contains(&answer.question))
                    .collect();
                let participants: HashSet<Uuid> =
                    matching.iter().map(|answer| answer.user_id).collect();
                let follow_ups: usize = matching
                    .iter()
                    .map(|answer| answers.follow_ups_for(answer))
                    .sum();
                ThinkingSpacePublicQuestion {
                    question: FacilitatorText::from_facilitator(opening_question.text.clone()),
                    participants: Count(participants.len() as u64),
                    follow_ups: Count(follow_ups as u64),
                }
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;
    use crate::models::thinking_space_answer::AnswerStatus;

    fn answer(user: Uuid, root: Option<Uuid>, question: &str) -> ThinkingSpaceAnswer {
        ThinkingSpaceAnswer {
            id: Uuid::new_v4(),
            workflow_step_id: Uuid::new_v4(),
            user_id: user,
            root_question_id: root,
            is_follow_up: root.is_some(),
            question: question.into(),
            answer: "a private answer".into(),
            other_questions: vec![],
            status: AnswerStatus::Approved,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn opening_question(text: &str, other_wordings: &[&str]) -> OpeningQuestion {
        OpeningQuestion {
            text: text.into(),
            wordings: std::iter::once(text)
                .chain(other_wordings.iter().copied())
                .map(String::from)
                .collect(),
        }
    }

    #[test]
    fn counts_people_and_follow_ups_per_opening_question() {
        let (ann, bob) = (Uuid::new_v4(), Uuid::new_v4());
        let ann_transport = answer(ann, None, "Transport?");
        let bob_transport = answer(bob, None, "Còmhdhail?");
        let ann_housing = answer(ann, None, "Housing?");
        let follow_ups_by_opening_answer =
            HashMap::from([(ann_transport.id, 2), (bob_transport.id, 1)]);
        let answers = SharedAnswers {
            participants: 2,
            opening_answers: vec![ann_transport, bob_transport, ann_housing],
            follow_ups_by_opening_answer,
        };

        let stats = key_stats(&answers);
        assert_eq!(stats.participants, Count(2));
        assert_eq!(stats.follow_ups_per_participant, None);

        let configured = [
            opening_question("Transport?", &["Còmhdhail?"]),
            opening_question("Housing?", &[]),
        ];
        let questions = questions(&answers, &configured).questions;
        assert_eq!(questions.len(), 2);
        assert_eq!(questions[0].participants, Count(2));
        assert_eq!(questions[0].follow_ups, Count(3));
        assert_eq!(questions[1].participants, Count(1));
        assert_eq!(questions[1].follow_ups, Count(0));
        let json = serde_json::to_value(&questions).unwrap().to_string();
        assert!(json.contains("Transport?"));
        assert!(!json.contains("private answer"));
    }

    #[test]
    fn shows_follow_ups_per_participant_from_three_participants() {
        let opening_answers: Vec<_> = (0..3)
            .map(|_| answer(Uuid::new_v4(), None, "Transport?"))
            .collect();
        let follow_ups_by_opening_answer =
            HashMap::from([(opening_answers[0].id, 2), (opening_answers[1].id, 1)]);
        let answers = SharedAnswers {
            participants: 3,
            opening_answers,
            follow_ups_by_opening_answer,
        };

        assert_eq!(
            key_stats(&answers).follow_ups_per_participant,
            Some(Score(1.0))
        );
    }

    #[test]
    fn leaves_out_question_text_the_step_does_not_ask() {
        let answers = SharedAnswers {
            participants: 1,
            opening_answers: vec![answer(Uuid::new_v4(), None, "Text a participant made up")],
            follow_ups_by_opening_answer: HashMap::new(),
        };

        let questions = questions(&answers, &[opening_question("Transport?", &[])]).questions;
        let json = serde_json::to_value(&questions).unwrap().to_string();
        assert!(!json.contains("made up"));
        assert_eq!(questions[0].participants, Count(0));
    }
}
