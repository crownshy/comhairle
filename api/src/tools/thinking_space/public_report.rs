//! Public report data for the Thinking Space section blocks (ADR-0061).
//!
//! Only aggregates leave here. A participant's answers and summary are shared with the
//! organisers under the step's consent switch, never with the report's readers.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use aide::axum::ApiRouter;
use aide::axum::routing::get_with;
use axum::extract::{Json, Query, State};
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
use crate::models::users::User;
use crate::routes::auth::RequiredUser;
use crate::tools::public_report_data::anonymous::{
    Count, FacilitatorText, PublicReportData, Score, public_report_data,
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

public_report_data! {
    pub struct ThinkingSpacePublicKeyStats {
        /// People who finished and agreed to share their summary with the organisers.
        pub participants: Count,
        pub follow_ups_per_participant: Score,
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
    Query(PublicReportQuery { workflow_step_id }): Query<PublicReportQuery>,
) -> Result<Json<ThinkingSpacePublicQuestions>, ComhairleError> {
    serve_public(&state, workflow_step_id, QUESTIONS, build_questions).await
}

#[instrument(err(Debug), skip(state))]
async fn preview_questions(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Query(PublicReportQuery { workflow_step_id }): Query<PublicReportQuery>,
) -> Result<Json<ThinkingSpacePublicQuestions>, ComhairleError> {
    serve_preview(&state, &user, workflow_step_id, build_questions).await
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
    build: impl AsyncFnOnce(&ComhairleState, &Uuid) -> Result<Data, ComhairleError>,
) -> Result<Json<Data>, ComhairleError> {
    let data = serve_public_report_data::<ThinkingSpaceTool, _>(
        &state.db,
        &workflow_step_id,
        section_block,
        None,
        async |_: ThinkingSpaceToolConfig| build(state, &workflow_step_id).await,
    )
    .await?;
    Ok(Json(data))
}

async fn serve_preview<Data: PublicReportData>(
    state: &Arc<ComhairleState>,
    user: &User,
    workflow_step_id: Uuid,
    build: impl AsyncFnOnce(&ComhairleState, &Uuid) -> Result<Data, ComhairleError>,
) -> Result<Json<Data>, ComhairleError> {
    let data = serve_report_preview_data::<ThinkingSpaceTool, _>(
        state,
        user,
        &workflow_step_id,
        async |_: ThinkingSpaceToolConfig| build(state, &workflow_step_id).await,
    )
    .await?;
    Ok(Json(data))
}

async fn build_key_stats(
    state: &ComhairleState,
    workflow_step_id: &Uuid,
) -> Result<ThinkingSpacePublicKeyStats, ComhairleError> {
    Ok(key_stats(
        &load_shared_answers(state, workflow_step_id).await?,
    ))
}

async fn build_questions(
    state: &ComhairleState,
    workflow_step_id: &Uuid,
) -> Result<ThinkingSpacePublicQuestions, ComhairleError> {
    Ok(questions(
        &load_shared_answers(state, workflow_step_id).await?,
    ))
}

async fn build_themes(
    state: &ComhairleState,
    workflow_step_id: &Uuid,
) -> Result<ThinkingSpacePublicThemes, ComhairleError> {
    let collection = latest_theme_collection(&state.db, workflow_step_id, THEME_ORIGIN)
        .await?
        .unwrap_or_default();
    Ok(ThinkingSpacePublicThemes {
        themes: collection.iter().map(public_theme).collect(),
    })
}

/// The answers of everyone who agreed to share, grouped the way the admin Insights page
/// groups them: opening answers per person, follow-ups per opening answer.
struct SharedAnswers {
    participants: usize,
    roots: Vec<ThinkingSpaceAnswer>,
    follow_ups_by_root: HashMap<Uuid, usize>,
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
    let mut roots: Vec<ThinkingSpaceAnswer> = vec![];
    let mut follow_ups_by_root: HashMap<Uuid, usize> = HashMap::new();
    for answer in answers {
        if !sharing.contains(&answer.user_id) {
            continue;
        }
        match answer.root_question_id {
            Some(root) => *follow_ups_by_root.entry(root).or_default() += 1,
            None => roots.push(answer),
        }
    }
    roots.sort_by_key(|root| root.created_at);

    Ok(SharedAnswers {
        participants: sharing.len(),
        roots,
        follow_ups_by_root,
    })
}

fn key_stats(answers: &SharedAnswers) -> ThinkingSpacePublicKeyStats {
    let follow_ups: usize = answers
        .roots
        .iter()
        .map(|root| {
            answers
                .follow_ups_by_root
                .get(&root.id)
                .copied()
                .unwrap_or(0)
        })
        .sum();
    let per_participant = if answers.participants == 0 {
        0.0
    } else {
        follow_ups as f64 / answers.participants as f64
    };
    ThinkingSpacePublicKeyStats {
        participants: Count(answers.participants as u64),
        follow_ups_per_participant: Score(per_participant),
    }
}

/// Opening questions in the order people first met them. The question text is the
/// facilitator's, copied onto each opening answer when it was asked.
fn questions(answers: &SharedAnswers) -> ThinkingSpacePublicQuestions {
    let mut order: Vec<String> = vec![];
    let mut people: HashMap<&str, HashSet<Uuid>> = HashMap::new();
    let mut follow_ups: HashMap<&str, usize> = HashMap::new();
    for root in &answers.roots {
        if !people.contains_key(root.question.as_str()) {
            order.push(root.question.clone());
        }
        people
            .entry(root.question.as_str())
            .or_default()
            .insert(root.user_id);
        *follow_ups.entry(root.question.as_str()).or_default() += answers
            .follow_ups_by_root
            .get(&root.id)
            .copied()
            .unwrap_or(0);
    }
    ThinkingSpacePublicQuestions {
        questions: order
            .iter()
            .map(|question| ThinkingSpacePublicQuestion {
                participants: Count(people[question.as_str()].len() as u64),
                follow_ups: Count(follow_ups[question.as_str()] as u64),
                question: FacilitatorText::from_facilitator(question.clone()),
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

    #[test]
    fn counts_people_and_follow_ups_per_opening_question() {
        let (ann, bob) = (Uuid::new_v4(), Uuid::new_v4());
        let ann_transport = answer(ann, None, "Transport?");
        let bob_transport = answer(bob, None, "Transport?");
        let ann_housing = answer(ann, None, "Housing?");
        let follow_ups_by_root = HashMap::from([(ann_transport.id, 2), (bob_transport.id, 1)]);
        let answers = SharedAnswers {
            participants: 2,
            roots: vec![ann_transport, bob_transport, ann_housing],
            follow_ups_by_root,
        };

        let stats = key_stats(&answers);
        assert_eq!(stats.participants, Count(2));
        assert_eq!(stats.follow_ups_per_participant, Score(1.5));

        let questions = questions(&answers).questions;
        assert_eq!(questions.len(), 2);
        assert_eq!(questions[0].participants, Count(2));
        assert_eq!(questions[0].follow_ups, Count(3));
        assert_eq!(questions[1].participants, Count(1));
        assert_eq!(questions[1].follow_ups, Count(0));
        let json = serde_json::to_value(&questions).unwrap().to_string();
        assert!(json.contains("Transport?"));
        assert!(!json.contains("private answer"));
    }
}
