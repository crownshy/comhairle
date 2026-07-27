use std::sync::Arc;

use aide::axum::{
    ApiRouter,
    routing::{get_with, patch_with, post_with},
};
use axum::{
    Extension, Json,
    extract::{Path, State},
};
use axum_extra::extract::CookieJar;
use hyper::StatusCode;
use tracing::{instrument, warn};
use uuid::Uuid;

use crate::{
    ComhairleState,
    error::ComhairleError,
    middleware::request_logging::ClientIp,
    models::{
        self, breakout_plan, conversation, event,
        event_attendance::{self, CreateEventAttendance},
        invites::{CreateInviteDTO, DailyResponseStats, InviteType, PartialInvite},
        users, workflow,
    },
    routes::{
        auth::{OtpSignupRequest, create_session_cookie},
        invites::dto::InviteDto,
    },
};

use super::auth::{OptionalUser, RequiredAdminUser, RequiredUser};

pub mod dto;

#[instrument(err(Debug), skip(state))]
async fn accept_invite(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path((conversation_id, invite_id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<InviteDto>), ComhairleError> {
    let invite = models::invites::get_by_id(&state.db, &invite_id).await?;
    let conversation = models::conversation::get_by_id(&state.db, &conversation_id).await?;

    // Check to see if the invite is valid
    invite.is_still_valid()?;
    invite.is_for_user(&user)?;

    if invite.event_id.is_none() {
        // Get the workflow to sign up to either explicitly from the invite
        // or from the default conversation workflow
        let workflow_id = match (invite.workflow_id, conversation.default_workflow_id) {
            (Some(invite_workflow), _) => Ok(invite_workflow),
            (None, Some(conversation_workflow)) => Ok(conversation_workflow),
            (None, None) => Err(ComhairleError::NoWorkflowFoundForInvite),
        }?;

        workflow::register_user(&state.db, &workflow_id, &user).await?;
    }

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
    let invite = models::invites::get_by_id(&state.db, &invite_id).await?;

    // Check to see if the invite is valid
    invite.is_still_valid()?;
    invite.is_for_user(&user)?;

    invite
        .reject(&state.db, &user)
        .await
        .map(|new_invite| (StatusCode::OK, Json(new_invite.into())))
}

#[instrument(err(Debug), skip(state))]
async fn create_conversation_invite(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    RequiredAdminUser(admin_user): RequiredAdminUser,
    Json(create_invite): Json<CreateInviteDTO>,
) -> Result<(StatusCode, Json<InviteDto>), ComhairleError> {
    let conversation = models::conversation::get_by_id(&state.db, &conversation_id).await?;

    if conversation.owner_id != admin_user.id {
        return Err(ComhairleError::UserIsNotConversationOwner);
    }

    // TODO We might need something in here to check that the user in the user type
    // exists

    // Create the invite
    let invite = models::invites::create(
        &state.db,
        create_invite,
        &conversation_id,
        Some(admin_user.id),
    )
    .await?;

    // Send out an email notification if we can
    match &invite.invite_type {
        InviteType::Email(email) => {
            state
                .mailer
                .send_conversation_invite_email(
                    &state,
                    email,
                    conversation.id,
                    admin_user.id,
                    invite.id,
                    &conversation.primary_locale,
                )
                .await?;
        }
        InviteType::User(recipient_id) => {
            // TODO: variant doesn't appear to be in use. If required in future
            // consider how to adjust template variables and default slots for
            // injecting user data, or split into separate email templates with
            // separate schemas
            let recipient = models::users::get_user_by_id(recipient_id, &state.db).await?;
            if let Some(email) = &recipient.email {
                state
                    .mailer
                    .send_conversation_invite_email(
                        &state,
                        email,
                        conversation.id,
                        admin_user.id,
                        invite.id,
                        &conversation.primary_locale,
                    )
                    .await?;
            }
        }
        models::invites::InviteType::Open | models::invites::InviteType::SingleUse => {}
    };

    let invite = invite.into();
    Ok((StatusCode::CREATED, Json(invite)))
}

#[instrument(err(Debug), skip(state))]
async fn create_event_invite(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    RequiredAdminUser(user): RequiredAdminUser,
    Json(create_invite): Json<CreateInviteDTO>,
) -> Result<(StatusCode, Json<InviteDto>), ComhairleError> {
    if create_invite.event_id.is_none() {
        return Err(ComhairleError::BadRequest("Missing event_id".to_string()));
    }

    let conversation = models::conversation::get_by_id(&state.db, &conversation_id).await?;

    // Create the invite
    let invite =
        models::invites::create(&state.db, create_invite, &conversation_id, Some(user.id)).await?;

    let InviteType::Email(email) = &invite.invite_type else {
        return Err(ComhairleError::InvalidInviteType);
    };

    let event_id = &invite.event_id.ok_or(ComhairleError::InvalidInviteType)?;

    state
        .mailer
        .send_event_registration_email(
            &state,
            email,
            *event_id,
            user.id,
            invite.id,
            &conversation.primary_locale,
        )
        .await?;

    Ok((StatusCode::CREATED, Json(invite.into())))
}

#[instrument(err(Debug), skip(state))]
async fn get_invite(
    State(state): State<Arc<ComhairleState>>,
    Path((_, invite_id)): Path<(Uuid, Uuid)>,
    OptionalUser(user): OptionalUser,
) -> Result<(StatusCode, Json<InviteDto>), ComhairleError> {
    let invite = models::invites::get_by_id(&state.db, &invite_id).await?;
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
    models::invites::get_by_id(&state.db, &invite_id).await?;

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
    let invites = models::invites::list_for_conversation(&state.db, &conversation_id)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok((StatusCode::OK, Json(invites)))
}

#[instrument(err(Debug), skip(state))]
async fn list_invites_for_event(
    State(state): State<Arc<ComhairleState>>,
    Path((_conversation_id, event_id)): Path<(Uuid, Uuid)>,
    RequiredAdminUser(_): RequiredAdminUser,
) -> Result<(StatusCode, Json<Vec<InviteDto>>), ComhairleError> {
    let invites = models::invites::list_for_event(&state.db, &event_id)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok((StatusCode::OK, Json(invites)))
}

#[instrument(err(Debug), skip(state, client_ip))]
async fn auto_register_event_attendance(
    State(state): State<Arc<ComhairleState>>,
    Extension(client_ip): Extension<ClientIp>,
    Path((conversation_id, invite_id)): Path<(Uuid, Uuid)>,
    jar: CookieJar,
) -> Result<(CookieJar, (StatusCode, Json<InviteDto>)), ComhairleError> {
    let invite = models::invites::get_by_id(&state.db, &invite_id).await?;
    invite.is_still_valid()?;

    let conversation = conversation::get_by_id(&state.db, &conversation_id).await?;
    let event_id = invite
        .event_id
        .ok_or_else(|| ComhairleError::InvalidInviteResource("Missing event_id".to_string()))?;
    let event =
        event::get_localized_by_id(&state.db, &event_id, &conversation.primary_locale).await?;

    let email = match &invite.invite_type {
        InviteType::Email(email) => email,
        _ => return Err(ComhairleError::InvalidInviteType),
    };

    let user = match users::get_user_by_email(email, &state.db).await.ok() {
        Some(existing) => existing,
        None => {
            let new_user = users::create_otp_user(
                &OtpSignupRequest {
                    email: email.to_string(),
                    username: None,
                },
                &state.db,
            )
            .await?;

            // Best-effort: record the signup IP for the freshly created account.
            if let Err(error) = users::set_signup_ip(&new_user.id, &client_ip.0, &state.db).await {
                warn!(
                    "Failed to record signup IP for user {}: {error}",
                    new_user.id
                );
            }

            new_user
        }
    };

    // Register for event — existing users may already be registered, so treat
    // as a soft failure and log error rather than a hard failure
    if let Err(error) = event_attendance::create(
        &state.db,
        &CreateEventAttendance {
            user_id: user.id,
            event_id,
            role: "participant".to_string(),
        },
    )
    .await
    {
        warn!("Error registering user for event: {error}");
    }

    // Resolve this user's reserved seat (or slot them in) in the breakout plan.
    // Best-effort: never block sign-up on this.
    if let Err(error) = breakout_plan::ensure_user_slotted(
        &state.db,
        &event_id,
        &user.id,
        false,
        Some(invite_id),
        Some(email.as_str()),
    )
    .await
    {
        warn!("Failed to slot user into breakout plan: {error}");
    }

    let invite = invite.accept(&state.db, &user).await?;

    let cookie = create_session_cookie(&user, &state);

    event
        .schedule_event_reminders(
            &state.db,
            &user,
            conversation.owner_id,
            &conversation.primary_locale,
        )
        .await?;

    let event_owner = users::get_user_by_id(&conversation.owner_id, &state.db).await?;

    state
        .mailer
        .send_event_confirmation_email(
            &state,
            email,
            event_id,
            event_owner.id,
            &conversation.primary_locale,
        )
        .await?;

    Ok((jar.add(cookie), (StatusCode::OK, Json(invite.into()))))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(create_conversation_invite, |op| {
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
        .api_route(
            "/events",
            post_with(create_event_invite, |op| {
                op.id("CreateEventInvite")
                    .summary("Create an event invite")
                    .description("Create an invite for a given event")
                    .tag("Invites")
                    .security_requirement("JWT")
                    .response::<201, Json<InviteDto>>()
            }),
        )
        .api_route(
            "/events/{event_id}",
            get_with(list_invites_for_event, |op| {
                op.id("ListInvitesForEvent")
                    .summary("Return a list of invite for an event")
                    .tag("Invites")
                    .security_requirement("JWT")
                    .response::<200, Json<Vec<InviteDto>>>()
            }),
        )
        .api_route(
            "/{invite_id}/events",
            post_with(auto_register_event_attendance, |op| {
                op.id("AutoRegisterEventAttendance")
                    .summary("Auto register event attendance")
                    .tag("Invites")
                    .response::<200, Json<InviteDto>>()
            }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use axum::body::Body;
    use serde_json::json;
    use sqlx::PgPool;
    use tracing_test::traced_test;

    use crate::{
        mailer::MockComhairleMailer,
        models::{
            invites::{InviteStatus, LoginBehaviour},
            model_test_helpers::get_random_conversation_id,
            users::UserAuthType,
        },
        routes::{events::dto::EventDto, user::dto::UserDto},
        setup_server,
        test_helpers::{UserSession, extract, test_state},
    };

    use super::*;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn admin_should_be_able_to_create_invitation(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let mut mailer = MockComhairleMailer::new();

        // Setup mailer expectations
        mailer
            .expect_send_conversation_invite_email()
            .once()
            .returning(|_, _, _, _, _, _| Box::pin(async move { Ok(()) }));

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

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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

        let mut regular_user_session = UserSession::new(
            "bob",
            crate::test_helpers::TEST_PASSWORD,
            "bob@some_email.com",
        );
        regular_user_session.signup(&app).await?;

        let mut wrong_regular_user_session = UserSession::new(
            "notbob",
            crate::test_helpers::TEST_PASSWORD,
            "not_bob@some_email.com",
        );
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

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn should_create_attendance_for_existing_user_and_sign_in(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let email = "test_email@test.com";
        let username = "test_name";
        let password = "test_PW_123_(*&)";

        let state = test_state().db(pool.clone()).call()?;
        let app = setup_server(Arc::new(state)).await?;
        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let (_, value, _) = session
            .create_random_event(&app, &conversation_id.to_string())
            .await?;
        let event: EventDto = serde_json::from_value(value)?;

        let create_invite = CreateInviteDTO {
            invite_type: InviteType::Email(email.to_string()),
            login_behaviour: LoginBehaviour::Manual,
            expires_at: None,
            label: None,
            event_id: Some(event.id),
        };
        let bytes = serde_json::to_vec(&create_invite)?;

        let (_, value, _) = session
            .post(
                &app,
                &format!("/conversation/{}/invite", conversation_id),
                bytes.into(),
            )
            .await?;
        let invite: InviteDto = serde_json::from_value(value)?;

        let (_, value, _) = session
            .post(
                &app,
                "/auth/signup",
                json!({ "email": email, "password": password, "username": username })
                    .to_string()
                    .into(),
            )
            .await?;
        let user: UserDto = serde_json::from_value(value)?;

        let (_, value, cookies) = session
            .post(
                &app,
                &format!(
                    "/conversation/{}/invite/{}/events",
                    conversation_id, invite.id
                ),
                Body::empty(),
            )
            .await?;
        let invite: InviteDto = serde_json::from_value(value)?;

        let attendance =
            event_attendance::get_by_event_and_user(&pool, &event.id, &user.id).await?;

        assert_eq!(
            invite.status,
            InviteStatus::Accepted,
            "incorrect invite status"
        );
        assert_eq!(
            attendance.user_id, user.id,
            "incorrect user_id on event attendance"
        );
        assert!(
            cookies.unwrap().to_str()?.contains("auth-token"),
            "missing auth-token cookie"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn should_continue_with_invite_update_if_user_already_registered(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let email = "test_email@test.com";
        let username = "test_name";
        let password = "test_PW_123_(*&)";

        let state = test_state().db(pool.clone()).call()?;
        let app = setup_server(Arc::new(state)).await?;
        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let (_, value, _) = session
            .create_random_event(&app, &conversation_id.to_string())
            .await?;
        let event: EventDto = serde_json::from_value(value)?;

        let create_invite = CreateInviteDTO {
            invite_type: InviteType::Email(email.to_string()),
            login_behaviour: LoginBehaviour::Manual,
            expires_at: None,
            label: None,
            event_id: Some(event.id),
        };
        let bytes = serde_json::to_vec(&create_invite)?;

        let (_, value, _) = session
            .post(
                &app,
                &format!("/conversation/{}/invite", conversation_id),
                bytes.into(),
            )
            .await?;
        let invite: InviteDto = serde_json::from_value(value)?;

        let (_, value, _) = session
            .post(
                &app,
                "/auth/signup",
                json!({ "email": email, "password": password, "username": username })
                    .to_string()
                    .into(),
            )
            .await?;
        let user: UserDto = serde_json::from_value(value)?;

        let _ = session
            .post(
                &app,
                &format!(
                    "/conversation/{}/events/{}/attendances",
                    conversation_id, event.id
                ),
                json!({ "role": "participant" }).to_string().into(),
            )
            .await?;

        let (_, value, cookies) = session
            .post(
                &app,
                &format!(
                    "/conversation/{}/invite/{}/events",
                    conversation_id, invite.id
                ),
                Body::empty(),
            )
            .await?;
        let invite: InviteDto = serde_json::from_value(value)?;

        let attendance =
            event_attendance::get_by_event_and_user(&pool, &event.id, &user.id).await?;

        assert_eq!(
            invite.status,
            InviteStatus::Accepted,
            "incorrect invite status"
        );
        assert_eq!(
            attendance.user_id, user.id,
            "incorrect user_id on event attendance"
        );
        assert!(
            cookies.unwrap().to_str()?.contains("auth-token"),
            "missing auth-token cookie"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn should_create_new_otp_user_with_attendance_and_signin(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let email = "test_email@test.com";

        let state = test_state().db(pool.clone()).call()?;
        let app = setup_server(Arc::new(state)).await?;
        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let (_, value, _) = session
            .create_random_event(&app, &conversation_id.to_string())
            .await?;
        let event: EventDto = serde_json::from_value(value)?;

        let create_invite = CreateInviteDTO {
            invite_type: InviteType::Email(email.to_string()),
            login_behaviour: LoginBehaviour::Manual,
            expires_at: None,
            label: None,
            event_id: Some(event.id),
        };
        let bytes = serde_json::to_vec(&create_invite)?;

        let (_, value, _) = session
            .post(
                &app,
                &format!("/conversation/{}/invite", conversation_id),
                bytes.into(),
            )
            .await?;
        let invite: InviteDto = serde_json::from_value(value)?;

        let (_, value, cookies) = session
            .post(
                &app,
                &format!(
                    "/conversation/{}/invite/{}/events",
                    conversation_id, invite.id
                ),
                Body::empty(),
            )
            .await?;
        let invite: InviteDto = serde_json::from_value(value)?;

        session.logout(&app).await?;

        let user = users::get_user_by_email(email, &pool).await?;
        let attendance =
            event_attendance::get_by_event_and_user(&pool, &event.id, &user.id).await?;

        assert_eq!(
            invite.status,
            InviteStatus::Accepted,
            "incorrect invite status"
        );
        assert_eq!(
            user.auth_type,
            UserAuthType::Otp,
            "incorrect user auth_type"
        );
        assert_eq!(
            attendance.user_id, user.id,
            "incorrect user_id on event attendance"
        );
        assert!(
            cookies.unwrap().to_str()?.contains("auth-token"),
            "missing auth-token cookie"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn should_fail_if_invite_type_incorrect(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool.clone()).call()?;
        let app = setup_server(Arc::new(state)).await?;
        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let (_, value, _) = session
            .create_random_event(&app, &conversation_id.to_string())
            .await?;
        let event: EventDto = serde_json::from_value(value)?;

        let create_invite = CreateInviteDTO {
            invite_type: InviteType::Open,
            login_behaviour: LoginBehaviour::Manual,
            expires_at: None,
            label: None,
            event_id: Some(event.id),
        };
        let bytes = serde_json::to_vec(&create_invite)?;

        let (_, value, _) = session
            .post(
                &app,
                &format!("/conversation/{}/invite", conversation_id),
                bytes.into(),
            )
            .await?;
        let invite: InviteDto = serde_json::from_value(value)?;

        let (_, value, _) = session
            .post(
                &app,
                &format!(
                    "/conversation/{}/invite/{}/events",
                    conversation_id, invite.id
                ),
                Body::empty(),
            )
            .await?;

        assert_eq!(
            value.get("err").unwrap(),
            "This invite is has an invalid type",
            "incorrect error message"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn should_fail_if_invite_missing_event_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool.clone()).call()?;
        let app = setup_server(Arc::new(state)).await?;
        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let create_invite = CreateInviteDTO {
            invite_type: InviteType::Email("test@test.com".to_string()),
            login_behaviour: LoginBehaviour::Manual,
            expires_at: None,
            label: None,
            event_id: None,
        };
        let bytes = serde_json::to_vec(&create_invite)?;

        let (_, value, _) = session
            .post(
                &app,
                &format!("/conversation/{}/invite", conversation_id),
                bytes.into(),
            )
            .await?;
        let invite: InviteDto = serde_json::from_value(value)?;

        let (_, value, _) = session
            .post(
                &app,
                &format!(
                    "/conversation/{}/invite/{}/events",
                    conversation_id, invite.id
                ),
                Body::empty(),
            )
            .await?;

        assert_eq!(
            value.get("err").unwrap(),
            "This invite has an invalid resource: Missing event_id",
            "incorrect error message"
        );

        Ok(())
    }
}
