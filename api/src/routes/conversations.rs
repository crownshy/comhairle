use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Json, Multipart, Path, Query, State},
    http::{HeaderValue, StatusCode},
    response::Response,
};

use aide::axum::{
    routing::{delete_with, get_with, post_with, put_with},
    ApiRouter,
};

use hyper::header::{CONTENT_DISPOSITION, CONTENT_TYPE};
use rand::{distributions::Alphanumeric, Rng};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::{
        conversation::{
            self, ConversationFilterOptions, ConversationOrderOptions,
            ConversationWithTranslations, CreateConversation, IdOrSlug, PartialConversation,
        },
        conversation_email_notification_recipients::{
            self as email_recipients_model, CreateConversationEmailNotificationRecipients,
        },
        notification::{self as notification_model, CreateNotification, NotificationContextType},
        notification_delivery::{
            self as notification_delivery_model, CreateNotificationDelivery, DeliveryMethod,
        },
        pagination::{OrderParams, PageOptions, PaginatedResults},
        user_conversation_preferences,
        user_participation::{self},
        workflow::{self, CreateWorkflow},
        workflow_step,
    },
    routes::{
        conversations::dto::{
            ConversationDto, ImportExportConversationDto, ImportExportConversationWithWorkflowDto,
            LocalizedConversationDto,
        },
        translations::LocaleExtractor,
        workflow_steps::dto::{ImportExportToolConfig, ImportExportWorkflowStepDto},
        workflows::dto::{ImportExportWorkflowDto, ImportExportWorkflowWithWorkflowStepsDto},
    },
    tools::ToolConfig,
    ComhairleState,
};

use super::auth::{is_user_admin, OptionalUser, RequiredAdminUser};

pub mod dto;

/// Create conversation handler
async fn create_conversation(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    Json(new_conversation): Json<CreateConversation>,
) -> Result<(StatusCode, Json<ConversationDto>), ComhairleError> {
    let conversation = conversation::create(
        &state.db,
        &state.bot_service,
        &state.config,
        &new_conversation,
        user.id,
        user.organization_id,
    )
    .await?;

    let conversation: ConversationDto = conversation.into();
    Ok((StatusCode::CREATED, Json(conversation)))
}

/// Update conversation handler
async fn update_conversation(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Path(id): Path<Uuid>,
    Json(conversation): Json<PartialConversation>,
) -> Result<(StatusCode, Json<ConversationDto>), ComhairleError> {
    let conversation = conversation::update(&state.db, &id, &conversation).await?;
    let conversation: ConversationDto = conversation.into();
    Ok((StatusCode::OK, Json(conversation)))
}

/// List conversations handler
async fn list_conversations(
    State(state): State<Arc<ComhairleState>>,
    OrderParams(order_options): OrderParams<ConversationOrderOptions>,
    Query(mut filter_options): Query<ConversationFilterOptions>,
    Query(page_options): Query<PageOptions>,
    LocaleExtractor(locale): LocaleExtractor,
) -> Result<(StatusCode, Json<PaginatedResults<LocalizedConversationDto>>), ComhairleError> {
    filter_options.enforce_live();

    let conversations = conversation::list(
        &state.db,
        page_options,
        order_options,
        filter_options,
        Some(locale),
    )
    .await?
    .into();

    Ok((StatusCode::OK, Json(conversations)))
}

async fn launch_conversation(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<ConversationDto>), ComhairleError> {
    let conversation = conversation::get_by_id(&state.db, &conversation_id).await?;
    if conversation.is_live {
        return Err(ComhairleError::ConversationAlreadyLive);
    }
    let conversation: ConversationDto = conversation::launch(&state.db, conversation_id, &state)
        .await?
        .into();
    Ok((StatusCode::OK, Json(conversation)))
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct GetConversationQuery {
    #[serde(rename = "withTranslations", default)]
    pub with_translations: bool,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(untagged)]
pub enum ConversationResponse {
    WithTranslations(ConversationWithTranslations),
    Localized(LocalizedConversationDto),
}

/// Get a specific conversation
#[instrument(err(Debug), skip(state))]
async fn get_conversation(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_ident): Path<IdOrSlug>,
    Query(query): Query<GetConversationQuery>,
    OptionalUser(user): OptionalUser,
    LocaleExtractor(locale): LocaleExtractor,
) -> Result<(StatusCode, Json<ConversationResponse>), ComhairleError> {
    info!("Attempting to get conversation {conversation_ident:#?}");

    // Get the original conversation first
    let original_conversation =
        conversation::get_by_id_or_slug(&state.db, &conversation_ident).await?;

    // If this isn't a live conversation and the user is not the owner
    if !original_conversation.is_live {
        if let Some(user) = &user {
            if user.id != original_conversation.owner_id {
                return Err(ComhairleError::UserIsNotConversationOwner);
            }
        } else {
            return Err(ComhairleError::UserIsNotConversationOwner);
        }
    }

    // Check if user is admin and withTranslations is requested
    let should_return_with_translations = query.with_translations
        && user
            .as_ref()
            .map(|u| is_user_admin(u, &state.config))
            .unwrap_or(false);

    if should_return_with_translations {
        // Convert to ConversationWithTranslations
        let conversation_with_translations =
            ConversationWithTranslations::from_original(&state.db, original_conversation, &locale)
                .await?;

        Ok((
            StatusCode::OK,
            Json(ConversationResponse::WithTranslations(
                conversation_with_translations,
            )),
        ))
    } else {
        // Return localized conversation as before
        info!("Trying to get localized translations for {locale}");
        let conversation: LocalizedConversationDto =
            conversation::get_localised_by_id_or_slug(&state.db, &conversation_ident, &locale)
                .await?
                .into();

        Ok((
            StatusCode::OK,
            Json(ConversationResponse::Localized(conversation)),
        ))
    }
}

/// Delete a specific conversation
async fn delete_conversation(
    State(state): State<Arc<ComhairleState>>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<ConversationDto>), ComhairleError> {
    let conversation = conversation::delete(&state.db, &state.bot_service, &id).await?;
    let conversation: ConversationDto = conversation.into();
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
#[serde(rename_all = "camelCase")]
pub struct RegisterEmailResponse {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub email: String,
    pub message: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SendEmailNotificationResponse {
    pub notification_id: Uuid,
    participants_notified: i32,
    message: String,
}

/// Send notification to all conversation participants
async fn send_notification_to_participants(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    Path(conversation_id): Path<Uuid>,
    Json(request): Json<SendNotificationRequest>,
) -> Result<(StatusCode, Json<SendEmailNotificationResponse>), ComhairleError> {
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
            Json(SendEmailNotificationResponse {
                notification_id: notification.id,
                participants_notified: 0,
                message: "No participants found for this conversation".to_string(),
            }),
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
        Json(SendEmailNotificationResponse {
            notification_id: notification.id,
            participants_notified: created_deliveries.len() as i32,
            message: format!(
                "Notification sent to {} participants",
                created_deliveries.len()
            ),
        }),
    ))
}

/// Register email for conversation updates
async fn register_email_for_updates(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    Json(request): Json<RegisterEmailRequest>,
) -> Result<(StatusCode, Json<RegisterEmailResponse>), ComhairleError> {
    // Verify conversation exists and is public
    let _conversation = conversation::get_by_id(&state.db, &conversation_id).await?;

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

async fn export_conversation_contacts(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, [(String, String); 2], String), ComhairleError> {
    // Verify conversation exists
    conversation::get_by_id(&state.db, &conversation_id).await?;

    // Get all contacts who opted in
    let contacts =
        user_conversation_preferences::get_contacts_for_export(&state.db, &conversation_id).await?;

    // Generate CSV
    let mut csv_output = Vec::new();
    {
        let mut writer = csv::Writer::from_writer(&mut csv_output);

        // Write headers
        writer.write_record(&[
            "Email",
            "User Type",
            "Conversation Updates",
            "Similar Conversations Updates",
            "Signup Date",
        ])?;

        // Write data rows
        for contact in contacts {
            writer.write_record(&[
                contact.email,
                contact.user_type,
                if contact.conversation_updates {
                    "Yes"
                } else {
                    "No"
                }
                .to_string(),
                if contact.similar_conversations_updates {
                    "Yes"
                } else {
                    "No"
                }
                .to_string(),
                contact.signup_date.to_rfc3339(),
            ])?;
        }

        writer.flush()?;
    }

    let csv_string = String::from_utf8(csv_output)?;
    let filename = format!(
        "conversation-contacts-{}.csv",
        chrono::Utc::now().format("%Y-%m-%d")
    );

    Ok((
        StatusCode::OK,
        [
            (
                "Content-Type".to_string(),
                "text/csv; charset=utf-8".to_string(),
            ),
            (
                "Content-Disposition".to_string(),
                format!("attachment; filename=\"{}\"", filename),
            ),
        ],
        csv_string,
    ))
}

#[instrument(err(Debug), skip(state))]
async fn export_conversation(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    RequiredAdminUser(_user): RequiredAdminUser,
    LocaleExtractor(locale): LocaleExtractor,
) -> Result<Response, ComhairleError> {
    let conversation =
        conversation::get_localised_by_id(&state.db, &conversation_id, &locale).await?;
    let workflow = workflow::get_by_conversation_id(&state.db, &conversation_id).await?;
    let workflow_steps = workflow_step::list_localized(&state.db, &workflow.id, &locale).await?;

    let conversation_dto: ImportExportConversationDto = conversation.into();
    let workflow_dto: ImportExportWorkflowDto = workflow.into();
    let mut workflow_step_dtos: Vec<ImportExportWorkflowStepDto> = vec![];

    for step in workflow_steps {
        // TODO: clone heyform with fresh credentials
        let export_config = match step.preview_tool_config {
            ToolConfig::Polis(_) => ImportExportToolConfig::Polis,
            ToolConfig::HeyForm(_) => ImportExportToolConfig::HeyForm,
            ToolConfig::Stories(_) => ImportExportToolConfig::Stories,
            ToolConfig::Learn(ref config) => ImportExportToolConfig::Learn(config.clone()), // TODO:
            // should figure out how to use clone_tool functionality
            ToolConfig::ElicitationBot(ref config) => {
                ImportExportToolConfig::ElicitationBot(config.clone())
            }
        };
        let mut step_dto: ImportExportWorkflowStepDto = step.into();
        step_dto.preview_tool_config = Some(export_config);
        workflow_step_dtos.push(step_dto);
    }

    let combined_workflow = ImportExportWorkflowWithWorkflowStepsDto {
        workflow: workflow_dto,
        workflow_steps: workflow_step_dtos,
    };
    let combined_conversation = ImportExportConversationWithWorkflowDto {
        conversation: conversation_dto,
        workflows: vec![combined_workflow],
    };

    let json = serde_json::to_vec(&combined_conversation)?;
    let content_disposition = HeaderValue::from_str(&format!(
        "attachment; filename=\"conversation-{conversation_id}.json\""
    ))?;

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "application/json")
        .header(CONTENT_DISPOSITION, content_disposition)
        .body(Body::from(json))?;

    Ok(response)
}

#[instrument(err(Debug), skip(state))]
async fn import_conversation(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    LocaleExtractor(locale): LocaleExtractor,
    mut form_data: Multipart,
) -> Result<(StatusCode, Json<ConversationDto>), ComhairleError> {
    let bytes = match form_data.next_field().await? {
        Some(field) => field.bytes().await?,
        None => return Err(ComhairleError::BadRequest("Missing form field".to_string())),
    };
    if form_data.next_field().await?.is_some() {
        return Err(ComhairleError::BadRequest(
            "Only one file import allowed".to_string(),
        ));
    }

    let import: ImportExportConversationWithWorkflowDto = serde_json::from_slice(&bytes)?;
    let mut imported_conversation = import.conversation;
    let imported_workflow =
        import
            .workflows
            .into_iter()
            .next()
            .ok_or(ComhairleError::CorruptedData(
                "Missing workflow".to_string(),
            ))?;

    // Add random suffix to slug to avoid unique constraint in db
    let slug_suffix: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    imported_conversation.slug = imported_conversation
        .slug
        .map(|s| format!("{s}-{slug_suffix}"));

    let create_conversation_params: CreateConversation = imported_conversation.into();
    let new_conversation = conversation::create(
        &state.db,
        &state.bot_service,
        &state.config,
        &create_conversation_params,
        user.id,
        user.organization_id,
    )
    .await?;

    let create_workflow_params: CreateWorkflow = imported_workflow.workflow.into();
    let _new_workflow = workflow::create(
        &state.db,
        &create_workflow_params,
        Some(new_conversation.id),
        None,
        user.id,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(new_conversation.into()))) // TODO: proper response
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
                    .response::<201, Json<ConversationDto>>()
            }),
        )
        .api_route(
            "/",
            get_with(list_conversations, |op| {
                op.id("ListConverastions")
                    .summary("List conversations with optional filtering and ordering")
                    .tag("Conversation")
                    .description("List conversations")
                    .response::<200, Json<PaginatedResults<LocalizedConversationDto>>>()
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
                    .response::<200, Json<ConversationDto>>()
            }),
        )
        .api_route(
            "/{conversation_id}",
            delete_with(delete_conversation, |op| {
                op.id("DeleteConversation")
                    .summary("Delete the conversation and all related content")
                    .tag("Conversation")
                    .description("Delete the conversation and all related content")
                    .response::<200, Json<ConversationDto>>()
            }),
        )
        .api_route(
            "/{conversation_id}/launch",
            put_with(launch_conversation, |op| {
                op.id("LaunchConversation")
                    .summary("Makes the conversation live")
                    .tag("Conversation")
                    .description("Makes the conversation live for participants")
                    .response::<200, Json<ConversationDto>>()
            }),
        )
        .api_route(
            "/{conversation_id}/notifications",
            post_with(send_notification_to_participants, |op| {
                op.id("SendNotificationToParticipants")
                    .summary("Send notification to all conversation participants")
                    .description("Creates a notification and sends it to all users participating in workflows within the conversation. Only conversation owners can send notifications.")
                    .response::<201, Json<SendEmailNotificationResponse>>()
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
            "/{conversation_id}/contacts/export",
            get_with(export_conversation_contacts, |op| {
                op.id("ExportConversationContacts")
                    .summary("Export contact list for conversation")
                    .description("Exports a CSV file containing all users who have opted in to receive email updates for this conversation")
                    .tag("Conversation")
            }),
        )
        .api_route(
            "/{conversation_id}/export",
            get_with(export_conversation, |op| {
                op.summary("Export a conversation")
                    .description("Exports a conversation, workflows, steps etc to a json file.")
                    .response::<200, Json<ImportExportConversationDto>>()
                    .tag("Conversation")
            }),
        )
        .api_route(
            "/import",
            post_with(import_conversation, |op| {
                op.summary("Import a conversation")
                    .description("Imports a conversation from an exported json file")
                    .response::<200, Json<()>>()
                    .tag("Conversation")
            }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use crate::bot_service::{ComhairleChat, ComhairleKnowledgeBase, MockComhairleBotService};
    use crate::routes::conversations::dto::{ConversationDto, LocalizedConversationDto};
    use crate::routes::conversations::ConversationResponse;
    use crate::routes::translations::dto::TextContentDto;
    use crate::test_helpers::{test_config, test_state};
    use crate::{setup_server, test_helpers::UserSession};
    use axum::http::StatusCode;
    use serde_json::json;
    use sqlx::PgPool;
    use std::collections::HashMap;
    use std::error::Error;
    use std::sync::Arc;

    #[sqlx::test]
    fn should_be_able_to_create_conversation_without_bot_service_resources(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let mut config = test_config()?;
        config.bot_service_host = None;
        config.bot_service_api_key = None;
        config.default_knowledge_base_id = None;
        config.elicitation_bot_agent_id = None;
        let state = test_state().db(pool).config(config).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_admin();

        session.signup(&app).await?;

        let (status, response, _) = session
            .create_conversation(
                &app,
                json! ({
                    "title" : "Test conversation",
                    "short_description" : "A test conversation",
                    "description" : "A longer description",
                    "image_url" : "http://someimage.png",
                    "tags" : ["one", "two", "three"],
                    "is_public" : false,
                    "is_live": true,
                    "is_invite_only" : false,
                    "slug" : "new_conversation",
                    "primary_locale" : "en",
                    "supported_languages" : ["en"]
                }),
            )
            .await?;
        let conversation: ConversationDto = serde_json::from_value(response)?;

        assert_eq!(status, StatusCode::CREATED, "Should be created");
        assert_eq!(
            conversation.image_url,
            "http://someimage.png".to_string(),
            "incorrect json response"
        );
        assert!(
            conversation.knowledge_base_id.is_none(),
            "incorrect knowledge_base_id"
        );
        assert!(conversation.chat_bot_id.is_none(), "incorrect chat_bot_id");

        Ok(())
    }

    #[sqlx::test]
    fn should_be_able_to_create_conversation_with_bot_service_resources(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let mut config = test_config()?;
        config.bot_service_host = Some("test_host".to_string());
        config.bot_service_api_key = Some("test_api_key".to_string());
        config.default_knowledge_base_id = Some("test_kb_id".to_string());
        config.elicitation_bot_agent_id = Some("test_ea_id".to_string());
        let state = test_state().db(pool).config(config).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_admin();

        session.signup(&app).await?;

        let (status, response, _) = session
            .create_conversation(
                &app,
                json! ({
                    "title" : "Test conversation",
                    "short_description" : "A test conversation",
                    "description" : "A longer description",
                    "image_url" : "http://someimage.png",
                    "tags" : ["one", "two", "three"],
                    "is_public" : false,
                    "is_live": true,
                    "is_invite_only" : false,
                    "slug" : "new_conversation",
                    "primary_locale" : "en",
                    "supported_languages" : ["en"]
                }),
            )
            .await?;
        let conversation: ConversationDto = serde_json::from_value(response)?;

        assert_eq!(status, StatusCode::CREATED, "Should be created");
        assert_eq!(
            conversation.image_url,
            "http://someimage.png".to_string(),
            "incorrect json response"
        );
        assert!(
            conversation.knowledge_base_id.is_some(),
            "incorrect knowledge_base_id"
        );
        assert!(conversation.chat_bot_id.is_some(), "incorrect chat_bot_id");

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
                    "is_live": true,
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
        let conversation: ConversationDto = serde_json::from_value(conversation)?;

        assert_eq!(status, StatusCode::OK, "Should update resource");
        assert!(conversation.is_public, "should have updated public status");

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
                    "is_live": true,
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
                    "is_live": true,
                    "is_invite_only" : false,
                    "slug" : "new_conversation",
                    "primary_locale" : "en",
                    "supported_languages" : ["en"]
                }),
            )
            .await?;

        let (_status, _result, _) = session
            .create_conversation(
                &app,
                json! ({
                    "title" : "Another Test",
                    "short_description" : "A test conversation",
                    "description" : "A longer description",
                    "image_url" : "http://someimage.png",
                    "tags" : ["one", "two", "three"],
                    "is_public" : true,
                    "is_live" : true,
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

        let conversations: Vec<LocalizedConversationDto> =
            serde_json::from_value(conversations.get("records").to_owned().unwrap().to_owned())
                .unwrap();

        assert_eq!(conversations[0].title, "Test conversation".to_string(),);

        assert_eq!(conversations[1].title, "Another Test".to_string());

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
                        "is_live" : true,
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
                    "is_live" : true,
                    "is_invite_only" : false,
                    "slug" : format!("target_slug"),
                    "primary_locale" : "en",
                    "supported_languages" : ["en"]
                }),
            )
            .await?;

        let url = format!("/conversation?keyword=target&offset={}&limit={}", 0, 10);
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
                        "is_live" : true,
                        "is_invite_only" : false,
                        "slug" : format!("{i}"),
                        "primary_locale" : "en",
                        "supported_languages" : ["en"]
                    }),
                )
                .await?;
        }

        // Testing ASC
        let url = "/conversation?sort=created_at+asc&limit=20";
        let (status, conversations, _) = session.get(&app, url).await?;

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
        let url = "/conversation?sort=created_at+desc&limit=20";
        let (status, conversations, _) = session.get(&app, url).await?;

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
                        "is_live" : true,
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
                    "is_public" : true,
                    "is_live": true,
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
                    "is_live" : true,
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
    fn should_be_able_to_get_a_created_conversation_with_translations(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_admin();

        session.signup(&app).await?;

        let (_, convo_res, _) = session
            .create_conversation(
                &app,
                json! ({
                    "title" : "Test conversation",
                    "short_description" : "A test conversation",
                    "description" : "A longer description",
                    "image_url" : "http://someimage.png",
                    "tags" : ["one", "two", "three"],
                    "is_public" : true,
                    "is_live": true,
                    "is_invite_only" : false,
                    "slug" : "new_conversation",
                    "primary_locale" : "en",
                    "supported_languages" : ["en"]
                }),
            )
            .await?;
        let conversation: ConversationDto = serde_json::from_value(convo_res)?;

        let create_privacy_policy = json!({
            "primary_locale": "en",
            "format": "plain",
            "content": "Test privacy policy"
        });
        let create_faqs = json!({
            "primary_locale": "en",
            "format": "plain",
            "content": "Test faqs"
        });
        let (_, privacy_policy_res, _) = session
            .post(
                &app,
                "/translations",
                create_privacy_policy.to_string().into(),
            )
            .await?;
        let (_, faqs_res, _) = session
            .post(&app, "/translations", create_faqs.to_string().into())
            .await?;
        let privacy_policy: TextContentDto = serde_json::from_value(privacy_policy_res)?;
        let faqs: TextContentDto = serde_json::from_value(faqs_res)?;

        let (_, update_res, _) = session
            .put(
                &app,
                &format!("/conversation/{}", conversation.id),
                json!({
                    "privacy_policy": privacy_policy.id,
                    "faqs": faqs.id,
                })
                .to_string()
                .into(),
            )
            .await?;
        println!();
        println!("    >>>>    Updated conversation: {update_res:#?}");
        println!();

        let (status, value, _) = session
            .get(
                &app,
                &format!("/conversation/{}?withTranslations=true", conversation.id),
            )
            .await?;
        let response: ConversationResponse = serde_json::from_value(value)?;

        assert_eq!(status, StatusCode::OK, "Sould get it fine");

        match response {
            ConversationResponse::WithTranslations(conversation) => {
                assert_eq!(
                    conversation.title,
                    "Test conversation".to_string(),
                    "incorrect localized top level fields"
                );
                assert_eq!(
                    conversation
                        .translations
                        .short_description
                        .text_translations[0]
                        .content,
                    "A test conversation",
                    "incorrect translation for required field"
                );
                assert_eq!(
                    conversation
                        .translations
                        .privacy_policy
                        .unwrap()
                        .text_translations[0]
                        .content,
                    "Test privacy policy",
                    "incorrect translation for optional privacy_policy"
                );
                assert_eq!(
                    conversation.translations.faqs.unwrap().text_translations[0].content,
                    "Test faqs",
                    "incorrect translation for optional faqs"
                );
            }
            _ => panic!("Expected ConversationResponse::WithTranslations"),
        }

        Ok(())
    }

    #[sqlx::test]
    fn should_be_able_to_delete_conversation(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let mut bot_service = MockComhairleBotService::new();
        bot_service
            .expect_create_knowledge_base()
            .once()
            .returning(|_, _| {
                Box::pin(async move {
                    Ok((
                        StatusCode::CREATED,
                        ComhairleKnowledgeBase {
                            ..Default::default()
                        },
                    ))
                })
            });
        bot_service.expect_create_chat().once().returning(|_| {
            Box::pin(async move {
                Ok((
                    StatusCode::CREATED,
                    ComhairleChat {
                        ..Default::default()
                    },
                ))
            })
        });
        bot_service
            .expect_delete_knowledge_base()
            .once()
            .returning(|_| Box::pin(async move { Ok(StatusCode::OK) }));
        bot_service
            .expect_delete_chat()
            .once()
            .returning(|_| Box::pin(async move { Ok(StatusCode::OK) }));
        let state = test_state()
            .db(pool)
            .bot_service(Arc::new(bot_service))
            .call()?;
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
                    "is_live" : false,
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
                    "is_live" : false,
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
                    "is_live" : false,
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
}
