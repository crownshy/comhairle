use std::sync::Arc;

use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
};

use aide::axum::{
    ApiRouter,
    routing::{delete_with, get_with, patch_with, post_with, put_with},
};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use uuid::Uuid;

use crate::{
    ComhairleState,
    error::ComhairleError,
    models::{
        conversation::{
            self, ConversationFilterOptions, ConversationOrderOptions,
            ConversationWithTranslations, CreateConversation, IdOrSlug, PartialConversation,
        },
        conversation_email_notification_recipients::{
            self as email_recipients_model, CreateConversationEmailNotificationRecipients,
        },
        media::{FromWithMedia, MediaResolver},
        notification::{self as notification_model, CreateNotification, NotificationContextType},
        notification_delivery::{
            self as notification_delivery_model, CreateNotificationDelivery, DeliveryMethod,
        },
        pagination::{OrderParams, PageOptions, PaginatedResults},
        user_conversation_preferences,
        user_participation::{self},
        user_profile,
    },
    routes::{
        conversations::dto::{ConversationDto, LocalizedConversationDto},
        translations::LocaleExtractor,
    },
};

use super::auth::{OptionalUser, RequiredAdminUser, is_user_admin};

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

/// Shallow-merge the request body into the conversation's `metadata` jsonb
/// column. Keys present in the body are written; keys not present are left
/// untouched. The body must be a JSON object.
async fn patch_conversation_metadata(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Path(id): Path<Uuid>,
    Json(patch): Json<serde_json::Value>,
) -> Result<(StatusCode, Json<ConversationDto>), ComhairleError> {
    let conversation = conversation::patch_metadata(&state.db, &id, &patch).await?;
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

    let results = conversation::list(
        &state.db,
        page_options,
        order_options,
        filter_options,
        Some(locale),
    )
    .await?;

    let media = MediaResolver::load(
        &state.db,
        &results
            .records
            .iter()
            .filter_map(|c| c.image)
            .collect::<Vec<_>>(),
    )
    .await?;

    let results_with_media: PaginatedResults<LocalizedConversationDto> =
        FromWithMedia::from_with_media(
            results,
            &media,
            &state.config.default_conversation_image_url,
        );

    Ok((StatusCode::OK, Json(results_with_media)))
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
        && user.is_some()
        && is_user_admin(&state, user.as_ref().unwrap()).await;

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
        let conversation =
            conversation::get_localised_by_id_or_slug(&state.db, &conversation_ident, &locale)
                .await?;

        let media = MediaResolver::load(
            &state.db,
            &conversation.image.map(|image| [image]).unwrap_or_default(),
        )
        .await?;

        let conversation = FromWithMedia::from_with_media(
            conversation,
            &media,
            &state.config.default_conversation_image_url,
        );

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
    /// Required when delivery_method is "email". HTML body produced by the
    /// rich text editor; rendered into a branded email template.
    pub html_content: Option<String>,
    /// When set, the email is delivered only to this address as a preview and
    /// no audit notification row or per-user delivery is created. Requires
    /// delivery_method = email.
    pub test_email_recipient: Option<String>,
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
    /// Emails that the SMTP server rejected. Only populated for the email
    /// delivery path; always empty for in-app notifications.
    #[serde(default)]
    failed_recipients: Vec<String>,
}

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotificationRecipientsResponse {
    /// Number of distinct workflow participants (in-app delivery target count).
    pub participant_count: i32,
    /// Email addresses opted in to broadcast emails for this conversation.
    pub email_recipients: Vec<String>,
    /// Convenience count of `email_recipients`.
    pub email_recipient_count: i32,
}

/// Preview the recipient lists for the notify page (count + email list).
async fn get_notification_recipients(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    Path(conversation_id): Path<Uuid>,
) -> Result<(StatusCode, Json<NotificationRecipientsResponse>), ComhairleError> {
    let conversation = conversation::get_by_id(&state.db, &conversation_id).await?;
    if conversation.owner_id != user.id {
        return Err(ComhairleError::UserNotAuthorized);
    }

    let participant_ids =
        user_participation::get_participant_user_ids_for_conversation(&state.db, &conversation_id)
            .await?;
    let email_recipients =
        user_conversation_preferences::get_opted_in_broadcast_emails(&state.db, &conversation_id)
            .await?;

    Ok((
        StatusCode::OK,
        Json(NotificationRecipientsResponse {
            participant_count: participant_ids.len() as i32,
            email_recipient_count: email_recipients.len() as i32,
            email_recipients,
        }),
    ))
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

    let delivery_method = request
        .delivery_method
        .clone()
        .unwrap_or(DeliveryMethod::InApp);

    if let Some(test_recipient) = request.test_email_recipient.clone() {
        if !matches!(delivery_method, DeliveryMethod::Email) {
            return Err(ComhairleError::BadRequest(
                "test_email_recipient is only valid when delivery_method is email".into(),
            ));
        }
        return send_test_broadcast_email(&state, request, test_recipient).await;
    }

    match delivery_method {
        DeliveryMethod::Email => {
            send_broadcast_email_to_opted_in(&state, &conversation_id, request).await
        }
        DeliveryMethod::InApp => {
            send_in_app_notification_to_participants(&state, &conversation_id, request).await
        }
    }
}

async fn send_test_broadcast_email(
    state: &Arc<ComhairleState>,
    request: SendNotificationRequest,
    recipient: String,
) -> Result<(StatusCode, Json<SendEmailNotificationResponse>), ComhairleError> {
    let html_content = request.html_content.ok_or_else(|| {
        ComhairleError::BadRequest("html_content is required when delivery_method is email".into())
    })?;

    state
        .mailer
        .send_conversation_broadcast_email(&recipient, &request.title, &html_content)
        .map_err(|e| {
            tracing::warn!(
                "Failed to send test broadcast email to {}: {:?}",
                recipient,
                e
            );
            ComhairleError::BadRequest(format!("Failed to send test email: {}", e))
        })?;

    Ok((
        StatusCode::OK,
        Json(SendEmailNotificationResponse {
            notification_id: Uuid::nil(),
            participants_notified: 1,
            message: format!("Test email sent to {}", recipient),
            failed_recipients: vec![],
        }),
    ))
}

async fn send_in_app_notification_to_participants(
    state: &Arc<ComhairleState>,
    conversation_id: &Uuid,
    request: SendNotificationRequest,
) -> Result<(StatusCode, Json<SendEmailNotificationResponse>), ComhairleError> {
    let create_notification = CreateNotification {
        title: request.title,
        content: request.content,
        notification_type: request.notification_type,
        context_type: Some(NotificationContextType::Conversation),
        context_id: Some(*conversation_id),
    };

    let notification = notification_model::create(&state.db, &create_notification).await?;

    let participant_user_ids =
        user_participation::get_participant_user_ids_for_conversation(&state.db, conversation_id)
            .await?;

    if participant_user_ids.is_empty() {
        return Ok((
            StatusCode::OK,
            Json(SendEmailNotificationResponse {
                notification_id: notification.id,
                participants_notified: 0,
                message: "No participants found for this conversation".to_string(),
                failed_recipients: vec![],
            }),
        ));
    }

    let deliveries: Vec<CreateNotificationDelivery> = participant_user_ids
        .into_iter()
        .map(|user_id| CreateNotificationDelivery {
            notification_id: notification.id,
            user_id,
            delivery_method: Some(DeliveryMethod::InApp),
        })
        .collect();

    let created_deliveries =
        notification_delivery_model::create_bulk(&state.db, &deliveries).await?;

    let notification_level = notification.notification_type.to_string();

    for delivery in &created_deliveries {
        let _ = crate::websockets::handlers::notifications::NotificationMessageHandler::send_notification_to_user(
            state,
            &delivery.user_id,
            &notification.id,
            &notification.title,
            &notification.content,
            &notification_level,
        )
        .await;
    }

    Ok((
        StatusCode::CREATED,
        Json(SendEmailNotificationResponse {
            notification_id: notification.id,
            participants_notified: created_deliveries.len() as i32,
            message: format!(
                "Notification sent to {} participants",
                created_deliveries.len()
            ),
            failed_recipients: vec![],
        }),
    ))
}

async fn send_broadcast_email_to_opted_in(
    state: &Arc<ComhairleState>,
    conversation_id: &Uuid,
    request: SendNotificationRequest,
) -> Result<(StatusCode, Json<SendEmailNotificationResponse>), ComhairleError> {
    let html_content = request.html_content.ok_or_else(|| {
        ComhairleError::BadRequest("html_content is required when delivery_method is email".into())
    })?;

    // Create a notification row for the audit trail. The HTML body lives in
    // `content` so the record reflects what was actually sent; no per-user
    // NotificationDelivery rows are created because anonymous opt-ins have
    // no associated user.
    let create_notification = CreateNotification {
        title: request.title.clone(),
        content: html_content.clone(),
        notification_type: request.notification_type,
        context_type: Some(NotificationContextType::Conversation),
        context_id: Some(*conversation_id),
    };

    let notification = notification_model::create(&state.db, &create_notification).await?;

    let recipient_emails =
        user_conversation_preferences::get_opted_in_broadcast_emails(&state.db, conversation_id)
            .await?;

    if recipient_emails.is_empty() {
        return Ok((
            StatusCode::OK,
            Json(SendEmailNotificationResponse {
                notification_id: notification.id,
                participants_notified: 0,
                message: "No participants have opted in to email updates for this conversation"
                    .to_string(),
                failed_recipients: vec![],
            }),
        ));
    }

    let mut sent = 0i32;
    let mut failed_recipients: Vec<String> = Vec::new();
    let mut last_error: Option<String> = None;
    for email in &recipient_emails {
        match state
            .mailer
            .send_conversation_broadcast_email(email, &request.title, &html_content)
        {
            Ok(()) => sent += 1,
            Err(e) => {
                tracing::warn!("Failed to send broadcast email to {}: {:?}", email, e);
                failed_recipients.push(email.clone());
                last_error = Some(e.to_string());
            }
        }
    }

    let message = match (sent, failed_recipients.len()) {
        (0, n) => format!(
            "Failed to send to all {} recipients: {}",
            n,
            last_error.unwrap_or_else(|| "unknown error".into())
        ),
        (s, 0) => format!("Email sent to {} recipients", s),
        (s, n) => format!(
            "Email sent to {} of {} recipients ({} failed)",
            s,
            s + n as i32,
            n
        ),
    };

    Ok((
        StatusCode::CREATED,
        Json(SendEmailNotificationResponse {
            notification_id: notification.id,
            participants_notified: sent,
            message,
            failed_recipients,
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
        writer.write_record([
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

async fn export_conversation_demographics(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    RequiredAdminUser(user): RequiredAdminUser,
) -> Result<(StatusCode, [(String, String); 2], String), ComhairleError> {
    // Verify conversation exists and user is owner
    let conversation = conversation::get_by_id(&state.db, &conversation_id).await?;

    if conversation.owner_id != user.id {
        return Err(ComhairleError::UserNotAuthorized);
    }

    // Get demographic data for export
    let demographics =
        user_profile::get_demographics_for_export(&state.db, &conversation_id).await?;

    // Generate CSV
    let mut csv_output = Vec::new();
    {
        let mut writer = csv::Writer::from_writer(&mut csv_output);

        // Write headers
        writer.write_record([
            "User ID",
            "Ethnicity",
            "Age",
            "Gender",
            "Zipcode",
            "Political Party",
            "Profile Created At",
        ])?;

        // Write data rows
        for profile in demographics {
            writer.write_record(&[
                profile.user_id.to_string(),
                profile.ethnicity.unwrap_or_default(),
                profile.age.map(|a| a.to_string()).unwrap_or_default(),
                profile.gender.unwrap_or_default(),
                profile.zipcode.unwrap_or_default(),
                profile.political_party.unwrap_or_default(),
                profile.created_at.to_rfc3339(),
            ])?;
        }

        writer.flush()?;
    }

    let csv_string = String::from_utf8(csv_output)?;
    let filename = format!(
        "conversation-demographics-{}.csv",
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
            "/{conversation_id}/metadata",
            patch_with(patch_conversation_metadata, |op| {
                op.id("PatchConversationMetadata")
                    .summary("Shallow-merge keys into conversation metadata")
                    .tag("Conversation")
                    .description(
                        "Accepts a JSON object and merges it into the conversation's \
                         `metadata` jsonb column at the top level. Keys in the body \
                         overwrite existing keys; keys not present are left untouched. \
                         Nested objects are replaced, not deep-merged.",
                    )
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
            "/{conversation_id}/notifications/recipients",
            get_with(get_notification_recipients, |op| {
                op.id("GetNotificationRecipients")
                    .summary("Preview notification recipients")
                    .description("Returns participant count for in-app delivery and the list of email addresses opted in to broadcast emails. Owner-only.")
                    .response::<200, Json<NotificationRecipientsResponse>>()
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
            "/{conversation_id}/demographics/export",
            get_with(export_conversation_demographics, |op| {
                op.id("ExportConversationDemographics")
                    .summary("Export demographics for conversation participants")
                    .description("Exports a CSV file containing demographic data for users participating in the conversation's workflow. Only includes consented users. Requires conversation ownership.")
                    .tag("Conversation")
            }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use crate::bot_service::{ComhairleChat, ComhairleKnowledgeBase, MockComhairleBotService};
    use crate::bulk_storage_service::{MockBulkStorageService, UploadResult};
    use crate::config::BotServiceConfig;
    use crate::routes::conversations::ConversationResponse;
    use crate::routes::conversations::dto::{ConversationDto, LocalizedConversationDto};
    use crate::routes::media::dto::MediaDto;
    use crate::routes::translations::dto::TextContentDto;
    use crate::test_helpers::{multipart_body_builder, test_config, test_state};
    use crate::{setup_server, test_helpers::UserSession};
    use axum::http::StatusCode;
    use serde_json::json;
    use sqlx::PgPool;
    use std::collections::HashMap;
    use std::error::Error;
    use std::sync::Arc;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn should_be_able_to_create_conversation_without_bot_service_resources(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let mut config = test_config()?;
        config.bot_service = None;
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
        assert!(
            conversation.knowledge_base_id.is_none(),
            "incorrect knowledge_base_id"
        );
        assert!(conversation.chat_bot_id.is_none(), "incorrect chat_bot_id");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn should_be_able_to_create_conversation_with_bot_service_resources(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let mut config = test_config()?;
        config.bot_service = Some(BotServiceConfig {
            host: "test_host".to_string(),
            api_key: "test_api_key".to_string(),
            default_knowledge_base_id: "test_kb_id".to_string(),
            thinking_space_agent_id: "test_ta_id".to_string(),
            thinking_space_summary_agent_id: "test_ta_summary_id".to_string(),
            elicitation_bot_agent_id: "test_ea_id".to_string(),
        });
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
        assert!(
            conversation.knowledge_base_id.is_some(),
            "incorrect knowledge_base_id"
        );
        assert!(conversation.chat_bot_id.is_some(), "incorrect chat_bot_id");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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
    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn should_be_able_to_get_conversation_with_media(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let boundary = "test-boundary";
        let filename = "test_file.jpg";
        let content_type = "image/jpeg";

        let mut bulk_storage_service = MockBulkStorageService::new();
        bulk_storage_service
            .expect_upload_file()
            .once()
            .returning(move |_, _, _| {
                Box::pin(async move {
                    Ok(UploadResult {
                        url: format!("https://storage.com/{}", filename),
                    })
                })
            });

        let state = test_state()
            .db(pool)
            .bulk_storage_service(Arc::new(bulk_storage_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_admin();

        session.signup(&app).await?;
        let (_, value, _) = session
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
        let conversation: ConversationDto = serde_json::from_value(value)?;

        let body = multipart_body_builder()
            .content("test-content")
            .boundary(boundary)
            .filename(filename)
            .content_type(content_type)
            .call();
        let (_, value, _) = session
            .post_multipart(&app, "/media", boundary, body.into())
            .await?;
        let media: Vec<MediaDto> = serde_json::from_value(value)?;

        session
            .update_conversation(
                &app,
                &conversation.id.to_string(),
                json!({ "image": media[0].id }),
            )
            .await?;

        let (_, value, _) = session
            .get(&app, &format!("/conversation/{}", conversation.id))
            .await?;
        let conversation: LocalizedConversationDto = serde_json::from_value(value)?;

        assert!(
            conversation.image_url.contains("/images/test_file.jpg"),
            "incorrect image_url"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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
    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn should_export_conversation_demographics(pool: PgPool) -> Result<(), Box<dyn Error>> {
        use crate::models::{user_participation, user_profile};
        use crate::routes::auth::SignupRequest;

        let state = Arc::new(test_state().db(pool.clone()).call()?);
        let app = setup_server(state.clone()).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        // Create conversation and workflow
        let (_, conversation, _) = admin_session
            .create_conversation(
                &app,
                json! ({
                    "title" : "Test conversation",
                    "short_description" : "A test conversation",
                    "description" : "A longer description",
                    "image_url" : "http://someimage.png",
                    "tags" : ["one", "two", "three"],
                    "is_public" : true,
                    "is_live" : true,
                    "is_invite_only" : false,
                    "slug" : "test_demo_conversation",
                    "primary_locale" : "en",
                    "supported_languages" : ["en"]
                }),
            )
            .await?;
        let conversation: ConversationDto = serde_json::from_value(conversation)?;

        let (_, workflow, _) = admin_session
            .create_random_workflow(&app, &conversation.id.to_string())
            .await?;
        let workflow: crate::routes::workflows::dto::WorkflowDto =
            serde_json::from_value(workflow)?;

        // Create test users with profiles
        let user1 = crate::models::users::create_user(
            &SignupRequest {
                username: "demo_user1".to_string(),
                password: "password".to_string(),
                email: "user1@test.com".to_string(),
                avatar_url: None,
            },
            &pool,
        )
        .await?;

        user_profile::create(
            &pool,
            &user_profile::CreateUserProfile {
                user_id: user1.id,
                consented: true,
                ethnicity: Some("Asian".to_string()),
                age: Some(25),
                gender: Some("Female".to_string()),
                zipcode: Some("12345".to_string()),
                political_party: Some("Independent".to_string()),
            },
        )
        .await?;

        let user2 = crate::models::users::create_user(
            &SignupRequest {
                username: "demo_user2".to_string(),
                password: "password".to_string(),
                email: "user2@test.com".to_string(),
                avatar_url: None,
            },
            &pool,
        )
        .await?;

        user_profile::create(
            &pool,
            &user_profile::CreateUserProfile {
                user_id: user2.id,
                consented: false, // This user should not appear in export
                ethnicity: Some("Hispanic".to_string()),
                age: Some(30),
                gender: Some("Male".to_string()),
                zipcode: Some("67890".to_string()),
                political_party: Some("Democrat".to_string()),
            },
        )
        .await?;

        // Register users to workflow
        user_participation::create(&pool, &user1.id, &workflow.id).await?;
        user_participation::create(&pool, &user2.id, &workflow.id).await?;

        // Export demographics as conversation owner
        use axum::body::Body;
        use axum::http::Request;
        use http_body_util::BodyExt;
        use tower::ServiceExt;

        let url = format!("/conversation/{}/demographics/export", conversation.id);
        let mut request = Request::builder().uri(&url).method("GET");

        if let Some(cookie) = &admin_session.cookie {
            request = request.header("Cookie", cookie);
        }

        let request = request.body(Body::empty()).unwrap();
        let response = app.clone().oneshot(request).await?;
        let status = response.status();

        assert_eq!(status, StatusCode::OK, "Should export successfully");

        let body = response.into_body().collect().await?.to_bytes();
        let csv_string = String::from_utf8(body.to_vec())?;

        // Verify CSV contains expected headers
        assert!(
            csv_string.contains("User ID"),
            "CSV should have User ID header"
        );
        assert!(
            csv_string.contains("Ethnicity"),
            "CSV should have Ethnicity header"
        );
        assert!(csv_string.contains("Age"), "CSV should have Age header");
        assert!(
            csv_string.contains("Gender"),
            "CSV should have Gender header"
        );

        // Verify only consented user appears
        assert!(
            csv_string.contains(&user1.id.to_string()),
            "Should include consented user"
        );
        assert!(
            csv_string.contains("Asian"),
            "Should include user1's ethnicity"
        );
        assert!(
            !csv_string.contains(&user2.id.to_string()),
            "Should not include non-consented user"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    fn should_deny_demographics_export_to_non_owner(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut owner_session = UserSession::new_admin();
        owner_session.signup(&app).await?;

        let mut non_owner_session = UserSession::new_admin();
        non_owner_session.signup(&app).await?;

        // Create conversation as owner
        let (_, conversation, _) = owner_session
            .create_conversation(
                &app,
                json! ({
                    "title" : "Test conversation",
                    "short_description" : "A test conversation",
                    "description" : "A longer description",
                    "image_url" : "http://someimage.png",
                    "tags" : ["one", "two", "three"],
                    "is_public" : true,
                    "is_live" : true,
                    "is_invite_only" : false,
                    "slug" : "auth_test_conversation",
                    "primary_locale" : "en",
                    "supported_languages" : ["en"]
                }),
            )
            .await?;
        let conversation: ConversationDto = serde_json::from_value(conversation)?;

        // Try to export as non-owner
        let url = format!("/conversation/{}/demographics/export", conversation.id);
        let (status, _, _) = non_owner_session.get(&app, &url).await?;

        assert_eq!(
            status,
            StatusCode::UNAUTHORIZED,
            "Non-owner should not be able to export demographics"
        );

        Ok(())
    }

    /// Returns a fresh mock mailer that records every email passed to
    /// `send_conversation_broadcast_email` into the given shared `Vec`,
    /// and is tolerant of unrelated mail calls triggered by signup, etc.
    fn recording_mailer(
        sent: std::sync::Arc<std::sync::Mutex<Vec<String>>>,
    ) -> crate::mailer::MockComhairleMailer {
        use crate::mailer::MockComhairleMailer;
        let mut mailer = MockComhairleMailer::new();
        mailer.expect_send_welcome_email().returning(|_, _| Ok(()));
        mailer
            .expect_send_verification_email()
            .returning(|_, _, _| Ok(()));
        mailer.expect_send_email().returning(|_, _, _, _, _| Ok(()));
        mailer
            .expect_send_otp_email()
            .returning(|_, _, _, _| Ok(()));
        mailer
            .expect_send_password_reset_email()
            .returning(|_, _, _| Ok(()));
        mailer
            .expect_send_event_registration_email()
            .returning(|_, _, _, _, _, _| Box::pin(async move { Ok(()) }));
        mailer
            .expect_send_event_confirmation_email()
            .returning(|_, _, _, _, _| Box::pin(async move { Ok(()) }));
        mailer
            .expect_send_event_reminder()
            .returning(|_, _, _, _| Ok(()));
        mailer.expect_send_conversation_broadcast_email().returning(
            move |email, _subject, _html| {
                sent.lock().unwrap().push(email.to_string());
                Ok(())
            },
        );
        mailer
    }

    /// Helper: create the conversation and return its id.
    async fn make_conversation(
        session: &mut UserSession,
        app: &axum::Router,
        slug: &str,
    ) -> Result<uuid::Uuid, Box<dyn Error>> {
        let (_, conversation, _) = session
            .create_conversation(
                app,
                json!({
                    "title" : "Broadcast test",
                    "short_description" : "A test conversation",
                    "description" : "A longer description",
                    "image_url" : "http://someimage.png",
                    "tags" : ["one"],
                    "is_public" : true,
                    "is_live" : true,
                    "is_invite_only" : false,
                    "slug" : slug,
                    "primary_locale" : "en",
                    "supported_languages" : ["en"]
                }),
            )
            .await?;
        let conversation: ConversationDto = serde_json::from_value(conversation)?;
        Ok(conversation.id)
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn broadcast_email_only_sent_to_opted_in_authenticated_users(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn Error>> {
        use crate::models::user_conversation_preferences::{
            self, CreateUserConversationPreferences,
        };
        use crate::models::users::create_user;
        use crate::routes::auth::SignupRequest;
        use std::sync::{Arc, Mutex};

        let sent: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let mailer = recording_mailer(sent.clone());

        let state = test_state()
            .db(pool.clone())
            .mailer(Arc::new(mailer))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin = UserSession::new_admin();
        admin.signup(&app).await?;

        let conversation_id = make_conversation(&mut admin, &app, "opt_in_authed").await?;

        // user1 — opted in
        let user1 = create_user(
            &SignupRequest {
                username: "opt_in_user".into(),
                password: "password".into(),
                email: "user1@test.com".into(),
                avatar_url: None,
            },
            &pool,
        )
        .await?;
        user_conversation_preferences::create(
            &pool,
            &CreateUserConversationPreferences {
                user_id: user1.id,
                conversation_id,
                receive_updates_by_notification: Some(false),
                receive_updates_by_email: Some(true),
                receive_similar_conversation_updates_by_email: Some(false),
                receive_similar_conversation_updates_by_notification: Some(false),
            },
        )
        .await?;

        // user2 — explicitly opted out (preferences row exists, flag false)
        let user2 = create_user(
            &SignupRequest {
                username: "opt_out_user".into(),
                password: "password".into(),
                email: "user2@test.com".into(),
                avatar_url: None,
            },
            &pool,
        )
        .await?;
        user_conversation_preferences::create(
            &pool,
            &CreateUserConversationPreferences {
                user_id: user2.id,
                conversation_id,
                receive_updates_by_notification: Some(true),
                receive_updates_by_email: Some(false),
                receive_similar_conversation_updates_by_email: Some(false),
                receive_similar_conversation_updates_by_notification: Some(false),
            },
        )
        .await?;

        // user3 — no preferences row at all (default = not opted in)
        let _user3 = create_user(
            &SignupRequest {
                username: "no_prefs_user".into(),
                password: "password".into(),
                email: "user3@test.com".into(),
                avatar_url: None,
            },
            &pool,
        )
        .await?;

        let (status, body, _) = admin
            .post(
                &app,
                &format!("/conversation/{conversation_id}/notifications"),
                json!({
                    "title": "Hello",
                    "content": "{}",
                    "delivery_method": "email",
                    "html_content": "<p>Hi</p>",
                })
                .to_string()
                .into(),
            )
            .await?;

        assert_eq!(
            status,
            StatusCode::CREATED,
            "broadcast send should succeed, got body: {body:#?}"
        );

        let recorded = sent.lock().unwrap().clone();
        assert_eq!(
            recorded,
            vec!["user1@test.com".to_string()],
            "broadcast must reach only the opted-in authenticated user"
        );

        let notified = body.get("participantsNotified").and_then(|v| v.as_i64());
        assert_eq!(notified, Some(1), "participantsNotified should be 1");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn broadcast_email_respects_anonymous_opt_in_flag(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn Error>> {
        use crate::models::conversation_email_notification_recipients::{
            self as anon_model, CreateConversationEmailNotificationRecipients,
        };
        use std::sync::{Arc, Mutex};

        let sent: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let mailer = recording_mailer(sent.clone());

        let state = test_state()
            .db(pool.clone())
            .mailer(Arc::new(mailer))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin = UserSession::new_admin();
        admin.signup(&app).await?;

        let conversation_id = make_conversation(&mut admin, &app, "opt_in_anon").await?;

        // Anonymous opt-in
        anon_model::create(
            &pool,
            &CreateConversationEmailNotificationRecipients {
                conversation_id,
                email: "anon_in@test.com".into(),
                receive_updates_by_email: true,
                receive_similar_conversation_updates_by_email: false,
            },
        )
        .await?;

        // Anonymous opt-out (e.g. registered only for similar-conversation updates)
        anon_model::create(
            &pool,
            &CreateConversationEmailNotificationRecipients {
                conversation_id,
                email: "anon_out@test.com".into(),
                receive_updates_by_email: false,
                receive_similar_conversation_updates_by_email: true,
            },
        )
        .await?;

        let (status, _, _) = admin
            .post(
                &app,
                &format!("/conversation/{conversation_id}/notifications"),
                json!({
                    "title": "Hello",
                    "content": "{}",
                    "delivery_method": "email",
                    "html_content": "<p>Hi</p>",
                })
                .to_string()
                .into(),
            )
            .await?;
        assert_eq!(status, StatusCode::CREATED);

        let recorded = sent.lock().unwrap().clone();
        assert_eq!(
            recorded,
            vec!["anon_in@test.com".to_string()],
            "broadcast must reach only the opted-in anonymous registrant"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn broadcast_email_returns_zero_when_nobody_is_opted_in(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn Error>> {
        use std::sync::{Arc, Mutex};

        let sent: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let mailer = recording_mailer(sent.clone());

        let state = test_state()
            .db(pool.clone())
            .mailer(Arc::new(mailer))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin = UserSession::new_admin();
        admin.signup(&app).await?;

        let conversation_id = make_conversation(&mut admin, &app, "opt_in_empty").await?;

        let (status, body, _) = admin
            .post(
                &app,
                &format!("/conversation/{conversation_id}/notifications"),
                json!({
                    "title": "Hello",
                    "content": "{}",
                    "delivery_method": "email",
                    "html_content": "<p>Hi</p>",
                })
                .to_string()
                .into(),
            )
            .await?;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            body.get("participantsNotified").and_then(|v| v.as_i64()),
            Some(0)
        );
        assert!(
            sent.lock().unwrap().is_empty(),
            "no mail should be sent when nobody is opted in"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn recipients_endpoint_lists_only_opted_in_emails(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn Error>> {
        use crate::models::conversation_email_notification_recipients::{
            self as anon_model, CreateConversationEmailNotificationRecipients,
        };
        use crate::models::user_conversation_preferences::{
            self, CreateUserConversationPreferences,
        };
        use crate::models::users::create_user;
        use crate::routes::auth::SignupRequest;
        use std::sync::{Arc, Mutex};

        let sent: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let mailer = recording_mailer(sent);

        let state = test_state()
            .db(pool.clone())
            .mailer(Arc::new(mailer))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin = UserSession::new_admin();
        admin.signup(&app).await?;

        let conversation_id = make_conversation(&mut admin, &app, "recipients_preview").await?;

        // Authenticated opt-in
        let user_in = create_user(
            &SignupRequest {
                username: "in_user".into(),
                password: "password".into(),
                email: "authed_in@test.com".into(),
                avatar_url: None,
            },
            &pool,
        )
        .await?;
        user_conversation_preferences::create(
            &pool,
            &CreateUserConversationPreferences {
                user_id: user_in.id,
                conversation_id,
                receive_updates_by_notification: Some(false),
                receive_updates_by_email: Some(true),
                receive_similar_conversation_updates_by_email: Some(false),
                receive_similar_conversation_updates_by_notification: Some(false),
            },
        )
        .await?;

        // Authenticated opt-out
        let user_out = create_user(
            &SignupRequest {
                username: "out_user".into(),
                password: "password".into(),
                email: "authed_out@test.com".into(),
                avatar_url: None,
            },
            &pool,
        )
        .await?;
        user_conversation_preferences::create(
            &pool,
            &CreateUserConversationPreferences {
                user_id: user_out.id,
                conversation_id,
                receive_updates_by_notification: Some(false),
                receive_updates_by_email: Some(false),
                receive_similar_conversation_updates_by_email: Some(false),
                receive_similar_conversation_updates_by_notification: Some(false),
            },
        )
        .await?;

        // Anonymous opt-in + opt-out
        anon_model::create(
            &pool,
            &CreateConversationEmailNotificationRecipients {
                conversation_id,
                email: "anon_in@test.com".into(),
                receive_updates_by_email: true,
                receive_similar_conversation_updates_by_email: false,
            },
        )
        .await?;
        anon_model::create(
            &pool,
            &CreateConversationEmailNotificationRecipients {
                conversation_id,
                email: "anon_out@test.com".into(),
                receive_updates_by_email: false,
                receive_similar_conversation_updates_by_email: true,
            },
        )
        .await?;

        let url = format!("/conversation/{conversation_id}/notifications/recipients");
        let (status, body, _) = admin.get(&app, &url).await?;
        assert_eq!(status, StatusCode::OK);

        let mut emails: Vec<String> = body
            .get("emailRecipients")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        emails.sort();

        assert_eq!(
            emails,
            vec![
                "anon_in@test.com".to_string(),
                "authed_in@test.com".to_string()
            ],
            "recipients endpoint must list only opted-in emails (auth + anon)"
        );
        assert_eq!(
            body.get("emailRecipientCount").and_then(|v| v.as_i64()),
            Some(2)
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn recipients_endpoint_denies_non_owner(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut owner = UserSession::new_admin();
        owner.signup(&app).await?;
        let mut intruder = UserSession::new("intruder", "password", "intruder@test.com");
        intruder.signup(&app).await?;

        let conversation_id = make_conversation(&mut owner, &app, "recipients_auth").await?;

        let url = format!("/conversation/{conversation_id}/notifications/recipients");
        let (status, _, _) = intruder.get(&app, &url).await?;

        assert_eq!(
            status,
            StatusCode::UNAUTHORIZED,
            "non-owner must not see recipient preview"
        );

        Ok(())
    }
}
