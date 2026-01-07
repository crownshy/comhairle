use std::sync::Arc;

use aide::axum::{
    routing::{delete_with, get_with, post_with, put_with},
    ApiRouter,
};
use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{
    bot_service::ComhairleChatSession,
    error::ComhairleError,
    routes::{
        auth::{RequiredAdminUser, RequiredUser},
        bot::GetQueryParams,
    },
    ComhairleState,
};

#[instrument(err(Debug), skip(state))]
async fn list(
    State(state): State<Arc<ComhairleState>>,
    Path(chat_id): Path<String>,
    Query(params): Query<GetQueryParams>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<Vec<ComhairleChatSession>>), ComhairleError> {
    let (_, sessions) = state
        .bot_service
        .list_chat_sessions(&chat_id, Some(params))
        .await?;

    Ok((StatusCode::OK, Json(sessions)))
}

#[instrument(err(Debug), skip(state))]
async fn get(
    State(state): State<Arc<ComhairleState>>,
    Path((chat_id, session_id)): Path<(String, String)>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<ComhairleChatSession>), ComhairleError> {
    let (_, session) = state
        .bot_service
        .get_chat_session(&session_id, &chat_id)
        .await?;

    Ok((StatusCode::OK, Json(session)))
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Default)]
pub struct CreateChatSessionRequest {
    pub name: String,
    pub user_id: Option<String>,
}

async fn create(
    State(state): State<Arc<ComhairleState>>,
    Path(chat_id): Path<String>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(payload): Json<CreateChatSessionRequest>,
) -> Result<(StatusCode, Json<ComhairleChatSession>), ComhairleError> {
    let (_, session) = state
        .bot_service
        .create_chat_session(&chat_id, payload)
        .await?;

    Ok((StatusCode::CREATED, Json(session)))
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone, PartialEq, Default)]
pub struct UpdateChatSessionRequest {
    pub name: Option<String>,
    pub user_id: Option<String>,
}

async fn update(
    State(state): State<Arc<ComhairleState>>,
    Path((chat_id, session_id)): Path<(String, String)>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(payload): Json<UpdateChatSessionRequest>,
) -> Result<(StatusCode, Json<ComhairleChatSession>), ComhairleError> {
    let (_, session) = state
        .bot_service
        .update_chat_session(&session_id, &chat_id, payload)
        .await?;

    Ok((StatusCode::OK, Json(session)))
}

async fn delete(
    State(state): State<Arc<ComhairleState>>,
    Path((chat_id, session_id)): Path<(String, String)>,
) -> Result<StatusCode, ComhairleError> {
    let _ = state
        .bot_service
        .delete_chat_session(&session_id, &chat_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone, PartialEq)]
pub struct ChatConversationRequest {
    pub question: String,
    pub user_id: Option<String>,
}

#[instrument(err(Debug), skip(state))]
async fn converse_with_chat(
    State(state): State<Arc<ComhairleState>>,
    Path((chat_id, session_id)): Path<(String, String)>,
    RequiredUser(_user): RequiredUser,
    Json(payload): Json<ChatConversationRequest>,
) -> Result<impl IntoResponse, ComhairleError> {
    let stream = state
        .bot_service
        .converse_with_chat(&session_id, &chat_id, payload)
        .await?;

    Ok(Body::from_stream(stream))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(list, |op| {
                op.id("GetChatSessions")
                    .tag("Bot Chat Sessions")
                    .summary("Get a list of chat sessions")
                    .response::<200, Json<Vec<ComhairleChatSession>>>()
            }),
        )
        .api_route(
            "/{session_id}",
            get_with(get, |op| {
                op.id("GetChatSession")
                    .tag("Bot Chat Sessions")
                    .summary("Get a chat session by id")
                    .response::<200, Json<ComhairleChatSession>>()
            }),
        )
        .api_route(
            "/",
            post_with(create, |op| {
                op.id("CreateChatSession")
                    .tag("Bot Chat Sessions")
                    .summary("Create a new session for a chat bot")
                    .response::<201, Json<ComhairleChatSession>>()
            }),
        )
        .api_route(
            "/{session_id}",
            put_with(update, |op| {
                op.id("UpdateChatSession")
                    .tag("Bot Chat Sessions")
                    .summary("Update a chat bot session")
                    .response::<200, Json<ComhairleChatSession>>()
            }),
        )
        .api_route(
            "/{session_id}",
            delete_with(delete, |op| {
                op.id("DeleteChatSession")
                    .tag("Bot Chat Sessions")
                    .summary("Delete a session from a chat bot")
                    .response::<204, ()>()
            }),
        )
        .route("/{session_id}", post(converse_with_chat))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot_service::MockComhairleBotService;
    use crate::{
        setup_server,
        test_helpers::{test_state, UserSession},
    };
    use std::error::Error;
    use std::{pin::Pin, sync::Arc};

    use axum::body::{to_bytes, Body, Bytes};
    use futures::{stream, Stream};
    use mockall::predicate::eq;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn should_return_session_list(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let session = ComhairleChatSession {
            chat_id: "123".to_string(),
            name: Some("test_session".to_string()),
            ..Default::default()
        };
        let params = GetQueryParams {
            page: Some(2),
            ..Default::default()
        };

        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_list_chat_sessions()
            .once()
            .with(eq("123"), eq(Some(params)))
            .returning(move |_, _| {
                Box::pin({
                    let session = session.clone();
                    async move { Ok((StatusCode::OK, vec![session.clone()])) }
                })
            });

        let state = test_state()
            .db(pool)
            .bot_service(Arc::new(bot_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;
        let (status, value, _) = admin_session
            .get(&app, "/bot/chats/123/sessions?page=2")
            .await?;
        let json: Vec<ComhairleChatSession> = serde_json::from_value(value)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(
            json[0].chat_id,
            "123".to_string(),
            "incorrect json response"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_return_single_session(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let session = ComhairleChatSession {
            id: "456".to_string(),
            name: Some("test_session".to_string()),
            ..Default::default()
        };

        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_get_chat_session()
            .once()
            .with(eq("456"), eq("123"))
            .returning(move |_, _| {
                Box::pin({
                    let session = session.clone();
                    async move { Ok((StatusCode::OK, session.clone())) }
                })
            });

        let state = test_state()
            .db(pool)
            .bot_service(Arc::new(bot_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;
        let (status, response, _) = admin_session
            .get(&app, "/bot/chats/123/sessions/456")
            .await?;
        let json: ComhairleChatSession = serde_json::from_value(response)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(json.id, "456".to_string(), "incorrect json response");

        Ok(())
    }

    #[sqlx::test]
    async fn should_create_and_return_a_session(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let session = ComhairleChatSession {
            chat_id: "123".to_string(),
            name: Some("test_session".to_string()),
            ..Default::default()
        };

        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_create_chat_session()
            .once()
            .returning(move |_, _| {
                let session = session.clone();
                Box::pin(async move { Ok((StatusCode::OK, session.clone())) })
            });

        let state = test_state()
            .db(pool)
            .bot_service(Arc::new(bot_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        let create_request = CreateChatSessionRequest {
            name: "test_session".to_string(),
            ..Default::default()
        };
        let bytes = serde_json::to_vec(&create_request)?;
        let body = Body::from(bytes);
        let (status, response, _) = admin_session
            .post(&app, "/bot/chats/123/sessions", body)
            .await?;

        assert!(status.is_success(), "error response status");
        assert_eq!(
            response.get("name").and_then(|v| v.as_str()).unwrap(),
            "test_session",
            "incorrect json response"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_update_and_return_session(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let session = ComhairleChatSession {
            chat_id: "123".to_string(),
            name: Some("test_session".to_string()),
            ..Default::default()
        };

        let update_request = UpdateChatSessionRequest {
            name: Some("test_session".to_string()),
            ..Default::default()
        };
        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_update_chat_session()
            .once()
            .with(eq("456"), eq("123"), eq(update_request.clone()))
            .returning(move |_, _, _| {
                let session = session.clone();
                Box::pin(async move { Ok((StatusCode::OK, session.clone())) })
            });

        let state = test_state()
            .db(pool)
            .bot_service(Arc::new(bot_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        let bytes = serde_json::to_vec(&update_request)?;
        let body = Body::from(bytes);
        let (status, response, _) = admin_session
            .put(&app, "/bot/chats/123/sessions/456", body)
            .await?;

        assert!(status.is_success(), "error response status");
        assert_eq!(
            response.get("name").and_then(|v| v.as_str()).unwrap(),
            "test_session",
            "incorrect json response"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_delete_session(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_delete_chat_session()
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
            .delete(&app, "/bot/chats/123/sessions/456")
            .await?;

        assert!(status.is_success(), "error response status");

        Ok(())
    }

    #[sqlx::test]
    async fn should_converse_with_chat_and_return_byte_stream(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let converse_request = ChatConversationRequest {
            question: "Test question?".to_string(),
            user_id: None,
        };

        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_converse_with_chat()
            .once()
            .with(eq("456"), eq("123"), eq(converse_request.clone()))
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

        let body = serde_json::to_vec(&converse_request)?;
        let (status, body, _) = admin_session
            .post_raw_response(&app, "/bot/chats/123/sessions/456", Body::from(body))
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
