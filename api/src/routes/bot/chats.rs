use std::sync::Arc;

use aide::axum::{
    routing::{delete_with, get_with, post_with, put_with},
    ApiRouter,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{
    bot_service::{ComhairleChat, ComhairleLlm, ComhairlePrompt},
    error::ComhairleError,
    routes::{auth::RequiredAdminUser, bot::GetQueryParams},
    ComhairleState,
};

#[instrument(err(Debug), skip(state))]
async fn get(
    State(state): State<Arc<ComhairleState>>,
    Path(chat_id): Path<String>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<ComhairleChat>), ComhairleError> {
    let (_, chat) = state.bot_service.get_chat(&chat_id).await?;

    Ok((StatusCode::OK, Json(chat)))
}

#[instrument(err(Debug), skip(state))]
async fn list(
    State(state): State<Arc<ComhairleState>>,
    Query(params): Query<GetQueryParams>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<Vec<ComhairleChat>>), ComhairleError> {
    let (_, chats) = state.bot_service.list_chats(Some(params)).await?;

    Ok((StatusCode::OK, Json(chats)))
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Default)]
pub struct CreateChatRequest {
    pub name: String,
    pub knowledge_base_ids: Option<Vec<String>>,
    pub llm_model: Option<ComhairleLlm>,
    pub prompt: Option<ComhairlePrompt>,
}

#[instrument(err(Debug), skip(state))]
async fn create(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(payload): Json<CreateChatRequest>,
) -> Result<(StatusCode, Json<ComhairleChat>), ComhairleError> {
    let (_, chat) = state.bot_service.create_chat(payload).await?;

    Ok((StatusCode::CREATED, Json(chat)))
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Default, Clone, PartialEq)]
pub struct UpdateChatRequest {
    pub name: Option<String>,
    pub knowledge_base_ids: Option<Vec<String>>,
    pub llm_model: Option<ComhairleLlm>,
    pub prompt: Option<ComhairlePrompt>,
}

#[instrument(err(Debug), skip(state))]
async fn update(
    State(state): State<Arc<ComhairleState>>,
    Path(chat_id): Path<String>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(payload): Json<UpdateChatRequest>,
) -> Result<(StatusCode, Json<ComhairleChat>), ComhairleError> {
    let (_, chat) = state.bot_service.update_chat(&chat_id, payload).await?;

    Ok((StatusCode::OK, Json(chat)))
}

#[instrument(err(Debug), skip(state))]
async fn delete(
    State(state): State<Arc<ComhairleState>>,
    Path(chat_id): Path<String>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<StatusCode, ComhairleError> {
    let _ = state.bot_service.delete_chat(&chat_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(list, |op| {
                op.id("ListChats")
                    .summary("Get a list of chat bots")
                    .response::<200, Json<Vec<ComhairleChat>>>()
            }),
        )
        .api_route(
            "/{chat_id}",
            get_with(get, |op| {
                op.id("GetChat")
                    .summary("Get a chat bot by id")
                    .response::<200, Json<ComhairleChat>>()
            }),
        )
        .api_route(
            "/",
            post_with(create, |op| {
                op.id("CreateChat")
                    .summary("Create a new chat bot")
                    .response::<201, Json<ComhairleChat>>()
            }),
        )
        .api_route(
            "/{chat_id}",
            put_with(update, |op| {
                op.id("UpdateChat")
                    .summary("Update a chat bot")
                    .response::<200, Json<ComhairleChat>>()
            }),
        )
        .api_route(
            "/{chat_id}",
            delete_with(delete, |op| {
                op.id("DeleteChat")
                    .summary("Delete a chat bot")
                    .response::<204, ()>()
            }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::sync::Arc;

    use axum::body::Body;
    use mockall::predicate::eq;
    use sqlx::PgPool;

    use crate::bot_service::MockComhairleBotService;
    use crate::{
        setup_server,
        test_helpers::{test_state, UserSession},
    };

    #[sqlx::test]
    async fn should_get_list_of_chats(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let chat = ComhairleChat {
            name: "test_chat".to_string(),
            ..Default::default()
        };
        let params = GetQueryParams {
            page: Some(2),
            ..Default::default()
        };

        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_list_chats()
            .once()
            .with(eq(Some(params)))
            .returning(move |_| {
                let chat = chat.clone();
                Box::pin(async move { Ok((StatusCode::OK, vec![chat.clone()])) })
            });

        let state = test_state()
            .db(pool)
            .bot_service(Arc::new(bot_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;
        let (status, response, _) = admin_session.get(&app, "/bot/chats?page=2").await?;
        let json: Vec<ComhairleChat> = serde_json::from_value(response)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(
            json[0].name,
            "test_chat".to_string(),
            "incorrect json response"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_return_single_chat(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let chat = ComhairleChat {
            name: "test_chat".to_string(),
            ..Default::default()
        };

        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_get_chat()
            .once()
            .with(eq("123"))
            .returning(move |_| {
                Box::pin({
                    let chat = chat.clone();
                    async move { Ok((StatusCode::OK, chat.clone())) }
                })
            });

        let state = test_state()
            .db(pool)
            .bot_service(Arc::new(bot_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;
        let (status, response, _) = admin_session.get(&app, "/bot/chats/123").await?;
        let json: ComhairleChat = serde_json::from_value(response)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(
            json.name,
            "test_chat".to_string(),
            "incorrect json response"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_create_and_return_a_chat(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let chat = ComhairleChat {
            name: "test_chat".to_string(),
            ..Default::default()
        };

        let mut bot_service = MockComhairleBotService::new();
        bot_service.expect_create_chat().once().returning(move |_| {
            let chat = chat.clone();
            Box::pin(async move { Ok((StatusCode::OK, chat.clone())) })
        });

        let state = test_state()
            .db(pool)
            .bot_service(Arc::new(bot_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        let create_request = CreateChatRequest {
            name: "test_chat".to_string(),
            ..Default::default()
        };
        let bytes = serde_json::to_vec(&create_request)?;
        let body = Body::from(bytes);
        let (status, response, _) = admin_session.post(&app, "/bot/chats", body).await?;

        assert!(status.is_success(), "error response status");
        assert_eq!(
            response.get("name").and_then(|v| v.as_str()).unwrap(),
            "test_chat",
            "incorrect json response"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_update_and_return_chat(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let chat = ComhairleChat {
            name: "test_chat".to_string(),
            ..Default::default()
        };

        let update_request = UpdateChatRequest {
            name: Some("test_chat".to_string()),
            ..Default::default()
        };
        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_update_chat()
            .once()
            .with(eq("123"), eq(update_request.clone()))
            .returning(move |_, _| {
                let chat = chat.clone();
                Box::pin(async move { Ok((StatusCode::OK, chat.clone())) })
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
        let (status, response, _) = admin_session.put(&app, "/bot/chats/123", body).await?;

        assert!(status.is_success(), "error response status");
        assert_eq!(
            response.get("name").and_then(|v| v.as_str()).unwrap(),
            "test_chat",
            "incorrect json response"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_delete_chat(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_delete_chat()
            .once()
            .with(eq("123".to_string()))
            .returning(|_| Box::pin(async move { Ok(StatusCode::OK) }));

        let state = test_state()
            .db(pool)
            .bot_service(Arc::new(bot_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        let (status, _, _) = admin_session.delete(&app, "/bot/chats/123").await?;

        assert!(status.is_success(), "error response status");

        Ok(())
    }
}
