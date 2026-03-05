use std::{collections::HashMap, fmt::Display, sync::Arc};

use aide::{
    axum::{
        routing::{delete_with, get_with, post_with, put_with},
        ApiRouter,
    },
    OperationIo,
};
use axum::{
    extract::{FromRequestParts, Path, State},
    http::{request::Parts, StatusCode},
    Json,
};
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::{
        conversation::{self, PartialConversation},
        user_participation::{self, UserParticipation},
        workflow::{self, CreateWorkflow, PartialWorkflow, WorkflowStats},
        workflow_step::{self, WorkflowStep},
    },
    routes::{
        auth::{RequiredAdminUser, RequiredUser},
        workflows::dto::WorkflowDto,
    },
    ComhairleState,
};

pub mod dto;

/// Path extractor for route handlers nested under both:
/// - `/conversations/:conversation_id/...`
/// - `/conversations/:conversation_id/events/:event_id/...`
///
/// Workflow and workflow step handlers can be mounted under either route, so this
/// extractor normalises path parameter extraction into a single type — always
/// providing `conversation_id` and optionally providing `event_id` when the
/// handler is invoked from the event-scoped route.
///
/// # Errors
///
/// Returns [`ComhairleError::BadRequest`] if `conversation_id` is absent from
/// the path, which should only occur if this extractor is used on a route that
/// does not include the `:conversation_id` segment.
#[derive(Debug, Clone, OperationIo)]
pub struct SourcePathCtx {
    pub conversation_id: Uuid,
    pub event_id: Option<Uuid>,
}

impl FromRequestParts<Arc<ComhairleState>> for SourcePathCtx {
    type Rejection = ComhairleError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<ComhairleState>,
    ) -> Result<Self, Self::Rejection> {
        let Path(params) = Path::<HashMap<String, Uuid>>::from_request_parts(parts, state).await?;

        let conversation_id = params
            .get("conversation_id")
            .cloned()
            .ok_or_else(|| ComhairleError::BadRequest("Missing conversation_id".into()))?;

        Ok(Self {
            conversation_id,
            event_id: params.get("event_id").cloned(),
        })
    }
}

/// Path extractor that reliably retrieves `:workflow_id` from routes nested
/// under either:
/// - `/conversations/:conversation_id/workflows/:workflow_id/...`
/// - `/conversations/:conversation_id/events/:event_id/workflows/:workflow_id/...`
///
/// Intended to be used alongside [`SourcePathCtx`], which handles the
/// `conversation_id` / `event_id` portion of the same routes.
///
/// # Errors
///
/// Returns [`ComhairleError::BadRequest`] if `workflow_id` is absent from the
/// path, which should only occur if this extractor is used on a route that does
/// not include the `:workflow_id` segment.
#[derive(Debug, Clone, OperationIo)]
pub struct WorkflowPathCtx {
    pub workflow_id: Uuid,
}

impl FromRequestParts<Arc<ComhairleState>> for WorkflowPathCtx {
    type Rejection = ComhairleError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<ComhairleState>,
    ) -> Result<Self, Self::Rejection> {
        let Path(params) = Path::<HashMap<String, Uuid>>::from_request_parts(parts, state).await?;

        let workflow_id = params
            .get("workflow_id")
            .cloned()
            .ok_or_else(|| ComhairleError::BadRequest("Missing workflow_id".into()))?;

        Ok(Self { workflow_id })
    }
}

/// Return the first step in the workflow that is not "done" for the
/// current user
async fn active_step_for_user(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    WorkflowPathCtx { workflow_id }: WorkflowPathCtx,
) -> Result<(StatusCode, Json<Option<WorkflowStep>>), ComhairleError> {
    let result =
        workflow_step::get_current_active_step_for_user(&state.db, &user.id, &workflow_id).await?;
    Ok((StatusCode::OK, Json(result)))
}

/// Register user on workflow
/// This end point will create a user participation
/// entry and a UserProgress entry for each of the
/// workflow_steps in this workflow
async fn register_user_for_workflow(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    WorkflowPathCtx { workflow_id }: WorkflowPathCtx,
) -> Result<(StatusCode, Json<UserParticipation>), ComhairleError> {
    let user_participation = workflow::register_user(&state.db, &workflow_id, &user).await?;
    Ok((StatusCode::CREATED, Json(user_participation)))
}

/// Remove a user from a given workflow
async fn deregister_user_on_workflow(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    WorkflowPathCtx { workflow_id }: WorkflowPathCtx,
) -> Result<(StatusCode, Json<UserParticipation>), ComhairleError> {
    let user_participation = user_participation::delete(&state.db, &user.id, &workflow_id).await?;
    Ok((StatusCode::OK, Json(user_participation)))
}

/// Returns the participation
/// status of a user on a workflow
async fn get_user_participation(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    WorkflowPathCtx { workflow_id }: WorkflowPathCtx,
) -> Result<(StatusCode, Json<Option<UserParticipation>>), ComhairleError> {
    let user_participation = user_participation::get(&state.db, &user.id, &workflow_id).await?;
    Ok((StatusCode::OK, Json(user_participation)))
}

/// Create workflow handler
async fn create_workflow(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    SourcePathCtx {
        conversation_id,
        event_id,
    }: SourcePathCtx,
    Json(new_workflow): Json<CreateWorkflow>,
) -> Result<(StatusCode, Json<WorkflowDto>), ComhairleError> {
    if event_id.is_some() {
        let workflow = workflow::create(&state.db, &new_workflow, None, event_id, user.id).await?;

        Ok((StatusCode::CREATED, Json(workflow.into())))
    } else {
        let conversation = conversation::get_by_id(&state.db, &conversation_id).await?;
        let workflow = workflow::create(
            &state.db,
            &new_workflow,
            Some(conversation_id),
            None,
            user.id,
        )
        .await?;
        // If the conversation does not have a default workflow
        // set this to be the default workflow
        if conversation.default_workflow_id.is_none() {
            conversation::update(
                &state.db,
                &conversation.id,
                &PartialConversation {
                    default_workflow_id: Some(workflow.id),
                    ..Default::default()
                },
            )
            .await?;
        }

        Ok((StatusCode::CREATED, Json(workflow.into())))
    }
}

async fn get_workflow_stats(
    State(state): State<Arc<ComhairleState>>,
    WorkflowPathCtx { workflow_id }: WorkflowPathCtx,
) -> Result<(StatusCode, Json<WorkflowStats>), ComhairleError> {
    let stats = workflow::stats(&state.db, workflow_id).await?;
    Ok((StatusCode::OK, Json(stats)))
}

/// Update workflow handler
async fn update_workflow(
    State(state): State<Arc<ComhairleState>>,
    WorkflowPathCtx { workflow_id }: WorkflowPathCtx,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(workflow): Json<PartialWorkflow>,
) -> Result<Json<WorkflowDto>, ComhairleError> {
    let workflow = workflow::update(&state.db, workflow_id, &workflow)
        .await?
        .into();
    Ok(Json(workflow))
}

/// List workflows handler
async fn list_workflows(
    State(state): State<Arc<ComhairleState>>,
    SourcePathCtx {
        conversation_id,
        event_id,
    }: SourcePathCtx,
) -> Result<(StatusCode, Json<Vec<WorkflowDto>>), ComhairleError> {
    let workflows = workflow::list(&state.db, conversation_id, event_id)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok((StatusCode::OK, Json(workflows)))
}

/// Get a specific workflow
async fn get_workflow(
    State(state): State<Arc<ComhairleState>>,
    WorkflowPathCtx { workflow_id }: WorkflowPathCtx,
) -> Result<(StatusCode, Json<WorkflowDto>), ComhairleError> {
    let workflow = workflow::get_by_id(&state.db, &workflow_id).await?.into();

    Ok((StatusCode::OK, Json(workflow)))
}

/// Delete a specific workflow
async fn delete_workflow(
    State(state): State<Arc<ComhairleState>>,
    WorkflowPathCtx { workflow_id }: WorkflowPathCtx,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<WorkflowDto>), ComhairleError> {
    let workflow = workflow::delete(&state.db, &workflow_id).await?.into();
    Ok((StatusCode::OK, Json(workflow)))
}

pub enum WorkflowRouterContext {
    Conversation,
    Event,
}

impl Display for WorkflowRouterContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkflowRouterContext::Conversation => write!(f, "Conversation"),
            WorkflowRouterContext::Event => write!(f, "Event"),
        }
    }
}

pub fn router(state: Arc<ComhairleState>, ctx: WorkflowRouterContext) -> ApiRouter {
    let router = ApiRouter::new()
        .api_route(
            "/",
            post_with(create_workflow, |op| {
                op.id(&format!("Create{ctx}Workflow"))
                    .tag("Workflow")
                    .security_requirement("JWT")
                    .summary("Create a new workflow on the conversation")
                    .response::<201, Json<WorkflowDto>>()
            }),
        )
        .api_route(
            "/",
            get_with(list_workflows, |op| {
                op.id(&format!("List{ctx}Workflows"))
                    .tag("Workflow")
                    .summary("List all workflows on this converastion")
                    .response::<200, Json<Vec<WorkflowDto>>>()
            }),
        )
        .api_route(
            "/{workflow_id}",
            get_with(get_workflow, |op| {
                op.id(&format!("Get{ctx}Workflow"))
                    .tag("Workflow")
                    .summary("Get the specified workflow")
                    .response::<200, Json<WorkflowDto>>()
            }),
        )
        .api_route(
            "/{workflow_id}",
            put_with(update_workflow, |op| {
                op.id(&format!("Update{ctx}Workflow"))
                    .tag("Workflow")
                    .security_requirement("JWT")
                    .summary("Update the workflow")
                    .response::<201, Json<WorkflowDto>>()
            }),
        )
        .api_route(
            "/{workflow_id}",
            delete_with(delete_workflow, |op| {
                op.id(&format!("Delete{ctx}Workflow"))
                    .tag("Workflow")
                    .security_requirement("JWT")
                    .summary("Delete the workflow and it's associated workflow steps")
                    .response::<201, Json<WorkflowDto>>()
            }),
        );

    let router = match ctx {
        WorkflowRouterContext::Conversation => router
            .api_route(
                "/{workflow_id}/next",
                get_with(active_step_for_user, |op| {
                    op.id(&format!("Next{ctx}WorkflowStepForUser"))
                        .tag("Workflow")
                        .security_requirement("JWT")
                        .summary("Gets the next undone workflow step for the current user")
                        .response::<201, Json<Option<WorkflowStep>>>()
                }),
            )
            .api_route(
                "/{workflow_id}/stats",
                get_with(get_workflow_stats, |op| {
                    op.id(&format!("Get{ctx}WorkflowStats"))
                        .tag("Workflow")
                        .summary("Gets participation stats for a workflow")
                        .response::<201, Json<WorkflowStats>>()
                }),
            )
            .api_route(
                "/{workflow_id}/register",
                post_with(register_user_for_workflow, |op| {
                    op.id(&format!("RegisterUserFor{ctx}Workflow"))
                        .tag("Workflow")
                        .security_requirement("JWT")
                        .summary("Register the currently logged in user for this workflow")
                        .response::<201, Json<UserParticipation>>()
                }),
            )
            .api_route(
                "/{workflow_id}/leave",
                delete_with(deregister_user_on_workflow, |op| {
                    op.id(&format!("UnregisterUserFor{ctx}Workflow"))
                        .tag("Workflow")
                        .security_requirement("JWT")
                        .summary("Unregisters the current user on this workflow")
                        .response::<200, Json<UserParticipation>>()
                }),
            )
            .api_route(
                "/{workflow_id}/participation",
                get_with(get_user_participation, |op| {
                    op.id(&format!("GetUser{ctx}Participation"))
                        .tag("Workflow")
                        .security_requirement("JWT")
                        .summary("Returns the status of the current user on this workflow")
                        .response::<200, Json<Option<UserParticipation>>>()
                }),
            ),
        _ => router,
    };

    router.with_state(state)
}

#[cfg(test)]
mod tests {

    use crate::{
        models::{
            model_test_helpers::{get_random_conversation_id, setup_default_app_and_session},
            workflow::WorkflowStats,
        },
        routes::{events::dto::EventDto, workflows::dto::WorkflowDto},
        setup_server,
        test_helpers::{extract, test_state, UserSession},
    };
    use axum::{body::Body, http::StatusCode};
    use serde_json::json;
    use sqlx::PgPool;
    use std::{error::Error, sync::Arc};

    #[sqlx::test]
    fn should_be_able_to_create_a_workflow_on_a_conversatin(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_admin();

        session.signup(&app).await?;

        let (_, conversation, _) = session.create_random_conversation(&app).await?;

        let id = conversation.get("id").to_owned().unwrap().to_owned();
        let id: String = serde_json::from_value(id).unwrap();

        let (status, workflow, _) = session
            .post(
                &app,
                &format!("/conversation/{id}/workflow"),
                json!({
                    "name": "simple workflow",
                    "description": "A super simple workflow",
                    "is_active" : true,
                    "is_public" : true,
                    "auto_login" :false
                })
                .to_string()
                .into(),
            )
            .await?;
        let workflow: WorkflowDto = serde_json::from_value(workflow).unwrap();

        assert_eq!(status, StatusCode::CREATED, "should have been created");

        assert_eq!(
            workflow.conversation_id.unwrap().to_string(),
            id,
            "Should be assigned to the correct conversation"
        );
        assert!(workflow.event_id.is_none(), "incorrect value for event_id");

        Ok(())
    }

    #[sqlx::test]
    fn should_create_a_workflow_for_an_event(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let (_, event, _) = session
            .create_random_event(&app, &conversation_id.to_string())
            .await?;
        let event: EventDto = serde_json::from_value(event)?;

        let (_, workflow, _) = session
            .post(
                &app,
                &format!(
                    "/conversation/{}/events/{}/workflows",
                    conversation_id, event.id
                ),
                json!({
                    "name": "simple workflow",
                    "description": "A super simple workflow",
                    "is_active" : true,
                    "is_public" : true,
                    "auto_login" :false
                })
                .to_string()
                .into(),
            )
            .await?;

        let workflow: WorkflowDto = serde_json::from_value(workflow)?;

        assert_eq!(workflow.event_id.unwrap(), event.id, "incorrect event_id");
        assert!(
            workflow.conversation_id.is_none(),
            "incorrect value for conversation_id"
        );

        Ok(())
    }

    #[sqlx::test]
    fn should_be_able_to_list_workflows_on_a_conversation(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_admin();

        session.signup(&app).await?;

        let (_, conversation, _) = session.create_random_conversation(&app).await?;
        let (_, conversation2, _) = session.create_random_conversation(&app).await?;

        let id: String = extract("id", &conversation);
        let id2: String = extract("id", &conversation2);

        let (_, workflow1, _) = session.create_random_workflow(&app, &id).await?;
        let (_, workflow2, _) = session.create_random_workflow(&app, &id).await?;
        let (_, workflow3, _) = session.create_random_workflow(&app, &id2).await?;

        let (status, workflows, _) = session
            .get(&app, &format!("/conversation/{id}/workflow"))
            .await?;

        let workflows: Vec<serde_json::Value> = serde_json::from_value(workflows).unwrap();

        assert_eq!(status, StatusCode::OK, "Should get OK status");

        assert_eq!(
            workflows.len(),
            2,
            "Should get the correct number of workflows back"
        );

        let ids: Vec<String> = workflows.iter().map(|a| extract("id", a)).collect();
        let workflow1_id = extract("id", &workflow1);
        let workflow2_id = extract("id", &workflow2);
        let workflow3_id = extract("id", &workflow3);
        assert!(
            ids.contains(&workflow1_id),
            "Should contain the first workflow"
        );
        assert!(
            ids.contains(&workflow2_id),
            "Should contain the second workflow"
        );
        assert!(
            !ids.contains(&workflow3_id),
            "Should not contain the third workflow"
        );
        Ok(())
    }

    #[sqlx::test]
    fn should_be_able_to_retrive_a_workflow(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_admin();

        session.signup(&app).await?;
        let (_, conversation, _) = session.create_random_conversation(&app).await?;
        let (_, conversation2, _) = session.create_random_conversation(&app).await?;

        let id: String = extract("id", &conversation);
        let id2: String = extract("id", &conversation2);

        session.create_random_workflow(&app, &id).await?;
        let (_, workflow2, _) = session.create_random_workflow(&app, &id).await?;
        session.create_random_workflow(&app, &id2).await?;

        let workflow2_id: String = extract("id", &workflow2);

        let (status, workflow, _) = session
            .get(&app, &format!("/conversation/{id}/workflow/{workflow2_id}"))
            .await?;

        assert_eq!(status, StatusCode::OK, "Should get OK status");

        let back_id: String = extract("id", &workflow);

        assert_eq!(
            back_id, workflow2_id,
            "Should get the correct workflow back"
        );

        Ok(())
    }

    #[sqlx::test]
    fn should_list_workflows_for_an_event(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let (_, event, _) = session
            .create_random_event(&app, &conversation_id.to_string())
            .await?;
        let event: EventDto = serde_json::from_value(event)?;
        let _ = session
            .create_random_event_workflow(&app, &conversation_id.to_string(), &event.id.to_string())
            .await?;
        let _ = session
            .create_random_event_workflow(&app, &conversation_id.to_string(), &event.id.to_string())
            .await?;
        let _ = session
            .create_random_event_workflow(&app, &conversation_id.to_string(), &event.id.to_string())
            .await?;

        let (_, workflow, _) = session
            .get(
                &app,
                &format!(
                    "/conversation/{}/events/{}/workflows",
                    conversation_id, event.id
                ),
            )
            .await?;
        let workflows: Vec<WorkflowDto> = serde_json::from_value(workflow)?;

        assert_eq!(workflows.len(), 3, "incorrect number of workflows");
        assert!(
            workflows[0].event_id.is_some(),
            "incorrect value for event_id"
        );
        assert!(
            workflows[0].conversation_id.is_none(),
            "incorrect value for event_id"
        );

        Ok(())
    }

    #[sqlx::test]
    fn should_get_a_workflow_for_an_event(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let (_, event, _) = session
            .create_random_event(&app, &conversation_id.to_string())
            .await?;
        let event: EventDto = serde_json::from_value(event)?;
        let (_, new_workflow, _) = session
            .create_random_event_workflow(&app, &conversation_id.to_string(), &event.id.to_string())
            .await?;
        let new_workflow: WorkflowDto = serde_json::from_value(new_workflow)?;

        let (_, workflow, _) = session
            .get(
                &app,
                &format!(
                    "/conversation/{}/events/{}/workflows/{}",
                    conversation_id, event.id, new_workflow.id
                ),
            )
            .await?;
        let workflow: WorkflowDto = serde_json::from_value(workflow)?;

        assert_eq!(workflow.id, new_workflow.id, "ids do not match");

        Ok(())
    }

    #[sqlx::test]
    fn should_be_able_to_delete_a_workflow(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_admin();

        session.signup(&app).await?;

        let (_, conversation, _) = session.create_random_conversation(&app).await?;
        let (_, conversation2, _) = session.create_random_conversation(&app).await?;

        let id: String = extract("id", &conversation);
        let id2: String = extract("id", &conversation2);

        session.create_random_workflow(&app, &id).await?;

        let (_, workflow2, _) = session.create_random_workflow(&app, &id).await?;

        session.create_random_workflow(&app, &id2).await?;

        let workflow2_id: String = extract("id", &workflow2);

        let url = format!("/conversation/{id}/workflow/{workflow2_id}");

        let (status, _, _) = session.delete(&app, &url).await?;

        assert_eq!(status, StatusCode::OK, "Should get OK status");

        let (status, _, _) = session.get(&app, &url).await?;

        assert_eq!(status, StatusCode::NOT_FOUND, "It should be gone");

        Ok(())
    }

    #[sqlx::test]
    fn should_delete_an_event_workflow(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let (_, event, _) = session
            .create_random_event(&app, &conversation_id.to_string())
            .await?;
        let event: EventDto = serde_json::from_value(event)?;
        let (_, new_workflow, _) = session
            .create_random_event_workflow(&app, &conversation_id.to_string(), &event.id.to_string())
            .await?;
        let new_workflow: WorkflowDto = serde_json::from_value(new_workflow)?;

        let _ = session
            .delete(
                &app,
                &format!(
                    "/conversation/{}/events/{}/workflows/{}",
                    conversation_id, event.id, new_workflow.id
                ),
            )
            .await?;

        let (_, value, _) = session
            .get(
                &app,
                &format!(
                    "/conversation/{}/events/{}/workflows/{}",
                    conversation_id, event.id, new_workflow.id
                ),
            )
            .await?;

        assert_eq!(
            value.get("err").and_then(|v| v.as_str()).unwrap(),
            "Workflow not found",
            "incorrect error json"
        );

        Ok(())
    }

    #[sqlx::test]
    fn should_be_able_to_update_a_workflow(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_admin();

        session.signup(&app).await?;

        let (_, conversation, _) = session.create_random_conversation(&app).await?;
        let (_, conversation2, _) = session.create_random_conversation(&app).await?;

        let id: String = extract("id", &conversation);
        let id2: String = extract("id", &conversation2);

        session.create_random_workflow(&app, &id).await?;

        let (_, workflow2, _) = session.create_random_workflow(&app, &id).await?;

        session.create_random_workflow(&app, &id2).await?;

        let workflow2_id: String = extract("id", &workflow2);

        let url = format!("/conversation/{id}/workflow/{workflow2_id}");

        let (status, _, _) = session
            .put(
                &app,
                &url,
                json!({
                    "name": "new_name"
                })
                .to_string()
                .into(),
            )
            .await?;

        assert_eq!(status, StatusCode::OK, "Should get OK status");

        let (status, workflow, _) = session.get(&app, &url).await?;

        assert_eq!(status, StatusCode::OK, "It should still be there");
        let name: String = extract("name", &workflow);
        assert_eq!(name, "new_name", "Should have an updated name");

        Ok(())
    }

    #[sqlx::test]
    fn should_update_an_event_workflow(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let (_, event, _) = session
            .create_random_event(&app, &conversation_id.to_string())
            .await?;
        let event: EventDto = serde_json::from_value(event)?;
        let (_, new_workflow, _) = session
            .create_random_event_workflow(&app, &conversation_id.to_string(), &event.id.to_string())
            .await?;
        let new_workflow: WorkflowDto = serde_json::from_value(new_workflow)?;

        assert!(new_workflow.is_public, "is_public before update");
        assert!(new_workflow.is_active, "is_active before update");

        let (_, value, _) = session
            .put(
                &app,
                &format!(
                    "/conversation/{}/events/{}/workflows/{}",
                    conversation_id, event.id, new_workflow.id
                ),
                json!({
                    "is_active": false,
                    "is_public": false
                })
                .to_string()
                .into(),
            )
            .await?;
        println!();
        println!("    >>>>    Value: {:#?}", value);
        println!();
        let workflow: WorkflowDto = serde_json::from_value(value)?;

        assert!(!workflow.is_public, "is_public after update");
        assert!(!workflow.is_active, "is_active after update");

        Ok(())
    }

    #[sqlx::test]
    fn should_get_the_correct_stats_for_a_workflow(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_admin();

        session.signup(&app).await?;

        let (_, conversation, _) = session.create_random_conversation(&app).await?;

        let id: String = extract("id", &conversation);

        let (_, workflow, _) = session.create_random_workflow(&app, &id).await?;

        let workflow_id: String = extract("id", &workflow);

        let steps = session
            .create_random_workflow_steps(&app, &id, &workflow_id, 10)
            .await?;

        for i in 0..10 {
            let mut session = UserSession::new(
                &format!("test_user_{i}"),
                "test_password",
                &format!("test.user_{i}@gmail.com"),
            );
            session.signup(&app).await?;

            let url = format!("/conversation/{id}/workflow/{workflow_id}/register");
            session.post(&app, &url, Body::empty()).await?;

            for j in 0..i {
                let workflow_step_id: String = extract("id", steps.get(j).unwrap());
                let url = format!(
                    "/conversation/{id}/workflow/{workflow_id}/progress/{workflow_step_id}"
                );

                session
                    .put(&app, &url, json!("done").to_string().into())
                    .await?;
            }
        }

        let url = format!("/conversation/{id}/workflow/{workflow_id}/stats");

        let (code, stats, _) = session.get(&app, &url).await?;
        let stats: WorkflowStats = serde_json::from_value(stats)?;
        assert_eq!(code, StatusCode::OK, "should get response");
        assert_eq!(
            stats.total_users, 10,
            "should get correct count of participatnts"
        );
        let step_stats = stats.step_stats;

        for (index, stats) in step_stats.iter().enumerate() {
            let count = stats.completed;
            assert_eq!(
                index as i32,
                9 - count,
                "should get the correct count for each step"
            );
        }

        Ok(())
    }
}
