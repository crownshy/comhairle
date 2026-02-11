use std::sync::Arc;

use aide::{
    axum::{
        routing::{get_with, post_with},
        ApiRouter,
    },
    OperationIo,
};
use async_trait::async_trait;
use axum::{
    body::Body,
    extract::{Json, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    bot_service::{AgentConversationRequest, ComhairleAgentSession},
    error::ComhairleError,
    models::{
        bot_service_user_session::{self, BotServiceSessionContext},
        workflow_step,
    },
    routes::auth::RequiredUser,
    tools::ToolConfig,
    ComhairleState,
};

use super::{ToolConfigSanitize, ToolImpl};

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema, PartialEq)]
pub struct ElicitationBotToolConfig {
    pub topic: String,
}

impl ToolConfigSanitize for ElicitationBotToolConfig {
    fn sanatize(&self) -> Self {
        self.clone()
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct ElicitationBotToolSetup {
    pub topic: String,
    pub conversation_id: String,
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct ElicitationBotReport;

async fn elicitation_bot_setup(
    config: &ElicitationBotToolSetup,
) -> Result<ElicitationBotToolConfig, ComhairleError> {
    Ok(ElicitationBotToolConfig {
        topic: config.topic.clone(),
    })
}

// Keep public function for backwards compatibility
pub async fn setup(
    config: &ElicitationBotToolSetup,
) -> Result<ElicitationBotToolConfig, ComhairleError> {
    elicitation_bot_setup(config).await
}

/// Zero-sized marker type for ElicitationBot tool implementation
pub struct ElicitationBotTool;

#[async_trait]
impl ToolImpl for ElicitationBotTool {
    type Config = ElicitationBotToolConfig;
    type Setup = ElicitationBotToolSetup;
    type Report = ElicitationBotReport;

    async fn setup(
        setup: &Self::Setup,
        _state: &Arc<ComhairleState>,
    ) -> Result<Self::Config, ComhairleError> {
        elicitation_bot_setup(setup).await
    }

    async fn clone_tool(
        config: &Self::Config,
        _state: &Arc<ComhairleState>,
    ) -> Result<Self::Config, ComhairleError> {
        // ElicitationBot tool is cloneable as-is
        Ok(config.clone())
    }

    fn sanitize(config: Self::Config) -> Self::Config {
        config.sanatize()
    }

    fn routes(state: &Arc<ComhairleState>) -> ApiRouter {
        ApiRouter::new()
            .api_route(
                "/elicitation_bot/workflow_step/{workflow_step_id}",
                get_with(get_session_history, |op| {
                    op.id("GetElicitationBotSessionHistory")
                        .tag("Tools")
                        .summary("Get user session history for an elicitation bot")
                        .security_requirement("JWT")
                        .description("Returns a user session for an elicitation bot including message history")
                        .response::<200, Json<ComhairleAgentSession>>()
                }),
            )
            .api_route(
                "/elicitation_bot/workflow_step/{workflow_step_id}",
                post_with(converse, |op| {
                    op.tag("Tools")
                        .summary("Converse with an elicitation bot")
                        .security_requirement("JWT")
                        .description(
"
Streamed LLM response.
⚠️ This endpoint returns a streaming response on success.
Generated API clients are NOT suitable for consuming this endpoint.
Use a raw HTTP request and process the response body incrementally.
"
                        )
                }),
            )
            .with_state(state.clone())
    }
}

#[instrument(err(Debug), skip(state))]
async fn get_session_history(
    State(state): State<Arc<ComhairleState>>,
    Path(workflow_step_id): Path<Uuid>,
    RequiredUser(user): RequiredUser,
) -> Result<(StatusCode, Json<ComhairleAgentSession>), ComhairleError> {
    let user_session = bot_service_user_session::get_or_create(
        &state,
        BotServiceSessionContext::ElicitationBot,
        &user.id,
        None,
        Some(&workflow_step_id),
    )
    .await?;

    let (_, session) = state
        .bot_service
        .get_agent_session(
            &user_session.bot_service_session_id,
            &state.config.elicitation_bot_agent_id,
        )
        .await?;

    Ok((StatusCode::OK, Json(session)))
}

#[derive(Deserialize, Debug, JsonSchema, Clone, PartialEq)]
pub struct ConversationRequest {
    pub question: String,
}

// Wrapper struct required as a workaround to generate documentation for handlers
// that return `axum::body::Body` for streamed responses.
#[derive(OperationIo)]
pub struct StreamBody(Body);

impl IntoResponse for StreamBody {
    fn into_response(self) -> Response {
        self.0.into_response()
    }
}

#[instrument(err(Debug), skip(state))]
async fn converse(
    State(state): State<Arc<ComhairleState>>,
    Path(workflow_step_id): Path<Uuid>,
    RequiredUser(user): RequiredUser,
    Json(payload): Json<ConversationRequest>,
) -> Result<StreamBody, ComhairleError> {
    let workflow_step = workflow_step::get_by_id(&state.db, &workflow_step_id).await?;

    // TODO: think more creafully how we handle this in preview mode
    let tool_config = match (workflow_step.tool_config, workflow_step.preview_tool_config) {
        (Some(ToolConfig::ElicitationBot(config)), _) => config,
        (None, ToolConfig::ElicitationBot(config)) => config,

        _ => {
            return Err(ComhairleError::ToolConfigError(
                "incorrect config type".to_string(),
            ))
        }
    };

    let session = bot_service_user_session::get_or_create(
        &state,
        BotServiceSessionContext::ElicitationBot,
        &user.id,
        None,
        Some(&workflow_step_id),
    )
    .await?;

    let payload = AgentConversationRequest {
        question: payload.question,
        topic: tool_config.topic.clone(),
    };
    let stream = state
        .bot_service
        .converse_with_agent(
            &session.bot_service_session_id,
            &state.config.elicitation_bot_agent_id,
            payload,
        )
        .await?;

    let body = Body::from_stream(stream);
    Ok(StreamBody(body))
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[sqlx::test]
    // async fn should_converse_with_agent_and_return_byte_stream(
    //     pool: PgPool,
    // ) -> Result<(), Box<dyn Error>> {
    //     let mut bot_service = MockComhairleBotService::new();
    //     bot_service.expect_create_agent_session().returning(|_| {
    //         Box::pin(async move {
    //             Ok((
    //                 StatusCode::CREATED,
    //                 ComhairleAgentSession {
    //                     ..Default::default()
    //                 },
    //             ))
    //         })
    //     });
    //     bot_service
    //         .expect_create_knowledge_base()
    //         .once()
    //         .returning(|_, _| {
    //             Box::pin(async move {
    //                 Ok((
    //                     StatusCode::CREATED,
    //                     ComhairleKnowledgeBase {
    //                         ..Default::default()
    //                     },
    //                 ))
    //             })
    //         });
    //     bot_service.expect_create_chat().once().returning(|_| {
    //         Box::pin(async move {
    //             Ok((
    //                 StatusCode::CREATED,
    //                 ComhairleChat {
    //                     ..Default::default()
    //                 },
    //             ))
    //         })
    //     });
    //     bot_service
    //         .expect_converse_with_agent()
    //         .once()
    //         .with(always(), always(), always())
    //         .returning(move |_, _, _| {
    //             Box::pin(async move {
    //                 let chunks = vec![
    //                     Ok(Bytes::from_static(b"test ")),
    //                     Ok(Bytes::from_static(b"stream")),
    //                 ];
    //                 let stream = stream::iter(chunks);
    //                 let stream: Pin<Box<dyn Stream<Item = Result<Bytes, ComhairleError>> + Send>> =
    //                     Box::pin(stream);
    //
    //                 Ok(stream)
    //             })
    //         });
    //
    //     let state = test_state()
    //         .db(pool)
    //         .bot_service(Arc::new(bot_service))
    //         .call()?;
    //     let app = setup_server(Arc::new(state)).await?;
    //
    //     let mut admin_session = UserSession::new_admin();
    //     admin_session.signup(&app).await?;
    //
    //     let (_, conversation, _) = admin_session.create_random_conversation(&app).await?;
    //     let conversation_id: String = extract("id", &conversation);
    //     let (_, workflow, _) = admin_session
    //         .create_random_workflow(&app, &conversation_id)
    //         .await?;
    //     let workflow_id: String = extract("id", &workflow);
    //     let (_, workflow_step, _) = admin_session
    //         .post(
    //             &app,
    //             &format!("/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step"),
    //             json!({
    //             "name": "Workflow step",
    //             "step_order": 2,
    //             "activation_rule" : "manual",
    //             "description": "An elicitaiton bot workflow step",
    //             "is_offline": false,
    //             "required":true,
    //             "tool_setup": {
    //                 "type": "elicitationbot",
    //                 "topic": "topic",
    //                 "conversation_id": conversation_id
    //             }})
    //             .to_string()
    //             .into(),
    //         )
    //         .await?;
    //     let workflow_step_id = Uuid::parse_str(&extract::<String>("id", &workflow_step))?;
    //
    //     let converse_request = AgentConversationRequestExt {
    //         question: "Test question?".to_string(),
    //         topic: "renewable energy".to_string(),
    //     };
    //     let body = serde_json::to_vec(&converse_request)?;
    //     let (status, body, _) = admin_session
    //         .post_raw_response(&app, &format!("/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step/{workflow_step_id}/converse_elicitation_bot"), Body::from(body))
    //         .await?;
    //
    //     let bytes = to_bytes(body, usize::MAX).await?;
    //     let text_content = String::from_utf8(bytes.to_vec())?;
    //
    //     assert!(status.is_success(), "error response status");
    //     assert_eq!(
    //         text_content,
    //         "test stream".to_string(),
    //         "incorrect text content"
    //     );
    //
    //     Ok(())
    // }

    // #[sqlx::test]
    // async fn should_converse_with_agent_and_return_byte_stream(
    //     pool: PgPool,
    // ) -> Result<(), Box<dyn Error>> {
    //     let mut bot_service = MockComhairleBotService::new();
    //     bot_service.expect_create_agent_session().returning(|_| {
    //         Box::pin(async move {
    //             Ok((
    //                 StatusCode::CREATED,
    //                 ComhairleAgentSession {
    //                     ..Default::default()
    //                 },
    //             ))
    //         })
    //     });
    //     bot_service
    //         .expect_create_knowledge_base()
    //         .once()
    //         .returning(|_, _| {
    //             Box::pin(async move {
    //                 Ok((
    //                     StatusCode::CREATED,
    //                     ComhairleKnowledgeBase {
    //                         ..Default::default()
    //                     },
    //                 ))
    //             })
    //         });
    //     bot_service.expect_create_chat().once().returning(|_| {
    //         Box::pin(async move {
    //             Ok((
    //                 StatusCode::CREATED,
    //                 ComhairleChat {
    //                     ..Default::default()
    //                 },
    //             ))
    //         })
    //     });
    //     bot_service
    //         .expect_converse_with_agent()
    //         .once()
    //         .with(always(), always(), always())
    //         .returning(move |_, _, _| {
    //             Box::pin(async move {
    //                 let chunks = vec![
    //                     Ok(Bytes::from_static(b"test ")),
    //                     Ok(Bytes::from_static(b"stream")),
    //                 ];
    //                 let stream = stream::iter(chunks);
    //                 let stream: Pin<Box<dyn Stream<Item = Result<Bytes, ComhairleError>> + Send>> =
    //                     Box::pin(stream);
    //
    //                 Ok(stream)
    //             })
    //         });
    //
    //     let state = test_state()
    //         .db(pool)
    //         .bot_service(Arc::new(bot_service))
    //         .call()?;
    //     let app = setup_server(Arc::new(state)).await?;
    //
    //     let mut admin_session = UserSession::new_admin();
    //     admin_session.signup(&app).await?;
    //
    //     let (_, conversation, _) = admin_session.create_random_conversation(&app).await?;
    //     let conversation_id: String = extract("id", &conversation);
    //     let (_, workflow, _) = admin_session
    //         .create_random_workflow(&app, &conversation_id)
    //         .await?;
    //     let workflow_id: String = extract("id", &workflow);
    //     let (_, workflow_step, _) = admin_session
    //         .post(
    //             &app,
    //             &format!("/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step"),
    //             json!({
    //             "name": "Workflow step",
    //             "step_order": 2,
    //             "activation_rule" : "manual",
    //             "description": "An elicitaiton bot workflow step",
    //             "is_offline": false,
    //             "required":true,
    //             "tool_setup": {
    //                 "type": "elicitationbot",
    //                 "topic": "topic",
    //                 "conversation_id": conversation_id
    //             }})
    //             .to_string()
    //             .into(),
    //         )
    //         .await?;
    //     let workflow_step_id = Uuid::parse_str(&extract::<String>("id", &workflow_step))?;
    //
    //     let converse_request = AgentConversationRequestExt {
    //         question: "Test question?".to_string(),
    //         topic: "renewable energy".to_string(),
    //     };
    //     let body = serde_json::to_vec(&converse_request)?;
    //     let (status, body, _) = admin_session
    //         .post_raw_response(&app, &format!("/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step/{workflow_step_id}/converse_elicitation_bot"), Body::from(body))
    //         .await?;
    //
    //     let bytes = to_bytes(body, usize::MAX).await?;
    //     let text_content = String::from_utf8(bytes.to_vec())?;
    //
    //     assert!(status.is_success(), "error response status");
    //     assert_eq!(
    //         text_content,
    //         "test stream".to_string(),
    //         "incorrect text content"
    //     );
    //
    //     Ok(())
    // }
}
