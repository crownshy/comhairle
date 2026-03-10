use std::sync::Arc;

use aide::{
    axum::{
        routing::{get_with, post_with},
        ApiRouter,
    },
    OperationIo,
};
use axum::{
    body::Body,
    extract::{Json, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    bot_service::{ChatConversationRequest, ComhairleChatSession},
    error::ComhairleError,
    models::{
        bot_service_user_session::{self, BotServiceSessionContext},
        conversation,
    },
    routes::auth::RequiredUser,
    ComhairleState,
};

#[instrument(err(Debug), skip(state))]
pub async fn get_session(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    RequiredUser(user): RequiredUser,
) -> Result<(StatusCode, Json<ComhairleChatSession>), ComhairleError> {
    let bot_service = match &state.bot_service {
        Some(bs) => bs,
        None => return Err(ComhairleError::UninitializedBotService),
    };

    let conversation = conversation::get_by_id(&state.db, &conversation_id).await?;
    let session = bot_service_user_session::get_or_create(
        &state,
        BotServiceSessionContext::QaBot,
        &user.id,
        Some(&conversation_id),
        None,
    )
    .await?;

    let chat_bot_id = match conversation.chat_bot_id {
        Some(id) => id,
        None => {
            return Err(ComhairleError::CorruptedData(
                "Missing chat_bot_id on conversation: {conversation_id}".to_string(),
            ))
        }
    };

    let (_, session) = bot_service
        .get_chat_session(&session.bot_service_session_id, &chat_bot_id)
        .await?;

    Ok((StatusCode::OK, Json(session)))
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
    Path(conversation_id): Path<Uuid>,
    RequiredUser(user): RequiredUser,
    Json(payload): Json<ChatConversationRequest>,
) -> Result<StreamBody, ComhairleError> {
    let bot_service = match &state.bot_service {
        Some(bs) => bs,
        None => return Err(ComhairleError::UninitializedBotService),
    };

    let conversation = conversation::get_by_id(&state.db, &conversation_id).await?;
    let session = bot_service_user_session::get_or_create(
        &state,
        BotServiceSessionContext::QaBot,
        &user.id,
        Some(&conversation_id),
        None,
    )
    .await?;

    let chat_bot_id = match conversation.chat_bot_id {
        Some(id) => id,
        None => {
            return Err(ComhairleError::CorruptedData(
                "Missing chat_bot_id on conversation: {conversation_id}".to_string(),
            ))
        }
    };

    let stream = bot_service
        .converse_with_chat(&session.bot_service_session_id, &chat_bot_id, payload)
        .await?;
    let body = Body::from_stream(stream);

    Ok(StreamBody(body))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(get_session, |op| {
                op.id("GetChatSessionHistory")
                    .tag("Chats")
                    .summary("Retrieves a session for a conversation's QA chat bot including messages history for a user")
                    .security_requirement("JWT")
                    .response::<200, Json<ComhairleChatSession>>()
            }),
        )
        .api_route(
            "/", 
            post_with(converse, |op| {
                op.tag("Chats")
                    .summary("Converse with a conversation's QA chat bot")
                    .security_requirement("JWT")
                    .description("Streamed LLM response.\n\n⚠️ This endpoint returns a streaming response on success.\nGenerated API clients are NOT suitable for consuming this endpoint.\nUse a raw HTTP request and process the response body incrementally.")
            }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot_service::{ComhairleChat, ComhairleKnowledgeBase, MockComhairleBotService};
    use crate::{
        setup_server,
        test_helpers::{test_state, UserSession},
    };
    use std::error::Error;
    use std::{pin::Pin, sync::Arc};

    use axum::body::{to_bytes, Body, Bytes};
    use futures::{stream, Stream};
    use mockall::predicate::{always, eq};
    use serde_json::json;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn should_return_single_chat_session(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let chat_session = ComhairleChatSession {
            id: "456".to_string(),
            name: Some("test_session".to_string()),
            ..Default::default()
        };
        let create_session = chat_session.clone();

        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_create_knowledge_base()
            .once()
            .returning(|_, _| {
                Box::pin(async move {
                    Ok((
                        StatusCode::CREATED,
                        ComhairleKnowledgeBase {
                            id: "123".to_string(),
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
                        id: "123".to_string(),
                        ..Default::default()
                    },
                ))
            })
        });
        bot_service
            .expect_create_chat_session()
            .once()
            .with(eq("123"), always())
            .returning(move |_, _| {
                Box::pin({
                    let session = create_session.clone();
                    async move { Ok((StatusCode::OK, session)) }
                })
            });
        bot_service
            .expect_get_chat_session()
            .once()
            .with(eq("456"), eq("123"))
            .returning(move |_, _| {
                Box::pin({
                    let session = chat_session.clone();
                    async move { Ok((StatusCode::OK, session)) }
                })
            });

        let state = test_state()
            .db(pool)
            .bot_service(Arc::new(bot_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_admin();
        session.signup(&app).await?;
        let (_, conversation, _) = session
            .create_conversation(
                &app,
                json!({
                    "title" : "Test conversation",
                    "short_description" : "A test conversation",
                    "description" : "A longer description",
                    "image_url" : "http://someimage.png",
                    "tags" : ["one", "two", "three"],
                    "is_public" : false,
                    "is_live": true,
                    "is_invite_only" : false,
                    "slug" : "new_conversation",
                    "primary_locale" : "en",
                    "supported_languages" : ["en"],
                    "knowledge_base_id": "123",
                    "chat_bot_id": "123"
                }),
            )
            .await?;
        let id = conversation.get("id").and_then(|v| v.as_str()).unwrap();
        let (status, response, _) = session
            .get(&app, &format!("/conversation/{id}/chat_sessions"))
            .await?;
        let json: ComhairleChatSession = serde_json::from_value(response)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(json.id, "456".to_string(), "incorrect json response");

        Ok(())
    }

    #[sqlx::test]
    async fn should_converse_with_chat_and_return_byte_stream(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let converse_request = ChatConversationRequest {
            question: "Test question?".to_string(),
        };
        let chat_session = ComhairleChatSession {
            id: "456".to_string(),
            name: Some("test_session".to_string()),
            ..Default::default()
        };

        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_create_knowledge_base()
            .once()
            .returning(|_, _| {
                Box::pin(async move {
                    Ok((
                        StatusCode::CREATED,
                        ComhairleKnowledgeBase {
                            id: "123".to_string(),
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
                        id: "123".to_string(),
                        ..Default::default()
                    },
                ))
            })
        });
        bot_service
            .expect_create_chat_session()
            .once()
            .with(eq("123"), always())
            .returning(move |_, _| {
                Box::pin({
                    let session = chat_session.clone();
                    async move { Ok((StatusCode::OK, session)) }
                })
            });
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

        let mut session = UserSession::new_admin();
        session.signup(&app).await?;
        let (_, conversation, _) = session
            .create_conversation(
                &app,
                json!({
                    "title" : "Test conversation",
                    "short_description" : "A test conversation",
                    "description" : "A longer description",
                    "image_url" : "http://someimage.png",
                    "tags" : ["one", "two", "three"],
                    "is_public" : false,
                    "is_live": true,
                    "is_invite_only" : false,
                    "slug" : "new_conversation",
                    "primary_locale" : "en",
                    "supported_languages" : ["en"],
                    "knowledge_base_id": "123",
                    "chat_bot_id": "123"
                }),
            )
            .await?;
        let id = conversation.get("id").and_then(|v| v.as_str()).unwrap();

        let body = serde_json::to_vec(&converse_request)?;
        let (status, body, _) = session
            .post_raw_response(
                &app,
                &format!("/conversation/{id}/chat_sessions"),
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
