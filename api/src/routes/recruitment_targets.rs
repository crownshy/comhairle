use std::sync::Arc;

use aide::axum::{
    ApiRouter,
    routing::{delete_with, get_with, post_with, put_with},
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    ComhairleState,
    error::ComhairleError,
    models::{
        self,
        recruitment_target::{CreateRecruitmentTarget, PartialRecruitmentTarget},
    },
    routes::{
        auth::RequiredAdminUser, recruitment_targets::dto::RecruitmentTargetDto,
        workflows::WorkflowPathCtx,
    },
};

pub mod dto;

async fn create_recruitment_target(
    State(state): State<Arc<ComhairleState>>,
    WorkflowPathCtx { workflow_id }: WorkflowPathCtx,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(create_request): Json<CreateRecruitmentTarget>,
) -> Result<(StatusCode, Json<RecruitmentTargetDto>), ComhairleError> {
    let target = models::recruitment_target::create(&state.db, &workflow_id, &create_request)
        .await?
        .into();
    Ok((StatusCode::CREATED, Json(target)))
}

async fn list_recruitment_targets(
    State(state): State<Arc<ComhairleState>>,
    WorkflowPathCtx { workflow_id }: WorkflowPathCtx,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<Vec<RecruitmentTargetDto>>), ComhairleError> {
    let targets = models::recruitment_target::list_for_workflow(&state.db, &workflow_id)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok((StatusCode::OK, Json(targets)))
}

async fn get_recruitment_target(
    State(state): State<Arc<ComhairleState>>,
    Path((_conversation_id, _workflow_id, target_id)): Path<(Uuid, Uuid, Uuid)>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<RecruitmentTargetDto>), ComhairleError> {
    let target = models::recruitment_target::get_by_id(&state.db, &target_id)
        .await?
        .into();
    Ok((StatusCode::OK, Json(target)))
}

async fn update_recruitment_target(
    State(state): State<Arc<ComhairleState>>,
    Path((_conversation_id, _workflow_id, target_id)): Path<(Uuid, Uuid, Uuid)>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(update_request): Json<PartialRecruitmentTarget>,
) -> Result<(StatusCode, Json<RecruitmentTargetDto>), ComhairleError> {
    let target = models::recruitment_target::update(&state.db, &target_id, &update_request)
        .await?
        .into();
    Ok((StatusCode::OK, Json(target)))
}

async fn delete_recruitment_target(
    State(state): State<Arc<ComhairleState>>,
    Path((_conversation_id, _workflow_id, target_id)): Path<(Uuid, Uuid, Uuid)>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<RecruitmentTargetDto>), ComhairleError> {
    let target = models::recruitment_target::delete(&state.db, &target_id)
        .await?
        .into();
    Ok((StatusCode::OK, Json(target)))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(create_recruitment_target, |op| {
                op.id("CreateRecruitmentTarget")
                    .tag("Recruitment Target")
                    .security_requirement("JWT")
                    .summary("Create a recruitment target for a workflow")
                    .description(
                        "Records the target number of participants for a given demographic \
                         metric/bucket combination on this workflow. Upserts on \
                         (workflow_id, metric, bucket).",
                    )
                    .response::<201, Json<RecruitmentTargetDto>>()
            }),
        )
        .api_route(
            "/",
            get_with(list_recruitment_targets, |op| {
                op.id("ListRecruitmentTargets")
                    .tag("Recruitment Target")
                    .security_requirement("JWT")
                    .summary("List recruitment targets for a workflow")
                    .response::<200, Json<Vec<RecruitmentTargetDto>>>()
            }),
        )
        .api_route(
            "/{recruitment_target_id}",
            get_with(get_recruitment_target, |op| {
                op.id("GetRecruitmentTarget")
                    .tag("Recruitment Target")
                    .security_requirement("JWT")
                    .summary("Get a recruitment target by id")
                    .response::<200, Json<RecruitmentTargetDto>>()
            }),
        )
        .api_route(
            "/{recruitment_target_id}",
            put_with(update_recruitment_target, |op| {
                op.id("UpdateRecruitmentTarget")
                    .tag("Recruitment Target")
                    .security_requirement("JWT")
                    .summary("Update a recruitment target")
                    .response::<200, Json<RecruitmentTargetDto>>()
            }),
        )
        .api_route(
            "/{recruitment_target_id}",
            delete_with(delete_recruitment_target, |op| {
                op.id("DeleteRecruitmentTarget")
                    .tag("Recruitment Target")
                    .security_requirement("JWT")
                    .summary("Delete a recruitment target")
                    .response::<200, Json<RecruitmentTargetDto>>()
            }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::model_test_helpers::setup_default_app_and_session,
        routes::{conversations::dto::ConversationDto, workflows::dto::WorkflowDto},
    };
    use serde_json::json;
    use sqlx::PgPool;
    use std::error::Error;

    async fn setup(
        pool: &PgPool,
    ) -> Result<
        (
            axum::Router,
            crate::test_helpers::UserSession,
            String,
            String,
        ),
        Box<dyn Error>,
    > {
        let (app, mut session) = setup_default_app_and_session(pool).await?;
        let (_, conversation, _) = session.create_random_conversation(&app).await?;
        let conversation: ConversationDto = serde_json::from_value(conversation)?;
        let (_, workflow, _) = session
            .create_random_workflow(&app, &conversation.id.to_string())
            .await?;
        let workflow: WorkflowDto = serde_json::from_value(workflow)?;
        Ok((
            app,
            session,
            conversation.id.to_string(),
            workflow.id.to_string(),
        ))
    }

    #[sqlx::test]
    async fn should_create_recruitment_target_via_api(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session, conversation_id, workflow_id) = setup(&pool).await?;

        let url =
            format!("/conversation/{conversation_id}/workflow/{workflow_id}/recruitment_targets");

        let (status, target, _) = session
            .post(
                &app,
                &url,
                json!({
                    "metric": "age",
                    "bucket": "18-24",
                    "target_count": 30
                })
                .to_string()
                .into(),
            )
            .await?;

        assert_eq!(status, StatusCode::CREATED, "should be created");
        let target: RecruitmentTargetDto = serde_json::from_value(target)?;
        assert_eq!(target.metric, "age");
        assert_eq!(target.bucket, "18-24");
        assert_eq!(target.target_count, 30);
        assert_eq!(target.workflow_id.to_string(), workflow_id);
        Ok(())
    }

    #[sqlx::test]
    async fn should_list_recruitment_targets_via_api(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session, conversation_id, workflow_id) = setup(&pool).await?;

        let url =
            format!("/conversation/{conversation_id}/workflow/{workflow_id}/recruitment_targets");

        for (metric, bucket, count) in [
            ("age", "18-24", 30),
            ("age", "25-34", 40),
            ("gender", "female", 50),
        ] {
            session
                .post(
                    &app,
                    &url,
                    json!({
                        "metric": metric,
                        "bucket": bucket,
                        "target_count": count
                    })
                    .to_string()
                    .into(),
                )
                .await?;
        }

        let (status, targets, _) = session.get(&app, &url).await?;
        assert_eq!(status, StatusCode::OK);
        let targets: Vec<RecruitmentTargetDto> = serde_json::from_value(targets)?;
        assert_eq!(targets.len(), 3);
        Ok(())
    }

    #[sqlx::test]
    async fn should_update_recruitment_target_via_api(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session, conversation_id, workflow_id) = setup(&pool).await?;

        let base_url =
            format!("/conversation/{conversation_id}/workflow/{workflow_id}/recruitment_targets");

        let (_, target, _) = session
            .post(
                &app,
                &base_url,
                json!({"metric": "age", "bucket": "18-24", "target_count": 30})
                    .to_string()
                    .into(),
            )
            .await?;
        let target: RecruitmentTargetDto = serde_json::from_value(target)?;

        let (status, updated, _) = session
            .put(
                &app,
                &format!("{base_url}/{}", target.id),
                json!({"target_count": 99}).to_string().into(),
            )
            .await?;
        assert_eq!(status, StatusCode::OK);
        let updated: RecruitmentTargetDto = serde_json::from_value(updated)?;
        assert_eq!(updated.target_count, 99);
        Ok(())
    }

    #[sqlx::test]
    async fn should_delete_recruitment_target_via_api(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session, conversation_id, workflow_id) = setup(&pool).await?;

        let base_url =
            format!("/conversation/{conversation_id}/workflow/{workflow_id}/recruitment_targets");

        let (_, target, _) = session
            .post(
                &app,
                &base_url,
                json!({"metric": "age", "bucket": "18-24", "target_count": 30})
                    .to_string()
                    .into(),
            )
            .await?;
        let target: RecruitmentTargetDto = serde_json::from_value(target)?;

        let (status, _, _) = session
            .delete(&app, &format!("{base_url}/{}", target.id))
            .await?;
        assert_eq!(status, StatusCode::OK);

        let (status, _, _) = session
            .get(&app, &format!("{base_url}/{}", target.id))
            .await?;
        assert_eq!(status, StatusCode::NOT_FOUND);
        Ok(())
    }
}
