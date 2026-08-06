use std::sync::Arc;

use crate::models::polis_statement_aux;
use aide::axum::{
    ApiRouter,
    routing::{delete_with, get_with, post_with, put_with},
};
use async_trait::async_trait;
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::{
    ComhairleState,
    error::ComhairleError,
    models::{
        self,
        polis_statement_aux::{
            CreatePolisStatementAux, PolisStatementAux, PolisStatementAuxFilterOptions,
            ThemeStatistic, UpdatePolisStatementAux, UpsertFromPolis,
        },
    },
    routes::auth::{RequiredAdminUser, RequiredUser},
    wiki_poll_service::{
        ModerationStatus, WikiPoll, WikiPollConfigUpdate, WikiPollLogin, WikiPollService,
        polis_service::WikiPollReport,
    },
};

use super::{ToolConfig, ToolConfigSanitize, ToolImpl};

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema, PartialEq)]
pub struct PolisToolConfig {
    pub server_url: String,
    pub poll_id: String,
    pub admin_user: String,
    pub admin_password: String,
    pub required_votes: Option<i32>,
    #[serde(default = "default_show_remaining_statements")]
    pub show_remaining_statements: bool,
    // Mirror of the Polis conversation config. These are written through to
    // Polis via PolisUpdateConfig AND stored here so the Setup tab can pre-fill
    // them (Polis exposes no read path for topic/description/is_active/
    // strict_moderation).
    #[serde(default)]
    pub topic: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub is_active: Option<bool>,
    #[serde(default)]
    pub strict_moderation: Option<bool>,
    // comhairle-only display flag (not sent to Polis): style seed statements
    // with a "conversation starter" label in the participant embed.
    #[serde(default)]
    pub label_seeds_as_conversation_starter: bool,
}

fn default_show_remaining_statements() -> bool {
    true
}

impl ToolConfigSanitize for PolisToolConfig {
    fn sanitize(&self) -> Self {
        Self {
            admin_user: "".into(),
            admin_password: "".into(),
            server_url: self.server_url.clone(),
            poll_id: self.poll_id.clone(),
            required_votes: self.required_votes,
            show_remaining_statements: self.show_remaining_statements,
            topic: self.topic.clone(),
            description: self.description.clone(),
            is_active: self.is_active,
            strict_moderation: self.strict_moderation,
            label_seeds_as_conversation_starter: self.label_seeds_as_conversation_starter,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct PolisToolSetup {
    pub topic: String,
    pub required_votes: Option<i32>,
    #[serde(default = "default_show_remaining_statements")]
    pub show_remaining_statements: bool,
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct PolisReport;

/// Zero-sized marker type for Polis tool implementation
pub struct PolisTool;

#[async_trait]
impl ToolImpl for PolisTool {
    type Config = PolisToolConfig;
    type Setup = PolisToolSetup;
    type Report = PolisReport;

    async fn setup(
        setup: &Self::Setup,
        state: &Arc<ComhairleState>,
    ) -> Result<Self::Config, ComhairleError> {
        // Delegate to existing setup function
        polis_setup(setup, &state.config.polis_url, &state.wiki_poll_service).await
    }

    async fn clone_tool(
        config: &Self::Config,
        state: &Arc<ComhairleState>,
    ) -> Result<Self::Config, ComhairleError> {
        // Delegate to existing launch function
        launch(config, &state.wiki_poll_service).await
    }

    fn sanitize(config: Self::Config) -> Self::Config {
        config.sanitize()
    }

    async fn delete(
        config: &Self::Config,
        state: &Arc<ComhairleState>,
        _workflow_step_id: &Uuid,
    ) -> Result<(), ComhairleError> {
        let auth_cookies = state
            .wiki_poll_service
            .login(&WikiPollLogin {
                email: config.admin_user.clone(),
                password: config.admin_password.clone(),
            })
            .await?;

        let _poll = state
            .wiki_poll_service
            .delete_poll(&config.poll_id, &auth_cookies)
            .await?;

        Ok(())
    }

    fn routes(state: &Arc<ComhairleState>) -> ApiRouter {
        ApiRouter::new()
            .api_route(
                "/polis/report_data",
                get_with(get_report_data, |op| {
                    op.id("PolisGetReportData")
                        .tag("Tools")
                        .summary("Get Polis report data for a workflow step")
                        .description("Fetches the polis data export for a given workflow step")
                        .response::<200, Json<WikiPollReport>>()
                }),
            )
            .api_route(
                "/polis/config",
                put_with(update_polis_config, |op| {
                    op.id("PolisUpdateConfig")
                        .tag("Tools")
                        .summary("Update the Polis conversation configuration")
                        .description(
                            "Proxies topic, description, strict_moderation and is_active to the \
                             Polis conversation via the server-side admin session. Only provided \
                             fields are written.",
                        )
                        .response::<200, Json<WikiPoll>>()
                }),
            )
            .api_route(
                "/polis/seed",
                post_with(post_seed, |op| {
                    op.id("PolisPostSeed")
                        .tag("Tools")
                        .summary("Post a seed statement to the Polis conversation")
                        .description(
                            "Posts a moderator-authored seed statement (is_seed) to the active \
                             Polis poll via the server-side admin session. Re-sync to surface it \
                             in the local statement_aux table.",
                        )
                        .response::<201, Json<PostSeedResponse>>()
                }),
            )
            .api_route(
                "/polis/statement_aux",
                post_with(create_statement_aux, |op| {
                    op.id("PolisCreateStatementAux")
                        .tag("Tools")
                        .summary("Create auxiliary data for a Polis statement")
                        .description(
                            "Creates a polis_statement_aux row capturing statement text, \
                             moderation status, themes and the visible statement at \
                             submission time",
                        )
                        .response::<201, Json<PolisStatementAux>>()
                }),
            )
            .api_route(
                "/polis/statement_aux/{id}",
                put_with(update_statement_aux, |op| {
                    op.id("PolisUpdateStatementAux")
                        .tag("Tools")
                        .summary("Update auxiliary data for a Polis statement")
                        .description(
                            "Updates statement_text, moderation_status, themes, \
                             visible_statement_when_submitted, or moderation_reason",
                        )
                        .response::<200, Json<PolisStatementAux>>()
                }),
            )
            .api_route(
                "/polis/statement_aux",
                get_with(list_statement_aux, |op| {
                    op.id("PolisListStatementAux")
                        .tag("Tools")
                        .summary("List polis_statement_aux rows for a poll")
                        .description(
                            "Returns auxiliary statement data filtered by workflow_step_id \
                             and/or polis_conversation_id (at least one is required)",
                        )
                        .response::<200, Json<Vec<PolisStatementAux>>>()
                }),
            )
            .api_route(
                "/polis/statement_aux/sync",
                post_with(sync_statement_aux, |op| {
                    op.id("PolisSyncStatementAux")
                        .tag("Tools")
                        .summary("Sync polis_statement_aux with the live Polis poll")
                        .description(
                            "Fetches comments and xid mappings from Polis and upserts a row \
                             per statement. Existing rows have their statement_text and \
                             is_seed refreshed; moderation_status, moderation_reason, themes, \
                             visible_statement_when_submitted and user_id are preserved.",
                        )
                        .response::<200, Json<SyncStatementAuxResponse>>()
                }),
            )
            .api_route(
                "/polis/statement_aux/theme_stats",
                get_with(theme_stats, |op| {
                    op.id("PolisStatementAuxThemeStats")
                        .tag("Tools")
                        .summary("Statement counts per theme for a poll")
                        .description(
                            "Returns the count of polis_statement_aux rows tagged with each \
                             theme, filtered by workflow_step_id and/or polis_conversation_id \
                             (at least one is required)",
                        )
                        .response::<200, Json<Vec<ThemeStatistic>>>()
                }),
            )
            .api_route(
                "/polis/statement_aux/{id}/themes",
                post_with(add_statement_aux_theme, |op| {
                    op.id("PolisAddStatementAuxTheme")
                        .tag("Tools")
                        .summary("Add a theme to a polis_statement_aux row")
                        .description(
                            "Adds a theme to the statement's themes array. Idempotent: \
                             adding a theme that is already present is a no-op. Caller \
                             must be the owner of the conversation the statement belongs to.",
                        )
                        .response::<200, Json<PolisStatementAux>>()
                }),
            )
            .api_route(
                "/polis/statement_aux/{id}/themes",
                delete_with(remove_statement_aux_theme, |op| {
                    op.id("PolisRemoveStatementAuxTheme")
                        .tag("Tools")
                        .summary("Remove a theme from a polis_statement_aux row")
                        .description(
                            "Removes a theme from the statement's themes array. Idempotent: \
                             removing a theme that is not present is a no-op. Caller must be \
                             the owner of the conversation the statement belongs to.",
                        )
                        .response::<200, Json<PolisStatementAux>>()
                }),
            )
            .api_route(
                "/polis/statement_aux/{id}/moderate",
                post_with(moderate_statement_aux, |op| {
                    op.id("PolisModerateStatementAux")
                        .tag("Tools")
                        .summary("Moderate a Polis statement")
                        .description(
                            "Forwards a moderation decision (accept/reject) to the Polis \
                             server using the admin account, then updates the \
                             polis_statement_aux row's moderation_status and \
                             moderation_reason",
                        )
                        .response::<200, Json<PolisStatementAux>>()
                }),
            )
            .api_route(
                "/polis/statement_aux/moderate_batch",
                post_with(moderate_statement_aux_batch, |op| {
                    op.id("PolisModerateStatementAuxBatch")
                        .tag("Tools")
                        .summary("Moderate multiple Polis statements in one request")
                        .description(
                            "Forwards an accept/reject decision for many polis_statement_aux \
                             rows to Polis using a single admin login, then bulk-updates the \
                             rows that succeeded. All ids must belong to the same workflow \
                             step. Returns the updated rows plus any per-row failures.",
                        )
                        .response::<200, Json<ModerateStatementAuxBatchResponse>>()
                }),
            )
            .with_state(state.clone())
    }
}

#[derive(Error, Debug)]
pub enum PolisError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Failed to create new admin user")]
    FailedToCreateNewAdminUser(StatusCode),

    #[error("Failed to login")]
    FailedToLogin(StatusCode),

    #[error("Failed to create new poll")]
    FailedToCreateNewPoll(StatusCode),

    #[error("Failed to get comments {0}")]
    FailedToGetComments(StatusCode, String),

    #[error("Failed to get xids {0}")]
    FailedToGetXIDs(StatusCode, String),

    #[error("Failed to update poll {0}")]
    FailedPollUpdate(StatusCode, String),

    #[error("Failed to post seed comment {0}")]
    FailedToPostSeedComment(StatusCode, String),

    #[error("Failed to moderate comment {0}")]
    FailedToModerateComment(StatusCode, String),

    #[error("Failed to proxy route {from} : {to}")]
    ProxyError { from: String, to: String },
}

impl Into<StatusCode> for &PolisError {
    fn into(self) -> StatusCode {
        match self {
            PolisError::Http(err) => {
                if let Some(status) = err.status() {
                    status
                } else {
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            }
            PolisError::FailedToCreateNewAdminUser(status) => *status,
            PolisError::FailedToLogin(status) => *status,
            PolisError::FailedToCreateNewPoll(status) => *status,
            PolisError::FailedToGetComments(status, _) => *status,
            PolisError::FailedToGetXIDs(status, _) => *status,
            PolisError::FailedPollUpdate(status, _) => *status,
            PolisError::FailedToPostSeedComment(status, _) => *status,
            PolisError::FailedToModerateComment(status, _) => *status,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Deserialize, Serialize)]
struct NewAdminUser {
    pub hname: String,
    pub password: String,
    pub password2: String,
    pub email: String,
    pub gatekeeperTosPrivacy: bool,
}

#[derive(Deserialize, Serialize)]
struct PolisLogin {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct NewUserResp {
    pub uid: u32,
    pub hname: String,
    pub email: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct NewPollResp {
    conversation_id: String,
}

pub struct PolisClient {
    client: reqwest::Client,
    base_url: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PolisCommentCreateResponse {
    tid: u8,
    current_pid: u8,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginResp {
    pub uid: u32,
    pub email: String,
    pub token: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SetTopicRequest {
    pub topic: String,
    pub conversation_id: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct AdminLoginQuery {
    pub workflow_step_id: Uuid,
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct ReportDataQuery {
    pub workflow_step_id: Uuid,
}

/// Gets the polis report data for a workflow step

#[instrument(err(Debug), skip(state))]
async fn get_report_data(
    State(state): State<Arc<ComhairleState>>,
    Query(ReportDataQuery { workflow_step_id }): Query<ReportDataQuery>,
) -> Result<(StatusCode, Json<WikiPollReport>), ComhairleError> {
    let workflow_step = models::workflow_step::get_by_id(&state.db, &workflow_step_id).await?;

    let config = match (workflow_step.tool_config, workflow_step.preview_tool_config) {
        (Some(ToolConfig::Polis(config)), _) => config,
        (None, ToolConfig::Polis(config)) => config,
        _ => return Err(ComhairleError::WorkflowStepHasWrongType("Polis".into())),
    };

    // Get the report data
    let data = state
        .wiki_poll_service
        .get_report_data(&config.poll_id)
        .await?;

    Ok((StatusCode::OK, Json(data)))
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct UpdatePolisConfigRequest {
    pub workflow_step_id: Uuid,
    pub is_active: Option<bool>,
    pub topic: Option<String>,
    pub description: Option<String>,
    pub strict_moderation: Option<bool>,
}

/// Writes Polis conversation-level configuration (topic, description,
/// strict_moderation, is_active) through the server-side admin session.
#[instrument(err(Debug), skip(state))]
async fn update_polis_config(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    Json(request): Json<UpdatePolisConfigRequest>,
) -> Result<(StatusCode, Json<WikiPoll>), ComhairleError> {
    let workflow_step =
        models::workflow_step::get_by_id(&state.db, &request.workflow_step_id).await?;
    models::workflow::check_user_is_owner(&state.db, &workflow_step.workflow_id, &user.id).await?;

    let config = match (workflow_step.tool_config, workflow_step.preview_tool_config) {
        (Some(ToolConfig::Polis(config)), _) => config,
        (None, ToolConfig::Polis(config)) => config,
        _ => return Err(ComhairleError::WorkflowStepHasWrongType("Polis".into())),
    };

    let client = &state.wiki_poll_service;
    let auth_cookies = client
        .login(&WikiPollLogin {
            email: config.admin_user.clone(),
            password: config.admin_password.clone(),
        })
        .await?;

    let poll = client
        .update_poll_config(
            &config.poll_id,
            &auth_cookies,
            &WikiPollConfigUpdate {
                is_active: request.is_active,
                topic: request.topic,
                description: request.description,
                strict_moderation: request.strict_moderation,
            },
        )
        .await?;

    Ok((StatusCode::OK, Json(poll)))
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct PostSeedRequest {
    pub workflow_step_id: Uuid,
    pub statement_text: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct PostSeedResponse {
    /// The Polis statement id (tid) of the newly created seed.
    pub polis_statement_id: String,
}

/// Posts a moderator-authored seed statement to the active Polis poll through
/// the server-side admin session. Callers should re-sync afterwards to surface
/// the seed in the local statement_aux table.
#[instrument(err(Debug), skip(state))]
async fn post_seed(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    Json(request): Json<PostSeedRequest>,
) -> Result<(StatusCode, Json<PostSeedResponse>), ComhairleError> {
    let workflow_step =
        models::workflow_step::get_by_id(&state.db, &request.workflow_step_id).await?;
    models::workflow::check_user_is_owner(&state.db, &workflow_step.workflow_id, &user.id).await?;

    let config = match (workflow_step.tool_config, workflow_step.preview_tool_config) {
        (Some(ToolConfig::Polis(config)), _) => config,
        (None, ToolConfig::Polis(config)) => config,
        _ => return Err(ComhairleError::WorkflowStepHasWrongType("Polis".into())),
    };

    let client = &state.wiki_poll_service;
    let auth_cookies = client
        .login(&WikiPollLogin {
            email: config.admin_user.clone(),
            password: config.admin_password.clone(),
        })
        .await?;

    let polis_statement_id = client
        .post_seed_comment(&request.statement_text, &config.poll_id, &auth_cookies)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(PostSeedResponse { polis_statement_id }),
    ))
}

#[instrument(err(Debug), skip(state))]
async fn create_statement_aux(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Json(create_request): Json<CreatePolisStatementAux>,
) -> Result<(StatusCode, Json<PolisStatementAux>), ComhairleError> {
    let workflow_step =
        models::workflow_step::get_by_id(&state.db, &create_request.workflow_step_id).await?;

    models::user_participation::check_user_participating(
        &state.db,
        &workflow_step.workflow_id,
        &user.id,
    )
    .await?;

    let aux = models::polis_statement_aux::create(&state.db, user.id, &create_request).await?;
    Ok((StatusCode::CREATED, Json(aux)))
}

#[instrument(err(Debug), skip(state))]
async fn update_statement_aux(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path(id): Path<Uuid>,
    Json(update_request): Json<UpdatePolisStatementAux>,
) -> Result<(StatusCode, Json<PolisStatementAux>), ComhairleError> {
    models::polis_statement_aux::check_is_commentor(&state.db, &id, &user.id).await?;
    let aux = models::polis_statement_aux::update(&state.db, id, &update_request).await?;
    Ok((StatusCode::OK, Json(aux)))
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
struct StatementAuxFilterQuery {
    pub workflow_step_id: Option<Uuid>,
    pub polis_conversation_id: Option<String>,
}

#[instrument(err(Debug), skip(state))]
async fn list_statement_aux(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(_user): RequiredUser,
    Query(filter): Query<StatementAuxFilterQuery>,
) -> Result<(StatusCode, Json<Vec<PolisStatementAux>>), ComhairleError> {
    if filter.workflow_step_id.is_none() && filter.polis_conversation_id.is_none() {
        return Err(ComhairleError::BadRequest(
            "workflow_step_id or polis_conversation_id is required".into(),
        ));
    }
    let aux = models::polis_statement_aux::list(
        &state.db,
        filter.workflow_step_id,
        filter.polis_conversation_id,
        PolisStatementAuxFilterOptions::default(),
    )
    .await?;
    Ok((StatusCode::OK, Json(aux)))
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct SyncStatementAuxRequest {
    pub workflow_step_id: Uuid,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct SyncStatementAuxResponse {
    pub synced: usize,
    pub skipped_invalid_xid: usize,
    pub statements: Vec<PolisStatementAux>,
}

#[instrument(err(Debug), skip(state))]
async fn sync_statement_aux(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Json(SyncStatementAuxRequest { workflow_step_id }): Json<SyncStatementAuxRequest>,
) -> Result<(StatusCode, Json<SyncStatementAuxResponse>), ComhairleError> {
    let workflow_step = models::workflow_step::get_by_id(&state.db, &workflow_step_id).await?;
    models::workflow::check_user_is_owner(&state.db, &workflow_step.workflow_id, &user.id).await?;

    let config = match (workflow_step.tool_config, workflow_step.preview_tool_config) {
        (Some(ToolConfig::Polis(config)), _) => config,
        (None, ToolConfig::Polis(config)) => config,
        _ => return Err(ComhairleError::WorkflowStepHasWrongType("Polis".into())),
    };

    let response = sync_statement_aux_inner(&state, &workflow_step_id, &config).await?;
    Ok((StatusCode::OK, Json(response)))
}

/// Fetch comments and xid mappings from Polis and upsert a `polis_statement_aux`
/// row per statement. Shared by the HTTP sync endpoint and the workflow-step
/// launch path (so going live seeds the aux table immediately).
#[instrument(err(Debug), skip(state))]
pub async fn sync_statement_aux_inner(
    state: &Arc<ComhairleState>,
    workflow_step_id: &Uuid,
    config: &PolisToolConfig,
) -> Result<SyncStatementAuxResponse, ComhairleError> {
    let client = &state.wiki_poll_service;
    let auth_cookies = client
        .login(&WikiPollLogin {
            email: config.admin_user.clone(),
            password: config.admin_password.clone(),
        })
        .await?;

    let xid_rows = client.get_xids(&config.poll_id, &auth_cookies).await?;

    let pid_to_user_id: std::collections::HashMap<u32, Uuid> = xid_rows
        .into_iter()
        .filter_map(|row| Uuid::parse_str(&row.xid).ok().map(|u| (row.pid, u)))
        .collect();

    let comments = client.get_comments(&config.poll_id).await?;

    let mut statements = Vec::with_capacity(comments.len());
    let mut skipped_invalid_xid = 0;

    info!("COMMENTS {comments:#?}");

    for comment in comments {
        let user_id = pid_to_user_id.get(&comment.pid).copied();
        if user_id.is_none() && !comment.is_seed {
            skipped_invalid_xid += 1;
        }
        let aux = models::polis_statement_aux::upsert_from_polis(
            &state.db,
            &UpsertFromPolis {
                workflow_step_id: *workflow_step_id,
                user_id,
                zid: comment.pid as i32,
                polis_conversation_id: config.poll_id.clone(),
                polis_statement_id: comment.tid as i32,
                statement_text: comment.txt,
                is_seed: comment.is_seed,
                moderation_status: comment.moderation.try_into()?,
            },
        )
        .await?;
        statements.push(aux);
    }

    Ok(SyncStatementAuxResponse {
        synced: statements.len(),
        skipped_invalid_xid,
        statements,
    })
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ModerationDecisionRequest {
    Accept,
    Reject,
}

impl From<ModerationDecisionRequest> for ModerationStatus {
    fn from(value: ModerationDecisionRequest) -> Self {
        match value {
            ModerationDecisionRequest::Accept => ModerationStatus::Accepted,
            ModerationDecisionRequest::Reject => ModerationStatus::Rejected,
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct ModerateStatementAuxRequest {
    pub decision: ModerationDecisionRequest,
    pub moderation_reason: Option<String>,
}

#[instrument(err(Debug), skip(state))]
async fn moderate_statement_aux(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path(statement_id): Path<Uuid>,
    Json(request): Json<ModerateStatementAuxRequest>,
) -> Result<(StatusCode, Json<PolisStatementAux>), ComhairleError> {
    let aux = models::polis_statement_aux::get_by_id(&state.db, &statement_id).await?;

    polis_statement_aux::check_can_moderate(&state.db, &user, &aux.workflow_step_id).await?;

    let workflow_step = models::workflow_step::get_by_id(&state.db, &aux.workflow_step_id).await?;

    let config = match (workflow_step.tool_config, workflow_step.preview_tool_config) {
        (Some(ToolConfig::Polis(config)), _) => config,
        (None, ToolConfig::Polis(config)) => config,
        _ => return Err(ComhairleError::WorkflowStepHasWrongType("Polis".into())),
    };

    let client = &state.wiki_poll_service;
    let auth_cookies = client
        .login(&WikiPollLogin {
            email: config.admin_user.clone(),
            password: config.admin_password.clone(),
        })
        .await?;

    client
        .moderate_comment(
            &config.poll_id,
            aux.polis_statement_id,
            request.decision.into(),
            &auth_cookies,
        )
        .await?;

    let updated = models::polis_statement_aux::update(
        &state.db,
        statement_id,
        &UpdatePolisStatementAux {
            statement_text: None,
            moderation_status: Some(request.decision.into()),
            themes: None,
            visible_statement_when_submitted: None,
            moderation_reason: request.moderation_reason,
        },
    )
    .await?;

    Ok((StatusCode::OK, Json(updated)))
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct ModerateStatementAuxBatchRequest {
    /// polis_statement_aux ids to moderate. All must belong to the same workflow step.
    pub ids: Vec<Uuid>,
    pub decision: ModerationDecisionRequest,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct ModerateBatchFailure {
    pub id: Uuid,
    pub error: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct ModerateStatementAuxBatchResponse {
    /// Rows whose decision was applied in Polis and persisted locally.
    pub succeeded: Vec<PolisStatementAux>,
    /// Rows whose Polis moderation call failed; their status is left unchanged.
    pub failed: Vec<ModerateBatchFailure>,
}

#[instrument(err(Debug), skip(state))]
async fn moderate_statement_aux_batch(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Json(request): Json<ModerateStatementAuxBatchRequest>,
) -> Result<(StatusCode, Json<ModerateStatementAuxBatchResponse>), ComhairleError> {
    if request.ids.is_empty() {
        return Err(ComhairleError::BadRequest("ids must not be empty".into()));
    }

    // Load every target row up front: this validates the ids and gives us the
    // polis_statement_ids to forward, without a DB round-trip per row.
    let rows = models::polis_statement_aux::list(
        &state.db,
        None,
        None,
        PolisStatementAuxFilterOptions::by_ids(request.ids.clone()),
    )
    .await?;
    if rows.len() != request.ids.len() {
        return Err(ComhairleError::BadRequest(
            "one or more statement ids do not exist".into(),
        ));
    }

    // A selection always comes from a single moderation view (one workflow step,
    // one poll), so we authorize and log in to Polis exactly once for the batch.
    let workflow_step_id = rows[0].workflow_step_id;
    if rows.iter().any(|r| r.workflow_step_id != workflow_step_id) {
        return Err(ComhairleError::BadRequest(
            "all statements must belong to the same workflow step".into(),
        ));
    }

    polis_statement_aux::check_can_moderate(&state.db, &user, &workflow_step_id).await?;

    let workflow_step = models::workflow_step::get_by_id(&state.db, &workflow_step_id).await?;
    let config = match (workflow_step.tool_config, workflow_step.preview_tool_config) {
        (Some(ToolConfig::Polis(config)), _) => config,
        (None, ToolConfig::Polis(config)) => config,
        _ => return Err(ComhairleError::WorkflowStepHasWrongType("Polis".into())),
    };

    let client = &state.wiki_poll_service;
    let auth_cookies = client
        .login(&WikiPollLogin {
            email: config.admin_user.clone(),
            password: config.admin_password.clone(),
        })
        .await?;

    let status: ModerationStatus = request.decision.into();

    // Forward each decision to Polis, collecting per-row failures instead of
    // aborting the whole batch on the first error.
    let mut succeeded_ids: Vec<Uuid> = Vec::new();
    let mut failed: Vec<ModerateBatchFailure> = Vec::new();
    for row in &rows {
        match client
            .moderate_comment(
                &config.poll_id,
                row.polis_statement_id,
                status.clone(),
                &auth_cookies,
            )
            .await
        {
            Ok(()) => succeeded_ids.push(row.id),
            Err(e) => {
                tracing::error!("Batch moderate failed for {}: {e:?}", row.id);
                failed.push(ModerateBatchFailure {
                    id: row.id,
                    error: "Failed to moderate statement in Polis".into(),
                });
            }
        }
    }

    // Persist moderation_status for the rows Polis accepted, in one statement.
    let succeeded = models::polis_statement_aux::update_many(
        &state.db,
        &succeeded_ids,
        &UpdatePolisStatementAux {
            moderation_status: Some(status),
            ..Default::default()
        },
    )
    .await?;

    Ok((
        StatusCode::OK,
        Json(ModerateStatementAuxBatchResponse { succeeded, failed }),
    ))
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct ThemeRequest {
    pub theme: String,
}

#[instrument(err(Debug), skip(state))]
async fn add_statement_aux_theme(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path(statement_id): Path<Uuid>,
    Json(request): Json<ThemeRequest>,
) -> Result<(StatusCode, Json<PolisStatementAux>), ComhairleError> {
    let aux = models::polis_statement_aux::get_by_id(&state.db, &statement_id).await?;
    polis_statement_aux::check_can_moderate(&state.db, &user, &aux.workflow_step_id).await?;

    let updated =
        models::polis_statement_aux::add_theme(&state.db, statement_id, &request.theme).await?;
    Ok((StatusCode::OK, Json(updated)))
}

#[instrument(err(Debug), skip(state))]
async fn remove_statement_aux_theme(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path(statement_id): Path<Uuid>,
    Json(request): Json<ThemeRequest>,
) -> Result<(StatusCode, Json<PolisStatementAux>), ComhairleError> {
    let aux = models::polis_statement_aux::get_by_id(&state.db, &statement_id).await?;
    polis_statement_aux::check_can_moderate(&state.db, &user, &aux.workflow_step_id).await?;

    let updated =
        models::polis_statement_aux::remove_theme(&state.db, statement_id, &request.theme).await?;
    Ok((StatusCode::OK, Json(updated)))
}

#[instrument(err(Debug), skip(state))]
async fn theme_stats(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(_user): RequiredUser,
    Query(filter): Query<StatementAuxFilterQuery>,
) -> Result<(StatusCode, Json<Vec<ThemeStatistic>>), ComhairleError> {
    if filter.workflow_step_id.is_none() && filter.polis_conversation_id.is_none() {
        return Err(ComhairleError::BadRequest(
            "workflow_step_id or polis_conversation_id is required".into(),
        ));
    }
    let stats = models::polis_statement_aux::theme_stats(
        &state.db,
        filter.workflow_step_id,
        filter.polis_conversation_id,
    )
    .await?;
    Ok((StatusCode::OK, Json(stats)))
}

#[instrument(err(Debug), skip(client))]
pub async fn launch(
    preview_config: &PolisToolConfig,
    client: &Arc<dyn WikiPollService>,
) -> Result<PolisToolConfig, ComhairleError> {
    // Login as preview config user
    client
        .login(&WikiPollLogin {
            email: preview_config.admin_user.clone(),
            password: preview_config.admin_password.clone(),
        })
        .await?;

    // Need to also migrate the setting for moderation
    let seed_statements = client.get_comments(&preview_config.poll_id).await?;

    let live_poll_config = polis_setup(
        &PolisToolSetup {
            topic: "".into(),
            required_votes: preview_config.required_votes,
            show_remaining_statements: preview_config.show_remaining_statements,
        },
        &preview_config.server_url,
        client,
    )
    .await?;

    // Login as live config user
    let live_auth_cookies = client
        .login(&WikiPollLogin {
            email: live_poll_config.admin_user.clone(),
            password: live_poll_config.admin_password.clone(),
        })
        .await?;

    // TODO: filter seed statements.

    for comment in seed_statements {
        client
            .post_seed_comment(&comment.txt, &live_poll_config.poll_id, &live_auth_cookies)
            .await?;
    }

    Ok(live_poll_config)
}

async fn polis_setup(
    setup: &PolisToolSetup,
    polis_url: &str,
    client: &Arc<dyn WikiPollService>,
) -> Result<PolisToolConfig, ComhairleError> {
    let (email, password) = client.create_random_admin_user().await?;
    let auth_cookies = client
        .login(&WikiPollLogin {
            email: email.clone(),
            password: password.clone(),
        })
        .await?;
    let poll_id = client.create_poll(&auth_cookies).await?;

    Ok(PolisToolConfig {
        server_url: polis_url.to_string(),
        poll_id,
        admin_user: email,
        admin_password: password,
        required_votes: Some(setup.required_votes.unwrap_or(10)),
        show_remaining_statements: setup.show_remaining_statements,
        // Mirror Polis's creation defaults so the Setup tab reflects reality
        // before the admin edits anything.
        topic: None,
        description: None,
        is_active: Some(true),
        strict_moderation: Some(false),
        label_seeds_as_conversation_starter: false,
    })
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::sync::Arc;

    use axum::Router;
    use serde_json::json;
    use sqlx::PgPool;

    use crate::{
        models::{
            model_test_helpers::setup_default_app_and_session,
            polis_statement_aux::{self, CreatePolisStatementAux},
        },
        setup_server,
        test_helpers::{UserSession, extract, polis_tool_config, test_state},
        wiki_poll_service::{MockWikiPollService, WikiPollService, error::WikiPollServiceError},
    };

    use super::*;

    async fn setup_polis_aux(
        app: &Router,
        pool: &PgPool,
        session: &mut UserSession,
        themes: Vec<String>,
    ) -> Result<PolisStatementAux, Box<dyn Error>> {
        let (_, conversation, _) = session.create_random_conversation(app).await?;
        let conversation_id: String = extract("id", &conversation);

        let (_, workflow, _) = session
            .create_random_workflow(app, &conversation_id)
            .await?;
        let workflow_id: String = extract("id", &workflow);

        let (_, workflow_step, _) = session
            .post(
                app,
                &format!("/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step"),
                json!({
                    "name": "Polis step",
                    "step_order": 1,
                    "activation_rule": "manual",
                    "description": "polis step",
                    "is_offline": false,
                    "required": true,
                    "tool_setup": polis_tool_config(),
                })
                .to_string()
                .into(),
            )
            .await?;
        let workflow_step_id: String = extract("id", &workflow_step);
        let workflow_step_id = Uuid::parse_str(&workflow_step_id)?;

        let owner_id = session.id.expect("session to be signed up");
        let aux = polis_statement_aux::create(
            pool,
            owner_id,
            &CreatePolisStatementAux {
                workflow_step_id,
                zid: 1,
                polis_conversation_id: "test-poll".into(),
                polis_statement_id: 1,
                statement_text: "test statement".into(),
                is_seed: false,
                themes,
                ..Default::default()
            },
        )
        .await?;

        Ok(aux)
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn owner_can_add_theme(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let aux = setup_polis_aux(&app, &pool, &mut session, vec![]).await?;

        let (status, value, _) = session
            .post(
                &app,
                &format!("/tools/polis/statement_aux/{}/themes", aux.id),
                json!({ "theme": "alpha" }).to_string().into(),
            )
            .await?;

        assert_eq!(
            status,
            StatusCode::OK,
            "owner should be able to add a theme"
        );
        let updated: PolisStatementAux = serde_json::from_value(value)?;
        assert_eq!(updated.themes, vec!["alpha".to_string()]);
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn adding_theme_is_idempotent(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let aux = setup_polis_aux(&app, &pool, &mut session, vec!["alpha".into()]).await?;

        let (status, value, _) = session
            .post(
                &app,
                &format!("/tools/polis/statement_aux/{}/themes", aux.id),
                json!({ "theme": "alpha" }).to_string().into(),
            )
            .await?;

        assert_eq!(status, StatusCode::OK);
        let updated: PolisStatementAux = serde_json::from_value(value)?;
        assert_eq!(
            updated.themes,
            vec!["alpha".to_string()],
            "re-adding an existing theme should be a no-op"
        );
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn owner_can_remove_theme(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let aux = setup_polis_aux(
            &app,
            &pool,
            &mut session,
            vec!["alpha".into(), "beta".into()],
        )
        .await?;

        let (status, value, _) = session
            .delete_with_body(
                &app,
                &format!("/tools/polis/statement_aux/{}/themes", aux.id),
                json!({ "theme": "alpha" }).to_string().into(),
            )
            .await?;

        assert_eq!(status, StatusCode::OK);
        let updated: PolisStatementAux = serde_json::from_value(value)?;
        assert_eq!(updated.themes, vec!["beta".to_string()]);
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn removing_missing_theme_is_idempotent(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let aux = setup_polis_aux(&app, &pool, &mut session, vec!["alpha".into()]).await?;

        let (status, value, _) = session
            .delete_with_body(
                &app,
                &format!("/tools/polis/statement_aux/{}/themes", aux.id),
                json!({ "theme": "missing" }).to_string().into(),
            )
            .await?;

        assert_eq!(status, StatusCode::OK);
        let updated: PolisStatementAux = serde_json::from_value(value)?;
        assert_eq!(updated.themes, vec!["alpha".to_string()]);
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn non_owner_cannot_add_theme(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut owner) = setup_default_app_and_session(&pool).await?;
        let aux = setup_polis_aux(&app, &pool, &mut owner, vec![]).await?;

        let mut intruder = UserSession::new(
            "intruder",
            crate::test_helpers::TEST_PASSWORD,
            "intruder@example.com",
        );
        intruder.signup(&app).await?;

        let (status, _, _) = intruder
            .post(
                &app,
                &format!("/tools/polis/statement_aux/{}/themes", aux.id),
                json!({ "theme": "alpha" }).to_string().into(),
            )
            .await?;

        assert_eq!(
            status,
            StatusCode::FORBIDDEN,
            "non-owner should be forbidden"
        );

        let reloaded = polis_statement_aux::get_by_id(&pool, &aux.id).await?;
        assert!(reloaded.themes.is_empty(), "themes must not have changed");
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn non_owner_cannot_remove_theme(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut owner) = setup_default_app_and_session(&pool).await?;
        let aux = setup_polis_aux(&app, &pool, &mut owner, vec!["alpha".into()]).await?;

        let mut intruder = UserSession::new(
            "intruder",
            crate::test_helpers::TEST_PASSWORD,
            "intruder@example.com",
        );
        intruder.signup(&app).await?;

        let (status, _, _) = intruder
            .delete_with_body(
                &app,
                &format!("/tools/polis/statement_aux/{}/themes", aux.id),
                json!({ "theme": "alpha" }).to_string().into(),
            )
            .await?;

        assert_eq!(status, StatusCode::FORBIDDEN);

        let reloaded = polis_statement_aux::get_by_id(&pool, &aux.id).await?;
        assert_eq!(reloaded.themes, vec!["alpha".to_string()]);
        Ok(())
    }

    /// Create a conversation + workflow + Polis workflow step, returning the step id.
    async fn setup_polis_step(
        app: &Router,
        session: &mut UserSession,
    ) -> Result<Uuid, Box<dyn Error>> {
        let (_, conversation, _) = session.create_random_conversation(app).await?;
        let conversation_id: String = extract("id", &conversation);

        let (_, workflow, _) = session
            .create_random_workflow(app, &conversation_id)
            .await?;
        let workflow_id: String = extract("id", &workflow);

        let (_, workflow_step, _) = session
            .post(
                app,
                &format!("/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step"),
                json!({
                    "name": "Polis step",
                    "step_order": 1,
                    "activation_rule": "manual",
                    "description": "polis step",
                    "is_offline": false,
                    "required": true,
                    "tool_setup": polis_tool_config(),
                })
                .to_string()
                .into(),
            )
            .await?;
        let workflow_step_id: String = extract("id", &workflow_step);
        Ok(Uuid::parse_str(&workflow_step_id)?)
    }

    /// Insert a pending aux row using `tid` as both zid and polis_statement_id.
    async fn insert_pending_aux(
        pool: &PgPool,
        owner_id: Uuid,
        workflow_step_id: Uuid,
        tid: i32,
    ) -> Result<PolisStatementAux, Box<dyn Error>> {
        let aux = polis_statement_aux::create(
            pool,
            owner_id,
            &CreatePolisStatementAux {
                workflow_step_id,
                zid: tid,
                polis_conversation_id: "test-poll".into(),
                polis_statement_id: tid,
                statement_text: format!("statement {tid}"),
                is_seed: false,
                ..Default::default()
            },
        )
        .await?;
        Ok(aux)
    }

    /// A Polis mock that handles step provisioning normally but makes
    /// `moderate_comment` fail for the given statement ids (tids), so we can
    /// exercise the partial-failure path.
    fn poll_service_failing_tids(failing: Vec<i32>) -> Arc<dyn WikiPollService> {
        let mut service = MockWikiPollService::new();
        service
            .expect_create_random_admin_user()
            .returning(|| Box::pin(async { Ok(("u@mock.com".to_string(), "pw".to_string())) }));
        service
            .expect_login()
            .returning(|_| Box::pin(async { Ok("cookie".to_string()) }));
        service
            .expect_create_poll()
            .returning(|_| Box::pin(async { Ok("test_poll_id".to_string()) }));
        service
            .expect_moderate_comment()
            .returning(move |_poll, tid, _decision, _cookies| {
                let fails = failing.contains(&tid);
                Box::pin(async move {
                    if fails {
                        Err(WikiPollServiceError::UnknownModerationStatus)
                    } else {
                        Ok(())
                    }
                })
            });
        Arc::new(service)
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn owner_can_batch_moderate(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let workflow_step_id = setup_polis_step(&app, &mut session).await?;
        let owner_id = session.id.expect("session to be signed up");

        let a = insert_pending_aux(&pool, owner_id, workflow_step_id, 1).await?;
        let b = insert_pending_aux(&pool, owner_id, workflow_step_id, 2).await?;
        let c = insert_pending_aux(&pool, owner_id, workflow_step_id, 3).await?;

        let (status, value, _) = session
            .post(
                &app,
                "/tools/polis/statement_aux/moderate_batch",
                json!({ "ids": [a.id, b.id, c.id], "decision": "accept" })
                    .to_string()
                    .into(),
            )
            .await?;

        assert_eq!(status, StatusCode::OK);
        let response: ModerateStatementAuxBatchResponse = serde_json::from_value(value)?;
        assert_eq!(response.succeeded.len(), 3, "all three should succeed");
        assert!(response.failed.is_empty(), "no failures expected");
        assert!(
            response
                .succeeded
                .iter()
                .all(|s| s.moderation_status == ModerationStatus::Accepted),
            "returned rows should be accepted"
        );

        for aux in [&a, &b, &c] {
            let reloaded = polis_statement_aux::get_by_id(&pool, &aux.id).await?;
            assert_eq!(reloaded.moderation_status, ModerationStatus::Accepted);
        }
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn batch_moderate_reports_partial_failures(pool: PgPool) -> Result<(), Box<dyn Error>> {
        // Polis rejects the moderation for statement 2 only.
        let service = poll_service_failing_tids(vec![2]);
        let state = test_state()
            .db(pool.clone())
            .wiki_poll_service(service)
            .call()?;
        let app = setup_server(Arc::new(state)).await?;
        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let workflow_step_id = setup_polis_step(&app, &mut session).await?;
        let owner_id = session.id.expect("session to be signed up");

        let a = insert_pending_aux(&pool, owner_id, workflow_step_id, 1).await?;
        let b = insert_pending_aux(&pool, owner_id, workflow_step_id, 2).await?;
        let c = insert_pending_aux(&pool, owner_id, workflow_step_id, 3).await?;

        let (status, value, _) = session
            .post(
                &app,
                "/tools/polis/statement_aux/moderate_batch",
                json!({ "ids": [a.id, b.id, c.id], "decision": "accept" })
                    .to_string()
                    .into(),
            )
            .await?;

        assert_eq!(
            status,
            StatusCode::OK,
            "the batch is a 200 even when some rows fail"
        );
        let response: ModerateStatementAuxBatchResponse = serde_json::from_value(value)?;

        assert_eq!(response.succeeded.len(), 2, "statements 1 and 3 succeed");
        assert_eq!(response.failed.len(), 1, "statement 2 fails");
        assert_eq!(
            response.failed[0].id, b.id,
            "the failing row is statement 2"
        );

        // Only the rows Polis accepted are persisted; the failed one stays pending.
        assert_eq!(
            polis_statement_aux::get_by_id(&pool, &a.id)
                .await?
                .moderation_status,
            ModerationStatus::Accepted
        );
        assert_eq!(
            polis_statement_aux::get_by_id(&pool, &b.id)
                .await?
                .moderation_status,
            ModerationStatus::Pending,
            "the row Polis rejected must remain unchanged"
        );
        assert_eq!(
            polis_statement_aux::get_by_id(&pool, &c.id)
                .await?
                .moderation_status,
            ModerationStatus::Accepted
        );
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn non_owner_cannot_batch_moderate(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut owner) = setup_default_app_and_session(&pool).await?;
        let workflow_step_id = setup_polis_step(&app, &mut owner).await?;
        let owner_id = owner.id.expect("session to be signed up");
        let a = insert_pending_aux(&pool, owner_id, workflow_step_id, 1).await?;

        let mut intruder = UserSession::new(
            "intruder",
            crate::test_helpers::TEST_PASSWORD,
            "intruder@example.com",
        );
        intruder.signup(&app).await?;

        let (status, _, _) = intruder
            .post(
                &app,
                "/tools/polis/statement_aux/moderate_batch",
                json!({ "ids": [a.id], "decision": "accept" })
                    .to_string()
                    .into(),
            )
            .await?;

        assert_eq!(status, StatusCode::FORBIDDEN);
        let reloaded = polis_statement_aux::get_by_id(&pool, &a.id).await?;
        assert_eq!(
            reloaded.moderation_status,
            ModerationStatus::Pending,
            "a forbidden request must not change any row"
        );
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn batch_moderate_rejects_ids_from_different_steps(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let owner_id = session.id.expect("session to be signed up");

        let step_one = setup_polis_step(&app, &mut session).await?;
        let step_two = setup_polis_step(&app, &mut session).await?;
        let a = insert_pending_aux(&pool, owner_id, step_one, 1).await?;
        let b = insert_pending_aux(&pool, owner_id, step_two, 1).await?;

        let (status, _, _) = session
            .post(
                &app,
                "/tools/polis/statement_aux/moderate_batch",
                json!({ "ids": [a.id, b.id], "decision": "accept" })
                    .to_string()
                    .into(),
            )
            .await?;

        assert_eq!(
            status,
            StatusCode::BAD_REQUEST,
            "all ids must belong to the same workflow step"
        );

        // Neither row should have been touched.
        for aux in [&a, &b] {
            let reloaded = polis_statement_aux::get_by_id(&pool, &aux.id).await?;
            assert_eq!(reloaded.moderation_status, ModerationStatus::Pending);
        }
        Ok(())
    }
}
