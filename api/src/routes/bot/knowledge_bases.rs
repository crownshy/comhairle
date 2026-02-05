use std::sync::Arc;

use aide::axum::{
    routing::{delete_with, get_with, post_with, put_with},
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
    bot_service::{ComhairleKnowledgeBase, GetQueryParams},
    error::ComhairleError,
    routes::auth::RequiredAdminUser,
    ComhairleState,
};

#[instrument(err(Debug), skip(state))]
async fn list(
    State(state): State<Arc<ComhairleState>>,
    Query(params): Query<GetQueryParams>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<Vec<ComhairleKnowledgeBase>>), ComhairleError> {
    let (_, knowledge_bases) = state.bot_service.list_knowledge_bases(Some(params)).await?;

    Ok((StatusCode::OK, Json(knowledge_bases)))
}

#[instrument(err(Debug), skip(state))]
async fn get(
    State(state): State<Arc<ComhairleState>>,
    Path(knowledge_base_id): Path<String>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<ComhairleKnowledgeBase>), ComhairleError> {
    let (_, knowledge_base) = state
        .bot_service
        .get_knowledge_base(&knowledge_base_id)
        .await?;

    Ok((StatusCode::OK, Json(knowledge_base)))
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
struct CreateKnowledgeBaseRequest {
    name: String,
}

#[instrument(err(Debug), skip(state))]
async fn create(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(payload): Json<CreateKnowledgeBaseRequest>,
) -> Result<(StatusCode, Json<ComhairleKnowledgeBase>), ComhairleError> {
    let (_, knowledge_base) = state
        .bot_service
        .create_knowledge_base(payload.name, None)
        .await?;

    Ok((StatusCode::CREATED, Json(knowledge_base)))
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq, Clone)]
pub struct UpdateKnowledgeBaseRequest {
    pub name: Option<String>,
}

#[instrument(err(Debug), skip(state))]
async fn update(
    State(state): State<Arc<ComhairleState>>,
    Path(knowledge_base_id): Path<String>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(payload): Json<UpdateKnowledgeBaseRequest>,
) -> Result<(StatusCode, Json<ComhairleKnowledgeBase>), ComhairleError> {
    let (_, knowledge_base) = state
        .bot_service
        .update_knowledge_base(&knowledge_base_id, payload)
        .await?;

    Ok((StatusCode::OK, Json(knowledge_base)))
}

#[instrument(err(Debug), skip(state))]
async fn delete(
    State(state): State<Arc<ComhairleState>>,
    Path(knowledge_base_id): Path<String>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<StatusCode, ComhairleError> {
    let _ = state
        .bot_service
        .delete_knowledge_base(&knowledge_base_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(list, |op| {
                op.id("ListKnowledgeBases")
                    .tag("Bot Knowledge Bases")
                    .summary("Get a list of knowledge bases from RAG system")
                    .response::<200, Json<Vec<ComhairleKnowledgeBase>>>()
            }),
        )
        .api_route(
            "/{knowledge_base_id}",
            get_with(get, |op| {
                op.id("GetKnowledgeBase")
                    .tag("Bot Knowledge Bases")
                    .summary("Get a knowledge base from RAG system")
                    .response::<200, Json<ComhairleKnowledgeBase>>()
            }),
        )
        .api_route(
            "/",
            post_with(create, |op| {
                op.id("CreateKnowledgeBase")
                    .tag("Bot Knowledge Bases")
                    .summary("Create a knowledge base in RAG system")
                    .response::<201, Json<ComhairleKnowledgeBase>>()
            }),
        )
        .api_route(
            "/{knowledge_base_id}",
            put_with(update, |op| {
                op.id("UpdateKnowledgeBase")
                    .tag("Bot Knowledge Bases")
                    .summary("Update a knowledge base")
                    .response::<200, Json<ComhairleKnowledgeBase>>()
            }),
        )
        .api_route(
            "/{knowledge_base_id}",
            delete_with(delete, |op| {
                op.id("DeleteKnowledgeBase")
                    .tag("Bot Knowledge Bases")
                    .summary("Delete a knowledge from from RAG system")
                    .response::<204, ()>()
            }),
        )
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
    use std::sync::Arc;

    use axum::body::Body;
    use mockall::predicate::eq;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn should_return_knowledge_base_list(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let knowledge_base = ComhairleKnowledgeBase {
            name: "test_knowledge_base".to_string(),
            ..Default::default()
        };
        let params = GetQueryParams {
            page: Some(2),
            ..Default::default()
        };

        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_list_knowledge_bases()
            .once()
            .with(eq(Some(params)))
            .returning(move |_| {
                Box::pin({
                    let knowledge_base = knowledge_base.clone();
                    async move { Ok((StatusCode::OK, vec![knowledge_base.clone()])) }
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
            .get(&app, "/bot/knowledge_bases?page=2")
            .await?;
        let json: Vec<ComhairleKnowledgeBase> = serde_json::from_value(value)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(
            json[0].name,
            "test_knowledge_base".to_string(),
            "incorrect json response"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_return_single_knowledge_base(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let knowledge_base = ComhairleKnowledgeBase {
            name: "test_knowledge_base".to_string(),
            ..Default::default()
        };

        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_get_knowledge_base()
            .once()
            .with(eq("123"))
            .returning(move |_| {
                Box::pin({
                    let knowledge_base = knowledge_base.clone();
                    async move { Ok((StatusCode::OK, knowledge_base.clone())) }
                })
            });

        let state = test_state()
            .db(pool)
            .bot_service(Arc::new(bot_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;
        let (status, response, _) = admin_session.get(&app, "/bot/knowledge_bases/123").await?;
        let json: ComhairleKnowledgeBase = serde_json::from_value(response)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(
            json.name,
            "test_knowledge_base".to_string(),
            "incorrect json response"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_create_and_return_a_knowledgebase(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let knowledge_base = ComhairleKnowledgeBase {
            name: "test_knowledge_base".to_string(),
            ..Default::default()
        };

        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_create_knowledge_base()
            .once()
            .returning(move |_, _| {
                let knowledge_base = knowledge_base.clone();
                Box::pin(async move { Ok((StatusCode::OK, knowledge_base.clone())) })
            });

        let state = test_state()
            .db(pool)
            .bot_service(Arc::new(bot_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        let create_request = CreateKnowledgeBaseRequest {
            name: "test_knowledge_base".to_string(),
        };
        let bytes = serde_json::to_vec(&create_request)?;
        let body = Body::from(bytes);
        let (status, response, _) = admin_session
            .post(&app, "/bot/knowledge_bases", body)
            .await?;

        assert!(status.is_success(), "error response status");
        assert_eq!(
            response.get("name").and_then(|v| v.as_str()).unwrap(),
            "test_knowledge_base",
            "incorrect json response"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_update_and_return_knowledge_base(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let knowledge_base = ComhairleKnowledgeBase {
            name: "test_knowledge_base".to_string(),
            ..Default::default()
        };

        let update_request = UpdateKnowledgeBaseRequest {
            name: Some("test_knowledge_base".to_string()),
        };
        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_update_knowledge_base()
            .once()
            .with(eq("123"), eq(update_request.clone()))
            .returning(move |_, _| {
                let knowledge_base = knowledge_base.clone();
                Box::pin(async move { Ok((StatusCode::OK, knowledge_base.clone())) })
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
            .put(&app, "/bot/knowledge_bases/123", body)
            .await?;

        assert!(status.is_success(), "error response status");
        assert_eq!(
            response.get("name").and_then(|v| v.as_str()).unwrap(),
            "test_knowledge_base",
            "incorrect json response"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_delete_knowledge_base(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_delete_knowledge_base()
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

        let (status, _, _) = admin_session
            .delete(&app, "/bot/knowledge_bases/123")
            .await?;

        assert!(status.is_success(), "error response status");

        Ok(())
    }
}
