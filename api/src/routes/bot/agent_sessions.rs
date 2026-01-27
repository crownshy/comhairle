use std::sync::Arc;

use aide::axum::{
    routing::{delete_with, get_with, post_with},
    ApiRouter,
};
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{
    bot_service::ComhairleAgentSession,
    error::ComhairleError,
    routes::{auth::RequiredAdminUser, bot::GetQueryParams},
    ComhairleState,
};

#[instrument(err(Debug), skip(state))]
async fn list(
    State(state): State<Arc<ComhairleState>>,
    Path(agent_id): Path<String>,
    Query(params): Query<GetQueryParams>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<Vec<ComhairleAgentSession>>), ComhairleError> {
    let (_, sessions) = state
        .bot_service
        .list_agent_sessions(&agent_id, Some(params))
        .await?;

    Ok((StatusCode::OK, Json(sessions)))
}

#[instrument(err(Debug), skip(state))]
async fn get(
    State(state): State<Arc<ComhairleState>>,
    Path((agent_id, session_id)): Path<(String, String)>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<ComhairleAgentSession>), ComhairleError> {
    let (_, session) = state
        .bot_service
        .get_agent_session(&session_id, &agent_id)
        .await?;

    Ok((StatusCode::OK, Json(session)))
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Default)]
pub struct CreateAgentSessionRequest;

#[instrument(err(Debug), skip(state))]
async fn create(
    State(state): State<Arc<ComhairleState>>,
    Path(agent_id): Path<String>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<ComhairleAgentSession>), ComhairleError> {
    let (_, session) = state.bot_service.create_agent_session(&agent_id).await?;

    Ok((StatusCode::CREATED, Json(session)))
}

#[instrument(err(Debug), skip(state))]
async fn delete(
    State(state): State<Arc<ComhairleState>>,
    Path((agent_id, session_id)): Path<(String, String)>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<StatusCode, ComhairleError> {
    let _ = state
        .bot_service
        .delete_agent_session(&session_id, &agent_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/{session_id}",
            get_with(get, |op| {
                op.id("GetAgentSession")
                    .tag("Bot Agent Sessions")
                    .security_requirement("JWT")
                    .summary("Get an agent session by id")
                    .response::<200, Json<ComhairleAgentSession>>()
            }),
        )
        .api_route(
            "/",
            get_with(list, |op| {
                op.id("ListAgentSessions")
                    .tag("Bot Agent Sessions")
                    .security_requirement("JWT")
                    .summary("Get a list of agent sessions")
                    .response::<200, Json<Vec<ComhairleAgentSession>>>()
            }),
        )
        .api_route(
            "/",
            post_with(create, |op| {
                op.id("CreateAgentSessions")
                    .tag("Bot Agent Sessions")
                    .security_requirement("JWT")
                    .summary("Create a new session for an agent")
                    .response::<201, Json<ComhairleAgentSession>>()
            }),
        )
        .api_route(
            "/{session_id}",
            delete_with(delete, |op| {
                op.id("DeleteAgentSessions")
                    .tag("Bot Agent Sessions")
                    .security_requirement("JWT")
                    .summary("Delete an agent session")
                    .response::<204, ()>()
            }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{error::Error, pin::Pin};

    use crate::{
        bot_service::{
            ComhairleAgent, ComhairleChat, ComhairleKnowledgeBase, MockComhairleBotService,
        },
        setup_server,
        test_helpers::{extract, test_state, UserSession},
    };

    use axum::body::{to_bytes, Body, Bytes};
    use futures::{stream, Stream};
    use mockall::predicate::{always, eq};
    use serde_json::json;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn should_return_agent_session_list(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let session = ComhairleAgentSession {
            id: "456".to_string(),
            agent_id: "123".to_string(),
            configuration: json!({ "graph": {}, "components": {} }),
            messages: vec![],
        };
        let params = GetQueryParams {
            page: Some(2),
            ..Default::default()
        };

        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_list_agent_sessions()
            .once()
            .with(eq("123"), eq(Some(params)))
            .returning(move |_, _| {
                let session = session.clone();
                Box::pin(async move { Ok((StatusCode::OK, vec![session])) })
            });

        let state = test_state()
            .db(pool)
            .bot_service(Arc::new(bot_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;
        let (status, value, _) = admin_session
            .get(&app, "/bot/agents/123/sessions?page=2")
            .await?;
        let json: Vec<ComhairleAgentSession> = serde_json::from_value(value)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(
            json[0].agent_id,
            "123".to_string(),
            "incorrect json response"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_return_single_agent_session_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let session = ComhairleAgentSession {
            id: "456".to_string(),
            agent_id: "123".to_string(),
            configuration: json!({ "graph": {}, "components": {} }),
            messages: vec![],
        };

        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_get_agent_session()
            .once()
            .with(eq("456"), eq("123"))
            .returning(move |_, _| {
                let session = session.clone();
                Box::pin(async move { Ok((StatusCode::OK, session)) })
            });

        let state = test_state()
            .db(pool)
            .bot_service(Arc::new(bot_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;
        let (status, value, _) = admin_session
            .get(&app, "/bot/agents/123/sessions/456")
            .await?;
        let json: ComhairleAgentSession = serde_json::from_value(value)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(json.agent_id, "123".to_string(), "incorrect json response");

        Ok(())
    }

    #[sqlx::test]
    async fn should_create_an_agent_session(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let session = ComhairleAgentSession {
            id: "456".to_string(),
            agent_id: "123".to_string(),
            configuration: json!({ "graph": {}, "components": {} }),
            messages: vec![],
        };

        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_create_agent_session()
            .once()
            .with(eq("123"))
            .returning(move |_| {
                let session = session.clone();
                Box::pin(async move { Ok((StatusCode::OK, session)) })
            });

        let state = test_state()
            .db(pool)
            .bot_service(Arc::new(bot_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        let (status, value, _) = admin_session
            .post(&app, "/bot/agents/123/sessions", Body::empty())
            .await?;
        let json: ComhairleAgentSession = serde_json::from_value(value)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(json.agent_id, "123".to_string(), "incorrect json response");

        Ok(())
    }

    #[sqlx::test]
    async fn should_delete_an_agent_session(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_delete_agent_session()
            .once()
            .with(eq("456"), eq("123"))
            .returning(|_, _| Box::pin(async move { Ok(StatusCode::OK) }));

        let state = test_state()
            .db(pool)
            .bot_service(Arc::new(bot_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        let (status, _, _) = admin_session
            .delete(&app, "/bot/agents/123/sessions/456")
            .await?;

        assert!(status.is_success(), "error response status");

        Ok(())
    }

    #[sqlx::test]
    async fn should_converse_with_agent_and_return_byte_stream(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let mut bot_service = MockComhairleBotService::new();
        bot_service.expect_create_agent().once().returning(|_| {
            Box::pin(async move {
                Ok((
                    StatusCode::CREATED,
                    ComhairleAgent {
                        id: "456".to_string(),
                        ..Default::default()
                    },
                ))
            })
        });
        bot_service
            .expect_create_knowledge_base()
            .once()
            .returning(|_, _| {
                Box::pin(async move {
                    Ok((
                        StatusCode::CREATED,
                        ComhairleKnowledgeBase {
                            ..Default::default()
                        },
                    ))
                })
            });
        bot_service.expect_create_chat().once().returning(|_| {
            Box::pin(async move {
                Ok((
                    StatusCode::CREATED,
                    ComhairleChat {
                        ..Default::default()
                    },
                ))
            })
        });
        bot_service
            .expect_converse_with_agent()
            .once()
            .with(eq("456"), eq("123"), always())
            .returning(move |_, _, _| {
                Box::pin(async move {
                    let chunks = vec![
                        Ok(Bytes::from_static(b"test ")),
                        Ok(Bytes::from_static(b"stream")),
                    ];
                    let stream = stream::iter(chunks);
                    let stream: Pin<Box<dyn Stream<Item = Result<Bytes, ComhairleError>> + Send>> =
                        Box::pin(stream);

                    Ok(stream)
                })
            });

        let state = test_state()
            .db(pool)
            .bot_service(Arc::new(bot_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        let (_, conversation, _) = admin_session.create_random_conversation(&app).await?;
        let conversation_id: String = extract("id", &conversation);
        let (_, workflow, _) = admin_session
            .create_random_workflow(&app, &conversation_id)
            .await?;
        let workflow_id: String = extract("id", &workflow);
        let (_, workflow_step, _) = admin_session
            .post(
                &app,
                &format!("/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step"),
                json!({
                "name": "Workflow step",
                "step_order": 2,
                "activation_rule" : "manual",
                "description": "An elicitaiton bot workflow step",
                "is_offline": false,
                "required":true,
                "tool_setup": {
                    "type": "elicitationbot",
                    "bot_id": "123",
                    "topic": "topic",
                    "conversation_id": conversation_id,
                }})
                .to_string()
                .into(),
            )
            .await?;
        let workflow_step_id = Uuid::parse_str(&extract::<String>("id", &workflow_step))?;

        let converse_request = AgentConversationRequestExt {
            question: "Test question?".to_string(),
            workflow_step_id,
            topic: "renewable energy".to_string(),
        };
        let body = serde_json::to_vec(&converse_request)?;
        let (status, body, _) = admin_session
            .post_raw_response(&app, "/bot/agents/123/sessions/456", Body::from(body))
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
