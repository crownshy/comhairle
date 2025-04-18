use std::sync::Arc;

use aide::axum::{
    routing::{delete_with, get_with, post_with, put_with},
    ApiRouter,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use tracing::info;
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::workflow::{self, CreateWorkflow, PartialWorkflow, Workflow, WorkflowStats},
    ComhairleState,
};

use super::auth::{RequiredAdminUser, RequiredUser};

/// Create workflow handler
async fn create_workflow(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    Path(conversation_id): Path<Uuid>,
    Json(new_workflow): Json<CreateWorkflow>,
) -> Result<(StatusCode, Json<Workflow>), ComhairleError> {
    info!("Attempting to create workflow {new_workflow:#?}");
    let workflow = workflow::create(&state.db, &new_workflow, conversation_id, user.id).await?;
    Ok((StatusCode::CREATED, Json(workflow)))
}

async fn workflow_stats(
    State(state): State<Arc<ComhairleState>>,
    Path((_, workflow_id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<WorkflowStats>), ComhairleError> {
    let stats = workflow::stats(&state.db, workflow_id).await?;
    Ok((StatusCode::OK, Json(stats)))
}

/// Update workflow handler
async fn update_workflow(
    State(state): State<Arc<ComhairleState>>,
    Path((_, id)): Path<(Uuid, Uuid)>,
    RequiredAdminUser(user): RequiredAdminUser,
    Json(workflow): Json<PartialWorkflow>,
) -> Result<Json<Workflow>, ComhairleError> {
    let workflow = workflow::update(&state.db, id, &workflow).await?;
    Ok(Json(workflow))
}

/// List workflows handler
async fn list_workflows(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Vec<Workflow>>), ComhairleError> {
    let workflows = workflow::list(&state.db, conversation_id).await?;
    Ok((StatusCode::OK, Json(workflows)))
}

/// Get a specific workflow
async fn get_workflow(
    State(state): State<Arc<ComhairleState>>,
    Path((_, workflow_id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<Workflow>), ComhairleError> {
    info!("Attempting to get workflow {workflow_id:#?}");
    let workflow = workflow::get_by_id(&state.db, &workflow_id).await?;

    Ok((StatusCode::OK, Json(workflow)))
}

/// Delete a specific workflow
async fn delete_workflow(
    State(state): State<Arc<ComhairleState>>,
    Path((_, id)): Path<(Uuid, Uuid)>,
    RequiredAdminUser(user): RequiredAdminUser,
) -> Result<(StatusCode, Json<Workflow>), ComhairleError> {
    let workflow = workflow::delete(&state.db, &id).await?;
    Ok((StatusCode::OK, Json(workflow)))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(create_workflow, |op| {
                op.id("CreateWorkflow")
                    .summary("Create a new workflow on the conversation")
                    .response::<201, Json<Workflow>>()
            }),
        )
        .api_route(
            "/",
            get_with(list_workflows, |op| {
                op.id("ListWorkflows")
                    .summary("List all workflows on this converastion")
                    .response::<200, Json<Vec<Workflow>>>()
            }),
        )
        .api_route(
            "/{workflow_id}/stats",
            get_with(workflow_stats, |op| {
                op.id("GetWorkflowStats")
                    .summary("Gets participation stats for a workflow")
                    .response::<201, Json<WorkflowStats>>()
            }),
        )
        .api_route(
            "/{workflow_id}",
            get_with(get_workflow, |op| {
                op.id("GetWorkflow")
                    .summary("Get the specified workflow")
                    .response::<200, Json<Workflow>>()
            }),
        )
        .api_route(
            "/{workflow_id}",
            put_with(update_workflow, |op| {
                op.id("UpdateWorkflow")
                    .summary("Update the workflow")
                    .response::<201, Json<Workflow>>()
            }),
        )
        .api_route(
            "/{workflow_id}",
            delete_with(delete_workflow, |op| {
                op.id("DeleteWorkflow")
                    .summary("Delete the workflow and it's associated workflow steps")
                    .response::<201, Json<Workflow>>()
            }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {

    use crate::{
        config, setup_server,
        test_helpers::{extract, UserSession},
    };
    use axum::{body::Body, http::StatusCode};
    use serde_json::json;
    use sqlx::PgPool;
    use std::{collections::HashMap, error::Error};

    #[sqlx::test]
    fn should_be_able_to_create_a_workflow_on_a_conversatin(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let config = config::load()?;
        let app = setup_server(config, pool).await?;

        let mut session = UserSession::new(
            "test_user".into(),
            "test_password".into(),
            "test.user@gmail.com".into(),
        );

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
                    "is_public" : true
                })
                .to_string()
                .into(),
            )
            .await?;

        assert_eq!(status, StatusCode::CREATED, "should have been created");

        let conv_id = workflow
            .get("conversation_id")
            .to_owned()
            .unwrap()
            .to_owned();
        let conv_id: String = serde_json::from_value(conv_id).unwrap();

        assert_eq!(
            conv_id, id,
            "Should be assigned to the correct conversation"
        );
        Ok(())
    }

    #[sqlx::test]
    fn should_be_able_to_list_workflows_on_a_conversation(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let config = config::load()?;
        let app = setup_server(config, pool).await?;

        let mut session = UserSession::new(
            "test_user".into(),
            "test_password".into(),
            "test.user@gmail.com".into(),
        );

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
        let config = config::load()?;
        let app = setup_server(config, pool).await?;

        let mut session = UserSession::new(
            "test_user".into(),
            "test_password".into(),
            "test.user@gmail.com".into(),
        );

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
    fn should_be_able_to_delete_a_workflow(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let config = config::load()?;
        let app = setup_server(config, pool).await?;

        let mut session = UserSession::new(
            "test_user".into(),
            "test_password".into(),
            "test.user@gmail.com".into(),
        );

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
    fn should_be_able_to_update_a_workflow(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let config = config::load()?;
        let app = setup_server(config, pool).await?;

        let mut session = UserSession::new(
            "test_user".into(),
            "test_password".into(),
            "test.user@gmail.com".into(),
        );

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
    fn should_get_the_correct_stats_for_a_workflow(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let config = config::load()?;
        let app = setup_server(config, pool).await?;

        let mut session = UserSession::new(
            "test_user".into(),
            "test_password".into(),
            "test.user@gmail.com".into(),
        );

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
                "test_password".into(),
                &format!("test.user_{i}@gmail.com"),
            );
            session.signup(&app).await?;

            let url = format!("/conversation/{id}/workflow/{workflow_id}/participation");
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
        assert_eq!(code, StatusCode::OK, "should get response");
        let total: i32 = extract("total_users", &stats);
        assert_eq!(total, 10, "should get correct count of participatnts");

        let step_completion: HashMap<String, i32> = extract("users_completed_step", &stats);

        for (index, step) in steps.iter().enumerate() {
            let id: String = extract("id", &step);
            let count = step_completion.get(&id).unwrap();

            assert_eq!(
                index as i32,
                9 - count,
                "should get the correct count for each step"
            );
        }

        Ok(())
    }
}
