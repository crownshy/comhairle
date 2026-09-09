//! The feedback survey on a conversation's thank-you page.
//!
//! A HeyForm form owned by the conversation rather than by a workflow step. Participants read
//! a sanitized config to embed it; the conversation's admins create it, open it in the builder
//! (which needs the form's admin credentials, so they get the full config) and read its
//! insights.

use std::sync::Arc;

use aide::axum::{
    ApiRouter,
    routing::{delete_with, get_with, post_with},
};
use axum::{
    Json,
    extract::{Path, State},
};
use hyper::StatusCode;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::{instrument, warn};
use uuid::Uuid;

use crate::{
    ComhairleState,
    error::ComhairleError,
    models::{
        conversation::{self, Conversation},
        feedback_survey::{self, FeedbackSurvey},
        permissions::{Action, can_perform_resource_action},
        users::User,
    },
    tools::{
        ToolConfig, ToolSetup,
        heyform::{self, HeyFormToolSetup, SurveyInsights},
    },
};

use super::auth::{OptionalUser, RequiredAdminUser, RequiredUser};

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FeedbackSurveyDto {
    pub conversation_id: Uuid,
    pub tool_config: ToolConfig,
    /// Whether the requesting user has finished it. Always false for an anonymous read.
    pub completed: bool,
}

impl FeedbackSurveyDto {
    fn new(survey: FeedbackSurvey, completed: bool) -> Self {
        Self {
            conversation_id: survey.conversation_id,
            tool_config: survey.tool_config,
            completed,
        }
    }
}

/// Whether this user may set up, remove or read the responses of the survey: the owner, or
/// anyone with update rights on the conversation.
async fn can_manage(
    state: &Arc<ComhairleState>,
    conversation: &Conversation,
    user: &User,
) -> Result<bool, ComhairleError> {
    if user.id == conversation.owner_id {
        return Ok(true);
    }
    can_perform_resource_action(
        state,
        &conversation.id,
        Action::ConversationUpdate,
        &user.id,
        user.organization_id.as_ref(),
        Some(&conversation.owner_id),
    )
    .await
}

async fn managed_conversation(
    state: &Arc<ComhairleState>,
    conversation_id: &Uuid,
    user: &User,
) -> Result<Conversation, ComhairleError> {
    let conversation = conversation::get_by_id(&state.db, conversation_id).await?;
    if !can_manage(state, &conversation, user).await? {
        return Err(ComhairleError::UserNotAuthorized);
    }
    Ok(conversation)
}

fn heyform_config(survey: &FeedbackSurvey) -> Result<&heyform::HeyFormToolConfig, ComhairleError> {
    match &survey.tool_config {
        ToolConfig::HeyForm(config) => Ok(config),
        _ => Err(ComhairleError::WorkflowStepHasWrongType("HeyForm".into())),
    }
}

/// The survey, if the conversation has one. Admins get the full config so the builder can
/// sign in; everyone else gets it with the credentials stripped. Who may read at all follows
/// the conversation itself: anyone once it is live, before that only people with read rights.
#[instrument(err(Debug), skip(state))]
async fn get_feedback_survey(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    OptionalUser(user): OptionalUser,
) -> Result<(StatusCode, Json<Option<FeedbackSurveyDto>>), ComhairleError> {
    let conversation = conversation::get_by_id(&state.db, &conversation_id).await?;

    let manager = match &user {
        Some(user) => can_manage(&state, &conversation, user).await?,
        None => false,
    };

    if !manager && !conversation.is_live {
        let reader = match &user {
            Some(user) => {
                can_perform_resource_action(
                    &state,
                    &conversation.id,
                    Action::ConversationRead,
                    &user.id,
                    user.organization_id.as_ref(),
                    Some(&conversation.owner_id),
                )
                .await?
            }
            None => false,
        };
        if !reader {
            return Err(ComhairleError::UserNotAuthorized);
        }
    }

    let Some(survey) = feedback_survey::get_for_conversation(&state.db, &conversation_id).await?
    else {
        return Ok((StatusCode::OK, Json(None)));
    };
    let survey = if manager { survey } else { survey.sanitize() };

    let completed = match &user {
        Some(user) => feedback_survey::is_completed(&state.db, &conversation_id, &user.id).await?,
        None => false,
    };

    Ok((
        StatusCode::OK,
        Json(Some(FeedbackSurveyDto::new(survey, completed))),
    ))
}

/// Records that the requesting user finished the survey, so they are not asked again. The
/// embedded form tells the page when it is done; the page tells us. HeyForm holds the answers
/// themselves, tagged with the same user id, so this is a marker, not the record.
#[instrument(err(Debug), skip(state))]
async fn complete_feedback_survey(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    RequiredUser(user): RequiredUser,
) -> Result<StatusCode, ComhairleError> {
    if feedback_survey::get_for_conversation(&state.db, &conversation_id)
        .await?
        .is_none()
    {
        return Err(ComhairleError::ResourceNotFound("Feedback survey".into()));
    }

    feedback_survey::mark_completed(&state.db, &conversation_id, &user.id).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Creates the survey's form on HeyForm and attaches it. Idempotent: a conversation that
/// already has one gets it back rather than a second form.
#[instrument(err(Debug), skip(state))]
async fn create_feedback_survey(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    RequiredAdminUser(user): RequiredAdminUser,
) -> Result<(StatusCode, Json<FeedbackSurveyDto>), ComhairleError> {
    let conversation = managed_conversation(&state, &conversation_id, &user).await?;

    if let Some(existing) =
        feedback_survey::get_for_conversation(&state.db, &conversation_id).await?
    {
        return Ok((
            StatusCode::OK,
            Json(FeedbackSurveyDto::new(existing, false)),
        ));
    }

    let setup = ToolSetup::HeyForm(HeyFormToolSetup {
        server_url: state.config.heyform_url.clone(),
    });
    let tool_config = setup.setup(&state, &conversation.primary_locale).await?;

    let survey = feedback_survey::create(&state.db, &conversation_id, tool_config).await?;

    Ok((
        StatusCode::CREATED,
        Json(FeedbackSurveyDto::new(survey, false)),
    ))
}

/// Detaches the survey and deletes its form, responses included. The row goes first so a
/// form the server refuses to delete cannot leave the page pointing at it.
#[instrument(err(Debug), skip(state))]
async fn delete_feedback_survey(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    RequiredAdminUser(user): RequiredAdminUser,
) -> Result<StatusCode, ComhairleError> {
    managed_conversation(&state, &conversation_id, &user).await?;

    let Some(survey) = feedback_survey::get_for_conversation(&state.db, &conversation_id).await?
    else {
        return Ok(StatusCode::NO_CONTENT);
    };

    feedback_survey::delete(&state.db, &conversation_id).await?;

    if let Err(e) = heyform::delete_form(heyform_config(&survey)?).await {
        warn!("Feedback survey form for {conversation_id} was detached but not deleted: {e:?}");
    }

    Ok(StatusCode::NO_CONTENT)
}

/// Per-question breakdown of the responses, labelled from the form schema.
#[instrument(err(Debug), skip(state))]
async fn feedback_survey_insights(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    RequiredAdminUser(user): RequiredAdminUser,
) -> Result<(StatusCode, Json<SurveyInsights>), ComhairleError> {
    managed_conversation(&state, &conversation_id, &user).await?;

    let survey = feedback_survey::get_for_conversation(&state.db, &conversation_id)
        .await?
        .ok_or_else(|| ComhairleError::ResourceNotFound("Feedback survey".into()))?;

    let insights = heyform::survey_insights(heyform_config(&survey)?).await?;

    Ok((StatusCode::OK, Json(insights)))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(get_feedback_survey, |op| {
                op.id("GetFeedbackSurvey")
                    .tag("Feedback survey")
                    .summary("Get the conversation's thank-you page feedback survey")
                    .description(
                        "Null when the conversation has none. Admins receive the form's \
                         credentials; everyone else receives a sanitized config.",
                    )
                    .response::<200, Json<Option<FeedbackSurveyDto>>>()
            }),
        )
        .api_route(
            "/",
            post_with(create_feedback_survey, |op| {
                op.id("CreateFeedbackSurvey")
                    .tag("Feedback survey")
                    .summary("Create a feedback survey for the conversation's thank-you page")
                    .description("Creates a HeyForm form and attaches it. Returns the existing survey if there already is one.")
                    .security_requirement("JWT")
                    .response::<201, Json<FeedbackSurveyDto>>()
            }),
        )
        .api_route(
            "/",
            delete_with(delete_feedback_survey, |op| {
                op.id("DeleteFeedbackSurvey")
                    .tag("Feedback survey")
                    .summary("Remove the conversation's feedback survey and its responses")
                    .security_requirement("JWT")
                    .response::<204, ()>()
            }),
        )
        .api_route(
            "/completion",
            post_with(complete_feedback_survey, |op| {
                op.id("CompleteFeedbackSurvey")
                    .tag("Feedback survey")
                    .summary("Record that the current user finished the feedback survey")
                    .security_requirement("JWT")
                    .response::<204, ()>()
            }),
        )
        .api_route(
            "/insights",
            get_with(feedback_survey_insights, |op| {
                op.id("GetFeedbackSurveyInsights")
                    .tag("Feedback survey")
                    .summary("Labelled survey insights for the conversation's feedback survey")
                    .security_requirement("JWT")
                    .response::<200, Json<SurveyInsights>>()
            }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use axum::body::Body;
    use serde_json::Value;
    use sqlx::PgPool;

    use crate::{
        models::{
            feedback_survey,
            model_test_helpers::{get_random_conversation_id, setup_default_app_and_session},
        },
        test_helpers::UserSession,
        tools::{ToolConfig, heyform::HeyFormToolConfig},
    };

    fn survey_config() -> ToolConfig {
        ToolConfig::HeyForm(HeyFormToolConfig {
            survey_id: "form-1".into(),
            survey_url: "localhost:1/form/form-1".into(),
            admin_user: "forms-admin@example.com".into(),
            admin_password: "secret".into(),
            workspace_id: "ws-1".into(),
            project_id: "proj-1".into(),
            // Nothing listens here, so a form delete fails fast rather than reaching a server.
            server_url: "localhost:1".into(),
        })
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn get_is_null_until_a_survey_exists(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let (status, body, _) = session
            .get(
                &app,
                &format!("/conversation/{conversation_id}/feedback_survey"),
            )
            .await?;

        assert_eq!(status, 200);
        assert_eq!(body, Value::Null);
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn the_owner_gets_the_credentials(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        feedback_survey::create(&pool, &conversation_id, survey_config()).await?;

        let (status, body, _) = session
            .get(
                &app,
                &format!("/conversation/{conversation_id}/feedback_survey"),
            )
            .await?;

        assert_eq!(status, 200);
        assert_eq!(body["conversationId"], conversation_id.to_string());
        assert_eq!(body["toolConfig"]["type"], "heyform");
        assert_eq!(body["toolConfig"]["survey_id"], "form-1");
        assert_eq!(body["toolConfig"]["admin_user"], "forms-admin@example.com");
        assert_eq!(body["toolConfig"]["admin_password"], "secret");
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn outsiders_cannot_read_before_launch(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        feedback_survey::create(&pool, &conversation_id, survey_config()).await?;
        // The helper creates a launched conversation; the point here is one that is not.
        sqlx::query("UPDATE conversation SET is_live = false WHERE id = $1")
            .bind(conversation_id)
            .execute(&pool)
            .await?;

        let mut guest = UserSession::new("guest", "guest", "guest@example.com");
        guest.signup_guest(&app).await?;

        let (status, _, _) = guest
            .get(
                &app,
                &format!("/conversation/{conversation_id}/feedback_survey"),
            )
            .await?;

        assert_ne!(status, 200);
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn participants_get_a_sanitized_config_once_live(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        // The helper's conversation is already launched.
        feedback_survey::create(&pool, &conversation_id, survey_config()).await?;

        let mut guest = UserSession::new("guest", "guest", "guest@example.com");
        guest.signup_guest(&app).await?;

        let (status, body, _) = guest
            .get(
                &app,
                &format!("/conversation/{conversation_id}/feedback_survey"),
            )
            .await?;

        assert_eq!(status, 200);
        assert_eq!(body["toolConfig"]["survey_id"], "form-1");
        assert_eq!(body["toolConfig"]["server_url"], "localhost:1");
        assert_eq!(body["toolConfig"]["admin_user"], "");
        assert_eq!(body["toolConfig"]["admin_password"], "");
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn completion_follows_the_user_not_the_browser(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        feedback_survey::create(&pool, &conversation_id, survey_config()).await?;

        let mut guest = UserSession::new("guest", "guest", "guest@example.com");
        guest.signup_guest(&app).await?;
        let url = format!("/conversation/{conversation_id}/feedback_survey");

        let (_, body, _) = guest.get(&app, &url).await?;
        assert_eq!(body["completed"], false);

        let (status, _, _) = guest
            .post(&app, &format!("{url}/completion"), Body::empty())
            .await?;
        assert_eq!(status, 204);

        let (_, body, _) = guest.get(&app, &url).await?;
        assert_eq!(body["completed"], true);

        // Another participant is still asked.
        let mut other = UserSession::new("other", "other", "other@example.com");
        other.signup_guest(&app).await?;
        let (_, body, _) = other.get(&app, &url).await?;
        assert_eq!(body["completed"], false);
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn delete_detaches_even_when_the_form_server_is_unreachable(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        feedback_survey::create(&pool, &conversation_id, survey_config()).await?;

        let (status, _, _) = session
            .delete(
                &app,
                &format!("/conversation/{conversation_id}/feedback_survey"),
            )
            .await?;
        assert_eq!(status, 204);

        let survey = feedback_survey::get_for_conversation(&pool, &conversation_id).await?;
        assert!(survey.is_none());

        // A second delete is a no-op, not an error.
        let (status, _, _) = session
            .delete(
                &app,
                &format!("/conversation/{conversation_id}/feedback_survey"),
            )
            .await?;
        assert_eq!(status, 204);
        Ok(())
    }
}
