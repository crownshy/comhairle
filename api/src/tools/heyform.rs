use std::collections::HashMap;
use std::sync::Arc;

use aide::axum::ApiRouter;
use aide::axum::routing::get_with;
use async_trait::async_trait;
use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use heyform_sdk::client::HeyFormClient;
use heyform_sdk::{
    CreateFormInput, CreateHiddenFieldInput, CreateTeamInput, Form, FormField, FormKind,
    FormReport, FormReportResponse, InteractiveMode, LoginInput, Parent, SignUpInput, Submission,
    Submissions,
};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::{Rng, distributions::Alphanumeric};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ComhairleState;
use crate::error::ComhairleError;
use crate::models;

use super::{ToolConfig, ToolConfigSanitize, ToolImpl};

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, PartialEq)]
pub struct HeyFormToolConfig {
    pub survey_id: String,
    pub survey_url: String,
    pub admin_user: String,
    pub admin_password: String,
    pub workspace_id: String,
    pub project_id: String,
    #[serde(default = "default_server_url")]
    pub server_url: String,
}

fn default_server_url() -> String {
    "forms.comhairle.scot".to_string()
}

#[inline(always)]
fn heyform_base_url(server_url: &str) -> String {
    if server_url.starts_with("http://") || server_url.starts_with("https://") {
        return server_url.to_string();
    }

    if cfg!(not(test)) {
        format!("https://{}", server_url)
    } else {
        format!("http://{}", server_url)
    }
}

impl ToolConfigSanitize for HeyFormToolConfig {
    fn sanitize(&self) -> Self {
        Self {
            survey_id: self.survey_id.clone(),
            survey_url: self.survey_url.clone(),
            admin_user: "".into(),
            admin_password: "".into(),
            workspace_id: self.workspace_id.clone(),
            project_id: self.project_id.clone(),
            server_url: self.server_url.clone(),
        }
    }
}
#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct HeyFormToolSetup {
    #[serde(default = "default_server_url")]
    pub server_url: String,
}

#[derive(PartialEq, Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct HeyFormReport;

fn generate_password() -> String {
    let mut rng = thread_rng();

    // Ensure at least one of each required character type
    let lowercase = (b'a'..=b'z').map(char::from).collect::<Vec<_>>();
    let uppercase = (b'A'..=b'Z').map(char::from).collect::<Vec<_>>();
    let numbers = (b'0'..=b'9').map(char::from).collect::<Vec<_>>();

    let mut password = vec![
        *lowercase
            .choose(&mut rng)
            .expect("Failed to choose a lowercase letter"),
        *uppercase
            .choose(&mut rng)
            .expect("Failed to choose an uppercase letter"),
        *numbers.choose(&mut rng).expect("Failed to choose a number"),
    ];

    // Fill remaining 9 characters from all alphanumeric
    let all_chars: Vec<char> = lowercase
        .into_iter()
        .chain(uppercase)
        .chain(numbers)
        .collect();

    for _ in 0..9 {
        password.push(
            *all_chars
                .choose(&mut rng)
                .expect("Failed to choose a character"),
        );
    }

    // Shuffle to avoid predictable pattern
    password.shuffle(&mut rng);
    password.into_iter().collect()
}

pub async fn launch(
    preview_config: &HeyFormToolConfig,
) -> Result<HeyFormToolConfig, ComhairleError> {
    let preview_client = HeyFormClient::new(heyform_base_url(&preview_config.server_url))?;
    let live_client = HeyFormClient::new(heyform_base_url(&preview_config.server_url))?;

    preview_client
        .login(LoginInput {
            email: preview_config.admin_user.clone(),
            password: preview_config.admin_password.clone(),
        })
        .await?;

    let preview_form = preview_client.get_form(&preview_config.survey_id).await?;

    let live_config = heyform_setup(&HeyFormToolSetup {
        server_url: preview_config.server_url.clone(),
    })
    .await?;

    live_client
        .login(LoginInput {
            email: live_config.admin_user.clone(),
            password: live_config.admin_password.clone(),
        })
        .await?;
    // Update newly created survey with form fields from preview survey
    live_client
        .update_poll(&live_config.survey_id, preview_form)
        .await?;

    Ok(live_config)
}

async fn heyform_setup(
    setup_config: &HeyFormToolSetup,
) -> Result<HeyFormToolConfig, ComhairleError> {
    let client = HeyFormClient::new(heyform_base_url(&setup_config.server_url))?;

    let username: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    let email = format!("{username}@comhairle.com");

    let password: String = generate_password();

    let signup_input = SignUpInput {
        name: username,
        email: email.clone(),
        password: password.clone(),
        team_id: None,
        invite_code: None,
    };

    client.signup(signup_input).await?;

    let login_input = LoginInput {
        email: email.clone(),
        password: password.clone(),
    };

    client.login(login_input).await?;

    let workspace_input = CreateTeamInput {
        name: "comhairle workspace".to_string(),
        avatar: None,
        members: None,
    };

    let workspace_id = client.create_workspace(workspace_input).await?;
    let project_id = client.get_teams().await?[0].projects[0].id.clone();

    let poll_input = CreateFormInput {
        project_id: project_id.clone(),
        name: Some("ComhairleForm".to_string()),
        interactive_mode: InteractiveMode::Conversational,
        kind: FormKind::Poll,
        name_schema: Some(vec![serde_json::json!({
            "id": "title",
            "title": "ComhairleForm",
            "kind": "title"
        })]),
    };

    let poll_id = client.create_poll(poll_input).await?;

    let hidden_field_input = CreateHiddenFieldInput {
        form_id: poll_id.clone(),
        field_id: "comhairle_user_id".to_string(),
        field_name: "comhairle_user_id".to_string(),
    };

    client.create_form_hidden_field(hidden_field_input).await?;

    let custom_css = r#"
        .heyform-container {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        }
        .heyform-question {
            color: #ffffff;
            font-family: 'Arial', sans-serif;
        }
        .heyform-button {
            background-color: #4CAF50;
            border: none;
            color: white;
            padding: 15px 32px;
            text-align: center;
            border-radius: 8px;
        }
    "#;
    client.set_custom_css(&poll_id, custom_css).await?;

    let poll_url = client.publish_poll(&poll_id, None).await?;

    Ok(HeyFormToolConfig {
        admin_user: email,
        admin_password: password,
        survey_url: poll_url,
        survey_id: poll_id,
        workspace_id,
        project_id,
        server_url: setup_config.server_url.to_string(),
    })
}

/// Zero-sized marker type for HeyForm tool implementation
pub struct HeyFormTool;

#[async_trait]
impl ToolImpl for HeyFormTool {
    type Config = HeyFormToolConfig;
    type Setup = HeyFormToolSetup;
    type Report = HeyFormReport;

    async fn setup(
        setup: &Self::Setup,
        _state: &Arc<ComhairleState>,
    ) -> Result<Self::Config, ComhairleError> {
        // Delegate to existing setup function
        heyform_setup(setup).await
    }

    async fn clone_tool(
        config: &Self::Config,
        _state: &Arc<ComhairleState>,
    ) -> Result<Self::Config, ComhairleError> {
        // Delegate to existing launch function
        launch(config).await
    }

    fn sanitize(config: Self::Config) -> Self::Config {
        config.sanitize()
    }

    async fn delete(
        config: &Self::Config,
        _state: &Arc<ComhairleState>,
        _workflow_step_id: &Uuid,
    ) -> Result<(), ComhairleError> {
        let client = HeyFormClient::new(heyform_base_url(&config.server_url))?;

        client
            .login(LoginInput {
                email: config.admin_user.clone(),
                password: config.admin_password.clone(),
            })
            .await?;

        client.delete_poll(&config.survey_id).await?;

        Ok(())
    }

    fn routes(state: &Arc<ComhairleState>) -> ApiRouter {
        ApiRouter::new()
            .api_route(
                "/survey_tool/workflow_step/{workflow_step_id}/form",
                get_with(form, |op| {
                    op.id("HeyFormGetForm")
                        .tag("Tools")
                        .summary("Get HeyForm form for a workflow step")
                        .description("Fetches the form for the HeyForm tool attached to a workflow step")
                        .response::<200, Json<Form>>()
                }),
            )
            .api_route(
                "/survey_tool/workflow_step/{workflow_step_id}/form_report",
                get_with(form_report, |op| {
                    op.id("HeyFormGetFormReport")
                        .tag("Tools")
                        .summary("Get HeyForm report for a workflow step")
                        .description("Fetches the form report for the HeyForm tool attached to a workflow step")
                        .response::<200, Json<FormReport>>()
                }),
            )
            .api_route(
                "/survey_tool/workflow_step/{workflow_step_id}/submissions",
                get_with(submissions, |op| {
                    op.id("HeyFormGetSubmissions")
                        .tag("Tools")
                        .summary("Get HeyForm submissions for a workflow step")
                        .description("Fetches the form submissions for the HeyForm tool attached to a workflow step")
                        .response::<200, Json<Submissions>>()
                }),
            )
            .api_route(
                "/survey_tool/workflow_step/{workflow_step_id}/insights",
                get_with(insights, |op| {
                    op.id("HeyFormGetInsights")
                        .tag("Tools")
                        .summary("Get labelled survey insights for a workflow step")
                        .description(
                            "Combines the HeyForm form definition with its aggregate report to \
                             produce a per-question breakdown with human-readable question titles \
                             and choice labels resolved from the form schema.",
                        )
                        .response::<200, Json<SurveyInsights>>()
                }),
            )
            .with_state(state.clone())
    }
}

async fn get_heyform_config_for_workflow_step(
    state: &Arc<ComhairleState>,
    workflow_step_id: Uuid,
) -> Result<HeyFormToolConfig, ComhairleError> {
    let workflow_step = models::workflow_step::get_by_id(&state.db, &workflow_step_id).await?;

    match (workflow_step.tool_config, workflow_step.preview_tool_config) {
        (Some(ToolConfig::HeyForm(config)), _) => Ok(config),
        (None, ToolConfig::HeyForm(config)) => Ok(config),
        _ => Err(ComhairleError::WorkflowStepHasWrongType("HeyForm".into())),
    }
}

/// Get heyform form data for a given survey tool configuration
pub async fn form(
    State(state): State<Arc<ComhairleState>>,
    Path(workflow_step_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Form>), ComhairleError> {
    let config = get_heyform_config_for_workflow_step(&state, workflow_step_id).await?;
    let client = HeyFormClient::new(heyform_base_url(&config.server_url))?;

    client
        .login(LoginInput {
            email: config.admin_user.clone(),
            password: config.admin_password.clone(),
        })
        .await?;

    let form = client.get_form(&config.survey_id).await?;

    Ok((StatusCode::OK, Json(form)))
}

/// Get heyform form report data for a given survey tool configuration
pub async fn form_report(
    State(state): State<Arc<ComhairleState>>,
    Path(workflow_step_id): Path<Uuid>,
) -> Result<(StatusCode, Json<FormReport>), ComhairleError> {
    let config = get_heyform_config_for_workflow_step(&state, workflow_step_id).await?;
    let client = HeyFormClient::new(heyform_base_url(&config.server_url))?;

    client
        .login(LoginInput {
            email: config.admin_user.clone(),
            password: config.admin_password.clone(),
        })
        .await?;

    let report = client.get_form_report(config.survey_id.clone()).await?;

    Ok((StatusCode::OK, Json(report)))
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct SubmissionsQuery {
    pub category: Option<String>,
}

/// Helper to fetch all pages of submissions for a survey from HeyForm.
pub async fn fetch_all_submissions(
    client: &HeyFormClient,
    survey_id: &str,
    category: &str,
) -> Result<Vec<Submission>, heyform_sdk::HeyFormError> {
    let mut submissions = Vec::new();
    let mut page = 1;
    loop {
        let page_submissions = client
            .get_form_submissions(survey_id.to_string(), category.to_string(), page)
            .await?;

        if page_submissions.submissions.is_empty() {
            break;
        }
        submissions.extend(page_submissions.submissions);
        page += 1;
    }
    Ok(submissions)
}

/// Get heyform submission data for a given survey tool configuration
/// Queries are paginated, so we want to collect all pages of submissions and return them as a single vector
pub async fn submissions(
    State(state): State<Arc<ComhairleState>>,
    Path(workflow_step_id): Path<Uuid>,
    Query(query): Query<SubmissionsQuery>,
) -> Result<(StatusCode, Json<Submissions>), ComhairleError> {
    let config = get_heyform_config_for_workflow_step(&state, workflow_step_id).await?;
    let client = HeyFormClient::new(heyform_base_url(&config.server_url))?;

    client
        .login(LoginInput {
            email: config.admin_user.clone(),
            password: config.admin_password.clone(),
        })
        .await?;

    let category = match query.category {
        Some(cat) if matches!(cat.as_str(), "inbox" | "spam" | "starred" | "archive") => {
            cat.to_string()
        }
        None => "inbox".to_string(), // Default to inbox if no category is provided
        _ => return Err(ComhairleError::BadRequest("Invalid category".into())),
    };

    let submissions = fetch_all_submissions(&client, &config.survey_id, &category).await?;

    Ok((
        StatusCode::OK,
        Json(Submissions {
            total: submissions.len() as u32,
            submissions,
        }),
    ))
}

// Keep public function for backwards compatibility
#[allow(
    dead_code,
    reason = "This function is kept for backwards compatibility."
)]
pub async fn setup(setup_config: &HeyFormToolSetup) -> Result<HeyFormToolConfig, ComhairleError> {
    heyform_setup(setup_config).await
}

/// A single answer choice within an insight question, with its response count.
#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct InsightChoice {
    /// The choice identifier as stored by HeyForm.
    pub id: String,
    /// Human-readable label for this choice.
    pub label: String,
    /// Number of times this choice was selected.
    pub count: i64,
}

/// One individual submission answer for a free-text or numeric question.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct InsightSubmission {
    /// The HeyForm submission identifier.
    pub submission_id: String,
    /// The raw answer value. For short-text questions this is a JSON string;
    /// for opinion-scale questions it is a JSON number; for other kinds it
    /// may be a structured object. Preserved as-is from the form report.
    pub value: serde_json::Value,
    /// The submission timestamp (end_at Unix epoch timestamp).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<i64>,
}

/// Per-question insight data: the question title resolved from the form schema
/// alongside the aggregate response breakdown from the form report.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct InsightQuestion {
    /// The field identifier as stored by HeyForm.
    pub id: String,
    /// The question kind/type (e.g., "short_text", "multiple_choice", "opinion_scale", "yes_no").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Human-readable question title extracted from the form schema.
    /// Falls back to the field ID when the schema contains no plain-text title.
    pub title: String,
    /// Number of times this question was answered.
    pub answered: u32,
    /// Total number of responses recorded for this question.
    pub total: u32,
    /// Field properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, serde_json::Value>>,
    /// Per-choice breakdown. Present only for choice-based question kinds
    /// (multiple-choice, single-choice, picture-choice, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choices: Option<Vec<InsightChoice>>,
    /// Individual submission answers. Present for free-text, opinion-scale,
    /// and other non-choice question kinds where the aggregate report carries
    /// no per-answer breakdown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submissions: Option<Vec<InsightSubmission>>,
}

/// The fully labelled survey insights for a workflow step, combining the
/// form schema with its aggregate report data.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SurveyInsights {
    pub questions: Vec<InsightQuestion>,
}

/// Extracts a plain-text title from a HeyForm field `title` value.
///
/// HeyForm encodes question titles as a JSON array of rich-text nodes. Nodes
/// can be:
/// - A plain string (the common case, e.g. `"What is your "`).
/// - A nested array representing a rich-text inline element such as a link
///   (`["a", ["age"], {...}]`); we extract the text from position 1 if it is
///   itself a string or a single-element string array.
/// - An object with a `"text"` key (an older/alternative HeyForm encoding).
///
/// All extracted text fragments are concatenated. Returns `None` when no
/// plain-text content can be found so callers can fall back to the field ID.
fn extract_field_title(title: &serde_json::Value) -> Option<String> {
    let nodes = title.as_array()?;
    let text: String = nodes
        .iter()
        .filter_map(|node| {
            if let Some(s) = node.as_str() {
                // Plain string node -- the most common format.
                return Some(s.to_owned());
            }
            if let Some(arr) = node.as_array() {
                // Nested array: ["a", <content>, <attrs>]
                // <content> at index 1 can be a plain string or a
                // single-element array containing a string.
                let content = arr.get(1)?;
                if let Some(s) = content.as_str() {
                    return Some(s.to_owned());
                }
                if let Some(inner) = content.as_array() {
                    if let Some(s) = inner.first().and_then(|v| v.as_str()) {
                        return Some(s.to_owned());
                    }
                }
                return None;
            }
            // Object node with a "text" key (alternative encoding).
            node.get("text").and_then(|t| t.as_str()).map(str::to_owned)
        })
        .collect();
    if text.is_empty() { None } else { Some(text) }
}

/// Checks if the HeyForm value is empty (the user skipped the question)
fn is_empty_value(value: &serde_json::Value) -> bool {
    let val = value.to_string();

    val == "\"\"" || val == "{\"value\":[]}"
}

/// Resolves the list of [`InsightChoice`]s for one question.
///
/// The report's `chooses` array already carries `id`, `label`, and `count`
/// for each choice. We trust those values directly, cross-referencing the form
/// field's `properties.choices` only to fill in the label when the report
/// omits it (which can happen for older HeyForm versions).
fn resolve_choices(
    report_response: &FormReportResponse,
    field_choices_by_id: &HashMap<String, String>,
) -> Option<Vec<InsightChoice>> {
    let chooses = report_response.chooses.as_deref()?;

    let resolved: Vec<InsightChoice> = chooses
        .iter()
        .filter_map(|choose| {
            let id = choose.get("id")?.as_str()?.to_owned();
            // Prefer the label already present in the report; fall back to
            // the form field's choice label, then the choice ID itself.
            let label = choose
                .get("label")
                .and_then(|l| l.as_str())
                .filter(|l| !l.is_empty())
                .map(str::to_owned)
                .or_else(|| field_choices_by_id.get(&id).cloned())
                .unwrap_or_else(|| id.clone());
            let count = choose.get("count").and_then(|c| c.as_i64()).unwrap_or(0);
            Some(InsightChoice { id, label, count })
        })
        .collect();

    if resolved.is_empty() {
        None
    } else {
        Some(resolved)
    }
}

/// Builds the fully labelled [`SurveyInsights`] by joining a form definition,
/// its aggregate report, and full submissions list.
///
/// This is the pure transformation layer. The handler fetches all resources
/// and delegates here, keeping network I/O and business logic separate.
pub fn build_survey_insights(
    form: &Form,
    report: &FormReport,
    submissions: &[Submission],
) -> SurveyInsights {
    // Index form fields by ID so look-ups are O(1).
    let mut fields_by_id: HashMap<String, FormField> = HashMap::new();
    for field in form.fields.as_deref().unwrap_or_default().iter() {
        if matches!(
            field.kind.as_str(),
            "statement" | "payment" | "welcome" | "thank_you"
        ) {
            continue;
        }

        fields_by_id.insert(field.id.clone(), field.clone());

        if field.kind == "group" {
            let fields = field
                .properties
                .as_ref()
                .and_then(|p| p.get("fields"))
                .and_then(|p| p.as_array());

            let Some(fields) = fields else {
                continue;
            };

            let parent = Parent {
                id: field.id.clone(),
                title: extract_field_title(field.title.as_ref().unwrap_or_default())
                    .unwrap_or_default(),
            };

            for subfield in fields.iter() {
                let subfield: Result<FormField, serde_json::Error> =
                    serde_json::from_value(subfield.clone());
                let Ok(mut subfield) = subfield else {
                    continue;
                };

                let properties = subfield.properties.as_mut();
                if let Some(properties) = properties {
                    properties.insert(
                        "parent".to_string(),
                        serde_json::to_value(parent.clone()).unwrap_or_default(),
                    );
                }

                fields_by_id.insert(subfield.id.clone(), subfield);
            }
        }
    }

    let mut submissions_by_field: HashMap<String, Vec<InsightSubmission>> = HashMap::new();
    let mut seen_answers: std::collections::HashSet<(String, String)> =
        std::collections::HashSet::new();

    // 1. Process full submissions from `Submissions` endpoint (which covers all question types including opinion_scale, etc.)
    for submission in submissions {
        let sub_id = &submission.id;
        let submitted_at = if submission.end_at != 0 {
            Some(submission.end_at)
        } else {
            None
        };
        for answer in &submission.answers {
            if let Some(field_id) = answer.get("id").and_then(|v| v.as_str()) {
                if let Some(val) = answer.get("value") {
                    if !val.is_null() && !is_empty_value(val) {
                        let key = (field_id.to_string(), sub_id.clone());
                        if seen_answers.insert(key) {
                            submissions_by_field
                                .entry(field_id.to_string())
                                .or_default()
                                .push(InsightSubmission {
                                    submission_id: sub_id.clone(),
                                    value: val.clone(),
                                    submitted_at,
                                });
                        }
                    }
                }
            }
        }
    }

    // 2. Fall back to report.submissions (for any answers present in report not already captured)
    for report_sub in &report.submissions {
        let field_id = &report_sub.id;
        for answer in &report_sub.answers {
            if let Some(val) = &answer.value {
                if !val.is_null() && !is_empty_value(val) {
                    let key = (field_id.clone(), answer.submission_id.clone());
                    if seen_answers.insert(key) {
                        let submitted_at = if answer.end_at != 0 {
                            Some(answer.end_at)
                        } else {
                            None
                        };
                        submissions_by_field
                            .entry(field_id.clone())
                            .or_default()
                            .push(InsightSubmission {
                                submission_id: answer.submission_id.clone(),
                                value: val.clone(),
                                submitted_at,
                            });
                    }
                }
            }
        }
    }

    let questions = report
        .responses
        .iter()
        .filter_map(|response| {
            let Some(field) = fields_by_id.get(&response.id) else {
                return None;
            };

            // Resolve the question title from the form field, falling back to
            // the field ID when no plain-text title is available.
            let title = field
                .title
                .as_ref()
                .and_then(extract_field_title)
                .unwrap_or_else(|| response.id.clone());

            // Build a choice-ID-to-label index from the form field properties
            // so that we can fill gaps left by the report.
            let field_choices_by_id: HashMap<String, String> = field
                .properties
                .as_ref()
                .and_then(|properties| properties.get("choices"))
                .and_then(|choices| choices.as_array())
                .map(|choices| {
                    choices
                        .iter()
                        .filter_map(|choice| {
                            let Some(choice) = choice.as_object() else {
                                return None;
                            };
                            let id = choice
                                .get("id")
                                .and_then(|id| id.as_str())
                                .map(|id| id.to_string())
                                .unwrap_or_default();
                            let label = choice
                                .get("label")
                                .and_then(|id| id.as_str())
                                .map(|id| id.to_string())
                                .unwrap_or_default();
                            Some((id, label))
                        })
                        .collect()
                })
                .unwrap_or_default();

            let choices = resolve_choices(response, &field_choices_by_id);

            let answered = submissions_by_field
                .get(&response.id)
                .unwrap_or(&Vec::new())
                .len() as u32;

            // Attach individual submission answers for non-choice questions.
            // Choice-based questions already have a full aggregate breakdown
            // via `choices`; for everything else (short_text, opinion_scale,
            // etc.) the per-answer values are the only meaningful content.
            let submissions = if choices.is_none() {
                submissions_by_field.get(response.id.as_str()).cloned()
            } else {
                None
            };

            Some(InsightQuestion {
                id: response.id.clone(),
                kind: Some(String::from(field.kind.clone())),
                title,
                answered,
                total: response.total,
                properties: field.properties.clone(),
                choices,
                submissions,
            })
        })
        .collect();

    SurveyInsights { questions }
}

/// Returns fully labelled survey insights for a workflow step.
///
/// Fetches the form schema, aggregate report, and full submissions from HeyForm concurrently,
/// then joins them so that each question and each choice carries a
/// human-readable label rather than an opaque ID, and non-choice questions carry answer values.
pub async fn insights(
    State(state): State<Arc<ComhairleState>>,
    Path(workflow_step_id): Path<Uuid>,
) -> Result<(StatusCode, Json<SurveyInsights>), ComhairleError> {
    let config = get_heyform_config_for_workflow_step(&state, workflow_step_id).await?;
    let client = HeyFormClient::new(heyform_base_url(&config.server_url))?;

    client
        .login(LoginInput {
            email: config.admin_user.clone(),
            password: config.admin_password.clone(),
        })
        .await?;

    // Fetch the form definition, aggregate report, and full submissions concurrently.
    let (form, report, submissions) = tokio::try_join!(
        client.get_form(&config.survey_id),
        client.get_form_report(config.survey_id.clone()),
        fetch_all_submissions(&client, &config.survey_id, "inbox"),
    )?;

    let survey_insights = build_survey_insights(&form, &report, &submissions);

    Ok((StatusCode::OK, Json(survey_insights)))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use axum::{Router, extract::State, routing::post};
    use heyform_sdk::{
        Form, FormField, FormReport, FormReportAnswer, FormReportResponse, FormReportSubmission,
        HiddenFieldAnswer, Submission, SubmissionCategory, Submissions,
    };
    use serde_json::json;
    use sqlx::PgPool;
    use tokio::net::TcpListener;

    use crate::{
        models::model_test_helpers::{
            get_random_conversation_id, get_random_workflow_id, setup_default_app_and_session,
        },
        routes::workflow_steps::dto::WorkflowStepDto,
        test_helpers::heyform_tool_config,
    };

    use super::*;

    use std::error::Error;

    #[derive(Clone)]
    struct MockHeyFormState {
        submission_calls: Arc<AtomicUsize>,
    }

    fn create_form_report() -> FormReport {
        FormReport {
            responses: vec![FormReportResponse {
                id: "report-1".to_string(),
                total: 42,
                kind: Some("poll".to_string()),
                title: Some("Test Form Report".to_string()),
                count: 42,
                average: 12f64,
                chooses: Some(vec![serde_json::json!({
                    "id": "choose-1",
                    "label": "Test Choose 1",
                    "image": "https://example.com/image.png",
                    "icon": {
                        "name": "star",
                        "color": "#FFD700",
                        "background": "#0000FF",
                    },
                    "color": "#FF0000",
                    "score": 10,
                    "is_expected": true,
                    "count": 5,
                })]),
            }],
            submissions: vec![FormReportSubmission {
                id: "submission-1".to_string(),
                answers: vec![FormReportAnswer {
                    submission_id: "submission-1".to_string(),
                    kind: "text".to_string(),
                    value: Some(serde_json::json!("Sample answer")),
                    end_at: 1735689600,
                }],
            }],
        }
    }

    fn create_submission() -> Submission {
        Submission {
            id: "submission-1".to_string(),
            category: Some(SubmissionCategory::Inbox),
            title: Some("First response".to_string()),
            answers: vec![HashMap::from([
                ("question-1".to_string(), serde_json::json!("Answer 1")),
                ("question-2".to_string(), serde_json::json!("Answer 2")),
            ])],
            hidden_fields: Some(vec![HiddenFieldAnswer {
                id: "hidden-1".to_string(),
                name: "comhairle_user_id".to_string(),
                value: Some("user-123".to_string()),
            }]),
            variables: Some(vec![serde_json::json!({
                "some_variable": "some_value",
            })]),
            end_at: 1735689600,
        }
    }

    fn create_submissions() -> Submissions {
        Submissions {
            total: 1,
            submissions: vec![create_submission()],
        }
    }

    async fn mock_graphql(
        State(state): State<MockHeyFormState>,
        Json(payload): Json<serde_json::Value>,
    ) -> Json<serde_json::Value> {
        let query = payload
            .get("query")
            .and_then(|value| value.as_str())
            .unwrap_or_default();
        let operation_name = payload
            .get("operationName")
            .and_then(|value| value.as_str())
            .unwrap_or_default();

        if query.contains("query signUp") {
            return Json(json!({
                "data": {
                    "signUp": true
                }
            }));
        }

        if query.contains("query login") {
            return Json(json!({
                "data": {
                    "login": true
                }
            }));
        }

        if operation_name == "createTeam" || query.contains("mutation createTeam") {
            return Json(json!({
                "data": {
                    "createTeam": "team-1"
                }
            }));
        }

        if query.contains("query {") && query.contains("teams {") {
            return Json(json!({
                "data": {
                    "teams": [
                        {
                            "id": "team-1",
                            "name": "comhairle workspace",
                            "ownerId": "owner-1",
                            "inviteCode": "invite-1",
                            "avatar": null,
                            "memberCount": 1,
                            "createdAt": "2025-01-01T00:00:00.000Z",
                            "projects": [
                                {
                                    "id": "project-1",
                                    "teamId": "team-1",
                                    "name": "project-1"
                                }
                            ]
                        }
                    ]
                }
            }));
        }

        if operation_name == "createFormHiddenField"
            || query.contains("mutation createFormHiddenField")
        {
            return Json(json!({
                "data": {
                    "createFormHiddenField": true
                }
            }));
        }

        if operation_name == "createForm" || query.contains("mutation createForm(") {
            return Json(json!({
                "data": {
                    "createForm": "form-1"
                }
            }));
        }

        if operation_name == "updateFormTheme" || query.contains("mutation updateFormTheme(") {
            return Json(json!({
                "data": {
                    "updateFormTheme": true
                }
            }));
        }

        if operation_name == "updateFormSchemas" || query.contains("mutation updateFormSchemas(") {
            return Json(json!({
                "data": {
                    "updateFormSchemas": true
                }
            }));
        }

        if operation_name == "updateForm" || query.contains("mutation updateForm(") {
            return Json(json!({
                "data": {
                    "updateForm": true
                }
            }));
        }

        if operation_name == "deleteForm" || query.contains("mutation deleteForm") {
            return Json(json!({
                "data": {
                    "deleteForm": true
                }
            }));
        }

        if query.contains("query formDetail") {
            return Json(json!({
                "data": {
                    "formDetail": {
                        "id": "form-1",
                        "teamId": "team-1",
                        "projectId": "project-1",
                        "name": "Test Form",
                        "description": null,
                        "interactiveMode": null,
                        "kind": null,
                        "settings": null,
                        "fields": [],
                        "themeSettings": null,
                        "draft": null,
                        "status": null
                    }
                }
            }));
        }

        if operation_name == "formReport" || query.contains("query formReport") {
            return Json(
                serde_json::to_value(json!({
                    "data": {
                        "formReport": create_form_report(),
                    }
                }))
                .expect("Failed to serialize FormReport"),
            );
        }

        if operation_name == "submissions" || query.contains("query submissions") {
            // First two calls return submissions with one submission, subsequent calls return empty submissions.
            // This allows us to test the pagination logic in the `submissions` function.
            let call_index = state.submission_calls.fetch_add(1, Ordering::SeqCst);
            if call_index < 2 {
                return Json(
                    serde_json::to_value(json!({
                        "data": {
                            "submissions": create_submissions(),
                        }
                    }))
                    .expect("Failed to serialize Submissions"),
                );
            }

            return Json(json!({
                "data": {
                    "submissions": {
                        "total": 0,
                        "submissions": []
                    }
                }
            }));
        }

        tracing::warn!("Mock HeyForm server received unexpected query: {}", query);

        Json(json!({ "data": {} }))
    }

    async fn start_mock_heyform_server()
    -> Result<(String, tokio::task::JoinHandle<()>), Box<dyn Error>> {
        let state = MockHeyFormState {
            submission_calls: Arc::new(AtomicUsize::new(0)),
        };

        let app = Router::new()
            .route("/graphql", post(mock_graphql))
            .with_state(state);

        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let address = listener.local_addr()?;
        let server = tokio::spawn(async move {
            let _ = axum::serve(listener, app).await;
        });

        Ok((address.to_string(), server))
    }

    async fn create_heyform_workflow_step(
        app: &axum::Router,
        session: &mut crate::test_helpers::UserSession,
        server_url: &str,
    ) -> Result<WorkflowStepDto, Box<dyn Error>> {
        let conversation_id = get_random_conversation_id(app, session).await?;
        let workflow_id = get_random_workflow_id(app, session).await?;

        let mut tool_setup = heyform_tool_config();
        tool_setup["server_url"] = json!(server_url);

        let (status, value, _) = session
            .create_workflow_step(
                app,
                &conversation_id.to_string(),
                &workflow_id.to_string(),
                json!({
                    "name": "test_workflow_step",
                    "step_order": 1,
                    "activation_rule": "manual",
                    "description": "A test workflow_step with heyform tool",
                    "is_offline": false,
                    "required": false,
                    "tool_setup": tool_setup,
                }),
            )
            .await?;

        assert!(status.is_success(), "error response status");

        Ok(serde_json::from_value(value)?)
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_return_form_report(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (mock_url, mock_server) = start_mock_heyform_server().await?;
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let workflow_step = create_heyform_workflow_step(&app, &mut session, &mock_url).await?;

        let (status, value, _) = session
            .get(
                &app,
                &format!(
                    "/tools/survey_tool/workflow_step/{}/form_report",
                    workflow_step.id
                ),
            )
            .await?;

        assert!(status.is_success(), "error response status");

        // Check that the report has the expected values
        let report: FormReport = serde_json::from_value(value)?;
        assert_eq!(report, create_form_report());

        mock_server.abort();
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_return_submissions(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (mock_url, mock_server) = start_mock_heyform_server().await?;
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let workflow_step = create_heyform_workflow_step(&app, &mut session, &mock_url).await?;

        let (status, value, _) = session
            .get(
                &app,
                &format!(
                    "/tools/survey_tool/workflow_step/{}/submissions?category=inbox",
                    workflow_step.id
                ),
            )
            .await?;

        assert!(status.is_success(), "error response status");

        let submissions = serde_json::from_value::<Submissions>(value)?;
        assert_ne!(submissions, create_submissions());
        assert_eq!(submissions.total, 2);
        assert!(
            submissions
                .submissions
                .iter()
                .all(|s| *s == create_submission())
        );

        mock_server.abort();
        Ok(())
    }

    // --- Unit tests for the pure build_survey_insights logic ---

    fn make_form_with_choice_field() -> Form {
        Form {
            id: "form-1".to_string(),
            team_id: "team-1".to_string(),
            project_id: "project-1".to_string(),
            name: Some("Test Form".to_string()),
            description: None,
            interactive_mode: None,
            kind: None,
            settings: None,
            fields: Some(vec![FormField {
                id: "q-1".to_string(),
                // HeyForm encodes titles as rich-text JSON arrays.
                title: Some(json!([{"text": "Favourite colour?"}])),
                description: None,
                kind: "multiple_choice".to_string(),
                validations: None,
                properties: Some(HashMap::from([(
                    "choices".to_string(),
                    json!([
                        {"id": "c-red",  "label": "Red"},
                        {"id": "c-blue", "label": "Blue"}
                    ]),
                )])),
                layout: None,
                width: None,
                hide: None,
                frozen: None,
            }]),
            theme_settings: None,
            draft: None,
            status: None,
        }
    }

    fn make_report_with_choices() -> FormReport {
        FormReport {
            responses: vec![FormReportResponse {
                id: "q-1".to_string(),
                kind: Some("multiple_choice".to_string()),
                title: Some("Favourite colour?".to_string()),
                total: 10,
                count: 10,
                average: 0.0,
                chooses: Some(vec![
                    json!({"id": "c-red",  "label": "Red",  "count": 7}),
                    json!({"id": "c-blue", "label": "Blue", "count": 3}),
                ]),
            }],
            submissions: vec![],
        }
    }

    #[test]
    fn build_survey_insights_resolves_question_title_object_node() {
        // Older / alternative encoding: [{"text": "..."}]
        let form = make_form_with_choice_field();
        let report = make_report_with_choices();
        let insights = build_survey_insights(&form, &report, &[]);

        assert_eq!(insights.questions.len(), 1);
        assert_eq!(insights.questions[0].title, "Favourite colour?");
        assert_eq!(
            insights.questions[0].kind.as_deref(),
            Some("multiple_choice")
        );
    }

    #[test]
    fn build_survey_insights_resolves_question_title_plain_string_nodes() {
        // Real HeyForm encoding: plain strings (and nested link arrays).
        // e.g. ["What is your ", ["a", ["age"], {...}], "?"]
        let mut form = make_form_with_choice_field();
        if let Some(fields) = form.fields.as_mut() {
            fields[0].title = Some(json!(["What is your ", ["a", ["age"], {}], "?"]));
        }
        let report = make_report_with_choices();
        let insights = build_survey_insights(&form, &report, &[]);

        assert_eq!(insights.questions[0].title, "What is your age?");
    }

    #[test]
    fn build_survey_insights_resolves_choice_labels_and_counts() {
        let form = make_form_with_choice_field();
        let report = make_report_with_choices();
        let insights = build_survey_insights(&form, &report, &[]);

        let choices = insights.questions[0]
            .choices
            .as_ref()
            .expect("choices present");
        assert_eq!(choices.len(), 2);

        let red = choices
            .iter()
            .find(|c| c.id == "c-red")
            .expect("red choice");
        assert_eq!(red.label, "Red");
        assert_eq!(red.count, 7);

        let blue = choices
            .iter()
            .find(|c| c.id == "c-blue")
            .expect("blue choice");
        assert_eq!(blue.label, "Blue");
        assert_eq!(blue.count, 3);
    }

    #[test]
    fn build_survey_insights_falls_back_to_field_id_when_no_title() {
        let mut form = make_form_with_choice_field();
        // Remove the title so the fallback path is exercised.
        if let Some(fields) = form.fields.as_mut() {
            fields[0].title = None;
        }
        let report = make_report_with_choices();
        let insights = build_survey_insights(&form, &report, &[]);

        assert_eq!(insights.questions[0].title, "q-1");
    }

    #[test]
    fn build_survey_insights_falls_back_to_form_choice_label_when_report_label_missing() {
        let form = make_form_with_choice_field();
        let mut report = make_report_with_choices();
        // Strip the label from the report's choice to test the fallback path.
        if let Some(chooses) = report.responses[0].chooses.as_mut() {
            chooses[0] = json!({"id": "c-red", "count": 7});
        }
        let insights = build_survey_insights(&form, &report, &[]);

        let choices = insights.questions[0]
            .choices
            .as_ref()
            .expect("choices present");
        let red = choices
            .iter()
            .find(|c| c.id == "c-red")
            .expect("red choice");
        // Label should come from the form field properties.
        assert_eq!(red.label, "Red");
    }

    #[test]
    fn build_survey_insights_attaches_submissions_for_non_choice_questions() {
        // A free-text question carries no `chooses`, so individual submission
        // values should be surfaced in `submissions`.
        let form = Form {
            id: "form-1".to_string(),
            team_id: "team-1".to_string(),
            project_id: "project-1".to_string(),
            name: Some("Test Form".to_string()),
            description: None,
            interactive_mode: None,
            kind: None,
            settings: None,
            fields: Some(vec![FormField {
                id: "q-text".to_string(),
                title: Some(json!(["What is your name?"])),
                description: None,
                kind: "short_text".to_string(),
                validations: None,
                properties: None,
                layout: None,
                width: None,
                hide: None,
                frozen: None,
            }]),
            theme_settings: None,
            draft: None,
            status: None,
        };
        let report = FormReport {
            responses: vec![FormReportResponse {
                id: "q-text".to_string(),
                kind: Some("short_text".to_string()),
                title: None,
                total: 2,
                count: 2,
                average: 0.0,
                chooses: None,
            }],
            submissions: vec![FormReportSubmission {
                id: "q-text".to_string(),
                answers: vec![
                    FormReportAnswer {
                        submission_id: "sub-1".to_string(),
                        kind: "short_text".to_string(),
                        value: Some(json!("Alice")),
                        end_at: 1000,
                    },
                    FormReportAnswer {
                        submission_id: "sub-2".to_string(),
                        kind: "short_text".to_string(),
                        value: Some(json!("Bob")),
                        end_at: 2000,
                    },
                ],
            }],
        };

        let insights = build_survey_insights(&form, &report, &[]);

        assert_eq!(insights.questions.len(), 1);
        let q = &insights.questions[0];
        assert_eq!(q.title, "What is your name?");
        assert!(q.choices.is_none(), "no choices for a short_text question");

        let subs = q.submissions.as_ref().expect("submissions present");
        assert_eq!(subs.len(), 2);
        assert!(
            subs.iter()
                .any(|s| s.submission_id == "sub-1" && s.value == json!("Alice"))
        );
        assert!(
            subs.iter()
                .any(|s| s.submission_id == "sub-2" && s.value == json!("Bob"))
        );
    }

    #[test]
    fn build_survey_insights_attaches_submissions_from_full_submissions_list() {
        let form = Form {
            id: "form-1".to_string(),
            team_id: "team-1".to_string(),
            project_id: "project-1".to_string(),
            name: Some("Test Form".to_string()),
            description: None,
            interactive_mode: None,
            kind: None,
            settings: None,
            fields: Some(vec![FormField {
                id: "q-scale".to_string(),
                title: Some(json!(["How much do you like ice cream?"])),
                description: None,
                kind: "opinion_scale".to_string(),
                validations: None,
                properties: None,
                layout: None,
                width: None,
                hide: None,
                frozen: None,
            }]),
            theme_settings: None,
            draft: None,
            status: None,
        };
        let report = FormReport {
            responses: vec![FormReportResponse {
                id: "q-scale".to_string(),
                kind: Some("opinion_scale".to_string()),
                title: None,
                total: 1,
                count: 1,
                average: 10.0,
                chooses: None,
            }],
            submissions: vec![],
        };
        let full_submissions = vec![Submission {
            id: "sub-100".to_string(),
            category: None,
            title: None,
            answers: vec![HashMap::from([
                ("id".to_string(), json!("q-scale")),
                ("kind".to_string(), json!("opinion_scale")),
                ("value".to_string(), json!(10)),
            ])],
            hidden_fields: None,
            variables: None,
            end_at: 1000,
        }];

        let insights = build_survey_insights(&form, &report, &full_submissions);

        assert_eq!(insights.questions.len(), 1);
        let q = &insights.questions[0];
        assert_eq!(q.title, "How much do you like ice cream?");
        assert!(q.choices.is_none());

        let subs = q.submissions.as_ref().expect("submissions present");
        assert_eq!(subs.len(), 1);
        assert_eq!(subs[0].submission_id, "sub-100");
        assert_eq!(subs[0].value, json!(10));
        assert_eq!(subs[0].submitted_at, Some(1000));
    }

    #[test]
    fn build_survey_insights_no_submissions_for_choice_questions() {
        // Choice questions must not also get a `submissions` list.
        let form = make_form_with_choice_field();
        let mut report = make_report_with_choices();
        // Add a submission entry for the same field to confirm it is suppressed.
        report.submissions.push(FormReportSubmission {
            id: "q-1".to_string(),
            answers: vec![FormReportAnswer {
                submission_id: "sub-x".to_string(),
                kind: "multiple_choice".to_string(),
                value: Some(json!("c-red")),
                end_at: 1000,
            }],
        });
        let insights = build_survey_insights(&form, &report, &[]);

        assert!(insights.questions[0].submissions.is_none());
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_return_insights(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (mock_url, mock_server) = start_mock_heyform_server().await?;
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let workflow_step = create_heyform_workflow_step(&app, &mut session, &mock_url).await?;

        let (status, value, _) = session
            .get(
                &app,
                &format!(
                    "/tools/survey_tool/workflow_step/{}/insights",
                    workflow_step.id
                ),
            )
            .await?;

        assert!(status.is_success(), "error response status");

        let insights: SurveyInsights = serde_json::from_value(value)?;
        // The mock form_report returns one response ("report-1") with one choose.
        // The mock form returns no fields, so titles fall back to field IDs.
        assert_eq!(insights.questions.len(), 1);
        assert_eq!(insights.questions[0].id, "report-1");
        // No matching form field => title falls back to the field ID.
        assert_eq!(insights.questions[0].title, "report-1");

        mock_server.abort();
        Ok(())
    }
}
