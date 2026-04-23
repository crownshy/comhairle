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
    fn sanitize(&self) -> Self {
        self.clone()
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct ElicitationBotToolSetup {
    pub topic: String,
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
        config.sanitize()
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
    let bot_service = state.required_bot_service()?;
    let elicitation_bot_agent_id = state
        .config
        .elicitation_bot_agent_id
        .as_ref()
        .ok_or(ComhairleError::NoBotServiceConfigured)?;

    let user_session = bot_service_user_session::get_or_create(
        &state,
        BotServiceSessionContext::ElicitationBot,
        &user.id,
        None,
        Some(&workflow_step_id),
    )
    .await?;

    let (_, session) = bot_service
        .get_agent_session(
            &user_session.bot_service_session_id,
            elicitation_bot_agent_id,
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
    let bot_service = state.required_bot_service()?;
    let elicitation_bot_agent_id = state
        .config
        .elicitation_bot_agent_id
        .as_ref()
        .ok_or(ComhairleError::NoBotServiceConfigured)?;

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
    let stream = bot_service
        .converse_with_agent(
            &session.bot_service_session_id,
            elicitation_bot_agent_id,
            payload,
        )
        .await?;

    let body = Body::from_stream(stream);
    Ok(StreamBody(body))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        bot_service::{ComhairleChat, ComhairleKnowledgeBase, MockComhairleBotService},
        setup_server,
        test_helpers::{elicitation_bot_tool_config, test_state, UserSession},
    };

    use axum::{
        body::{to_bytes, Bytes},
        Router,
    };
    use futures::{stream, Stream};
    use mockall::predicate::always;
    use serde_json::json;
    use sqlx::PgPool;
    use std::{error::Error, pin::Pin};

    fn mock_bot_service_for_workflow_step() -> MockComhairleBotService {
        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_create_knowledge_base()
            .returning(|_, _| {
                Box::pin(async move {
                    Ok((
                        StatusCode::CREATED,
                        ComhairleKnowledgeBase {
                            id: "kb-123".to_string(),
                            ..Default::default()
                        },
                    ))
                })
            });
        bot_service.expect_create_chat().returning(|_| {
            Box::pin(async move {
                Ok((
                    StatusCode::CREATED,
                    ComhairleChat {
                        id: "chat-123".to_string(),
                        ..Default::default()
                    },
                ))
            })
        });

        bot_service
    }

    fn build_bot_service<F>(configure: F) -> MockComhairleBotService
    where
        F: FnOnce(&mut MockComhairleBotService),
    {
        let mut bot_service = mock_bot_service_for_workflow_step();
        configure(&mut bot_service);
        bot_service
    }

    async fn setup_test_app_with_workflow_step<F>(
        pool: PgPool,
        configure_bot_service: F,
    ) -> Result<(Router, UserSession, String), Box<dyn Error>>
    where
        F: FnOnce(&mut MockComhairleBotService),
    {
        let bot_service = build_bot_service(configure_bot_service);
        let state = test_state()
            .db(pool)
            .bot_service(Arc::new(bot_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;
        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let (_, conversation, _) = session.create_random_conversation(&app).await?;
        let conversation_id = conversation["id"].as_str().unwrap().to_string();
        let (_, workflow, _) = session
            .create_random_workflow(&app, &conversation_id)
            .await?;
        let workflow_id = workflow["id"].as_str().unwrap().to_string();
        let (_, workflow_step, _) = session
            .create_workflow_step(
                &app,
                &conversation_id,
                &workflow_id,
                json!({
                    "name": "test_workflow_step",
                    "step_order": 1,
                    "activation_rule" : "manual",
                    "description": "A test workflow_step with elicitation bot",
                    "is_offline": false,
                    "required": false,
                    "can_revisit": false,
                    "tool_setup": elicitation_bot_tool_config()
                }),
            )
            .await?;
        let workflow_step_id = workflow_step["id"].as_str().unwrap().to_string();

        Ok((app, session, workflow_step_id))
    }

    #[sqlx::test]
    async fn should_get_agent_session(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session, workflow_step_id) =
            setup_test_app_with_workflow_step(pool, |bot_service| {
                bot_service.expect_create_agent_session().returning(|_| {
                    Box::pin(async move {
                        Ok((
                            StatusCode::CREATED,
                            ComhairleAgentSession {
                                id: "session-123".to_string(),
                                ..Default::default()
                            },
                        ))
                    })
                });
                bot_service.expect_get_agent_session().returning(|_, _| {
                    Box::pin(async move {
                        Ok((
                            StatusCode::CREATED,
                            ComhairleAgentSession {
                                id: "session-123".to_string(),
                                ..Default::default()
                            },
                        ))
                    })
                });
            })
            .await?;

        let (status, value, _) = session
            .get(
                &app,
                &format!("/tools/elicitation_bot/workflow_step/{workflow_step_id}"),
            )
            .await?;
        let session: ComhairleAgentSession = serde_json::from_value(value)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(session.id, "session-123", "incorrect json response");

        Ok(())
    }

    #[sqlx::test]
    async fn should_converse_with_agent_and_return_byte_stream(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session, workflow_step_id) =
            setup_test_app_with_workflow_step(pool, |bot_service| {
                bot_service.expect_create_agent_session().returning(|_| {
                    Box::pin(async move {
                        Ok((
                            StatusCode::CREATED,
                            ComhairleAgentSession {
                                ..Default::default()
                            },
                        ))
                    })
                });
                bot_service
                    .expect_converse_with_agent()
                    .once()
                    .with(always(), always(), always())
                    .returning(move |_, _, _| {
                        Box::pin(async move {
                            let chunks = vec![
                                Ok(Bytes::from_static(b"test ")),
                                Ok(Bytes::from_static(b"stream")),
                            ];
                            let stream = stream::iter(chunks);
                            let stream: Pin<
                                Box<dyn Stream<Item = Result<Bytes, ComhairleError>> + Send>,
                            > = Box::pin(stream);

                            Ok(stream)
                        })
                    });
            })
            .await?;

        let converse_request = AgentConversationRequest {
            question: "Test question?".to_string(),
            topic: "renewable energy".to_string(),
        };
        let body = serde_json::to_vec(&converse_request)?;
        let (status, body, _) = session
            .post_raw_response(
                &app,
                &format!("/tools/elicitation_bot/workflow_step/{workflow_step_id}"),
                Body::from(body),
            )
            .await?;

        let bytes = to_bytes(body, usize::MAX).await?;
        let text_content = String::from_utf8(bytes.to_vec())?;

        assert!(status.is_success(), "error response status");
        assert_eq!(
            text_content,
            "test stream".to_string(),
            "incorrect text content"
        );

        Ok(())
    }
}
