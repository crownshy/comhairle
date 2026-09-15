use std::sync::Arc;

use aide::axum::{
    ApiRouter,
    routing::{delete_with, get_with, post_with, put_with},
};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use tracing::instrument;
use uuid::Uuid;

use crate::models::{
    self,
    moderation_policy::{self, CreateModerationPolicy, DEFAULT_REASONS, UpdateModerationPolicy},
    permissions::{Action, ConversationResource},
    users::User,
};
use crate::routes::auth::{RequiredUser, authorize};
use crate::{ComhairleError, ComhairleState};
use dto::{DefaultModerationPolicyReasonDto, ModerationPolicyDto};

pub mod dto;

/// Policies are edited in Configure and read while moderating, so every route needs the
/// conversation update permission that moderating a statement already checks.
async fn authorize_policy_access(
    state: &Arc<ComhairleState>,
    user: &User,
    conversation_id: &Uuid,
) -> Result<(), ComhairleError> {
    let conversation = models::conversation::get_by_id(&state.db, conversation_id).await?;
    authorize(
        state,
        user,
        Action::ConversationUpdate,
        &ConversationResource {
            conversation_id: conversation.id,
            owner_id: conversation.owner_id,
        },
    )
    .await
}

#[instrument(err(Debug), skip(state))]
async fn list_policies(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path(conversation_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Vec<ModerationPolicyDto>>), ComhairleError> {
    authorize_policy_access(&state, &user, &conversation_id).await?;

    let policies = moderation_policy::list_for_conversation(&state.db, conversation_id).await?;

    Ok((
        StatusCode::OK,
        Json(policies.into_iter().map(Into::into).collect()),
    ))
}

#[instrument(err(Debug), skip(state))]
async fn get_default_reasons(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path(conversation_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Vec<DefaultModerationPolicyReasonDto>>), ComhairleError> {
    authorize_policy_access(&state, &user, &conversation_id).await?;

    let reasons = DEFAULT_REASONS
        .iter()
        .map(|(label, description)| DefaultModerationPolicyReasonDto {
            label: label.to_string(),
            description: description.to_string(),
        })
        .collect();

    Ok((StatusCode::OK, Json(reasons)))
}

#[instrument(err(Debug), skip(state))]
async fn get_policy(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path((conversation_id, moderation_policy_id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<ModerationPolicyDto>), ComhairleError> {
    authorize_policy_access(&state, &user, &conversation_id).await?;

    let policy =
        moderation_policy::get_by_id(&state.db, conversation_id, moderation_policy_id).await?;

    Ok((StatusCode::OK, Json(policy.into())))
}

#[instrument(err(Debug), skip(state))]
async fn create_policy(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path(conversation_id): Path<Uuid>,
    Json(payload): Json<CreateModerationPolicy>,
) -> Result<(StatusCode, Json<ModerationPolicyDto>), ComhairleError> {
    authorize_policy_access(&state, &user, &conversation_id).await?;

    let policy = moderation_policy::create(&state.db, conversation_id, &payload).await?;

    Ok((StatusCode::CREATED, Json(policy.into())))
}

#[instrument(err(Debug), skip(state))]
async fn update_policy(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path((conversation_id, moderation_policy_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateModerationPolicy>,
) -> Result<(StatusCode, Json<ModerationPolicyDto>), ComhairleError> {
    authorize_policy_access(&state, &user, &conversation_id).await?;

    let policy =
        moderation_policy::update(&state.db, conversation_id, moderation_policy_id, &payload)
            .await?;

    Ok((StatusCode::OK, Json(policy.into())))
}

#[instrument(err(Debug), skip(state))]
async fn delete_policy(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path((conversation_id, moderation_policy_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, ComhairleError> {
    authorize_policy_access(&state, &user, &conversation_id).await?;

    moderation_policy::delete(&state.db, conversation_id, moderation_policy_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(list_policies, |op| {
                op.id("ListConversationModerationPolicies")
                    .tag("ModerationPolicies")
                    .summary("List a conversation's moderation policies")
                    .description("Lists each policy with its reasons in display order")
                    .security_requirement("JWT")
                    .response::<200, Json<Vec<ModerationPolicyDto>>>()
            }),
        )
        .api_route(
            "/",
            post_with(create_policy, |op| {
                op.id("CreateConversationModerationPolicy")
                    .tag("ModerationPolicies")
                    .summary("Create a moderation policy")
                    .description(
                        "Creates a policy on the conversation. Without reasons it starts from \
                        the default reasons.",
                    )
                    .security_requirement("JWT")
                    .response::<201, Json<ModerationPolicyDto>>()
            }),
        )
        .api_route(
            "/default",
            get_with(get_default_reasons, |op| {
                op.id("GetDefaultModerationPolicyReasons")
                    .tag("ModerationPolicies")
                    .summary("Get the default reject reasons")
                    .description("The reasons a Polis step uses when it has no moderation policy")
                    .security_requirement("JWT")
                    .response::<200, Json<Vec<DefaultModerationPolicyReasonDto>>>()
            }),
        )
        .api_route(
            "/{moderation_policy_id}",
            get_with(get_policy, |op| {
                op.id("GetConversationModerationPolicy")
                    .tag("ModerationPolicies")
                    .summary("Get a moderation policy")
                    .security_requirement("JWT")
                    .response::<200, Json<ModerationPolicyDto>>()
            }),
        )
        .api_route(
            "/{moderation_policy_id}",
            put_with(update_policy, |op| {
                op.id("UpdateConversationModerationPolicy")
                    .tag("ModerationPolicies")
                    .summary("Replace a moderation policy's name and reasons")
                    .description(
                        "Reasons sent with an id are updated in place, reasons without one are \
                        added, and reasons left out are deleted. The list order becomes the \
                        display order.",
                    )
                    .security_requirement("JWT")
                    .response::<200, Json<ModerationPolicyDto>>()
            }),
        )
        .api_route(
            "/{moderation_policy_id}",
            delete_with(delete_policy, |op| {
                op.id("DeleteConversationModerationPolicy")
                    .tag("ModerationPolicies")
                    .summary("Delete a moderation policy")
                    .description("Fails with 409 while a workflow step still uses the policy")
                    .security_requirement("JWT")
                    .response::<204, ()>()
            }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use axum::Router;
    use serde_json::{Value, json};
    use sqlx::PgPool;

    use super::*;

    use std::error::Error;

    use crate::models::model_test_helpers::{
        get_random_conversation_id, setup_default_app_and_session,
    };
    use crate::routes::workflows::dto::WorkflowDto;
    use crate::test_helpers::{UserSession, polis_tool_config};

    async fn create_policy(
        app: &Router,
        session: &mut UserSession,
        conversation_id: Uuid,
    ) -> Result<ModerationPolicyDto, Box<dyn Error>> {
        let (status, res, _) = session
            .post(
                app,
                &format!("/conversation/{conversation_id}/moderation_policies"),
                json!({ "name": "Policy" }).to_string().into(),
            )
            .await?;
        assert_eq!(status, StatusCode::CREATED, "policy not created: {res}");

        Ok(serde_json::from_value(res)?)
    }

    /// Creates a Polis step and returns its url and preview tool config.
    async fn create_polis_step(
        app: &Router,
        session: &mut UserSession,
        conversation_id: Uuid,
    ) -> Result<(String, Value), Box<dyn Error>> {
        let (_, workflow, _) = session
            .create_random_workflow(app, &conversation_id.to_string())
            .await?;
        let workflow: WorkflowDto = serde_json::from_value(workflow)?;
        let steps_url = format!(
            "/conversation/{conversation_id}/workflow/{}/workflow_step",
            workflow.id
        );

        let (status, step, _) = session
            .post(
                app,
                &steps_url,
                json!({
                    "name": "Polis step",
                    "step_order": 1,
                    "activation_rule": "manual",
                    "description": "A Polis step",
                    "is_offline": false,
                    "required": true,
                    "tool_setup": polis_tool_config()
                })
                .to_string()
                .into(),
            )
            .await?;
        assert_eq!(status, StatusCode::CREATED, "step not created: {step}");

        let step_id = step["id"].as_str().ok_or("missing step id")?;
        Ok((
            format!("{steps_url}/{step_id}"),
            step["previewToolConfig"].clone(),
        ))
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_list_and_get_policy(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let created = create_policy(&app, &mut session, conversation_id).await?;
        assert_eq!(
            created.reasons.len(),
            DEFAULT_REASONS.len(),
            "policy didn't start from the defaults"
        );

        let (_, res, _) = session
            .get(
                &app,
                &format!("/conversation/{conversation_id}/moderation_policies"),
            )
            .await?;
        let policies: Vec<ModerationPolicyDto> = serde_json::from_value(res)?;
        assert_eq!(policies.len(), 1, "incorrect policy count");

        let (status, res, _) = session
            .get(
                &app,
                &format!(
                    "/conversation/{conversation_id}/moderation_policies/{}",
                    created.id
                ),
            )
            .await?;
        assert_eq!(status, StatusCode::OK, "incorrect status code");
        let fetched: ModerationPolicyDto = serde_json::from_value(res)?;
        assert_eq!(fetched.id, created.id, "ids don't match");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_default_reasons(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let (status, res, _) = session
            .get(
                &app,
                &format!("/conversation/{conversation_id}/moderation_policies/default"),
            )
            .await?;
        assert_eq!(status, StatusCode::OK, "incorrect status code");
        let reasons: Vec<DefaultModerationPolicyReasonDto> = serde_json::from_value(res)?;
        assert_eq!(
            reasons.len(),
            DEFAULT_REASONS.len(),
            "incorrect reason count"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_replace_reasons_and_reject_repeated_labels(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let created = create_policy(&app, &mut session, conversation_id).await?;
        let policy_url = format!(
            "/conversation/{conversation_id}/moderation_policies/{}",
            created.id
        );

        let (status, res, _) = session
            .put(
                &app,
                &policy_url,
                json!({
                    "name": "Short list",
                    "reasons": [
                        { "id": created.reasons[4].id, "label": "Duplicate" },
                        { "label": "Spam", "description": "Posted again and again" }
                    ]
                })
                .to_string()
                .into(),
            )
            .await?;
        assert_eq!(status, StatusCode::OK, "update failed: {res}");
        let updated: ModerationPolicyDto = serde_json::from_value(res)?;
        let labels: Vec<&str> = updated
            .reasons
            .iter()
            .map(|reason| reason.label.as_str())
            .collect();
        assert_eq!(labels, ["Duplicate", "Spam"], "incorrect reasons");

        let (status, _, _) = session
            .put(
                &app,
                &policy_url,
                json!({
                    "name": "Short list",
                    "reasons": [{ "label": "Spam" }, { "label": "SPAM" }]
                })
                .to_string()
                .into(),
            )
            .await?;
        assert_eq!(status, StatusCode::BAD_REQUEST, "repeated label accepted");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_404_for_policy_in_another_conversation(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let other_conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let created = create_policy(&app, &mut session, other_conversation_id).await?;

        let (status, _, _) = session
            .get(
                &app,
                &format!(
                    "/conversation/{conversation_id}/moderation_policies/{}",
                    created.id
                ),
            )
            .await?;
        assert_eq!(status, StatusCode::NOT_FOUND, "incorrect status code");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_attach_policy_to_step_and_block_deleting_it(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let created = create_policy(&app, &mut session, conversation_id).await?;
        let (step_url, mut preview_tool_config) =
            create_polis_step(&app, &mut session, conversation_id).await?;

        preview_tool_config["moderation_policy_id"] = json!(created.id);
        let (status, res, _) = session
            .put(
                &app,
                &step_url,
                json!({ "preview_tool_config": preview_tool_config })
                    .to_string()
                    .into(),
            )
            .await?;
        assert_eq!(status, StatusCode::OK, "step update failed: {res}");
        assert_eq!(
            res["previewToolConfig"]["moderation_policy_id"],
            json!(created.id),
            "policy id not saved on the step"
        );

        let policy_url = format!(
            "/conversation/{conversation_id}/moderation_policies/{}",
            created.id
        );
        let (status, _, _) = session.delete(&app, &policy_url).await?;
        assert_eq!(status, StatusCode::CONFLICT, "policy in use was deleted");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_delete_unused_policy(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let created = create_policy(&app, &mut session, conversation_id).await?;
        let policy_url = format!(
            "/conversation/{conversation_id}/moderation_policies/{}",
            created.id
        );

        let (status, _, _) = session.delete(&app, &policy_url).await?;
        assert_eq!(status, StatusCode::NO_CONTENT, "incorrect status code");

        let (status, _, _) = session.get(&app, &policy_url).await?;
        assert_eq!(status, StatusCode::NOT_FOUND, "policy still exists");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_reject_step_policy_from_another_conversation(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let other_conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let other_policy = create_policy(&app, &mut session, other_conversation_id).await?;
        let (step_url, mut preview_tool_config) =
            create_polis_step(&app, &mut session, conversation_id).await?;

        preview_tool_config["moderation_policy_id"] = json!(other_policy.id);
        let (status, _, _) = session
            .put(
                &app,
                &step_url,
                json!({ "preview_tool_config": preview_tool_config })
                    .to_string()
                    .into(),
            )
            .await?;
        assert_eq!(
            status,
            StatusCode::BAD_REQUEST,
            "policy from another conversation accepted"
        );

        Ok(())
    }
}
