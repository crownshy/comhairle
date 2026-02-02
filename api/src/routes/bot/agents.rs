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
    bot_service::ComhairleAgent,
    error::ComhairleError,
    routes::{auth::RequiredAdminUser, bot::GetQueryParams},
    ComhairleState,
};

#[instrument(err(Debug), skip(state))]
pub async fn get(
    State(state): State<Arc<ComhairleState>>,
    Path(agent_id): Path<String>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<ComhairleAgent>), ComhairleError> {
    let (_, agent) = state.bot_service.get_agent(&agent_id).await?;

    Ok((StatusCode::OK, Json(agent)))
}

#[instrument(err(Debug), skip(state))]
pub async fn list(
    State(state): State<Arc<ComhairleState>>,
    Query(params): Query<GetQueryParams>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<Vec<ComhairleAgent>>), ComhairleError> {
    let (_, agents) = state.bot_service.list_agents(Some(params)).await?;

    Ok((StatusCode::OK, Json(agents)))
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct CreateAgentRequest {
    pub name: String,
}

pub async fn create(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(payload): Json<CreateAgentRequest>,
) -> Result<(StatusCode, Json<ComhairleAgent>), ComhairleError> {
    let (_, agent) = state.bot_service.create_agent(payload).await?;

    Ok((StatusCode::CREATED, Json(agent)))
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Default, Clone, PartialEq)]
pub struct UpdateAgentRequest {
    pub name: Option<String>,
    pub topic: Option<String>,
}

pub async fn update(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Path(agent_id): Path<String>,
    Json(payload): Json<UpdateAgentRequest>,
) -> Result<(StatusCode, Json<ComhairleAgent>), ComhairleError> {
    let (_, agent) = state.bot_service.update_agent(&agent_id, payload).await?;

    Ok((StatusCode::OK, Json(agent)))
}

pub async fn delete(
    State(state): State<Arc<ComhairleState>>,
    Path(agent_id): Path<String>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<StatusCode, ComhairleError> {
    let _ = state.bot_service.delete_agent(&agent_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(list, |op| {
                op.id("ListAgents")
                    .tag("Bot Agents")
                    .summary("Get a list of agents")
                    .response::<200, Json<Vec<ComhairleAgent>>>()
            }),
        )
        .api_route(
            "/{agent_id}",
            get_with(get, |op| {
                op.id("GetAgent")
                    .tag("Bot Agents")
                    .summary("Get an agent by id")
                    .response::<200, Json<ComhairleAgent>>()
            }),
        )
        .api_route(
            "/",
            post_with(create, |op| {
                op.id("CreateAgent")
                    .tag("Bot Agents")
                    .summary("Create an agent")
                    .response::<201, Json<ComhairleAgent>>()
            }),
        )
        .api_route(
            "/{agent_id}",
            put_with(update, |op| {
                op.id("UpdateAgent")
                    .tag("Bot Agents")
                    .summary("Update an agent")
                    .response::<200, Json<ComhairleAgent>>()
            }),
        )
        .api_route(
            "/{agent_id}",
            delete_with(delete, |op| {
                op.id("DeleteAgent")
                    .tag("Bot Agents")
                    .summary("Delete an agent")
                    .response::<204, ()>()
            }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use crate::{
        bot_service::MockComhairleBotService,
        setup_server,
        test_helpers::{test_state, UserSession},
    };

    use super::*;
    use std::error::Error;

    use axum::body::Body;
    use mockall::predicate::eq;
    use serde_json::json;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn should_create_agent(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let agent = ComhairleAgent {
            id: "123".to_string(),
            name: "test_agent".to_string(),
            configuration: serde_json::json!({ "edges": [], "nodes": [] }),
        };

        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_create_agent()
            .once()
            .returning(move |_| {
                let agent = agent.clone();
                Box::pin(async move { Ok((StatusCode::OK, agent)) })
            });

        let state = test_state()
            .db(pool)
            .bot_service(Arc::new(bot_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;
        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        let create_agent = CreateAgentRequest {
            name: "test_agent".to_string(),
        };
        let bytes = serde_json::to_vec(&create_agent)?;
        let body = Body::from(bytes);
        let (status, value, _) = admin_session.post(&app, "/bot/agents", body).await?;

        assert!(status.is_success(), "error response status");
        assert_eq!(
            value.get("name").and_then(|v| v.as_str()).unwrap(),
            "test_agent",
            "incorrect json response"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_get_list_of_agents(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let agent = ComhairleAgent {
            id: "123".to_string(),
            name: "test_agent".to_string(),
            configuration: json!({ "graph": {}, "components": {} }),
        };
        let params = GetQueryParams {
            page: Some(2),
            ..Default::default()
        };

        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_list_agents()
            .once()
            .with(eq(Some(params)))
            .returning(move |_| {
                let agent = agent.clone();
                Box::pin(async move { Ok((StatusCode::OK, vec![agent])) })
            });

        let state = test_state()
            .db(pool)
            .bot_service(Arc::new(bot_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;
        let (status, response, _) = admin_session.get(&app, "/bot/agents?page=2").await?;
        let json: Vec<ComhairleAgent> = serde_json::from_value(response)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(
            json[0].name,
            "test_agent".to_string(),
            "incorrect json response"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_return_single_agent_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let agent = ComhairleAgent {
            id: "123".to_string(),
            name: "test_agent".to_string(),
            configuration: json!({ "graph": {}, "components": {} }),
        };

        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_get_agent()
            .once()
            .with(eq("123"))
            .returning(move |_| {
                let agent = agent.clone();
                Box::pin(async move { Ok((StatusCode::OK, agent)) })
            });

        let state = test_state()
            .db(pool)
            .bot_service(Arc::new(bot_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;
        let (status, response, _) = admin_session.get(&app, "/bot/agents/123").await?;
        let json: ComhairleAgent = serde_json::from_value(response)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(
            json.name,
            "test_agent".to_string(),
            "incorrect json response"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_update_an_agent(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let agent = ComhairleAgent {
            id: "123".to_string(),
            name: "test_agent".to_string(),
            configuration: json!({ "graph": {}, "components": {} }),
        };
        let update = UpdateAgentRequest {
            name: Some("test_agent".to_string()),
            ..Default::default()
        };

        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_update_agent()
            .once()
            .with(eq("123"), eq(update.clone()))
            .returning(move |_, _| {
                let agent = agent.clone();
                Box::pin(async move { Ok((StatusCode::OK, agent)) })
            });

        let state = test_state()
            .db(pool)
            .bot_service(Arc::new(bot_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        let bytes = serde_json::to_vec(&update)?;
        let body = Body::from(bytes);
        let (status, response, _) = admin_session.put(&app, "/bot/agents/123", body).await?;
        let json: ComhairleAgent = serde_json::from_value(response)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(
            json.name,
            "test_agent".to_string(),
            "incorrect json response"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_delete_an_agent(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_delete_agent()
            .once()
            .with(eq("123"))
            .returning(|_| Box::pin(async move { Ok(StatusCode::OK) }));

        let state = test_state()
            .db(pool)
            .bot_service(Arc::new(bot_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        let (status, _, _) = admin_session.delete(&app, "/bot/agents/123").await?;

        assert!(status.is_success(), "error response status");

        Ok(())
    }
}
