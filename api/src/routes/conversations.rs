use std::sync::Arc;

use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
};

use aide::axum::{
    routing::{delete_with, get_with, post_with, put_with},
    ApiRouter,
};

use schemars::JsonSchema;
use serde::Deserialize;
use tracing::info;
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::{
        conversation::{
            self, Conversation, ConversationFilterOptions, ConversationOrderOptions,
            CreateConversation, PartialConversation,
        },
        pagination::{OrderParams, PageOptions, PaginatedResults},
    },
    ComhairleState,
};

use super::auth::{RequiredAdminUser, RequiredUser};

/// Create conversation handler
async fn create_conversation(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    Json(new_converastion): Json<CreateConversation>,
) -> Result<(StatusCode, Json<Conversation>), ComhairleError> {
    info!("Attempting to create conversation");
    let conversation = conversation::create(&state.db, &new_converastion, user.id).await?;
    Ok((StatusCode::CREATED, Json(conversation)))
}

/// Update conversation handler
async fn update_conversation(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    Path(id): Path<Uuid>,
    Json(conversation): Json<PartialConversation>,
) -> Result<(StatusCode, Json<Conversation>), ComhairleError> {
    let conversation = conversation::update(&state.db, id, &conversation).await?;
    Ok((StatusCode::OK, Json(conversation)))
}

/// List conversations handler
async fn list_conversations(
    State(state): State<Arc<ComhairleState>>,
    OrderParams(order_options): OrderParams<ConversationOrderOptions>,
    Query(filter_options): Query<ConversationFilterOptions>,
    Query(page_options): Query<PageOptions>,
) -> Result<(StatusCode, Json<PaginatedResults<Conversation>>), ComhairleError> {
    let conversations =
        conversation::list(&state.db, page_options, order_options, filter_options).await?;
    Ok((StatusCode::OK, Json(conversations)))
}

/// For extracting an id or slug from Path
#[derive(Deserialize, Debug, JsonSchema)]
#[serde(untagged)]
enum IdOrSlug {
    Id(Uuid),
    Slug(String),
}

/// Get a specific conversation
async fn get_conversation(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_ident): Path<IdOrSlug>,
) -> Result<(StatusCode, Json<Conversation>), ComhairleError> {
    info!("Attempting to get conversation {conversation_ident:#?}");

    let conversation = match conversation_ident {
        IdOrSlug::Id(id) => conversation::get_by_id(&state.db, &id).await?,
        IdOrSlug::Slug(slug) => conversation::get_by_slug(&state.db, &slug).await?,
    };

    Ok((StatusCode::OK, Json(conversation)))
}

/// Delete a specific conversation
async fn delete_conversation(
    State(state): State<Arc<ComhairleState>>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Conversation>), ComhairleError> {
    let conversation = conversation::delete(&state.db, &id).await?;
    Ok((StatusCode::OK, Json(conversation)))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(create_conversation, |op| {
                op.id("CreateConversation")
                    .summary("Create a new conversation")
                    .description("Creates a new conversation")
                    .response::<201, Json<Conversation>>()
            }),
        )
        .api_route(
            "/",
            get_with(list_conversations, |op| {
                op.id("ListConverastions")
                    .summary("List conversations with optional filtering and ordering")
                    .description("List conversations")
                    .response::<200, Json<PaginatedResults<Conversation>>>()
            }),
        )
        .api_route(
            "/{conversation_id}",
            get_with(get_conversation, |op| {
                op.id("GetConversation")
                    .summary("Get a conversation by id or slug")
                    .description("Get a converation by id or slug")
                    .response::<200, Json<Conversation>>()
            }),
        )
        .api_route(
            "/{conversation_id}",
            put_with(update_conversation, |op| {
                op.id("UpdateConversation")
                    .summary("Update a conversation")
                    .description("Update a conversation")
                    .response::<200, Json<Conversation>>()
            }),
        )
        .api_route(
            "/{conversation_id}",
            delete_with(delete_conversation, |op| {
                op.id("DeleteConversation")
                    .summary("Delete the conversation and all related content")
                    .description("Delete the conversation and all related content")
                    .response::<200, Json<Conversation>>()
            }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {

    use crate::test_helpers::test_config;
    use crate::{setup_server, test_helpers::UserSession};
    use axum::http::StatusCode;
    use serde_json::json;
    use sqlx::PgPool;
    use std::collections::HashMap;
    use std::error::Error;

    #[sqlx::test]
    fn should_be_able_to_create_conversation(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let config = test_config()?;

        let app = setup_server(config, pool).await?;

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
                    "slug" : "new_conversation"
                }),
            )
            .await?;

        assert_eq!(status, StatusCode::CREATED, "Should be created");

        Ok(())
    }

    #[sqlx::test]
    fn should_be_able_to_update_a_conversation(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let config = test_config()?;
        let app = setup_server(config, pool).await?;

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
                    "slug" : "new_conversation"
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
                    "short_description": "new description",
                    "is_public":true
                }),
            )
            .await?;

        assert_eq!(status, StatusCode::OK, "Should update resource");

        assert_eq!(
            conversation.get("short_description"),
            Some(&json!("new description")),
            "should have updated description"
        );

        assert_eq!(
            conversation.get("is_public"),
            Some(&json!(true)),
            "should have updated public status"
        );
        Ok(())
    }

    #[sqlx::test]
    fn should_not_be_able_to_udpate_owner_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let config = test_config()?;
        let app = setup_server(config, pool).await?;

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
                    "slug" : "new_conversation"
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
                    "owner_id": "c4beb9cf-55f5-4afe-ba82-0923877dd9e5",
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
        let config = test_config()?;
        let app = setup_server(config, pool).await?;

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
                    "slug" : "new_conversation"
                }),
            )
            .await?;

        session
            .create_conversation(
                &app,
                json! ({
                    "title" : "Another Test",
                    "short_description" : "A test conversation",
                    "description" : "A longer description",
                    "image_url" : "http://someimage.png",
                    "tags" : ["one", "two", "three"],
                    "is_public" : true,
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
        let config = test_config()?;
        let app = setup_server(config, pool).await?;

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
                        "slug" : format!("{i}")
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
                    "slug" : format!("target_slug")
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
        let config = test_config()?;
        let app = setup_server(config, pool).await?;

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
                        "slug" : format!("{i}")
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
        let config = test_config()?;
        let app = setup_server(config, pool).await?;

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
                        "slug" : format!("{i}")
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
        let config = test_config()?;
        let app = setup_server(config, pool).await?;

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
                    "slug" : "new_conversation"
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
                    "slug" : "new_conversation_two"
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
        let config = test_config()?;
        let app = setup_server(config, pool).await?;

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
                    "slug" : "new_conversation"
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
        let config = test_config()?;
        let app = setup_server(config, pool).await?;

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
                    "slug" : "new_conversation"
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
                    "slug" : "new_conversation"
                }),
            )
            .await?;

        assert_eq!(status, StatusCode::CONFLICT, "Slugs should be unique");

        Ok(())
    }
}
