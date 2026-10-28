//! Public report data for HeyForm questions (ADR-0061).
//!
//! One section block, `heyform-question`, embedded once per question with the question id as
//! the embed's item. Only questions whose answers come from choices or a scale the facilitator
//! set up can go public. Free text, contact details, dates and uploads would carry whatever a
//! participant typed.

use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use aide::axum::ApiRouter;
use aide::axum::routing::get_with;
use axum::extract::{Json, Query, State};
use heyform_sdk::LoginInput;
use heyform_sdk::client::HeyFormClient;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::instrument;
use uuid::Uuid;

use super::{
    HeyFormToolConfig, InsightQuestion, build_survey_insights, fetch_all_submissions,
    heyform_base_url,
};
use crate::ComhairleState;
use crate::error::ComhairleError;
use crate::routes::auth::RequiredUser;
use crate::tools::ToolConfig;
use crate::tools::public_report_data::anonymous::{
    Count, FacilitatorText, ScalePoint, public_report_data,
};
use crate::tools::public_report_data::{
    check_can_update_step, serve_public_report_data, serve_report_data_preview,
};

/// The section block type, as the report stores it (`lib/reports/embeds.ts`).
const QUESTION: &str = "heyform-question";

/// Question kinds answered by picking from the facilitator's choices.
const CHOICE_KINDS: [&str; 3] = ["yes_no", "multiple_choice", "picture_choice"];
/// Question kinds answered with a point on a scale.
const SCALE_KINDS: [&str; 2] = ["rating", "opinion_scale"];

public_report_data! {
    pub struct HeyFormPublicChoice {
        pub label: FacilitatorText,
        pub count: Count,
    }
}

public_report_data! {
    pub struct HeyFormPublicScalePoint {
        pub point: ScalePoint,
        pub count: Count,
    }
}

public_report_data! {
    pub struct HeyFormPublicScale {
        /// The top of the scale, such as 5 on a 1 to 5 rating.
        pub max: ScalePoint,
        pub left_label: Option<FacilitatorText>,
        pub center_label: Option<FacilitatorText>,
        pub right_label: Option<FacilitatorText>,
        /// Only the points someone picked.
        pub points: Vec<HeyFormPublicScalePoint>,
    }
}

public_report_data! {
    /// One question's answers, as counts. `choices` is filled for choice questions and
    /// `scale` for rating and opinion scale questions.
    pub struct HeyFormPublicQuestion {
        pub title: FacilitatorText,
        /// The HeyForm question kind, such as `multiple_choice` or `rating`.
        pub kind: &'static str,
        pub answered: Count,
        pub choices: Vec<HeyFormPublicChoice>,
        pub scale: Option<HeyFormPublicScale>,
    }
}

/// A question the report could embed, for the add-chart dialog.
#[derive(Serialize, Debug, JsonSchema)]
pub struct HeyFormEmbeddableQuestion {
    pub id: String,
    pub title: String,
    pub kind: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct QuestionQuery {
    workflow_step_id: Uuid,
    question_id: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct StepQuery {
    workflow_step_id: Uuid,
}

pub fn routes() -> ApiRouter<Arc<ComhairleState>> {
    ApiRouter::new()
        .api_route(
            "/survey_tool/public_report/question",
            get_with(get_question, |op| {
                op.id("HeyFormGetPublicQuestion")
                    .tag("Tools")
                    .summary("One question's answers for a published report")
                    .description(
                        "Counts per choice or scale point for a choice, rating or opinion \
                         scale question. 404 unless the conversation's published report \
                         embeds this question.",
                    )
                    .response::<200, Json<HeyFormPublicQuestion>>()
            }),
        )
        .api_route(
            "/survey_tool/report_preview/question",
            get_with(preview_question, |op| {
                op.id("HeyFormPreviewQuestion")
                    .tag("Tools")
                    .summary("One question's answers as the published report will show them")
                    .description(
                        "The same data as HeyFormGetPublicQuestion, for facilitators who can \
                         update the conversation, before the report is published.",
                    )
                    .response::<200, Json<HeyFormPublicQuestion>>()
            }),
        )
        .api_route(
            "/survey_tool/report_preview/questions",
            get_with(embeddable_questions, |op| {
                op.id("HeyFormListEmbeddableQuestions")
                    .tag("Tools")
                    .summary("Questions the report can embed")
                    .description(
                        "The step's choice, rating and opinion scale questions, for \
                         facilitators who can update the conversation.",
                    )
                    .response::<200, Json<Vec<HeyFormEmbeddableQuestion>>>()
            }),
        )
}

#[instrument(err(Debug), skip(state))]
async fn get_question(
    State(state): State<Arc<ComhairleState>>,
    Query(QuestionQuery {
        workflow_step_id,
        question_id,
    }): Query<QuestionQuery>,
) -> Result<Json<HeyFormPublicQuestion>, ComhairleError> {
    let question = serve_public_report_data(
        &state.db,
        &workflow_step_id,
        QUESTION,
        Some(&question_id),
        heyform_config,
        async |config| public_question(&config, &question_id).await,
    )
    .await?;
    Ok(Json(question))
}

#[instrument(err(Debug), skip(state))]
async fn preview_question(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Query(QuestionQuery {
        workflow_step_id,
        question_id,
    }): Query<QuestionQuery>,
) -> Result<Json<HeyFormPublicQuestion>, ComhairleError> {
    let question = serve_report_data_preview(
        &state,
        &user,
        &workflow_step_id,
        heyform_config,
        async |config| public_question(&config, &question_id).await,
    )
    .await?;
    Ok(Json(question))
}

#[instrument(err(Debug), skip(state))]
async fn embeddable_questions(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Query(StepQuery { workflow_step_id }): Query<StepQuery>,
) -> Result<Json<Vec<HeyFormEmbeddableQuestion>>, ComhairleError> {
    let step = check_can_update_step(&state, &user, &workflow_step_id).await?;
    let config = heyform_config(step.tool_config.unwrap_or(step.preview_tool_config))
        .ok_or_else(|| ComhairleError::WorkflowStepHasWrongType("HeyForm".into()))?;
    let questions = load_insights(&config)
        .await?
        .into_iter()
        .filter_map(|question| {
            let kind = public_kind(question.kind.as_deref())?;
            Some(HeyFormEmbeddableQuestion {
                id: question.id,
                title: question.title,
                kind: kind.to_owned(),
            })
        })
        .collect();
    Ok(Json(questions))
}

fn heyform_config(config: ToolConfig) -> Option<HeyFormToolConfig> {
    match config {
        ToolConfig::HeyForm(heyform) => Some(heyform),
        _ => None,
    }
}

/// The same per-question breakdown the admin Insights tab shows, before it's cut down.
async fn load_insights(config: &HeyFormToolConfig) -> Result<Vec<InsightQuestion>, ComhairleError> {
    let client = HeyFormClient::new(heyform_base_url(&config.server_url))?;
    client
        .login(LoginInput {
            email: config.admin_user.clone(),
            password: config.admin_password.clone(),
        })
        .await?;
    let (form, report, submissions) = tokio::try_join!(
        client.get_form(&config.survey_id),
        client.get_form_report(config.survey_id.clone()),
        fetch_all_submissions(&client, &config.survey_id, "inbox"),
    )?;
    Ok(build_survey_insights(&form, &report, &submissions).questions)
}

async fn public_question(
    config: &HeyFormToolConfig,
    question_id: &str,
) -> Result<HeyFormPublicQuestion, ComhairleError> {
    let not_found = || ComhairleError::ResourceNotFound(format!("Question {question_id}"));
    let question = load_insights(config)
        .await?
        .into_iter()
        .find(|question| question.id == question_id)
        .ok_or_else(not_found)?;
    to_public_question(&question).ok_or_else(not_found)
}

/// The allow-listed kind, as a `&'static str` so it can go out as a fixed label.
fn public_kind(kind: Option<&str>) -> Option<&'static str> {
    let kind = kind?;
    CHOICE_KINDS
        .into_iter()
        .chain(SCALE_KINDS)
        .find(|allowed| *allowed == kind)
}

/// `None` for kinds that can't go public.
fn to_public_question(question: &InsightQuestion) -> Option<HeyFormPublicQuestion> {
    let kind = public_kind(question.kind.as_deref())?;
    let is_scale = SCALE_KINDS.contains(&kind);
    Some(HeyFormPublicQuestion {
        title: FacilitatorText::from_facilitator(question.title.clone()),
        kind,
        answered: Count(question.answered.into()),
        choices: if is_scale {
            vec![]
        } else {
            choice_counts(question)
        },
        scale: is_scale.then(|| scale_counts(question)),
    })
}

/// Counts for the choices the facilitator set up, in their order. The report can carry other
/// labels, such as an "Other" a participant typed, so those are left out. Yes/no questions have
/// no choices of their own in the form, so their report labels are HeyForm's own.
fn choice_counts(question: &InsightQuestion) -> Vec<HeyFormPublicChoice> {
    let reported = question.choices.as_deref().unwrap_or_default();
    let defined = defined_choices(question.properties.as_ref());
    if defined.is_empty() && question.kind.as_deref() == Some("yes_no") {
        return reported
            .iter()
            .map(|choice| HeyFormPublicChoice {
                label: FacilitatorText::from_facilitator(choice.label.clone()),
                count: Count(choice.count.max(0) as u64),
            })
            .collect();
    }
    defined
        .into_iter()
        .map(|(id, label)| HeyFormPublicChoice {
            label: FacilitatorText::from_facilitator(label),
            count: Count(
                reported
                    .iter()
                    .find(|choice| choice.id == id)
                    .map_or(0, |choice| choice.count.max(0) as u64),
            ),
        })
        .collect()
}

fn defined_choices(properties: Option<&HashMap<String, Value>>) -> Vec<(String, String)> {
    properties
        .and_then(|properties| properties.get("choices"))
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|choice| {
            let id = choice.get("id")?.as_str()?.to_owned();
            let label = choice.get("label")?.as_str()?.to_owned();
            Some((id, label))
        })
        .collect()
}

/// How many picked each point. Answers that aren't a whole number on the scale are skipped.
fn scale_counts(question: &InsightQuestion) -> HeyFormPublicScale {
    let mut counts: BTreeMap<u32, u64> = BTreeMap::new();
    for submission in question.submissions.as_deref().unwrap_or_default() {
        if let Some(point) = scale_point(&submission.value) {
            *counts.entry(point).or_default() += 1;
        }
    }
    let properties = question.properties.as_ref();
    let max = properties
        .and_then(|properties| properties.get("total"))
        .and_then(Value::as_u64)
        .and_then(|total| u32::try_from(total).ok())
        .or_else(|| counts.keys().max().copied())
        .unwrap_or(0);
    let label = |key: &str| {
        properties
            .and_then(|properties| properties.get(key))
            .and_then(Value::as_str)
            .filter(|label| !label.is_empty())
            .map(|label| FacilitatorText::from_facilitator(label.to_owned()))
    };
    HeyFormPublicScale {
        max: ScalePoint(max),
        left_label: label("leftLabel"),
        center_label: label("centerLabel"),
        right_label: label("rightLabel"),
        points: counts
            .into_iter()
            .filter(|(point, _)| *point <= max)
            .map(|(point, count)| HeyFormPublicScalePoint {
                point: ScalePoint(point),
                count: Count(count),
            })
            .collect(),
    }
}

fn scale_point(value: &Value) -> Option<u32> {
    let point = match value {
        Value::Number(number) => number.as_u64()?,
        Value::String(text) => text.trim().parse().ok()?,
        _ => return None,
    };
    u32::try_from(point).ok()
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::tools::heyform::{InsightChoice, InsightSubmission};

    fn question(kind: &str, properties: Value) -> InsightQuestion {
        InsightQuestion {
            id: "q1".into(),
            kind: Some(kind.into()),
            title: "How do you get to school?".into(),
            answered: 3,
            total: 4,
            properties: serde_json::from_value(properties).unwrap(),
            choices: None,
            submissions: None,
        }
    }

    fn answer(value: Value) -> InsightSubmission {
        InsightSubmission {
            submission_id: "s".into(),
            value,
            submitted_at: None,
        }
    }

    #[test]
    fn free_text_and_contact_questions_never_go_public() {
        for kind in [
            "short_text",
            "long_text",
            "email",
            "phone_number",
            "date",
            "file_upload",
        ] {
            assert!(
                to_public_question(&question(kind, json!({}))).is_none(),
                "{kind} went public"
            );
        }
    }

    #[test]
    fn choice_questions_count_only_the_facilitators_choices() {
        let mut bus = question(
            "multiple_choice",
            json!({ "choices": [
                { "id": "a", "label": "Bus" },
                { "id": "b", "label": "Walk" }
            ] }),
        );
        bus.choices = Some(vec![
            InsightChoice {
                id: "a".into(),
                label: "Bus".into(),
                count: 2,
            },
            InsightChoice {
                id: "other".into(),
                label: "my mum's car".into(),
                count: 1,
            },
        ]);

        let public = to_public_question(&bus).unwrap();
        let counts: Vec<_> = public
            .choices
            .iter()
            .map(|c| (c.label.clone(), c.count))
            .collect();
        assert_eq!(
            counts,
            vec![
                (FacilitatorText::from_facilitator("Bus".into()), Count(2)),
                (FacilitatorText::from_facilitator("Walk".into()), Count(0)),
            ]
        );
        assert!(public.scale.is_none());
    }

    #[test]
    fn scale_questions_count_each_point_picked() {
        let mut rating = question("rating", json!({ "total": 5, "leftLabel": "Not at all" }));
        rating.submissions = Some(vec![
            answer(json!(4)),
            answer(json!("4")),
            answer(json!(2)),
            answer(json!(9)),
            answer(json!("not a number")),
        ]);

        let scale = to_public_question(&rating).unwrap().scale.unwrap();
        assert_eq!(scale.max, ScalePoint(5));
        assert_eq!(
            scale.left_label,
            Some(FacilitatorText::from_facilitator("Not at all".into()))
        );
        let points: Vec<_> = scale.points.iter().map(|p| (p.point, p.count)).collect();
        assert_eq!(
            points,
            vec![(ScalePoint(2), Count(1)), (ScalePoint(4), Count(2))]
        );
    }
}
