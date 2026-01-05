use std::sync::Arc;

use aide::axum::{
    routing::{get_with, post_with, put_with},
    ApiRouter,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::{
        self,
        bot_service_user_session::{self, BotServiceUserSessionDto, CreateBotServiceUserSession},
        conversation::{
            Conversation, ConversationFilterOptions, ConversationOrderOptions,
            LocalisedConversation,
        },
        pagination::{OrderParams, PageOptions, PaginatedResults},
        users::{UpdateUserRequest, UpgradeAccountRequest, User},
    },
    ComhairleState,
};

use super::auth::{is_user_admin, RequiredAdminUser, RequiredUser};

pub async fn get_user_owned_conversations(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    OrderParams(order_options): OrderParams<ConversationOrderOptions>,
    Query(filter_options): Query<ConversationFilterOptions>,
    Query(page_options): Query<PageOptions>,
) -> Result<(StatusCode, Json<PaginatedResults<LocalisedConversation>>), ComhairleError> {
    let conversations = models::conversation::list_owned(
        &state.db,
        user.id,
        page_options,
        order_options,
        filter_options,
        Some("en".to_string()),
    )
    .await?;
    Ok((StatusCode::OK, Json(conversations)))
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub enum ResourceRole {
    Admin,
    SuperAdmin,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub enum ResourceType {
    Site,
    Conversation(Uuid),
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct UserRoles {
    pub resource: ResourceType,
    pub roles: Vec<ResourceRole>,
}

pub async fn get_conversations_user_participating_in(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
) -> Result<(StatusCode, Json<Vec<Conversation>>), ComhairleError> {
    let conversations =
        models::conversation::list_for_user_participation(&state.db, &user.id).await?;
    Ok((StatusCode::OK, Json(conversations)))
}

pub async fn get_user_roles(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
) -> Result<(StatusCode, Json<Vec<UserRoles>>), ComhairleError> {
    let mut roles = vec![];

    if is_user_admin(&user, &state.config) {
        roles.push(UserRoles {
            resource: ResourceType::Site,
            roles: vec![ResourceRole::Admin],
        });
    }

    Ok((StatusCode::OK, Json(roles)))
}

pub async fn update_user_details(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Json(update_request): Json<UpdateUserRequest>,
) -> Result<(StatusCode, Json<User>), ComhairleError> {
    let updated_user = models::users::update_user(&user.id, &update_request, &state.db).await?;
    Ok((StatusCode::OK, Json(updated_user)))
}

pub async fn upgrade_account(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Json(upgrade_request): Json<UpgradeAccountRequest>,
) -> Result<(StatusCode, Json<User>), ComhairleError> {
    let upgraded_user =
        models::users::upgrade_account(&user.id, &upgrade_request, &state.db).await?;
    Ok((StatusCode::OK, Json(upgraded_user)))
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
struct CreateBotUserSessionRequest {
    conversation_id: Uuid,
}

#[instrument(err(Debug), skip(state))]
async fn create_bot_service_user_session(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Json(payload): Json<CreateBotUserSessionRequest>,
) -> Result<(StatusCode, Json<BotServiceUserSessionDto>), ComhairleError> {
    let create_bot_session = CreateBotServiceUserSession {
        conversation_id: payload.conversation_id,
        user_id: user.id,
    };
    let bot_user_session =
        bot_service_user_session::create(&state.db, &state.bot_service, &create_bot_session)
            .await?;

    let bot_user_session: BotServiceUserSessionDto = bot_user_session.into();
    Ok((StatusCode::CREATED, Json(bot_user_session)))
}

#[instrument(err(Debug), skip(state))]
async fn get_bot_service_user_session(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path(conversation_id): Path<Uuid>,
) -> Result<(StatusCode, Json<BotServiceUserSessionDto>), ComhairleError> {
    let session =
        bot_service_user_session::get_by_conversation_id(&state.db, user.id, conversation_id).await;

    // If we didn't find a session create one
    let session = match session {
        Ok(session) => Ok(session),
        Err(ComhairleError::NoBotUserSession) => {
            bot_service_user_session::create(
                &state.db,
                &state.bot_service,
                &CreateBotServiceUserSession {
                    conversation_id,
                    user_id: user.id,
                },
            )
            .await
        }
        Err(e) => Err(e),
    }?;
    let session: BotServiceUserSessionDto = session.into();

    Ok((StatusCode::OK, Json(session)))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/roles",
            get_with(get_user_roles, |op| {
                op.id("GetUserRoles")
                    .tag("User")
                    .description("Gets a list of roles the current user has")
                    .security_requirement("JWT")
                    .response::<201, Json<Vec<UserRoles>>>()
            }),
        )
        .api_route(
            "/conversations",
            get_with(get_conversations_user_participating_in, |op| {
                op.id("GetConversationsUserIsParticipatingIn")
                    .tag("User")
                    .description(
                        "Returns a list of all the conversations the user has taken part in",
                    )
                    .security_requirement("JWT")
                    .response::<201, Json<Vec<Conversation>>>()
            }),
        )
        .api_route(
            "/owned_conversations",
            get_with(get_user_owned_conversations, |op| {
                op.id("GetOwnedConversations")
                    .tag("User")
                    .description("Gets a list of the conversations a user owns")
                    .security_requirement("JWT")
                    .response::<201, Json<PaginatedResults<LocalisedConversation>>>()
            }),
        )
        .api_route(
            "/details",
            put_with(update_user_details, |op| {
                op.id("UpdateUserDetails")
                    .tag("User")
                    .description("Update user details (username and/or password)")
                    .security_requirement("JWT")
                    .response::<200, Json<User>>()
            }),
        )
        .api_route(
            "/upgrade",
            put_with(upgrade_account, |op| {
                op.id("UpgradeAccount")
                    .tag("User")
                    .description("Upgrade anonymous account to email/password account")
                    .security_requirement("JWT")
                    .response::<200, Json<User>>()
            }),
        )
        .api_route(
            "/bot_service_sessions",
            post_with(create_bot_service_user_session, |op| {
                op.id("CreateBotServiceUserSession")
                    .tag("User")
                    .summary("Create a chat bot session by conversation id for user")
                    .security_requirement("JWT")
                    .response::<201, Json<BotServiceUserSessionDto>>()
            }),
        )
        .api_route(
            "/bot_service_sessions/{conversation_id}",
            get_with(get_bot_service_user_session, |op| {
                op.id("GetServiceUserSession")
                    .tag("User")
                    .summary("Get a bot service session for a user by conversation if")
                    .security_requirement("JWT")
                    .response::<200, Json<BotServiceUserSessionDto>>()
            }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        setup_server,
        test_helpers::{test_state, UserSession},
    };
    use std::{error::Error, sync::Arc};

    use axum::body::Body;
    use serde_json::Value;
    use sqlx::PgPool;
    use uuid::Uuid;

    #[sqlx::test]
    async fn should_create_bot_session_for_user(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        let conversation_json: Value =
            serde_json::from_str(include_str!("../../../fixtures/conversation.json"))?;
        let (_, conversation, _) = admin_session
            .create_conversation(&app, conversation_json)
            .await?;
        let conversation_id = conversation.get("id").and_then(|v| v.as_str()).unwrap();
        let conversation_id = Uuid::parse_str(conversation_id)?;

        let username = "test";
        let password = "test_password";
        let email = "test_email";

        let mut user_session = UserSession::new(username, password, email);
        user_session.signup(&app).await?;

        let create_request = CreateBotUserSessionRequest { conversation_id };
        let bytes = serde_json::to_vec(&create_request)?;
        let body = Body::from(bytes);
        let (status, value, _) = user_session
            .post(&app, "/user/bot_service_sessions", body)
            .await?;

        assert!(status.is_success(), "error response status");
        assert_eq!(
            conversation_id,
            Uuid::parse_str(
                value
                    .get("conversation_id")
                    .and_then(|v| v.as_str())
                    .unwrap(),
            )
            .unwrap(),
            "response contains incorrect conversation id"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_get_bot_session_for_current_user_by_conversation_id(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        let conversation_json: Value =
            serde_json::from_str(include_str!("../../../fixtures/conversation.json"))?;
        let (_, conversation, _) = admin_session
            .create_conversation(&app, conversation_json)
            .await?;
        let conversation_id = conversation.get("id").and_then(|v| v.as_str()).unwrap();

        let username = "test";
        let password = "test_password";
        let email = "test_email";

        let mut user_session = UserSession::new(username, password, email);
        user_session.signup(&app).await?;

        let (status, value, _) = user_session
            .get(
                &app,
                &format!("/user/bot_service_sessions/{conversation_id}"),
            )
            .await?;

        assert!(status.is_success(), "error response status");
        assert_eq!(
            conversation_id,
            value
                .get("conversation_id")
                .and_then(|v| v.as_str())
                .unwrap(),
            "response contains incorrect conversation id"
        );
        assert_eq!(
            user_session.id.unwrap().to_string(),
            value
                .get("user_id")
                .and_then(|v| v.as_str())
                .unwrap()
                .to_string(),
            "response contains incorrect user id"
        );

        Ok(())
    }
}
