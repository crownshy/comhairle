use std::sync::Arc;

use aide::OperationIo;
use aide::axum::ApiRouter;
use aide::axum::routing::{delete_with, get_with, post_with, put_with};
use axum::RequestPartsExt;
use axum::extract::{FromRequestParts, Json, Path, State};
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum_extra::extract::cookie::CookieJar;
use schemars::JsonSchema;
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::ComhairleState;
use crate::error::ComhairleError;
use crate::models::conversation::ConversationIden;
use crate::models::permissions::{
    self, can_perform_action, conversation as conversation_permissions,
    organization as organization_permissions, system,
};
use crate::models::translations::{
    self, CreateTextTranslation, TextContentId, TranslationScope, UpdateTextContent,
    UpdateTextTranslation,
};
use crate::routes::auth::RequiredUser;
use crate::routes::translations::dto::{TextContentDto, TextTranslationDto};

pub mod dto;

/// An extractor to get the user's locale preference from the COMHAIRLE_LOCALE cookie
/// Returns "en" as default if the cookie is not present or invalid
#[derive(Debug, Clone, OperationIo)]
pub struct LocaleExtractor(pub String);

impl FromRequestParts<Arc<ComhairleState>> for LocaleExtractor {
    type Rejection = ComhairleError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &Arc<ComhairleState>,
    ) -> Result<Self, Self::Rejection> {
        let jar = parts
            .extract::<CookieJar>()
            .await
            .map_err(|e| ComhairleError::LocaleError(e.to_string()))?;

        let locale = jar
            .get("COMHAIRLE_LOCALE")
            .map(|cookie| cookie.value().to_string())
            .unwrap_or_else(|| "en".to_string());

        Ok(LocaleExtractor(locale))
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct TextContentWithTranslations {
    #[serde(flatten)]
    pub text_content: TextContentDto,
    pub translations: Vec<TextTranslationDto>,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct CreateTextContentRequest {
    pub primary_locale: String,
    pub format: translations::TextFormat,
    pub content: String,
    pub scope: TranslationScope,
}

#[derive(Clone, Copy)]
enum TextContentAccess {
    Read,
    UpdateSource,
    TranslateLocale,
}

async fn authorize_scope(
    state: &Arc<ComhairleState>,
    scope: TranslationScope,
    user_id: &uuid::Uuid,
    access: TextContentAccess,
) -> Result<(), ComhairleError> {
    let allowed = match scope {
        TranslationScope::System => {
            can_perform_action(
                state,
                &permissions::SYSTEM_RESOURCE_ID,
                system::Action::Translate,
                user_id,
                None,
            )
            .await?
        }
        TranslationScope::Conversation(conversation_id) => {
            let (sql, values) = Query::select()
                .column(ConversationIden::OwnerId)
                .from(ConversationIden::Table)
                .and_where(Expr::col(ConversationIden::Id).eq(conversation_id))
                .build_sqlx(PostgresQueryBuilder);
            let owner_id: Option<uuid::Uuid> = sqlx::query_scalar_with(&sql, values)
                .fetch_optional(&state.db)
                .await?;
            let action = match access {
                TextContentAccess::Read => conversation_permissions::Action::Read,
                TextContentAccess::UpdateSource => conversation_permissions::Action::Update,
                TextContentAccess::TranslateLocale => conversation_permissions::Action::Translate,
            };
            can_perform_action(state, &conversation_id, action, user_id, owner_id.as_ref()).await?
        }
        TranslationScope::Organization(organization_id) => {
            let action = match access {
                TextContentAccess::Read => organization_permissions::Action::Read,
                TextContentAccess::UpdateSource => organization_permissions::Action::Update,
                TextContentAccess::TranslateLocale => organization_permissions::Action::Translate,
            };
            can_perform_action(state, &organization_id, action, user_id, None).await?
        }
    };

    if allowed {
        Ok(())
    } else {
        Err(ComhairleError::UserNotAuthorized)
    }
}

async fn authorize_text_content(
    state: &Arc<ComhairleState>,
    content_id: &TextContentId,
    user_id: &uuid::Uuid,
    access: TextContentAccess,
) -> Result<translations::TextContent, ComhairleError> {
    let text_content = translations::get_text_content_by_id(&state.db, content_id).await?;
    authorize_text_content_record(state, &text_content, user_id, access).await?;
    Ok(text_content)
}

fn text_content_scope(
    text_content: &translations::TextContent,
) -> Result<TranslationScope, ComhairleError> {
    match (text_content.conversation_id, text_content.organization_id) {
        (Some(id), None) => Ok(TranslationScope::Conversation(id)),
        (None, Some(id)) => Ok(TranslationScope::Organization(id)),
        (None, None) => Ok(TranslationScope::System),
        (Some(_), Some(_)) => Err(ComhairleError::BadRequest(
            "TextContent has an invalid resource scope".into(),
        )),
    }
}

async fn authorize_text_content_record(
    state: &Arc<ComhairleState>,
    text_content: &translations::TextContent,
    user_id: &uuid::Uuid,
    access: TextContentAccess,
) -> Result<(), ComhairleError> {
    authorize_scope(state, text_content_scope(text_content)?, user_id, access).await
}

async fn authorize_locale(
    state: &Arc<ComhairleState>,
    content_id: &TextContentId,
    locale: &str,
    user_id: &uuid::Uuid,
) -> Result<translations::TextContent, ComhairleError> {
    let content =
        authorize_text_content(state, content_id, user_id, TextContentAccess::Read).await?;
    authorize_locale_record(state, &content, locale, user_id).await?;
    Ok(content)
}

async fn authorize_locale_record(
    state: &Arc<ComhairleState>,
    content: &translations::TextContent,
    locale: &str,
    user_id: &uuid::Uuid,
) -> Result<(), ComhairleError> {
    let access = if locale == content.primary_locale {
        TextContentAccess::UpdateSource
    } else {
        TextContentAccess::TranslateLocale
    };
    authorize_scope(state, text_content_scope(content)?, user_id, access).await
}

/// Get a TextContent with all its translations
#[instrument(err(Debug), skip(state))]
async fn get_text_content_with_translations(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path(text_content_id): Path<TextContentId>,
) -> Result<(StatusCode, Json<TextContentWithTranslations>), ComhairleError> {
    let text_content =
        authorize_text_content(&state, &text_content_id, &user.id, TextContentAccess::Read)
            .await?
            .into();
    let translations =
        translations::get_text_translations_by_content_id(&state.db, &text_content_id)
            .await?
            .into_iter()
            .map(Into::into)
            .collect();

    Ok((
        StatusCode::OK,
        Json(TextContentWithTranslations {
            text_content,
            translations,
        }),
    ))
}

/// Create new TextContent
#[instrument(err(Debug), skip(state))]
async fn create_text_content(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Json(request): Json<CreateTextContentRequest>,
) -> Result<(StatusCode, Json<TextContentDto>), ComhairleError> {
    authorize_scope(
        &state,
        request.scope,
        &user.id,
        TextContentAccess::UpdateSource,
    )
    .await?;
    let text_content = translations::new_translation(
        &mut *state.db.acquire().await?,
        request.scope,
        &request.primary_locale,
        &request.content,
        request.format,
    )
    .await?
    .into();
    Ok((StatusCode::CREATED, Json(text_content)))
}

/// Update TextContent
#[instrument(err(Debug), skip(state))]
async fn update_text_content(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path(text_content_id): Path<TextContentId>,
    Json(update): Json<UpdateTextContent>,
) -> Result<(StatusCode, Json<TextContentDto>), ComhairleError> {
    let mut transaction = state.db.begin().await?;
    let existing =
        translations::get_text_content_by_id_for_update(&mut transaction, &text_content_id).await?;
    authorize_text_content_record(&state, &existing, &user.id, TextContentAccess::UpdateSource)
        .await?;
    let text_content =
        translations::update_text_content(&mut *transaction, &text_content_id, &update)
            .await?
            .into();
    transaction.commit().await?;

    Ok((StatusCode::OK, Json(text_content)))
}

/// Delete TextContent and all its translations
#[instrument(err(Debug), skip(state))]
async fn delete_text_content(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path(text_content_id): Path<TextContentId>,
) -> Result<(StatusCode, Json<TextContentDto>), ComhairleError> {
    authorize_text_content(
        &state,
        &text_content_id,
        &user.id,
        TextContentAccess::UpdateSource,
    )
    .await?;
    let text_content = translations::delete_text_content(&state.db, &text_content_id)
        .await?
        .into();

    Ok((StatusCode::OK, Json(text_content)))
}

/// Get a specific translation for a TextContent and locale
#[instrument(err(Debug), skip(state))]
async fn get_text_translation(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path((text_content_id, locale)): Path<(TextContentId, String)>,
) -> Result<(StatusCode, Json<TextTranslationDto>), ComhairleError> {
    authorize_text_content(&state, &text_content_id, &user.id, TextContentAccess::Read).await?;
    let translation = translations::get_text_translation_by_content_and_locale(
        &state.db,
        &text_content_id,
        &locale,
    )
    .await?
    .into();

    Ok((StatusCode::OK, Json(translation)))
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct CreateOrUpdateTextTranslationRequest {
    pub content: String,
    /// Whether this translation was generated by AI (defaults to false)
    pub ai_generated: Option<bool>,
    /// Whether this translation requires human validation (defaults to false)
    pub requires_validation: Option<bool>,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct TranslateTextRequest {
    pub scope: TranslationScope,
    pub source_content: String,
    pub source_locale: String,
    pub target_locale: String,
}

#[derive(Serialize, JsonSchema, Debug)]
pub struct TranslatedText {
    pub content: String,
    pub requires_validation: bool,
}

async fn translate_text(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Json(request): Json<TranslateTextRequest>,
) -> Result<Json<TranslatedText>, ComhairleError> {
    authorize_scope(
        &state,
        request.scope,
        &user.id,
        TextContentAccess::TranslateLocale,
    )
    .await?;
    let translator = state
        .translation_service
        .as_ref()
        .ok_or(ComhairleError::NoTranslationServiceConfigured)?;
    let content = translator
        .translate_from_to(
            &request.source_content,
            &request.source_locale,
            &request.target_locale,
        )
        .await?;
    Ok(Json(TranslatedText {
        content,
        requires_validation: true,
    }))
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct TranslateTextContentSourceRequest {
    pub source_content: String,
    pub source_locale: String,
}

async fn translate_text_content_source(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path((text_content_id, locale)): Path<(TextContentId, String)>,
    Json(request): Json<TranslateTextContentSourceRequest>,
) -> Result<Json<TranslatedText>, ComhairleError> {
    authorize_locale(&state, &text_content_id, &locale, &user.id).await?;

    let translator = state
        .translation_service
        .as_ref()
        .ok_or(ComhairleError::NoTranslationServiceConfigured)?;
    let content = translator
        .translate_from_to(&request.source_content, &request.source_locale, &locale)
        .await?;
    Ok(Json(TranslatedText {
        content,
        requires_validation: true,
    }))
}

/// Create or update a translation for a specific TextContent and locale
#[instrument(err(Debug), skip(state))]
async fn create_or_update_text_translation(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path((text_content_id, locale)): Path<(TextContentId, String)>,
    Json(request): Json<CreateOrUpdateTextTranslationRequest>,
) -> Result<(StatusCode, Json<TextTranslationDto>), ComhairleError> {
    let mut transaction = state.db.begin().await?;
    let text_content =
        translations::get_text_content_by_id_for_update(&mut transaction, &text_content_id).await?;
    authorize_text_content_record(&state, &text_content, &user.id, TextContentAccess::Read).await?;
    authorize_locale_record(&state, &text_content, &locale, &user.id).await?;
    // Check if translation already exists
    match translations::get_text_translation_by_content_and_locale(
        &mut *transaction,
        &text_content_id,
        &locale,
    )
    .await
    {
        Ok(existing_translation) => {
            // Update existing translation
            let update = UpdateTextTranslation {
                locale: Some(locale.clone()),
                content: Some(request.content),
                ai_generated: request.ai_generated,
                requires_validation: request.requires_validation,
            };
            let updated_translation = translations::update_text_translation(
                &mut *transaction,
                &existing_translation.id,
                &update,
            )
            .await?;
            transaction.commit().await?;
            Ok((StatusCode::OK, Json(updated_translation.into())))
        }
        Err(ComhairleError::ResourceNotFound(_)) => {
            // Create new translation
            let create_request = CreateTextTranslation {
                content_id: text_content_id,
                locale,
                content: request.content,
                ai_generated: request.ai_generated,
                requires_validation: request.requires_validation,
            };
            let translation =
                translations::create_text_translation(&mut *transaction, &create_request).await?;
            transaction.commit().await?;
            Ok((StatusCode::CREATED, Json(translation.into())))
        }
        Err(error) => Err(error),
    }
}

/// Update a specific translation
#[instrument(err(Debug), skip(state))]
async fn update_text_translation(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path((text_content_id, locale)): Path<(TextContentId, String)>,
    Json(update): Json<UpdateTextTranslation>,
) -> Result<(StatusCode, Json<TextTranslationDto>), ComhairleError> {
    let mut transaction = state.db.begin().await?;
    let text_content =
        translations::get_text_content_by_id_for_update(&mut transaction, &text_content_id).await?;
    authorize_text_content_record(&state, &text_content, &user.id, TextContentAccess::Read).await?;
    authorize_locale_record(&state, &text_content, &locale, &user.id).await?;
    if let Some(new_locale) = update.locale.as_deref()
        && new_locale != locale
    {
        authorize_locale_record(&state, &text_content, new_locale, &user.id).await?;
    }

    // First get the existing translation to get its ID
    let existing_translation = translations::get_text_translation_by_content_and_locale(
        &mut *transaction,
        &text_content_id,
        &locale,
    )
    .await?;

    let updated_translation =
        translations::update_text_translation(&mut *transaction, &existing_translation.id, &update)
            .await?;
    transaction.commit().await?;

    Ok((StatusCode::OK, Json(updated_translation.into())))
}

// Automatically translate a specific translation
#[instrument(err(Debug), skip(state))]
async fn auto_translate(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path((text_content_id, locale)): Path<(TextContentId, String)>,
) -> Result<(StatusCode, Json<TextTranslationDto>), ComhairleError> {
    let mut transaction = state.db.begin().await?;
    let text_content =
        translations::get_text_content_by_id_for_update(&mut transaction, &text_content_id).await?;
    authorize_text_content_record(&state, &text_content, &user.id, TextContentAccess::Read).await?;
    authorize_locale_record(&state, &text_content, &locale, &user.id).await?;
    if let Some(translation_service) = &state.translation_service {
        let new_translation = translations::auto_generate_translation_in_connection(
            &mut transaction,
            translation_service,
            &text_content_id,
            &locale,
        )
        .await?
        .into();
        transaction.commit().await?;
        Ok((StatusCode::OK, Json(new_translation)))
    } else {
        Err(ComhairleError::NoTranslationServiceConfigured)
    }
}

// Automatically translate all languages for this text content
#[instrument(err(Debug), skip(state))]
async fn auto_translate_all(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path(text_content_id): Path<TextContentId>,
) -> Result<(StatusCode, Json<TextContentWithTranslations>), ComhairleError> {
    let mut transaction = state.db.begin().await?;
    let content =
        translations::get_text_content_by_id_for_update(&mut transaction, &text_content_id).await?;
    authorize_text_content_record(&state, &content, &user.id, TextContentAccess::Read).await?;
    authorize_text_content_record(
        &state,
        &content,
        &user.id,
        TextContentAccess::TranslateLocale,
    )
    .await?;
    if let Some(translation_service) = &state.translation_service {
        let text_content = content.into();
        let translations = translations::auto_generate_all_translations_in_connection(
            &mut transaction,
            translation_service,
            &text_content_id,
        )
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
        let result = TextContentWithTranslations {
            text_content,
            translations,
        };
        transaction.commit().await?;
        Ok((StatusCode::OK, Json(result)))
    } else {
        Err(ComhairleError::NoTranslationServiceConfigured)
    }
}

/// Delete a specific translation
#[instrument(err(Debug), skip(state))]
async fn delete_text_translation(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Path((text_content_id, locale)): Path<(TextContentId, String)>,
) -> Result<(StatusCode, Json<TextTranslationDto>), ComhairleError> {
    let mut transaction = state.db.begin().await?;
    let text_content =
        translations::get_text_content_by_id_for_update(&mut transaction, &text_content_id).await?;
    authorize_text_content_record(&state, &text_content, &user.id, TextContentAccess::Read).await?;
    authorize_locale_record(&state, &text_content, &locale, &user.id).await?;
    // First get the existing translation to get its ID
    let existing_translation = translations::get_text_translation_by_content_and_locale(
        &mut *transaction,
        &text_content_id,
        &locale,
    )
    .await?;

    let deleted_translation =
        translations::delete_text_translation(&mut *transaction, &existing_translation.id)
            .await?
            .into();
    transaction.commit().await?;

    Ok((StatusCode::OK, Json(deleted_translation)))
}

pub fn router() -> ApiRouter<Arc<ComhairleState>> {
    ApiRouter::new()
        // TextContent routes
        .api_route(
            "/",
            post_with(create_text_content, |op| {
                op.id("CreateTextContent")
                    .tag("Translations")
                    .summary("Create new TextContent")
                    .description("Create a new TextContent entry that can hold translations")
                    .response::<201, Json<TextContentDto>>()
            }),
        )
        .api_route(
            "/{text_content_id}",
            get_with(get_text_content_with_translations, |op| {
                op.id("GetTextContentWithTranslations")
                    .tag("Translations")
                    .summary("Get TextContent with all translations")
                    .description("Get a TextContent entry with all its translations")
                    .response::<200, Json<TextContentWithTranslations>>()
            }),
        )
        .api_route(
            "/{text_content_id}",
            put_with(update_text_content, |op| {
                op.id("UpdateTextContent")
                    .tag("Translations")
                    .summary("Update TextContent")
                    .description("Update a TextContent entry")
                    .response::<200, Json<TextContentDto>>()
            }),
        )
        .api_route(
            "/{text_content_id}",
            delete_with(delete_text_content, |op| {
                op.id("DeleteTextContent")
                    .tag("Translations")
                    .summary("Delete TextContent")
                    .description("Delete a TextContent entry and all its translations")
                    .response::<200, Json<TextContentDto>>()
            }),
        )
        // TextTranslation routes
        .api_route(
            "/{text_content_id}/{locale}",
            get_with(get_text_translation, |op| {
                op.id("GetTextTranslation")
                    .tag("Translations")
                    .summary("Get translation for specific locale")
                    .description("Get a translation for a specific TextContent and locale")
                    .response::<200, Json<TextTranslationDto>>()
            }),
        )
        .api_route(
            "/{text_content_id}/{locale}",
            post_with(create_or_update_text_translation, |op| {
                op.id("CreateOrUpdateTextTranslation")
                    .tag("Translations")
                    .summary("Create or update translation")
                    .description(
                        "Create a new translation or update existing one for a specific locale",
                    )
                    .response::<200, Json<TextTranslationDto>>()
                    .response::<201, Json<TextTranslationDto>>()
            }),
        )
        .api_route(
            "/{text_content_id}/{locale}",
            put_with(update_text_translation, |op| {
                op.id("UpdateTextTranslation")
                    .tag("Translations")
                    .summary("Update translation")
                    .description("Update an existing translation for a specific locale")
                    .response::<200, Json<TextTranslationDto>>()
            }),
        )
        .api_route(
            "/{text_content_id}/{locale}",
            delete_with(delete_text_translation, |op| {
                op.id("DeleteTextTranslation")
                    .tag("Translations")
                    .summary("Delete translation")
                    .description("Delete a translation for a specific locale")
                    .response::<200, Json<TextTranslationDto>>()
            }),
        )
        .api_route(
            "/{text_content_id}/translate",
            post_with(auto_translate_all, |op| {
                op.id("GenerateAllTranslations")
                    .tag("Translations")
                    .summary("Generate all translations for this Text Content")
                    .description("Use the default locale content as the reference text and generate automatic translations for each language form it")
                    .response::<200, Json<TextContentWithTranslations>>()
            }),
        )
        .api_route(
            "/{text_content_id}/{locale}/translate",
            post_with(auto_translate, |op| {
                op.id("AutomaticallyGenerateTranslation")
                    .tag("Translations")
                    .summary("Automatically generate this language")
                    .description("Use the primary_locale language and translate this language from it using the tarnslation service")
                    .response::<200, Json<TextTranslationDto>>()
            }),
        )
        .api_route(
            "/{text_content_id}/{locale}/translate-source",
            post_with(translate_text_content_source, |op| {
                op.id("TranslateTextContentSource")
                    .tag("Translations")
                    .summary("Translate supplied source text into a target locale")
                    .response::<200, Json<TranslatedText>>()
            }),
        )
        .api_route(
            "/translate",
            post_with(translate_text, |op| {
                op.id("TranslateText")
                    .tag("Translations")
                    .summary("Translate text under a resource scope")
                    .response::<200, Json<TranslatedText>>()
            }),
        )
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::sync::Arc;

    use axum::body::Body;
    use axum::http::StatusCode;
    use mockall::predicate;
    use serde_json::json;

    use super::*;
    use crate::models::permissions::{
        self, PermissionRole, conversation as conversation_permissions,
    };
    use crate::models::translations::TextFormat;
    use crate::setup_server;
    use crate::test_helpers::{UserSession, extract, test_config, test_state};
    use crate::translation_service::MockTranslationService;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn system_administrators_can_translate_system_content(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let mut service = MockTranslationService::new();
        service
            .expect_translate_from_to()
            .times(4)
            .returning(|_, _, _| Ok("Automatic translation".into()));
        let state = Arc::new(
            test_state()
                .db(pool)
                .translation_service(Arc::new(service))
                .call()?,
        );
        let app = setup_server(state.clone()).await?;
        let mut admin = UserSession::new_admin();
        admin.signup(&app).await?;
        let mut translator = UserSession::new_guest();
        translator.signup_guest(&app).await?;
        let user_id = translator.id.unwrap();
        permissions::grant_role(
            &state,
            permissions::GrantRoleRequest {
                actor_id: permissions::ActorId::User(user_id),
                granted_by: &admin.id.unwrap(),
                grant_reason: "System translation test",
                permission_triplet: system::Role::Admin.system_triplet()?,
            },
        )
        .await?;

        let mut last_content_id = None;
        for source in ["First source", "Second source"] {
            let content = translations::new_translation(
                &mut *state.db.acquire().await?,
                TranslationScope::System,
                "en",
                source,
                TextFormat::Rich,
            )
            .await?;
            let path = format!("/translations/{}", content.id);
            let (status, _, _) = translator.get(&app, &path).await?;
            assert_eq!(status, StatusCode::OK);
            let (status, _, _) = translator
                .post(
                    &app,
                    &format!("{path}/fr"),
                    json!({ "content": "French translation" })
                        .to_string()
                        .into(),
                )
                .await?;
            assert_eq!(status, StatusCode::CREATED);
            let (status, _, _) = translator
                .put(
                    &app,
                    &format!("{path}/fr"),
                    json!({ "locale": "fr", "content": "Updated translation" })
                        .to_string()
                        .into(),
                )
                .await?;
            assert_eq!(status, StatusCode::OK);
            let (status, _, _) = translator
                .post(&app, &format!("{path}/fr/translate"), Body::empty())
                .await?;
            assert_eq!(status, StatusCode::OK);
            let (status, _, _) = translator
                .post(&app, &format!("{path}/translate"), Body::empty())
                .await?;
            assert_eq!(status, StatusCode::OK);
            let (status, response, _) = translator.get(&app, &format!("{path}/fr")).await?;
            assert_eq!(status, StatusCode::OK);
            let translated: TextTranslationDto = serde_json::from_value(response)?;
            assert_eq!(translated.content, "Automatic translation");
            let primary = translations::get_text_translation_by_content_and_locale(
                &state.db,
                &content.id,
                "en",
            )
            .await?;
            assert_eq!(primary.content, source);
            let (status, _, _) = translator.delete(&app, &format!("{path}/fr")).await?;
            assert_eq!(status, StatusCode::OK);
            last_content_id = Some(content.id);
        }
        permissions::revoke_role(
            &state,
            permissions::RevokeRoleRequest {
                actor_id: permissions::ActorId::User(user_id),
                permission_triplet: system::Role::Admin.system_triplet()?,
            },
        )
        .await?;
        let (status, _, _) = translator
            .get(&app, &format!("/translations/{}", last_content_id.unwrap()))
            .await?;
        assert_eq!(status, StatusCode::FORBIDDEN);
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn system_administrators_can_edit_system_content(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let mut service = MockTranslationService::new();
        service
            .expect_translate_from_to()
            .once()
            .returning(|_, _, _| Ok("Automatic source".into()));
        let state = Arc::new(
            test_state()
                .db(pool)
                .translation_service(Arc::new(service))
                .call()?,
        );
        let app = setup_server(state.clone()).await?;
        let mut admin = UserSession::new_admin();
        admin.signup(&app).await?;
        let mut translator = UserSession::new_guest();
        translator.signup_guest(&app).await?;
        let content = translations::new_translation(
            &mut *state.db.acquire().await?,
            TranslationScope::System,
            "en",
            "Source",
            TextFormat::Rich,
        )
        .await?;
        let path = format!("/translations/{}", content.id);
        let (status, _, _) = translator.get(&app, &path).await?;
        assert_eq!(status, StatusCode::FORBIDDEN);
        permissions::grant_role(
            &state,
            permissions::GrantRoleRequest {
                actor_id: permissions::ActorId::User(translator.id.unwrap()),
                granted_by: &admin.id.unwrap(),
                grant_reason: "System translation boundary test",
                permission_triplet: system::Role::Admin.system_triplet()?,
            },
        )
        .await?;
        let (status, response, _) = translator
            .post(
                &app,
                "/translations",
                json!({
                    "primary_locale": "en",
                    "format": "plain",
                    "content": "New content",
                    "scope": { "type": "system" }
                })
                .to_string()
                .into(),
            )
            .await?;
        assert_eq!(status, StatusCode::CREATED);
        let created: TextContentDto = serde_json::from_value(response)?;
        let created_path = format!("/translations/{}", created.id);
        let (status, response, _) = translator
            .put(
                &app,
                &created_path,
                json!({ "primary_locale": "fr", "format": "markdown" })
                    .to_string()
                    .into(),
            )
            .await?;
        assert_eq!(status, StatusCode::OK);
        let updated: TextContentDto = serde_json::from_value(response)?;
        assert_eq!(updated.primary_locale, "fr");
        assert_eq!(updated.format, TextFormat::Markdown);
        let (status, _, _) = translator.delete(&app, &created_path).await?;
        assert_eq!(status, StatusCode::OK);
        let (status, _, _) = translator.get(&app, &created_path).await?;
        assert_eq!(status, StatusCode::NOT_FOUND);
        let primary_path = format!("{path}/en");
        let (status, _, _) = translator
            .post(
                &app,
                &primary_path,
                json!({ "content": "Changed source" }).to_string().into(),
            )
            .await?;
        assert_eq!(status, StatusCode::OK);
        let (status, _, _) = translator
            .put(
                &app,
                &primary_path,
                json!({ "content": "Updated source" }).to_string().into(),
            )
            .await?;
        assert_eq!(status, StatusCode::OK);
        let primary =
            translations::get_text_translation_by_content_and_locale(&state.db, &content.id, "en")
                .await?;
        assert_eq!(primary.content, "Updated source");
        let (status, _, _) = translator
            .post(&app, &format!("{primary_path}/translate"), Body::empty())
            .await?;
        assert_eq!(status, StatusCode::OK);
        let (status, _, _) = translator
            .post(
                &app,
                &format!("{path}/fr"),
                json!({ "content": "French translation" })
                    .to_string()
                    .into(),
            )
            .await?;
        assert_eq!(status, StatusCode::CREATED);
        let (status, _, _) = translator
            .put(
                &app,
                &format!("{path}/fr"),
                json!({ "locale": "de", "content": "Renamed translation" })
                    .to_string()
                    .into(),
            )
            .await?;
        assert_eq!(status, StatusCode::OK);
        let renamed =
            translations::get_text_translation_by_content_and_locale(&state.db, &content.id, "de")
                .await?;
        assert_eq!(renamed.content, "Renamed translation");
        let primary =
            translations::get_text_translation_by_content_and_locale(&state.db, &content.id, "en")
                .await?;
        assert_eq!(primary.content, "Automatic source");
        let (status, _, _) = translator.delete(&app, &primary_path).await?;
        assert_eq!(status, StatusCode::OK);
        let (status, _, _) = translator
            .post(
                &app,
                &primary_path,
                json!({ "content": "Recreated source" }).to_string().into(),
            )
            .await?;
        assert_eq!(status, StatusCode::CREATED);
        for format in [TextFormat::Plain, TextFormat::Markdown] {
            let content = translations::new_translation(
                &mut *state.db.acquire().await?,
                TranslationScope::System,
                "en",
                "Non-rich text",
                format,
            )
            .await?;
            let path = format!("/translations/{}", content.id);
            let (status, _, _) = translator
                .post(
                    &app,
                    &format!("{path}/fr"),
                    json!({ "content": "Translated text" }).to_string().into(),
                )
                .await?;
            assert_eq!(status, StatusCode::CREATED);
            let (status, _, _) = translator
                .put(
                    &app,
                    &format!("{path}/en"),
                    json!({ "content": "Updated non-rich source" })
                        .to_string()
                        .into(),
                )
                .await?;
            assert_eq!(status, StatusCode::OK);
            let primary = translations::get_text_translation_by_content_and_locale(
                &mut *state.db.acquire().await?,
                &content.id,
                "en",
            )
            .await?;
            assert_eq!(primary.content, "Updated non-rich source");
        }
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn conversation_translators_cannot_edit_primary_content(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let mut config = test_config()?;
        config.bot_service = None;
        let state = Arc::new(test_state().db(pool).config(config).call()?);
        let app = setup_server(state.clone()).await?;

        let mut admin = UserSession::new_admin();
        admin.signup(&app).await?;
        let (status, response, _) = admin
            .create_conversation(
                &app,
                json!({
                    "title": "Translation permissions",
                    "short_description": "Test",
                    "description": "Test conversation",
                    "tags": [],
                    "is_public": false,
                    "is_live": false,
                    "is_invite_only": false,
                    "slug": "translation-permissions",
                    "primary_locale": "en",
                    "supported_languages": ["en", "fr"]
                }),
            )
            .await?;
        assert_eq!(status, StatusCode::CREATED);
        let conversation: crate::routes::conversations::dto::ConversationDto =
            serde_json::from_value(response)?;
        let content = translations::new_translation(
            &mut *state.db.acquire().await?,
            TranslationScope::Conversation(conversation.id),
            "en",
            "Conversation source",
            TextFormat::Plain,
        )
        .await?;
        let path = format!("/translations/{}", content.id);

        let mut system_admin = UserSession::new_guest();
        system_admin.signup_guest(&app).await?;
        permissions::grant_role(
            &state,
            permissions::GrantRoleRequest {
                actor_id: permissions::ActorId::User(system_admin.id.unwrap()),
                granted_by: &admin.id.unwrap(),
                grant_reason: "System translation scope test",
                permission_triplet: system::Role::Admin.system_triplet()?,
            },
        )
        .await?;
        let (status, _, _) = system_admin.get(&app, &path).await?;
        assert_eq!(status, StatusCode::FORBIDDEN);

        let mut translator = UserSession::new_guest();
        translator.signup_guest(&app).await?;
        permissions::grant_role(
            &state,
            permissions::GrantRoleRequest {
                actor_id: permissions::ActorId::User(translator.id.unwrap()),
                granted_by: &admin.id.unwrap(),
                grant_reason: "Conversation translation test",
                permission_triplet: conversation_permissions::Role::Translator
                    .triplet(&conversation.id)?,
            },
        )
        .await?;
        let (status, _, _) = translator.get(&app, &path).await?;
        assert_eq!(status, StatusCode::OK);
        let (status, _, _) = translator
            .post(
                &app,
                &format!("{path}/fr"),
                json!({ "content": "Traduction" }).to_string().into(),
            )
            .await?;
        assert_eq!(status, StatusCode::CREATED);
        let (status, _, _) = translator
            .post(
                &app,
                &format!("{path}/en"),
                json!({ "content": "Changed source" }).to_string().into(),
            )
            .await?;
        assert_eq!(status, StatusCode::FORBIDDEN);
        let (status, _, _) = translator
            .post(&app, &format!("{path}/en/translate"), Body::empty())
            .await?;
        assert_eq!(status, StatusCode::FORBIDDEN);
        let (status, _, _) = translator
            .post(
                &app,
                &format!("{path}/en/translate-source"),
                json!({ "source_content": "Source", "source_locale": "fr" })
                    .to_string()
                    .into(),
            )
            .await?;
        assert_eq!(status, StatusCode::FORBIDDEN);
        let (status, _, _) = translator.delete(&app, &format!("{path}/en")).await?;
        assert_eq!(status, StatusCode::FORBIDDEN);
        let (status, _, _) = translator
            .put(
                &app,
                &path,
                json!({ "primary_locale": "fr" }).to_string().into(),
            )
            .await?;
        assert_eq!(status, StatusCode::FORBIDDEN);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_create_text_content(pool: sqlx::PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        // Create TextContent
        let create_request = json!({
            "primary_locale": "en",
            "format": "plain",
            "scope": {"type": "system"},
            "content": "Original Translation"
        });

        let (status, response, _) = admin_session
            .post(&app, "/translations", create_request.to_string().into())
            .await?;

        assert_eq!(status, StatusCode::CREATED);
        let text_content: TextContentDto = serde_json::from_value(response)?;
        assert_eq!(text_content.primary_locale, "en");
        assert_eq!(text_content.format, TextFormat::Plain);

        let (status, response, _) = admin_session
            .get(&app, &format!("/translations/{}/en", text_content.id))
            .await?;

        let translation: TextTranslationDto = serde_json::from_value(response)?;

        assert_eq!(
            status,
            StatusCode::OK,
            "should have created primary translation"
        );
        assert_eq!(
            translation.content, "Original Translation",
            "should have correct content"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_get_text_content_with_translations(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        // Create TextContent
        let create_request = json!({
            "primary_locale": "en",
            "format": "rich",
            "scope": {"type": "system"},
            "content": "Hello World"
        });

        let (_status, response, _) = admin_session
            .post(&app, "/translations", create_request.to_string().into())
            .await?;

        let text_content: TextContentDto = serde_json::from_value(response)?;

        // Add a translation
        let translation_request = json!({
            "content_id": text_content.id,
            "content": "Bonjour le monde",
            "ai_generated": false,
            "requires_validation": false
        });

        admin_session
            .post(
                &app,
                &format!("/translations/{}/fr", text_content.id),
                translation_request.to_string().into(),
            )
            .await?;

        // Get TextContent with translations
        let (status, response, _) = admin_session
            .get(&app, &format!("/translations/{}", text_content.id))
            .await?;

        assert_eq!(status, StatusCode::OK);

        let result: TextContentWithTranslations = serde_json::from_value(response)?;
        assert_eq!(result.text_content.id, text_content.id);
        assert_eq!(result.translations.len(), 2);
        assert_eq!(result.translations[0].content, "Hello World");
        assert_eq!(result.translations[0].locale, "en");
        assert_eq!(result.translations[1].content, "Bonjour le monde");
        assert_eq!(result.translations[1].locale, "fr");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_update_text_content(pool: sqlx::PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        // Create TextContent
        let create_request = json!({
            "primary_locale": "en",
            "format": "plain",
            "scope": {"type": "system"},
            "content": "Original Translation"
        });

        let (_, response, _) = admin_session
            .post(&app, "/translations", create_request.to_string().into())
            .await?;

        let text_content: TextContentDto = serde_json::from_value(response)?;

        // Update TextContent
        let update_request = json!({
            "primary_locale": "es",
            "format": "rich",
            "content": "Original Translation"
        });

        let (status, response, _) = admin_session
            .put(
                &app,
                &format!("/translations/{}", text_content.id),
                update_request.to_string().into(),
            )
            .await?;

        assert_eq!(status, StatusCode::OK);

        let updated_content: TextContentDto = serde_json::from_value(response)?;
        assert_eq!(updated_content.primary_locale, "es");
        assert_eq!(updated_content.format, TextFormat::Rich);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_delete_text_content(pool: sqlx::PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        // Create TextContent
        let create_request = json!({
            "primary_locale": "en",
            "format": "plain",
            "scope": {"type": "system"},
            "content": "Original Translation"
        });

        let (_status, response, _) = admin_session
            .post(&app, "/translations", create_request.to_string().into())
            .await?;

        let text_content: TextContentDto = serde_json::from_value(response)?;

        // Delete TextContent
        let (status, response, _) = admin_session
            .delete(&app, &format!("/translations/{}", text_content.id))
            .await?;

        assert_eq!(status, StatusCode::OK);

        let deleted_content: TextContentDto = serde_json::from_value(response)?;
        assert_eq!(deleted_content.id, text_content.id);

        // Verify it's deleted
        let (status, _response, _) = admin_session
            .get(&app, &format!("/translations/{}", text_content.id))
            .await?;

        assert_eq!(status, StatusCode::NOT_FOUND);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_create_text_translation(pool: sqlx::PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        // Create TextContent first
        let create_request = json!({
            "primary_locale": "en",
            "format": "plain",
            "scope": {"type": "system"},
            "content": "Original Translation"
        });

        let (_, response, _) = admin_session
            .post(&app, "/translations", create_request.to_string().into())
            .await?;

        let text_content: TextContentDto = serde_json::from_value(response)?;

        // Create translation
        let translation_request = json!({
            "content": "Hola Mundo",
            "ai_generated": true,
            "requires_validation": true
        });

        let (status, response, _) = admin_session
            .post(
                &app,
                &format!("/translations/{}/es", text_content.id),
                translation_request.to_string().into(),
            )
            .await?;

        assert_eq!(status, StatusCode::CREATED);

        let translation: TextTranslationDto = serde_json::from_value(response)?;
        assert_eq!(translation.content, "Hola Mundo");
        assert_eq!(translation.locale, "es");
        assert!(translation.ai_generated);
        assert!(translation.requires_validation);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_auto_generate_translation(pool: sqlx::PgPool) -> Result<(), Box<dyn Error>> {
        let mut mock_translation_service = MockTranslationService::new();

        mock_translation_service
            .expect_translate_from_to()
            .with(
                predicate::eq("Original Translation"),
                predicate::eq("en"),
                predicate::eq("fr"),
            )
            .once()
            .returning(|text, from, to| Ok(format!("translated: {text} from {from} to {to}")));

        let state = test_state()
            .db(pool)
            .translation_service(Arc::new(mock_translation_service))
            .call()?;

        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        // Create TextContent and translation
        let create_request = json!({
            "primary_locale": "en",
            "format": "plain",
            "scope": {"type": "system"},
            "content": "Original Translation"
        });

        let (_, response, _) = admin_session
            .post(&app, "/translations", create_request.to_string().into())
            .await?;

        let text_content: TextContentDto = serde_json::from_value(response)?;

        let translation_request = json!({
            "content": "empty translation",
            "ai_generated": false,
            "requires_validation": false
        });

        let (_, _, _) = admin_session
            .post(
                &app,
                &format!("/translations/{}/fr", text_content.id),
                translation_request.to_string().into(),
            )
            .await?;

        let (status, _, _) = admin_session
            .post(
                &app,
                &format!("/translations/{}/fr/translate", text_content.id),
                Body::empty(),
            )
            .await?;

        assert_eq!(status, StatusCode::OK, "should successfully translate");

        // Get specific translation
        let (_, response, _) = admin_session
            .get(&app, &format!("/translations/{}/fr", text_content.id))
            .await?;

        let response_text: String = extract("content", &response);
        assert_eq!(
            response_text, "translated: Original Translation from en to fr",
            "should successfully translate"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_auto_generate_all_translations(pool: sqlx::PgPool) -> Result<(), Box<dyn Error>> {
        let mut mock_translation_service = MockTranslationService::new();

        mock_translation_service
            .expect_translate_from_to()
            .with(
                predicate::eq("Original Translation"),
                predicate::eq("en"),
                predicate::eq("fr"),
            )
            .once()
            .returning(|text, from, to| Ok(format!("translated: {text} from {from} to {to}")));

        mock_translation_service
            .expect_translate_from_to()
            .with(
                predicate::eq("Original Translation"),
                predicate::eq("en"),
                predicate::eq("gr"),
            )
            .once()
            .returning(|text, from, to| Ok(format!("translated: {text} from {from} to {to}")));

        let state = test_state()
            .db(pool)
            .translation_service(Arc::new(mock_translation_service))
            .call()?;

        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        // Create TextContent and translation
        let create_request = json!({
            "primary_locale": "en",
            "format": "plain",
            "scope": {"type": "system"},
            "content": "Original Translation"
        });

        let (_, response, _) = admin_session
            .post(&app, "/translations", create_request.to_string().into())
            .await?;

        let text_content: TextContentDto = serde_json::from_value(response)?;

        let translation_request = json!({
            "content": "empty french translation",
            "ai_generated": false,
            "requires_validation": false
        });

        admin_session
            .post(
                &app,
                &format!("/translations/{}/fr", text_content.id),
                translation_request.to_string().into(),
            )
            .await?;

        let translation_request = json!({
            "content": "empty galeic translation",
            "ai_generated": false,
            "requires_validation": false
        });

        admin_session
            .post(
                &app,
                &format!("/translations/{}/gr", text_content.id),
                translation_request.to_string().into(),
            )
            .await?;

        let (status, _, _) = admin_session
            .post(
                &app,
                &format!("/translations/{}/translate", text_content.id),
                Body::empty(),
            )
            .await?;

        assert_eq!(status, StatusCode::OK, "should successfully translate");

        // Get specific translation
        let (_, response, _) = admin_session
            .get(&app, &format!("/translations/{}/fr", text_content.id))
            .await?;

        let response_text: String = extract("content", &response);
        assert_eq!(
            response_text,
            "translated: Original Translation from en to fr".to_string(),
            "should successfully translate"
        );

        // Get specific translation
        let (_, response, _) = admin_session
            .get(&app, &format!("/translations/{}/gr", text_content.id))
            .await?;

        let response_text: String = extract("content", &response);
        assert_eq!(
            response_text,
            "translated: Original Translation from en to gr".to_string(),
            "should successfully translate"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_get_text_translation(pool: sqlx::PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        // Create TextContent and translation
        let create_request = json!({
            "primary_locale": "en",
            "format": "plain",
            "scope": {"type": "system"},
            "content": "Original Text"
        });

        let (_, response, _) = admin_session
            .post(&app, "/translations", create_request.to_string().into())
            .await?;

        let text_content: TextContentDto = serde_json::from_value(response)?;

        let translation_request = json!({
            "content": "Bonjour le monde",
            "ai_generated": false,
            "requires_validation": false
        });

        admin_session
            .post(
                &app,
                &format!("/translations/{}/fr", text_content.id),
                translation_request.to_string().into(),
            )
            .await?;

        // Get specific translation
        let (status, response, _) = admin_session
            .get(&app, &format!("/translations/{}/fr", text_content.id))
            .await?;

        assert_eq!(status, StatusCode::OK);

        let translation: TextTranslationDto = serde_json::from_value(response)?;
        assert_eq!(translation.content, "Bonjour le monde");
        assert_eq!(translation.locale, "fr");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_update_existing_translation_via_post(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        // Create TextContent and translation
        let create_request = json!({
            "primary_locale": "en",
            "format": "plain",
            "scope": {"type": "system"},
            "content": "Original Translation"
        });

        let (_, response, _) = admin_session
            .post(&app, "/translations", create_request.to_string().into())
            .await?;

        let text_content: TextContentDto = serde_json::from_value(response)?;

        // Create initial translation
        let initial_translation = json!({
            "content": "Original text",
            "ai_generated": false,
            "requires_validation": false
        });

        admin_session
            .post(
                &app,
                &format!("/translations/{}/de", text_content.id),
                initial_translation.to_string().into(),
            )
            .await?;

        // Update via POST (should update existing)
        let updated_translation = json!({
            "content": "Updated text",
            "ai_generated": true,
            "requires_validation": true
        });

        let (status, response, _) = admin_session
            .post(
                &app,
                &format!("/translations/{}/de", text_content.id),
                updated_translation.to_string().into(),
            )
            .await?;

        assert_eq!(status, StatusCode::OK);

        let translation: TextTranslationDto = serde_json::from_value(response)?;
        assert_eq!(translation.content, "Updated text");
        assert!(translation.ai_generated);
        assert!(translation.requires_validation);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_update_text_translation_via_put(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        // Create TextContent and translation
        let create_request = json!({
            "primary_locale": "en",
            "format": "plain",
            "scope": {"type": "system"},
            "content": "Original Translation"
        });

        let (_, response, _) = admin_session
            .post(&app, "/translations", create_request.to_string().into())
            .await?;

        let text_content: TextContentDto = serde_json::from_value(response)?;

        // Create initial translation
        let initial_translation = json!({
            "content": "Original text",
            "ai_generated": false,
            "requires_validation": false
        });

        admin_session
            .post(
                &app,
                &format!("/translations/{}/it", text_content.id),
                initial_translation.to_string().into(),
            )
            .await?;

        // Update via PUT
        let update_request = json!({
            "content": "Testo aggiornato",
            "ai_generated": true
        });

        let (status, response, _) = admin_session
            .put(
                &app,
                &format!("/translations/{}/it", text_content.id),
                update_request.to_string().into(),
            )
            .await?;

        assert_eq!(status, StatusCode::OK);

        let translation: TextTranslationDto = serde_json::from_value(response)?;
        assert_eq!(translation.content, "Testo aggiornato");
        assert!(translation.ai_generated);
        // requires_validation should remain unchanged
        assert!(!translation.requires_validation);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_delete_text_translation(pool: sqlx::PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        // Create TextContent and translation
        let create_request = json!({
            "primary_locale": "en",
            "format": "plain",
            "scope": {"type": "system"},
            "content": "Source text"
        });

        let (_status, response, _) = admin_session
            .post(&app, "/translations", create_request.to_string().into())
            .await?;

        let text_content: TextContentDto = serde_json::from_value(response)?;

        admin_session
            .post(
                &app,
                &format!("/translations/{}/fr", text_content.id),
                json!({
                    "content": "Text to delete",
                    "ai_generated": false,
                    "requires_validation": false
                })
                .to_string()
                .into(),
            )
            .await?;

        // Delete translation
        let (status, response, _) = admin_session
            .delete(&app, &format!("/translations/{}/fr", text_content.id))
            .await?;

        assert_eq!(status, StatusCode::OK);

        let deleted_translation: TextTranslationDto = serde_json::from_value(response)?;
        assert_eq!(deleted_translation.content, "Text to delete");

        // Verify it's deleted
        let (status, _response, _) = admin_session
            .get(&app, &format!("/translations/{}/fr", text_content.id))
            .await?;

        assert_eq!(status, StatusCode::NOT_FOUND);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_non_admin_access_denied(pool: sqlx::PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut regular_session = UserSession::new("user", "password", "user@example.com");
        regular_session.signup(&app).await?;

        // Try to create TextContent as regular user
        let create_request = json!({
            "primary_locale": "en",
            "format": "plain",
            "scope": {"type": "system"},
            "content": "Original Translation"
        });

        let (status, _response, _) = regular_session
            .post(&app, "/translations", create_request.to_string().into())
            .await?;

        // Should be forbidden or unauthorized
        assert!(status == StatusCode::UNAUTHORIZED || status == StatusCode::FORBIDDEN);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_get_nonexistent_text_content(pool: sqlx::PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        let fake_uuid = uuid::Uuid::new_v4();
        let (status, _response, _) = admin_session
            .get(&app, &format!("/translations/{}", fake_uuid))
            .await?;

        assert_eq!(status, StatusCode::NOT_FOUND);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_get_nonexistent_translation(pool: sqlx::PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        // Create TextContent but no translation
        let create_request = json!({
            "primary_locale": "en",
            "format": "plain",
            "scope": {"type": "system"},
            "content": "Original Translation"
        });

        let (_status, response, _) = admin_session
            .post(&app, "/translations", create_request.to_string().into())
            .await?;

        let text_content: TextContentDto = serde_json::from_value(response)?;

        // Try to get non-existent translation
        let (status, _response, _) = admin_session
            .get(
                &app,
                &format!("/translations/{}/nonexistent", text_content.id),
            )
            .await?;

        assert_eq!(status, StatusCode::NOT_FOUND);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_update_invalid_text_content(pool: sqlx::PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        let fake_uuid = uuid::Uuid::new_v4();
        let update_request = json!({
            "primary_locale": "es"
        });

        let (status, _response, _) = admin_session
            .put(
                &app,
                &format!("/translations/{}", fake_uuid),
                update_request.to_string().into(),
            )
            .await?;

        assert_eq!(status, StatusCode::NOT_FOUND);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_empty_update_request(pool: sqlx::PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        // Create TextContent
        let create_request = json!({
            "primary_locale": "en",
            "format": "plain",
            "scope": {"type": "system"},
            "content": "Original Translation"
        });

        let (_status, response, _) = admin_session
            .post(&app, "/translations", create_request.to_string().into())
            .await?;

        let text_content: TextContentDto = serde_json::from_value(response)?;

        // Try to update with empty object
        let empty_update = json!({});

        let (status, _response, _) = admin_session
            .put(
                &app,
                &format!("/translations/{}", text_content.id),
                empty_update.to_string().into(),
            )
            .await?;

        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);

        Ok(())
    }
}
