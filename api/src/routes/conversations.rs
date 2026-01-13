use std::sync::Arc;

use apalis::prelude::MessageQueue;
use axum::{
    extract::{Json, Multipart, Path, Query, State},
    http::StatusCode,
    routing::post,
};

use aide::axum::{
    routing::{delete_with, get_with, post_with, put_with},
    ApiRouter,
};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::{
        bot_service_user_session::{self, BotServiceUserSessionDto, CreateBotServiceUserSession},
        conversation::{
            self, Conversation, ConversationFilterOptions, ConversationOrderOptions,
            ConversationWithTranslations, CreateConversation, LocalisedConversation,
            PartialConversation,
        },
        conversation_email_notification_recipients::{
            self as email_recipients_model, CreateConversationEmailNotificationRecipients,
        },
        job::{self, CreateJob},
        notification::{self as notification_model, CreateNotification, NotificationContextType},
        notification_delivery::{
            self as notification_delivery_model, CreateNotificationDelivery, DeliveryMethod,
        },
        pagination::{OrderParams, PageOptions, PaginatedResults},
        user_participation::{self},
    },
    routes::auth::RequiredUser,
    workers::documents::DocumentJob,
    ComhairleState,
};

use super::auth::{is_user_admin, OptionalUser, RequiredAdminUser};

/// Create conversation handler
async fn create_conversation(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    Json(new_conversations): Json<CreateConversation>,
) -> Result<(StatusCode, Json<Conversation>), ComhairleError> {
    info!("Attempting to create conversation");
    let conversation = conversation::create(
        &state.db,
        &state.bot_service,
        &state.config,
        &new_conversations,
        user.id,
    )
    .await?;
    Ok((StatusCode::CREATED, Json(conversation)))
}

/// Update conversation handler
async fn update_conversation(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    Path(id): Path<Uuid>,
    Json(conversation): Json<PartialConversation>,
) -> Result<(StatusCode, Json<Conversation>), ComhairleError> {
    let conversation = conversation::update(&state.db, &id, &conversation).await?;
    Ok((StatusCode::OK, Json(conversation)))
}

/// List conversations handler
async fn list_conversations(
    State(state): State<Arc<ComhairleState>>,
    OrderParams(order_options): OrderParams<ConversationOrderOptions>,
    Query(filter_options): Query<ConversationFilterOptions>,
    Query(page_options): Query<PageOptions>,
) -> Result<(StatusCode, Json<PaginatedResults<LocalisedConversation>>), ComhairleError> {
    let conversations = conversation::list(
        &state.db,
        page_options,
        order_options,
        filter_options,
        Some("en".to_string()),
    )
    .await?;
    Ok((StatusCode::OK, Json(conversations)))
}

/// For extracting an id or slug from Path
#[derive(Deserialize, Debug, JsonSchema)]
#[serde(untagged)]
enum IdOrSlug {
    Id(Uuid),
    Slug(String),
}

#[derive(Deserialize, JsonSchema)]
pub struct GetConversationQuery {
    #[serde(rename = "withTranslations", default)]
    pub with_translations: bool,
}

#[derive(Serialize, JsonSchema)]
#[serde(untagged)]
pub enum ConversationResponse {
    Localised(LocalisedConversation),
    WithTranslations(ConversationWithTranslations),
}

/// Get a specific conversation
async fn get_conversation(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_ident): Path<IdOrSlug>,
    Query(query): Query<GetConversationQuery>,
    OptionalUser(user): OptionalUser,
) -> Result<(StatusCode, Json<ConversationResponse>), ComhairleError> {
    info!("Attempting to get conversation {conversation_ident:#?}");

    // Check if user is admin and withTranslations is requested
    let should_return_with_translations = query.with_translations
        && user
            .as_ref()
            .map(|u| is_user_admin(u, &state.config))
            .unwrap_or(false);

    if should_return_with_translations {
        // Get the original conversation first
        let original_conversation = match conversation_ident {
            IdOrSlug::Id(id) => conversation::get_by_id(&state.db, &id).await?,
            IdOrSlug::Slug(slug) => conversation::get_by_slug(&state.db, &slug).await?,
        };

        // Convert to ConversationWithTranslations
        let conversation_with_translations = ConversationWithTranslations::from_original(
            &state.db,
            original_conversation,
            "en", // TODO: Get locale from user preferences or query param
        )
        .await?;

        Ok((
            StatusCode::OK,
            Json(ConversationResponse::WithTranslations(
                conversation_with_translations,
            )),
        ))
    } else {
        // Return localized conversation as before
        let conversation = match conversation_ident {
            IdOrSlug::Id(id) => conversation::get_localised_by_id(&state.db, &id).await?,
            IdOrSlug::Slug(slug) => conversation::get_localised_by_slug(&state.db, &slug).await?,
        };

        Ok((
            StatusCode::OK,
            Json(ConversationResponse::Localised(conversation)),
        ))
    }
}

/// Delete a specific conversation
async fn delete_conversation(
    State(state): State<Arc<ComhairleState>>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Conversation>), ComhairleError> {
    let conversation = conversation::delete(&state.db, &id).await?;
    Ok((StatusCode::OK, Json(conversation)))
}

#[derive(Deserialize, JsonSchema)]
pub struct SendNotificationRequest {
    pub title: String,
    pub content: String,
    pub notification_type: Option<crate::models::notification::NotificationType>,
    pub delivery_method: Option<DeliveryMethod>,
}

#[derive(Deserialize, JsonSchema)]
pub struct RegisterEmailRequest {
    pub email: String,
    pub receive_updates_by_email: bool,
    pub receive_similar_conversation_updates_by_email: bool,
}

#[derive(Serialize, JsonSchema)]
pub struct RegisterEmailResponse {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub email: String,
    pub message: String,
}

/// Send notification to all conversation participants
async fn send_notification_to_participants(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    Path(conversation_id): Path<Uuid>,
    Json(request): Json<SendNotificationRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), ComhairleError> {
    // Verify conversation exists and user has permission
    let conversation = conversation::get_by_id(&state.db, &conversation_id).await?;

    if conversation.owner_id != user.id {
        return Err(ComhairleError::UserNotAuthorized);
    }

    // Create the notification
    let create_notification = CreateNotification {
        title: request.title,
        content: request.content,
        notification_type: request.notification_type,
        context_type: Some(NotificationContextType::Conversation),
        context_id: Some(conversation_id),
    };

    let notification = notification_model::create(&state.db, &create_notification).await?;

    // Get all participant user IDs for this conversation
    let participant_user_ids =
        user_participation::get_participant_user_ids_for_conversation(&state.db, &conversation_id)
            .await?;

    if participant_user_ids.is_empty() {
        return Ok((
            StatusCode::OK,
            Json(serde_json::json!({
                "notification_id": notification.id,
                "participants_notified": 0,
                "message": "No participants found for this conversation"
            })),
        ));
    }

    // Create deliveries for all participants
    let delivery_method = request.delivery_method.unwrap_or(DeliveryMethod::InApp);
    let deliveries: Vec<CreateNotificationDelivery> = participant_user_ids
        .into_iter()
        .map(|user_id| CreateNotificationDelivery {
            notification_id: notification.id,
            user_id,
            delivery_method: Some(delivery_method.clone()),
        })
        .collect();

    let created_deliveries =
        notification_delivery_model::create_bulk(&state.db, &deliveries).await?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "notification_id": notification.id,
            "participants_notified": created_deliveries.len(),
            "message": format!("Notification sent to {} participants", created_deliveries.len())
        })),
    ))
}

/// Register email for conversation updates
async fn register_email_for_updates(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    Json(request): Json<RegisterEmailRequest>,
) -> Result<(StatusCode, Json<RegisterEmailResponse>), ComhairleError> {
    // Verify conversation exists and is public
    let conversation = conversation::get_by_id(&state.db, &conversation_id).await?;

    // Check if email is already registered for this conversation
    if let Ok(_existing) = email_recipients_model::get_by_conversation_and_email(
        &state.db,
        &conversation_id,
        &request.email,
    )
    .await
    {
        return Ok((
            StatusCode::OK,
            Json(RegisterEmailResponse {
                id: _existing.id,
                conversation_id,
                email: request.email.clone(),
                message: "Email is already registered for updates on this conversation".to_string(),
            }),
        ));
    }

    // Create new email registration
    let create_request = CreateConversationEmailNotificationRecipients {
        conversation_id,
        email: request.email.clone(),
        receive_updates_by_email: request.receive_updates_by_email,
        receive_similar_conversation_updates_by_email: request
            .receive_similar_conversation_updates_by_email,
    };

    let recipient = email_recipients_model::create(&state.db, &create_request).await?;

    Ok((
        StatusCode::CREATED,
        Json(RegisterEmailResponse {
            id: recipient.id,
            conversation_id,
            email: request.email,
            message: "Successfully registered for email updates".to_string(),
        }),
    ))
}

#[instrument(err(Debug), skip(state))]
async fn create_conversation_bot_session(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path(conversation_id): Path<Uuid>,
) -> Result<(StatusCode, Json<BotServiceUserSessionDto>), ComhairleError> {
    let create_bot_session = CreateBotServiceUserSession {
        conversation_id,
        user_id: user.id,
    };
    let bot_user_session =
        bot_service_user_session::create(&state.db, &state.bot_service, &create_bot_session)
            .await?;

    let bot_user_session: BotServiceUserSessionDto = bot_user_session.into();
    Ok((StatusCode::CREATED, Json(bot_user_session)))
}

#[instrument(err(Debug), skip(state))]
async fn get_conversation_bot_session(
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

#[derive(Serialize, Deserialize, JsonSchema, Debug, PartialEq, Clone, Default)]
pub struct UploadFileRequest {
    pub filename: String,
    pub bytes: Vec<u8>,
}

#[derive(Serialize, JsonSchema, Debug)]
pub struct UploadFileResponse {
    message: String,
    job_ids: Vec<Uuid>,
}

#[instrument(err(Debug), skip(state))]
async fn upload_conversation_bot_document(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    RequiredAdminUser(_user): RequiredAdminUser,
    mut form_data: Multipart,
) -> Result<(StatusCode, Json<UploadFileResponse>), ComhairleError> {
    let mut files: Vec<UploadFileRequest> = Vec::new();
    let mut job_ids = vec![];

    while let Some(field) = form_data.next_field().await? {
        let filename = field.file_name().unwrap_or("<no filename>").to_string();
        let bytes = field.bytes().await?;

        let file = UploadFileRequest {
            filename,
            bytes: bytes.to_vec(),
        };
        files.push(file);
    }

    for file in files {
        let create_job = CreateJob {
            progress: Some(0.0),
            ..Default::default()
        };
        let job = job::create(&state.db, create_job).await?;
        job_ids.push(job.id);

        let worker_job = DocumentJob {
            job_id: job.id,
            conversation_id,
            document: file,
        };
        let mut lock = state.jobs.documents.lock().await;
        lock.enqueue(worker_job)
            .await
            .map_err(|_| ComhairleError::BackgroundJobFailedToQueue)?;
    }

    let json_response = UploadFileResponse {
        message: "Document uploads and parsing moved to background jobs".to_string(),
        job_ids,
    };

    Ok((StatusCode::OK, Json(json_response)))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(create_conversation, |op| {
                op.id("CreateConversation")
                    .summary("Create a new conversation")
                    .tag("Conversation")
                    .description("Creates a new conversation")
                    .response::<201, Json<LocalisedConversation>>()
            }),
        )
        .api_route(
            "/",
            get_with(list_conversations, |op| {
                op.id("ListConverastions")
                    .summary("List conversations with optional filtering and ordering")
                    .tag("Conversation")
                    .description("List conversations")
                    .response::<200, Json<PaginatedResults<LocalisedConversation>>>()
            }),
        )
        .api_route(
            "/{conversation_id}",
            get_with(get_conversation, |op| {
                op.id("GetConversation")
                    .summary("Get a conversation by id or slug")
                    .tag("Conversation")
                    .description("Get a conversation by id or slug. If user is admin and withTranslations=true, returns detailed translation data.")
                    .response::<200, Json<ConversationResponse>>()
            }),
        )
        .api_route(
            "/{conversation_id}",
            put_with(update_conversation, |op| {
                op.id("UpdateConversation")
                    .summary("Update a conversation")
                    .tag("Conversation")
                    .description("Update a conversation")
                    .response::<200, Json<LocalisedConversation>>()
            }),
        )
        .api_route(
            "/{conversation_id}",
            delete_with(delete_conversation, |op| {
                op.id("DeleteConversation")
                    .summary("Delete the conversation and all related content")
                    .tag("Conversation")
                    .description("Delete the conversation and all related content")
                    .response::<200, Json<LocalisedConversation>>()
            }),
        )
        .api_route(
            "/{conversation_id}/notifications",
            post_with(send_notification_to_participants, |op| {
                op.id("SendNotificationToParticipants")
                    .summary("Send notification to all conversation participants")
                    .description("Creates a notification and sends it to all users participating in workflows within the conversation. Only conversation owners can send notifications.")
                    .response::<201, Json<serde_json::Value>>()
                    .tag("Notifications")
            }),
        )
        .api_route(
            "/{conversation_id}/email-updates",
            post_with(register_email_for_updates, |op| {
                op.id("RegisterEmailForUpdates")
                    .summary("Register email address for conversation updates")
                    .description("Allows non-logged-in users to register their email address to receive updates about a public conversation. If the email is already registered, returns existing registration.")
                    .response::<201, Json<RegisterEmailResponse>>()
                    .response::<200, Json<RegisterEmailResponse>>()
                    .tag("Email Notifications")
            }),
        )
        .api_route(
            "/{conversation_id}/bot_service_sessions",
            post_with(create_conversation_bot_session, |op| {
                op.id("CreateConversationBotSession")
                    .tag("Conversation")
                    .summary("Create a user bot session for a conversation")
                    .security_requirement("JWT")
                    .response::<201, Json<BotServiceUserSessionDto>>()
            }),
        )
        .api_route(
            "/{conversation_id}/bot_service_sessions",
            get_with(get_conversation_bot_session, |op| {
                op.id("GetConversationBotSession")
                    .tag("Conversation")
                    .summary("Get a user bot session for a conversation")
                    .security_requirement("JWT")
                    .response::<200, Json<BotServiceUserSessionDto>>()
            }),
        )
        .route(
            "/{conversation_id}/upload_documents", post(upload_conversation_bot_document)
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {

    use crate::test_helpers::test_state;
    use crate::{setup_server, test_helpers::UserSession};
    use axum::{body::Body, http::StatusCode};
    use serde_json::{json, Value};
    use sqlx::PgPool;
    use std::collections::HashMap;
    use std::error::Error;
    use std::sync::Arc;
    use uuid::Uuid;

    #[sqlx::test]
    fn should_be_able_to_create_conversation(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_admin();

        session.signup(&app).await?;

        let (status, _, _) = session
            .create_conversation(
                &app,
                json! ({
                    "title" : "Test conversation",
                    "short_description" : "A test conversation",
                    "description" : "A longer description",
                    "image_url" : "http://someimage.png",
                    "tags" : ["one", "two", "three"],
                    "is_public" : false,
                    "is_invite_only" : false,
                    "slug" : "new_conversation",
                    "primary_locale" : "en",
                    "supported_languages" : ["en"]
                }),
            )
            .await?;

        assert_eq!(status, StatusCode::CREATED, "Should be created");

        Ok(())
    }

    #[sqlx::test]
    fn should_be_able_to_update_a_conversation(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let (status, conversation, _) = session
            .create_conversation(
                &app,
                json! ({
                    "title" : "Test conversation",
                    "short_description" : "A test conversation",
                    "description" : "A longer description",
                    "image_url" : "http://someimage.png",
                    "tags" : ["one", "two", "three"],
                    "is_public" : false,
                    "is_invite_only" : false,
                    "slug" : "new_conversation",
                    "primary_locale" : "en",
                    "supported_languages" : ["en"]
                }),
            )
            .await?;
        assert_eq!(status, StatusCode::CREATED, "Should be created");

        let id: String = serde_json::from_value(conversation.get("id").unwrap().clone()).unwrap();

        let (status, conversation, _) = session
            .update_conversation(
                &app,
                &id,
                json!({
                    "is_public":true
                }),
            )
            .await?;

        assert_eq!(status, StatusCode::OK, "Should update resource");

        assert_eq!(
            conversation.get("is_public"),
            Some(&json!(true)),
            "should have updated public status"
        );
        Ok(())
    }

    #[sqlx::test]
    fn should_not_be_able_to_udpate_owner_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_admin();

        session.signup(&app).await?;

        let (status, conversation, _) = session
            .create_conversation(
                &app,
                json! ({
                    "title" : "Test conversation",
                    "short_description" : "A test conversation",
                    "description" : "A longer description",
                    "image_url" : "http://someimage.png",
                    "tags" : ["one", "two", "three"],
                    "is_public" : false,
                    "is_invite_only" : false,
                    "slug" : "new_conversation",
                    "primary_locale" : "en",
                    "supported_languages" : ["en"]
                }),
            )
            .await?;

        assert_eq!(status, StatusCode::CREATED, "Should be created");

        let id: String = serde_json::from_value(conversation.get("id").unwrap().clone()).unwrap();

        let (status, _, _) = session
            .update_conversation(
                &app,
                &id,
                json!({
                    "owner_id": session.id.unwrap(),
                }),
            )
            .await?;

        assert_eq!(
            status,
            StatusCode::UNPROCESSABLE_ENTITY,
            "fail to update protected field owner id"
        );

        Ok(())
    }
    #[sqlx::test]
    fn should_be_able_to_list_conversations(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_admin();

        session.signup(&app).await?;

        session
            .create_conversation(
                &app,
                json! ({
                    "title" : "Test conversation",
                    "short_description" : "A test conversation",
                    "description" : "A longer description",
                    "image_url" : "http://someimage.png",
                    "tags" : ["one", "two", "three"],
                    "is_public" : true,
                    "is_invite_only" : false,
                    "slug" : "new_conversation",
                    "primary_locale" : "en",
                    "supported_languages" : ["en"]
                }),
            )
            .await?;

        let (status, result, _) = session
            .create_conversation(
                &app,
                json! ({
                    "title" : "Another Test",
                    "short_description" : "A test conversation",
                    "description" : "A longer description",
                    "image_url" : "http://someimage.png",
                    "tags" : ["one", "two", "three"],
                    "is_public" : true,
                    "primary_locale": "en",
                    "supported_languages":["en"],
                    "is_invite_only" : false,
                    "slug" : "new_new_conversation"
                }),
            )
            .await?;

        let (status, conversations, _) = session.list_conversations(&app, 0, 10).await?;

        assert_eq!(status, StatusCode::OK, "Should be found");

        let total: i32 =
            serde_json::from_value(conversations.get("total").to_owned().unwrap().to_owned())
                .unwrap();
        assert_eq!(total, 2, "Should have the right number of entries");

        let conversations: Vec<HashMap<String, serde_json::Value>> =
            serde_json::from_value(conversations.get("records").to_owned().unwrap().to_owned())
                .unwrap();

        assert_eq!(
            conversations[0].get("title"),
            Some(&json!("Test conversation"))
        );

        assert_eq!(conversations[1].get("title"), Some(&json!("Another Test")));

        Ok(())
    }

    #[sqlx::test]
    fn should_be_able_to_search_conversations(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_admin();

        session.signup(&app).await?;

        for i in 0..10 {
            session
                .create_conversation(
                    &app,
                    json! ({
                        "title" : format!("{i}"),
                        "short_description" : "A test conversation",
                        "description" : "A longer description",
                        "image_url" : "http://someimage.png",
                        "tags" : ["one", "two", "three"],
                        "is_public" : true,
                        "is_invite_only" : false,
                        "slug" : format!("{i}"),
                        "primary_locale" : "en",
                        "supported_languages" : ["en"]
                    }),
                )
                .await?;
        }

        session
            .create_conversation(
                &app,
                json! ({
                    "title" : format!("this is the target"),
                    "short_description" : "A test conversation",
                    "description" : "A longer description",
                    "image_url" : "http://someimage.png",
                    "tags" : ["one", "two", "three"],
                    "is_public" : true,
                    "is_invite_only" : false,
                    "slug" : format!("target_slug"),
                    "primary_locale" : "en",
                    "supported_languages" : ["en"]
                }),
            )
            .await?;

        let url = format!("/conversation?title=target&offset={}&limit={}", 0, 10);
        let (status, conversations, _) = session.get(&app, &url).await?;

        let conversations: Vec<serde_json::Value> =
            serde_json::from_value(conversations.get("records").to_owned().unwrap().to_owned())?;

        assert_eq!(status, StatusCode::OK, "Should have ok status");

        assert_eq!(conversations.len(), 1, "should only get one back ");

        Ok(())
    }

    #[sqlx::test]
    fn should_be_able_to_order_conversations(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_admin();

        session.signup(&app).await?;

        for i in 0..5 {
            session
                .create_conversation(
                    &app,
                    json! ({
                        "title" : format!("{i}"),
                        "short_description" : "A test conversation",
                        "description" : "A longer description",
                        "image_url" : "http://someimage.png",
                        "tags" : ["one", "two", "three"],
                        "is_public" : true,
                        "is_invite_only" : false,
                        "slug" : format!("{i}"),
                        "primary_locale" : "en",
                        "supported_languages" : ["en"]
                    }),
                )
                .await?;
        }

        // Testing ASC
        let url = format!("/conversation?sort=created_at+asc&limit=20");
        let (status, conversations, _) = session.get(&app, &url).await?;

        let conversations: Vec<HashMap<String, serde_json::Value>> =
            serde_json::from_value(conversations.get("records").to_owned().unwrap().to_owned())
                .unwrap();

        let titles: Vec<serde_json::Value> = conversations
            .iter()
            .map(|c| c.get("title").to_owned().unwrap().to_owned())
            .collect();

        assert_eq!(status, StatusCode::OK, "Should have ok status");

        assert_eq!(
            titles,
            vec![json!("0"), json!("1"), json!("2"), json!("3"), json!("4")],
            "should get the right records back"
        );

        // Testing DESC
        let url = format!("/conversation?sort=created_at+desc&limit=20");
        let (status, conversations, _) = session.get(&app, &url).await?;

        let conversations: Vec<HashMap<String, serde_json::Value>> =
            serde_json::from_value(conversations.get("records").to_owned().unwrap().to_owned())
                .unwrap();

        let titles: Vec<serde_json::Value> = conversations
            .iter()
            .map(|c| c.get("title").to_owned().unwrap().to_owned())
            .collect();

        assert_eq!(status, StatusCode::OK, "Should have ok status");

        assert_eq!(
            titles,
            vec![json!("4"), json!("3"), json!("2"), json!("1"), json!("0")],
            "should get the right records back"
        );
        Ok(())
    }

    #[sqlx::test]
    fn should_be_able_to_correctly_page_conversations(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_admin();

        session.signup(&app).await?;

        for i in 0..40 {
            session
                .create_conversation(
                    &app,
                    json! ({
                        "title" : format!("{i}"),
                        "short_description" : "A test conversation",
                        "description" : "A longer description",
                        "image_url" : "http://someimage.png",
                        "tags" : ["one", "two", "three"],
                        "is_public" : true,
                        "is_invite_only" : false,
                        "slug" : format!("{i}"),
                        "primary_locale" : "en",
                        "supported_languages" : ["en"]
                    }),
                )
                .await?;
        }

        let (status, conversations, _) = session.list_conversations(&app, 5, 3).await?;

        assert_eq!(status, StatusCode::OK, "Should be found");

        let total: i32 =
            serde_json::from_value(conversations.get("total").to_owned().unwrap().to_owned())
                .unwrap();
        assert_eq!(total, 40, "Should have the right total number of entries");

        let conversations: Vec<HashMap<String, serde_json::Value>> =
            serde_json::from_value(conversations.get("records").to_owned().unwrap().to_owned())
                .unwrap();

        let titles: Vec<serde_json::Value> = conversations
            .iter()
            .map(|c| c.get("title").to_owned().unwrap().to_owned())
            .collect();

        assert_eq!(
            titles,
            vec![json!("5"), json!("6"), json!("7")],
            "should get the right records back"
        );

        Ok(())
    }

    #[sqlx::test]
    fn should_be_able_to_get_a_created_conversation_by_id(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_admin();

        session.signup(&app).await?;

        let (_, convo1, _) = session
            .create_conversation(
                &app,
                json! ({
                    "title" : "Test conversation",
                    "short_description" : "A test conversation",
                    "description" : "A longer description",
                    "image_url" : "http://someimage.png",
                    "tags" : ["one", "two", "three"],
                    "is_public" : false,
                    "is_invite_only" : false,
                    "slug" : "new_conversation",
                    "primary_locale" : "en",
                    "supported_languages" : ["en"]
                }),
            )
            .await?;

        let (_, convo2, _) = session
            .create_conversation(
                &app,
                json! ({
                    "title" : "Second convo",
                    "short_description" : "another convo",
                    "description" : "A longer description",
                    "image_url" : "http://someimage.png",
                    "tags" : ["one", "three"],
                    "is_public" : false,
                    "is_invite_only" : false,
                    "slug" : "new_conversation_two",
                    "primary_locale" : "en",
                    "supported_languages" : ["en"]
                }),
            )
            .await?;

        let convo1: HashMap<String, Option<serde_json::Value>> = serde_json::from_value(convo1)?;
        let convo2: HashMap<String, Option<serde_json::Value>> = serde_json::from_value(convo2)?;

        let id1: String =
            serde_json::from_value(convo1.get("id").unwrap().clone().unwrap()).unwrap();

        let id2: String =
            serde_json::from_value(convo2.get("id").unwrap().clone().unwrap()).unwrap();

        let slug: String =
            serde_json::from_value(convo2.get("slug").unwrap().clone().unwrap()).unwrap();

        let (status, value, _) = session.get_conversation(&app, &id1).await?;

        assert_eq!(status, StatusCode::OK, "Sould get it fine");

        assert_eq!(
            value.get("id"),
            Some(&json!(id1)),
            "should get back the correct conversation by id "
        );

        let (_, value, _) = session.get_conversation(&app, &slug).await?;

        assert_eq!(
            value.get("id"),
            Some(&json!(id2)),
            "should get back the correct conversation by slug"
        );

        Ok(())
    }

    #[sqlx::test]
    fn should_be_able_to_delete_conversation(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_admin();

        session.signup(&app).await?;
        let (_, conversation, _) = session
            .create_conversation(
                &app,
                json! ({
                    "title" : "Test conversation",
                    "short_description" : "A test conversation",
                    "description" : "A longer description",
                    "image_url" : "http://someimage.png",
                    "tags" : ["one", "two", "three"],
                    "is_public" : false,
                    "is_invite_only" : false,
                    "slug" : "new_conversation",
                    "primary_locale" : "en",
                    "supported_languages" : ["en"]
                }),
            )
            .await?;

        let id = conversation.get("id").unwrap().to_owned();
        let id: String = serde_json::from_value(id).unwrap();

        let (status, _, _) = session.delete_conversation(&app, &id).await?;

        assert_eq!(status, StatusCode::OK, "Should report ok for deletion");

        let (status, _, _) = session.get_conversation(&app, &id).await?;
        assert_eq!(
            status,
            StatusCode::NOT_FOUND,
            "Should not be able to get conversation after deletion"
        );

        Ok(())
    }
    #[sqlx::test]
    fn conversation_slugs_should_be_unique(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_admin();

        session.signup(&app).await?;

        session
            .create_conversation(
                &app,
                json! ({
                    "title" : "Test conversation",
                    "short_description" : "A test conversation",
                    "description" : "A longer description",
                    "image_url" : "http://someimage.png",
                    "tags" : ["one", "two", "three"],
                    "is_public" : false,
                    "is_invite_only" : false,
                    "slug" : "new_conversation",
                    "primary_locale" : "en",
                    "supported_languages" : ["en"]
                }),
            )
            .await?;

        let (status, _, _) = session
            .create_conversation(
                &app,
                json! ({
                    "title" : "Second convo",
                    "short_description" : "another convo",
                    "description" : "A longer description",
                    "image_url" : "http://someimage.png",
                    "tags" : ["one", "three"],
                    "is_public" : false,
                    "is_invite_only" : false,
                    "slug" : "new_conversation",
                    "primary_locale" : "en",
                    "supported_languages" : ["en"]
                }),
            )
            .await?;

        assert_eq!(status, StatusCode::CONFLICT, "Slugs should be unique");

        Ok(())
    }

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

        let (status, value, _) = user_session
            .post(
                &app,
                &format!("/conversation/{}/bot_service_sessions", conversation_id),
                Body::empty(),
            )
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
                &format!("/conversation/{}/bot_service_sessions", conversation_id),
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
