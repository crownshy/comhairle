use std::sync::Arc;

use aide::axum::ApiRouter;
use aide::axum::routing::get_with;
use async_trait::async_trait;
use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use heyform_sdk::client::HeyFormClient;
use heyform_sdk::{
    CreateFormInput, CreateHiddenFieldInput, CreateTeamInput, FormKind, FormReport,
    InteractiveMode, LoginInput, SignUpInput, Submissions,
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
                "/survey_tool/workflow_step/{workflow_step_id}/form_report",
                get_with(form_report, |op| {
                    op.id("HeyFormGetFormReport")
                        .tag("Tools")
                        .summary("Get HeyForm report data for a workflow step")
                        .description("Fetches HeyForm report data for the HeyForm tool attached to a workflow step")
                        .response::<200, Json<FormReport>>()
                }),
            )
            .api_route(
                "/survey_tool/workflow_step/{workflow_step_id}/submissions",
                get_with(submissions, |op| {
                    op.id("HeyFormGetSubmissions")
                        .tag("Tools")
                        .summary("Get HeyForm submissions for a workflow step")
                        .description("Fetches paginated HeyForm submissions and returns them as a single list")
                        .response::<200, Json<Submissions>>()
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

    let mut submissions = Vec::new();
    let category = match query.category {
        Some(cat) if matches!(cat.as_str(), "inbox" | "spam" | "starred" | "archive") => {
            cat.to_string()
        }
        None => "inbox".to_string(), // Default to inbox if no category is provided
        _ => return Err(ComhairleError::BadRequest("Invalid category".into())),
    };
    let mut page = 1;
    loop {
        let page_submissions = client
            .get_form_submissions(config.survey_id.clone(), category.clone(), page)
            .await?;

        if page_submissions.submissions.is_empty() {
            break;
        }
        submissions.extend(page_submissions.submissions);
        page += 1;
    }

    Ok((
        StatusCode::OK,
        Json(Submissions {
            total: submissions.len() as i32,
            submissions,
        }),
    ))
}

// Keep public function for backwards compatibility
pub async fn setup(setup_config: &HeyFormToolSetup) -> Result<HeyFormToolConfig, ComhairleError> {
    heyform_setup(setup_config).await
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use axum::{Router, extract::State, routing::post};
    use heyform_sdk::{
        ChoiceIcon, Choose, FormReport, FormReportAnswer, FormReportResponse, FormReportSubmission,
        HiddenFieldAnswer, MultipleChoice, Submission, SubmissionCategory, Submissions,
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
                chooses: Some(vec![Choose::MultipleChoice(MultipleChoice {
                    id: "choose-1".to_string(),
                    label: "Test Choose 1".to_string(),
                    image: Some("https://example.com/image.png".to_string()),
                    icon: Some(ChoiceIcon {
                        name: "star".to_string(),
                        color: "#FFD700".to_string(),
                        background: "#0000FF".to_string(),
                    }),
                    color: Some("#FF0000".to_string()),
                    score: Some(10),
                    is_expected: Some(true),
                    count: 5,
                })]),
            }],
            submissions: vec![FormReportSubmission {
                r#_id: "submission-1".to_string(),
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
}
