use std::collections::HashMap;
use std::sync::Arc;

use aide::axum::{
    ApiRouter,
    routing::{delete_with, get_with, post_with, put_with},
};
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::middleware::from_fn_with_state;
use tracing::instrument;
use uuid::Uuid;

use crate::middleware::permissions::{PermissionRequirement, authorize};
use crate::models::permissions::{ConversationResource, conversation::Action};
use crate::models::statement::{self, CreateStatement, UpdateStatement};
use crate::routes::statements::dto::StatementDto;
use crate::{ComhairleError, ComhairleState};

pub mod dto;

fn path_param(params: &HashMap<String, Uuid>, name: &str) -> Result<Uuid, ComhairleError> {
    params
        .get(name)
        .copied()
        .ok_or_else(|| ComhairleError::ResourceNotFound(format!("Path must contain {name}")))
}

#[instrument(err(Debug), skip(state))]
async fn create_statement(
    State(state): State<Arc<ComhairleState>>,
    Path(params): Path<HashMap<String, Uuid>>,
    Json(payload): Json<CreateStatement>,
) -> Result<(StatusCode, Json<StatementDto>), ComhairleError> {
    let workflow_step_id = path_param(&params, "workflow_step_id")?;

    let created = statement::create(&state.db, workflow_step_id, &payload).await?;

    Ok((StatusCode::CREATED, Json(created.into())))
}

#[instrument(err(Debug), skip(state))]
async fn list_statements(
    State(state): State<Arc<ComhairleState>>,
    Path(params): Path<HashMap<String, Uuid>>,
) -> Result<(StatusCode, Json<Vec<StatementDto>>), ComhairleError> {
    let workflow_step_id = path_param(&params, "workflow_step_id")?;

    let statements = statement::list_for_workflow_step(&state.db, workflow_step_id).await?;

    Ok((
        StatusCode::OK,
        Json(statements.into_iter().map(Into::into).collect()),
    ))
}

#[instrument(err(Debug), skip(state))]
async fn update_statement(
    State(state): State<Arc<ComhairleState>>,
    Path(params): Path<HashMap<String, Uuid>>,
    Json(payload): Json<UpdateStatement>,
) -> Result<(StatusCode, Json<StatementDto>), ComhairleError> {
    let workflow_step_id = path_param(&params, "workflow_step_id")?;
    let statement_id = path_param(&params, "statement_id")?;

    let updated = statement::update(&state.db, workflow_step_id, statement_id, &payload).await?;

    Ok((StatusCode::OK, Json(updated.into())))
}

#[instrument(err(Debug), skip(state))]
async fn delete_statement(
    State(state): State<Arc<ComhairleState>>,
    Path(params): Path<HashMap<String, Uuid>>,
) -> Result<StatusCode, ComhairleError> {
    let workflow_step_id = path_param(&params, "workflow_step_id")?;
    let statement_id = path_param(&params, "statement_id")?;

    statement::delete(&state.db, workflow_step_id, statement_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub fn router() -> ApiRouter<Arc<ComhairleState>> {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(create_statement, |op| {
                op.id("CreateStatement")
                    .tag("Statements")
                    .summary("Create a statement on a workflow step")
                    .security_requirement("JWT")
                    .response::<201, Json<StatementDto>>()
            })
            .route_layer(from_fn_with_state(
                PermissionRequirement::<ConversationResource>::new(Action::Update),
                authorize::<ConversationResource>,
            )),
        )
        .api_route(
            "/",
            get_with(list_statements, |op| {
                op.id("ListStatements")
                    .tag("Statements")
                    .summary("List a workflow step's statements")
                    .security_requirement("JWT")
                    .response::<200, Json<Vec<StatementDto>>>()
            }),
        )
        .api_route(
            "/{statement_id}",
            put_with(update_statement, |op| {
                op.id("UpdateStatement")
                    .tag("Statements")
                    .summary("Update a statement's content and context")
                    .security_requirement("JWT")
                    .response::<200, Json<StatementDto>>()
            })
            .route_layer(from_fn_with_state(
                PermissionRequirement::<ConversationResource>::new(Action::Update),
                authorize::<ConversationResource>,
            )),
        )
        .api_route(
            "/{statement_id}",
            delete_with(delete_statement, |op| {
                op.id("DeleteStatement")
                    .tag("Statements")
                    .summary("Delete a statement")
                    .security_requirement("JWT")
                    .response::<204, ()>()
            })
            .route_layer(from_fn_with_state(
                PermissionRequirement::<ConversationResource>::new(Action::Update),
                authorize::<ConversationResource>,
            )),
        )
        .route_layer(from_fn_with_state(
            PermissionRequirement::<ConversationResource>::new(Action::Read),
            authorize::<ConversationResource>,
        ))
}

#[cfg(test)]
mod tests {
    use axum::Router;
    use serde_json::json;

    use super::*;

    use std::error::Error;

    use sqlx::PgPool;

    use crate::models::model_test_helpers::{
        get_random_conversation_id, get_random_workflow_id, setup_default_app_and_session,
    };
    use crate::routes::workflow_steps::dto::WorkflowStepDto;
    use crate::test_helpers::{UserSession, learn_tool_config};

    async fn create_step_and_url(
        app: &Router,
        session: &mut UserSession,
    ) -> Result<String, Box<dyn Error>> {
        let conversation_id = get_random_conversation_id(app, session).await?;
        let workflow_id = get_random_workflow_id(app, session, &conversation_id).await?;

        let (status, value, _) = session
            .create_workflow_step(
                app,
                &conversation_id.to_string(),
                &workflow_id.to_string(),
                json!({
                    "name": "step",
                    "step_order": 1,
                    "activation_rule": "manual",
                    "description": "A step",
                    "is_offline": false,
                    "required": false,
                    "tool_setup": learn_tool_config(),
                }),
            )
            .await?;
        assert_eq!(status, StatusCode::CREATED, "step not created: {value}");
        let step: WorkflowStepDto = serde_json::from_value(value)?;

        Ok(format!(
            "/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step/{}/statements",
            step.id
        ))
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_list_update_and_delete_statement(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let statements_url = create_step_and_url(&app, &mut session).await?;

        let (status, res, _) = session
            .post(
                &app,
                &statements_url,
                json!({
                    "content": "More buses",
                    "context": "Submitted via form",
                    "aiGenerated": true
                })
                .to_string()
                .into(),
            )
            .await?;
        assert_eq!(status, StatusCode::CREATED, "statement not created: {res}");
        let created: StatementDto = serde_json::from_value(res)?;
        assert!(created.ai_generated, "ai_generated not saved");

        let (status, res, _) = session.get(&app, &statements_url).await?;
        assert_eq!(status, StatusCode::OK, "list failed: {res}");
        let statements: Vec<StatementDto> = serde_json::from_value(res)?;
        assert_eq!(statements.len(), 1, "incorrect statement count");

        let statement_url = format!("{statements_url}/{}", created.id);
        let (status, res, _) = session
            .put(
                &app,
                &statement_url,
                json!({ "content": "Updated", "context": "Updated context" })
                    .to_string()
                    .into(),
            )
            .await?;
        assert_eq!(status, StatusCode::OK, "update failed: {res}");
        let updated: StatementDto = serde_json::from_value(res)?;
        assert_eq!(updated.content, "Updated", "content not updated");

        let (status, _, _) = session.delete(&app, &statement_url).await?;
        assert_eq!(status, StatusCode::NO_CONTENT, "statement not deleted");

        let (_, res, _) = session.get(&app, &statements_url).await?;
        let statements: Vec<StatementDto> = serde_json::from_value(res)?;
        assert!(statements.is_empty(), "statement still listed");

        Ok(())
    }
}
