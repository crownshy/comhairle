use std::sync::Arc;

use aide::axum::{
    routing::{get_with, patch_with, post_with},
    ApiRouter,
};
use axum::{
    extract::{Path, State},
    Json,
};
use hyper::StatusCode;
use minijinja::context;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::{
        self,
        invites::{CreateInviteDTO, DailyResponseStats, PartialInvite},
        workflow,
    },
    routes::invites::dto::InviteDto,
    ComhairleState,
};

use super::auth::{OptionalUser, RequiredAdminUser, RequiredUser};

pub mod dto;

#[instrument(err(Debug), skip(state))]
async fn accept_invite(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path((conversation_id, invite_id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<InviteDto>), ComhairleError> {
    let invite = models::invites::get(&state.db, &invite_id).await?;
    let conversation = models::conversation::get_by_id(&state.db, &conversation_id).await?;

    // Check to see if the invite is valid
    invite.is_still_valid()?;
    invite.is_for_user(&user)?;

    // Get the workflow to sign up to either explicitly from the invite
    // or from the default conversation workflow
    let workflow_id = match (invite.workflow_id, conversation.default_workflow_id) {
        (Some(invite_workflow), _) => Ok(invite_workflow),
        (None, Some(conversation_workflow)) => Ok(conversation_workflow),
        (None, None) => Err(ComhairleError::NoWorkflowFoundForInvite),
    }?;

    workflow::register_user(&state.db, &workflow_id, &user).await?;

    invite
        .accept(&state.db, &user)
        .await
        .map(|new_invite| (StatusCode::OK, Json(new_invite.into())))
}

#[instrument(err(Debug), skip(state))]
async fn reject_invite(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path((_, invite_id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<InviteDto>), ComhairleError> {
    let invite = models::invites::get(&state.db, &invite_id).await?;

    // Check to see if the invite is valid
    invite.is_still_valid()?;
    invite.is_for_user(&user)?;

    invite
        .reject(&state.db, &user)
        .await
        .map(|new_invite| (StatusCode::OK, Json(new_invite.into())))
}

#[instrument(err(Debug), skip(state))]
async fn create_invite(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    RequiredAdminUser(user): RequiredAdminUser,
    Json(create_invite): Json<CreateInviteDTO>,
) -> Result<(StatusCode, Json<InviteDto>), ComhairleError> {
    let conversation = models::conversation::get_by_id(&state.db, &conversation_id).await?;

    if conversation.owner_id != user.id {
        return Err(ComhairleError::UserIsNotConversationOwner);
    }

    // TODO We might need something in here to check that the user in the user type
    // exists

    // Create the invite
    let invite =
        models::invites::create(&state.db, create_invite, &conversation_id, &user.id).await?;

    // Send out an email notification if we can
    match &invite.invite_type {
        models::invites::InviteType::Email(email) => {
            state.mailer.send_email(
            email,
            "Invitation to take part in the National Performance Framework consultation",
            "conversation_invite.html",
            context! {
                conversation_hero => conversation.image_url,
                conversation_title=> conversation.title,
                invite_link => format!("{}/conversations/{}/invite/{}",state.config.domain, conversation.slug.unwrap_or_else(|| conversation.id.to_string()), invite.id )
            },
        )?;
        }
        models::invites::InviteType::User(user_id) => {
            let user = models::users::get_user_by_id(user_id, &state.db).await?;
            if let Some(email) = &user.email {
                state.mailer.send_email(
                email,
                "You have been invited to the conversation",
                "conversation_invite.html",
                context! {user=>user, conversation_hero => conversation.image_url , conversation_title=>conversation.title},
            )?;
            }
        }
        models::invites::InviteType::Open | models::invites::InviteType::SingleUse => {}
    };

    let invite = invite.into();
    Ok((StatusCode::CREATED, Json(invite)))
}

#[instrument(err(Debug), skip(state))]
async fn get_invite(
    State(state): State<Arc<ComhairleState>>,
    Path((_, invite_id)): Path<(Uuid, Uuid)>,
    OptionalUser(user): OptionalUser,
) -> Result<(StatusCode, Json<InviteDto>), ComhairleError> {
    let invite = models::invites::get(&state.db, &invite_id).await?;
    invite.is_still_valid()?;

    // If we have a signed in user
    // check to see if this invite is for them
    if let Some(user) = user {
        invite.is_for_user(&user)?;
        Ok((StatusCode::OK, Json(invite.into())))
    } else {
        // Otherwise allow the invite to be seen if it's
        // an email or open invite
        match invite.invite_type {
            models::invites::InviteType::Email(_)
            | models::invites::InviteType::Open
            | models::invites::InviteType::SingleUse => Ok((StatusCode::OK, Json(invite.into()))),
            models::invites::InviteType::User(_) => Err(ComhairleError::UserRequired),
        }
    }
}

#[instrument(err(Debug), skip(state))]
async fn get_invite_stats(
    State(state): State<Arc<ComhairleState>>,
    Path((_, invite_id)): Path<(Uuid, Uuid)>,
    RequiredAdminUser(user): RequiredAdminUser,
) -> Result<(StatusCode, Json<Vec<DailyResponseStats>>), ComhairleError> {
    // Check that invite exists
    models::invites::get(&state.db, &invite_id).await?;

    // Generate stats``
    let stats = models::invites::get_stats_for_invite(&state.db, &invite_id).await?;
    Ok((StatusCode::OK, Json(stats)))
}

#[instrument(err(Debug), skip(state))]
async fn update_invite(
    State(state): State<Arc<ComhairleState>>,
    Path((_, invite_id)): Path<(Uuid, Uuid)>,
    RequiredAdminUser(_): RequiredAdminUser,
    Json(partial_invite): Json<PartialInvite>,
) -> Result<(StatusCode, Json<InviteDto>), ComhairleError> {
    let invite = models::invites::update(&state.db, &invite_id, partial_invite)
        .await?
        .into();

    Ok((StatusCode::OK, Json(invite)))
}

#[instrument(err(Debug), skip(state))]
async fn delete_invite(
    State(state): State<Arc<ComhairleState>>,
    Path((_, invite_id)): Path<(Uuid, Uuid)>,
    RequiredAdminUser(_): RequiredAdminUser,
) -> Result<(StatusCode, Json<InviteDto>), ComhairleError> {
    let invite = models::invites::delete(&state.db, &invite_id).await?.into();

    Ok((StatusCode::OK, Json(invite)))
}

#[instrument(err(Debug), skip(state))]
async fn list_invites_for_conversation(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    RequiredAdminUser(_): RequiredAdminUser,
) -> Result<(StatusCode, Json<Vec<InviteDto>>), ComhairleError> {
    let invites = (models::invites::list_for_conversation(&state.db, &conversation_id).await?)
        .into_iter()
        .map(Into::into)
        .collect();
    Ok((StatusCode::OK, Json(invites)))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(create_invite, |op| {
                op.id("CreateInvite")
                    .summary("Create an invite")
                    .tag("Invites")
                    .security_requirement("JWT")
                    .response::<201, Json<InviteDto>>()
            }),
        )
        .api_route(
            "/{invite_id}",
            get_with(get_invite, |op| {
                op.id("GetInvite")
                    .summary("Get a specific invite")
                    .response::<200, Json<InviteDto>>()
            }),
        )
        .api_route(
            "/{invite_id}/stats",
            get_with(get_invite_stats, |op| {
                op.id("GetInviteStats")
                    .summary("Get the daily stats for a specific invite")
                    .tag("Invites")
                    .security_requirement("JWT")
                    .response::<200, Json<Vec<DailyResponseStats>>>()
            }),
        )
        .api_route(
            "/{invite_id}/accept",
            post_with(accept_invite, |op| {
                op.id("AcceptInvite")
                    .summary("Accept the invite if you are able")
                    .tag("Invites")
                    .security_requirement("JWT")
                    .response::<200, Json<InviteDto>>()
            }),
        )
        .api_route(
            "/{invite_id}/reject",
            post_with(reject_invite, |op| {
                op.id("RejectInvite")
                    .summary("Reject the invite if you are able")
                    .tag("Invites")
                    .security_requirement("JWT")
                    .response::<200, Json<InviteDto>>()
            }),
        )
        .api_route(
            "/{invite_id}",
            patch_with(update_invite, |op| {
                op.id("UpdateInvite")
                    .summary("Update an invite")
                    .tag("Invites")
                    .security_requirement("JWT")
                    .response::<200, Json<InviteDto>>()
            })
            .delete_with(delete_invite, |op| {
                op.id("DeleteInvite")
                    .summary("Destroy and invite")
                    .tag("Invites")
                    .security_requirement("JWT")
                    .response::<201, Json<InviteDto>>()
            }),
        )
        .api_route(
            "/",
            get_with(list_invites_for_conversation, |op| {
                op.id("ListInvitesForConversation")
                    .summary("Return a list of invites statements for a conversation")
                    .tag("Invites")
                    .security_requirement("JWT")
                    .response::<200, Json<Vec<InviteDto>>>()
            }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use axum::body::Body;
    use mockall::predicate::{always, eq};
    use serde_json::json;
    use sqlx::PgPool;
    use tracing_test::traced_test;

    use crate::{
        mailer::MockComhairleMailer,
        setup_server,
        test_helpers::{extract, test_state, UserSession},
    };

    use super::*;

    #[sqlx::test]
    fn admin_should_be_able_to_create_invitation(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let mut mailer = MockComhairleMailer::new();

        // Setup mailer expectations
        mailer
            .expect_send_email()
            .with(
                eq("stuart.lynn@gmail.com"),
                eq("Invitation to take part in the National Performance Framework consultation"),
                eq("conversation_invite.html"),
                always(),
            )
            .once()
            .returning(|_, _, _, _| Ok(()));

        mailer
            .expect_send_welcome_email()
            .once()
            .returning(|_, _| Ok(()));

        let state = test_state().db(pool).mailer(Arc::new(mailer)).call()?;
        let app = setup_server(Arc::new(state)).await?;
        let mut session = UserSession::new_admin();

        session.signup(&app).await?;

        let (_, conversation, _) = session.create_random_conversation(&app).await?;

        let conversation_id: String = extract("id", &conversation);
        let (status, _invite, _) = session
            .post(
                &app,
                &format!("/conversation/{conversation_id}/invite"),
                json!({"invite_type":{
                    "email":"stuart.lynn@gmail.com"
                }})
                .to_string()
                .into(),
            )
            .await?;

        assert_eq!(status, StatusCode::CREATED, "should be created ok");

        Ok(())
    }

    #[sqlx::test]
    fn non_admin_should_not_be_able_to_create_invitation(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;
        let mut session = UserSession::new_admin();

        session.signup(&app).await?;

        let (_, conversation, _) = session.create_random_conversation(&app).await?;

        let conversation_id: String = extract("id", &conversation);
        let mut regular_user_session = UserSession::new("bob", "bob", "bob@gmail.com");

        regular_user_session.signup(&app).await?;

        let (status, _invite, _) = regular_user_session
            .post(
                &app,
                &format!("/conversation/{conversation_id}/invite"),
                json!({"invite_type":{
                    "email":"stuart.lynn@gmail.com"
                }})
                .to_string()
                .into(),
            )
            .await?;

        assert_eq!(status, StatusCode::UNAUTHORIZED, "should be blocked");

        Ok(())
    }

    #[sqlx::test]
    #[traced_test]
    fn only_correct_user_should_be_able_to_accept_email_invitation(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;
        let mut session = UserSession::new_admin();

        session.signup(&app).await?;

        let (_, conversation, _) = session.create_random_conversation(&app).await?;
        let convo_id: String = extract("id", &conversation);
        session.create_random_workflow(&app, &convo_id).await?;

        let conversation_id: String = extract("id", &conversation);

        let mut regular_user_session = UserSession::new("bob", crate::test_helpers::TEST_PASSWORD, "bob@some_email.com");
        regular_user_session.signup(&app).await?;

        let mut wrong_regular_user_session =
            UserSession::new("notbob", crate::test_helpers::TEST_PASSWORD, "not_bob@some_email.com");
        wrong_regular_user_session.signup(&app).await?;

        let (_, invite, _) = session
            .post(
                &app,
                &format!("/conversation/{conversation_id}/invite"),
                json!({"invite_type":{
                    "email":"bob@some_email.com"
                }})
                .to_string()
                .into(),
            )
            .await?;

        let invite_id: String = extract("id", &invite);

        let (status, _, _) = wrong_regular_user_session
            .post(
                &app,
                &format!("/conversation/{conversation_id}/invite/{invite_id}/accept"),
                Body::empty(),
            )
            .await?;

        assert_eq!(
            status,
            StatusCode::UNAUTHORIZED,
            "Should not be able to accept invite"
        );

        let (status, _accept_response, _) = regular_user_session
            .post(
                &app,
                &format!("/conversation/{conversation_id}/invite/{invite_id}/accept"),
                Body::empty(),
            )
            .await?;

        assert_eq!(status, StatusCode::OK, "Should be ok");

        Ok(())
    }
}
