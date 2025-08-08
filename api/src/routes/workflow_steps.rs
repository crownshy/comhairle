use std::sync::Arc;

use aide::axum::routing::{delete_with, get_with, post_with, put_with};
use aide::axum::ApiRouter;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use tracing::info;
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::workflow_step::{self, CreateWorkflowStep, PartialWorkflowStep, WorkflowStep},
    ComhairleState,
};

use super::auth::{RequiredAdminUser, RequiredUser};
use crate::models::user_participation;

/// Create workflow handler
async fn create_workflow_step(
    State(state): State<Arc<ComhairleState>>,
    Path((_, workflow_id)): Path<(Uuid, Uuid)>,
    RequiredAdminUser(user): RequiredAdminUser,
    Json(new_workflow): Json<CreateWorkflowStep>,
) -> Result<(StatusCode, Json<WorkflowStep>), ComhairleError> {
    info!("Attempting to create workflow");
    let workflow = workflow_step::create(&state.db, &new_workflow, workflow_id).await?;
    Ok((StatusCode::CREATED, Json(workflow)))
}

/// Update workflow handler
async fn update_workflow_step(
    State(state): State<Arc<ComhairleState>>,
    Path((_, workflow_id, id)): Path<(Uuid, Uuid, Uuid)>,
    RequiredAdminUser(user): RequiredAdminUser,
    Json(workflow): Json<PartialWorkflowStep>,
) -> Result<(StatusCode, Json<WorkflowStep>), ComhairleError> {
    let workflow = workflow_step::update(&state.db, id, workflow_id, &workflow).await?;
    Ok((StatusCode::OK, Json(workflow)))
}

/// List workflows handler
async fn list_workflows_step(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path((_, workflow_id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<Vec<WorkflowStep>>), ComhairleError> {
    // Check to see if the user is a participant on this conversation
    let participation = user_participation::get(&state.db, &user.id, &workflow_id)
        .await
        .map_err(|_| ComhairleError::UserIsNotParticipatingInTheConversation)?;

    println!("{participation:#?}");
    let workflows = workflow_step::list(&state.db, workflow_id).await?;
    Ok((StatusCode::OK, Json(workflows)))
}

/// Get a specific workflow
async fn get_workflow_step(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path((_, _, workflow_step_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<(StatusCode, Json<WorkflowStep>), ComhairleError> {
    info!("Attempting to get workflow step  {workflow_step_id:#?}");
    let workflow = workflow_step::get_by_id(&state.db, &workflow_step_id).await?;

    Ok((StatusCode::OK, Json(workflow)))
}

/// Delete a specific workflow
async fn delete_workflow_step(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    Path((_, _, workflow_step_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<(StatusCode, Json<WorkflowStep>), ComhairleError> {
    let workflow = workflow_step::delete(&state.db, &workflow_step_id).await?;
    Ok((StatusCode::OK, Json(workflow)))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(create_workflow_step, |op| {
                op.id("CreateWorkflowStep")
                    .summary("Create a new workflow step")
                    .response::<201, Json<WorkflowStep>>()
            }),
        )
        .api_route(
            "/",
            get_with(list_workflows_step, |op| {
                op.id("ListWorkflowSteps")
                    .summary("List the workflow steps associated with this workflow")
                    .response::<200, Json<Vec<WorkflowStep>>>()
            }),
        )
        .api_route(
            "/{workflow_step_id}",
            get_with(get_workflow_step, |op| {
                op.id("GetWorkflowStep")
                    .summary("Get the specified workflow step")
                    .response::<200, Json<WorkflowStep>>()
            }),
        )
        .api_route(
            "/{workflow_step_id}",
            put_with(update_workflow_step, |op| {
                op.id("UpdateWorkflowStep")
                    .summary("Update the specifed workflow step")
                    .response::<200, Json<WorkflowStep>>()
            }),
        )
        .api_route(
            "/{workflow_step_id}",
            delete_with(delete_workflow_step, |op| {
                op.id("DeleteWorkflowStep")
                    .summary("Delete the specified workflow step")
                    .response::<200, Json<WorkflowStep>>()
            }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {

    use crate::{
        config,
        models::workflow_step,
        setup_server,
        test_helpers::{
            extract, learn_tool_config, polis_tool_config, test_config, test_state, UserSession,
        },
    };
    use axum::http::StatusCode;
    use serde_json::json;
    use sqlx::PgPool;
    use std::{error::Error, sync::Arc};

    #[sqlx::test]
    fn should_be_able_to_create_a_workflow_step(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_admin();

        session.signup(&app).await?;

        let (_, conversation, _) = session.create_random_conversation(&app).await?;

        let conversation_id: String = extract("id", &conversation);

        let (_, workflow, _) = session
            .create_random_workflow(&app, &conversation_id)
            .await?;

        let workflow_id: String = extract("id", &workflow);
        let (status, workflow_step, _) = session
            .post(
                &app,
                &format!("/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step"),
                json!({
                "name": "Worflow step",
                "step_order": 2,
                "activation_rule" : "manual",
                "description": "A manually retired polis workflow step",
                "is_offline": false,
                "tool_setup": {
                    "type": "polis",
                    "topic": "topic"
                }})
                .to_string()
                .into(),
            )
            .await?;

        assert_eq!(status, StatusCode::CREATED, "should have been created");
        Ok(())
    }

    #[sqlx::test]
    fn should_be_able_to_list_workflow_steps(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_admin();

        session.signup(&app).await?;

        let (_, conversation, _) = session.create_random_conversation(&app).await?;

        let conversation_id: String = extract("id", &conversation);

        let (_, workflow, _) = session
            .create_random_workflow(&app, &conversation_id)
            .await?;

        let workflow_id: String = extract("id", &workflow);

        session
            .post(
                &app,
                &format!("/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step"),
                json!({
                "name": "Polis Workflow step",
                "step_order": 1,
                "activation_rule" : "manual",
                "description": "A manually retired polis workflow step",
                "is_offline": false,
                "tool_setup": polis_tool_config()})
                .to_string()
                .into(),
            )
            .await?;

        session
            .post(
                &app,
                &format!("/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step"),
                json!({
                    "name": "Learn Workflow Step",
                    "step_order": 2,
                    "activation_rule" : "manual",
                    "description": "A manually retired learnworkflow step",
                    "is_offline": false,
                    "tool_setup": learn_tool_config()
                })
                .to_string()
                .into(),
            )
            .await?;

        let (status, workflows, _) = session
            .get(
                &app,
                &format!("/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step"),
            )
            .await?;

        assert_eq!(status, StatusCode::OK, "should be fine");

        let workflows: Vec<serde_json::Value> = serde_json::from_value(workflows)?;
        assert_eq!(workflows.len(), 2, "Should have two workflows");

        let workflow_1_return_name: String = extract("name", workflows.get(0).unwrap());
        let workflow_2_return_name: String = extract("name", workflows.get(1).unwrap());

        assert_eq!(
            "Polis Workflow step", workflow_1_return_name,
            "First workflow step should have the right name"
        );
        assert_eq!(
            "Learn Workflow Step", workflow_2_return_name,
            "Seccond workflow step should have the right name"
        );
        Ok(())
    }

    #[sqlx::test]
    fn should_be_able_to_retreive_workflow_step(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_admin();

        session.signup(&app).await?;

        let (_, conversation, _) = session.create_random_conversation(&app).await?;

        let conversation_id: String = extract("id", &conversation);

        let (_, workflow, _) = session
            .create_random_workflow(&app, &conversation_id)
            .await?;

        let workflow_id: String = extract("id", &workflow);

        session
            .post(
                &app,
                &format!("/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step"),
                json!({
                "name": "Polis Workflow Step",
                "step_order": 1,
                "activation_rule" : "manual",
                "description": "A manually retired polis workflow step",
                "is_offline": false,
                "tool_setup": polis_tool_config()})
                .to_string()
                .into(),
            )
            .await?;

        let (_, workflow_step, _) = session
            .post(
                &app,
                &format!("/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step"),
                json!({
                "name": "Learn Workflow Step",
                "step_order": 2,
                "activation_rule" : "manual",
                "description": "A manually retired learnworkflow step",
                "is_offline": false,
                "tool_setup": learn_tool_config()})
                .to_string()
                .into(),
            )
            .await?;

        let id: String = extract("id", &workflow_step);

        let (status, workflow, _) = session
            .get(
                &app,
                &format!(
                    "/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step/{id}"
                ),
            )
            .await?;

        assert_eq!(status, StatusCode::OK, "should be fine");

        let workflow_return_name: String = extract("name", &workflow);

        assert_eq!(
            "Learn Workflow Step", workflow_return_name,
            "should get back the right workflow_step"
        );
        Ok(())
    }

    #[sqlx::test]
    fn workflow_steps_should_reorder_when_a_step_is_deleted(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_admin();

        session.signup(&app).await?;

        let (_, conversation, _) = session.create_random_conversation(&app).await?;

        let conversation_id: String = extract("id", &conversation);

        let (_, workflow, _) = session
            .create_random_workflow(&app, &conversation_id)
            .await?;

        let workflow_id: String = extract("id", &workflow);

        let url = format!("/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step");

        let mut workflow_steps: Vec<serde_json::Value> = vec![];
        for no in 0..10 {
            let (_, step, _) = session
                .post(
                    &app,
                    &url,
                    json!({
                    "name": format!("{no}"),
                    "step_order": no+1,
                    "activation_rule" : "manual",
                    "description": "A manually retired polis workflow step",
                    "is_offline": false,
                    "tool_setup": learn_tool_config()})
                    .to_string()
                    .into(),
                )
                .await
                .expect("Workflow step to be created");
            workflow_steps.push(step);
        }

        let delete_id: String = extract("id", workflow_steps.get(4).unwrap());

        let (status, _, _) = session
            .delete(
                &app,
                &format!(
                "/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step/{delete_id}"
            ),
            )
            .await?;

        assert_eq!(status, StatusCode::OK, "should be deleted");

        let (_, steps, _) = session
            .get(
                &app,
                &format!("/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step"),
            )
            .await?;

        let steps: Vec<serde_json::Value> = serde_json::from_value(steps).unwrap();

        assert_eq!(steps.len(), 9, "should get the correct number of steps");

        let orders: Vec<i32> = steps
            .iter()
            .map(|s| extract::<i32>("step_order", s))
            .collect();

        let names: Vec<String> = steps.iter().map(|s| extract::<String>("name", s)).collect();

        assert_eq!(
            orders,
            [1, 2, 3, 4, 5, 6, 7, 8, 9],
            "should get back the correct orders"
        );
        assert_eq!(
            names,
            ["0", "1", "2", "3", "5", "6", "7", "8", "9"],
            "should get back the correct names"
        );

        Ok(())
    }

    #[sqlx::test]
    fn workflow_steps_should_return_in_correct_order(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_admin();

        session.signup(&app).await?;

        let (_, conversation, _) = session.create_random_conversation(&app).await?;

        let conversation_id: String = extract("id", &conversation);

        let (_, workflow, _) = session
            .create_random_workflow(&app, &conversation_id)
            .await?;

        let workflow_id: String = extract("id", &workflow);

        let url = format!("/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step");

        let mut workflow_steps: Vec<serde_json::Value> = vec![];
        for no in (0..10).rev() {
            let (status, step, _) = session
                .post(
                    &app,
                    &url,
                    json!({
                    "name": format!("{no}"),
                    "step_order": no+1,
                    "activation_rule" : "manual",
                    "description": "A manually retired polis workflow step",
                    "is_offline": false,
                    "tool_setup": learn_tool_config()})
                    .to_string()
                    .into(),
                )
                .await
                .expect("Workflow step to be created");
            workflow_steps.push(step);
        }

        let (_, steps, _) = session
            .get(
                &app,
                &format!("/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step"),
            )
            .await?;

        let steps: Vec<serde_json::Value> = serde_json::from_value(steps).unwrap();

        let orders: Vec<i32> = steps
            .iter()
            .map(|s| extract::<i32>("step_order", s))
            .collect();

        assert_eq!(
            orders,
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            "should get back the correct orders"
        );

        Ok(())
    }

    #[sqlx::test]
    fn workflow_steps_should_update_their_order_when_a_new_one_is_inserted(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_admin();

        session.signup(&app).await?;

        let (_, conversation, _) = session.create_random_conversation(&app).await?;

        let conversation_id: String = extract("id", &conversation);

        let (_, workflow, _) = session
            .create_random_workflow(&app, &conversation_id)
            .await?;

        let workflow_id: String = extract("id", &workflow);

        session
            .post(
                &app,
                &format!("/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step"),
                json!({
                "name": "Polis Workflow Step",
                "step_order": 1,
                "activation_rule" : "manual",
                "description": "A manually retired polis workflow step",
                "is_offline": false,
                "tool_setup": learn_tool_config()})
                .to_string()
                .into(),
            )
            .await?;

        session
            .post(
                &app,
                &format!("/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step"),
                json!({
                "name": "Learn Workflow Step",
                "step_order": 1,
                "activation_rule" : "manual",
                "description": "A manually retired learnworkflow step",
                "is_offline": false,
                "tool_setup": learn_tool_config()})
                .to_string()
                .into(),
            )
            .await?;

        let (_, workflows, _) = session
            .get(
                &app,
                &format!("/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step"),
            )
            .await?;

        let workflows: Vec<serde_json::Value> = serde_json::from_value(workflows)?;

        let first_name: String = extract("name", workflows.get(0).unwrap());
        let first_order: i32 = extract("step_order", workflows.get(0).unwrap());
        let seccond_name: String = extract("name", workflows.get(1).unwrap());
        let seccond_order: i32 = extract("step_order", workflows.get(1).unwrap());

        assert_eq!(
            first_name, "Learn Workflow Step",
            "Should correctly get the first step"
        );
        assert_eq!(first_order, 1, "Should correctly get the first step order");

        assert_eq!(
            seccond_name, "Polis Workflow Step",
            "Should correctly get the first step"
        );
        assert_eq!(
            seccond_order, 2,
            "Should correctly get the seccond step order"
        );
        Ok(())
    }

    #[sqlx::test]
    fn workflow_steps_should_rearange_properly_when_one_is_moved(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_admin();

        session.signup(&app).await?;

        let (_, conversation, _) = session.create_random_conversation(&app).await?;

        let conversation_id: String = extract("id", &conversation);

        let (_, workflow, _) = session
            .create_random_workflow(&app, &conversation_id)
            .await?;

        let workflow_id: String = extract("id", &workflow);

        let url = format!("/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step");

        let mut workflow_steps: Vec<serde_json::Value> = vec![];
        // Create a bunch of steps
        for no in 0..10 {
            let (_, step, _) = session
                .post(
                    &app,
                    &url,
                    json!({
                    "name": format!("{}", no+1),
                    "step_order": no+1,
                    "activation_rule" : "manual",
                    "description": "A manually retired polis workflow step",
                    "is_offline": false,
                    "tool_setup": learn_tool_config()})
                    .to_string()
                    .into(),
                )
                .await
                .expect("Workflow step to be created");
            workflow_steps.push(step);
        }

        // Update the fifth step to be the 7th
        let step_to_update = workflow_steps.get(0).expect("the 4th step to exisit");
        let update_id: String = extract("id", step_to_update);

        let (status, new_step, _) = session
            .put(
                &app,
                &format!("{url}/{update_id}"),
                json!({
                    "step_order" : 9
                })
                .to_string()
                .into(),
            )
            .await?;

        assert_eq!(status, StatusCode::OK, "Update should have been on");

        // get the steps
        let (_, steps, _) = session
            .get(
                &app,
                &format!("/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step"),
            )
            .await?;

        let steps: Vec<serde_json::Value> = serde_json::from_value(steps).unwrap();

        let orders: Vec<i32> = steps
            .iter()
            .map(|s| extract::<i32>("step_order", s))
            .collect();

        let names: Vec<String> = steps.iter().map(|s| extract::<String>("name", s)).collect();

        assert_eq!(
            orders,
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            "should get back the correct orders"
        );

        assert_eq!(
            names,
            ["2", "3", "4", "5", "6", "7", "8", "9", "1", "10"],
            "should get back the correct names"
        );
        Ok(())
    }
}
