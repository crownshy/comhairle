use crate::error::ComhairleError;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{enum_def, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "user_conversation_preferences")]
pub struct UserConversationPreferences {
    pub id: Uuid,
    pub user_id: Uuid,
    pub conversation_id: Uuid,
    pub receive_updates_by_notification: bool,
    pub receive_updates_by_email: bool,
    pub receive_similar_conversation_updates_by_email: bool,
    pub receive_similar_conversation_updates_by_notification: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct CreateUserConversationPreferences {
    pub user_id: Uuid,
    pub conversation_id: Uuid,
    pub receive_updates_by_notification: Option<bool>,
    pub receive_updates_by_email: Option<bool>,
    pub receive_similar_conversation_updates_by_email: Option<bool>,
    pub receive_similar_conversation_updates_by_notification: Option<bool>,
}

const DEFAULT_COLUMNS: [UserConversationPreferencesIden; 9] = [
    UserConversationPreferencesIden::Id,
    UserConversationPreferencesIden::UserId,
    UserConversationPreferencesIden::ConversationId,
    UserConversationPreferencesIden::ReceiveUpdatesByNotification,
    UserConversationPreferencesIden::ReceiveUpdatesByEmail,
    UserConversationPreferencesIden::ReceiveSimilarConversationUpdatesByEmail,
    UserConversationPreferencesIden::ReceiveSimilarConversationUpdatesByNotification,
    UserConversationPreferencesIden::CreatedAt,
    UserConversationPreferencesIden::UpdatedAt,
];

impl CreateUserConversationPreferences {
    pub fn columns(&self) -> Vec<UserConversationPreferencesIden> {
        vec![
            UserConversationPreferencesIden::UserId,
            UserConversationPreferencesIden::ConversationId,
            UserConversationPreferencesIden::ReceiveUpdatesByNotification,
            UserConversationPreferencesIden::ReceiveUpdatesByEmail,
            UserConversationPreferencesIden::ReceiveSimilarConversationUpdatesByEmail,
            UserConversationPreferencesIden::ReceiveSimilarConversationUpdatesByNotification,
        ]
    }

    pub fn values(&self) -> Vec<sea_query::SimpleExpr> {
        vec![
            self.user_id.into(),
            self.conversation_id.into(),
            self.receive_updates_by_notification.unwrap_or(true).into(),
            self.receive_updates_by_email.unwrap_or(true).into(),
            self.receive_similar_conversation_updates_by_email
                .unwrap_or(false)
                .into(),
            self.receive_similar_conversation_updates_by_notification
                .unwrap_or(false)
                .into(),
        ]
    }
}

pub async fn create_with_defaults(
    db: &PgPool,
    user_id: &Uuid,
    conversation_id: &Uuid,
) -> Result<UserConversationPreferences, ComhairleError> {
    create(
        db,
        &CreateUserConversationPreferences {
            user_id: (*user_id),
            conversation_id: (*conversation_id),
            receive_updates_by_notification: Some(false),
            receive_updates_by_email: Some(false),
            receive_similar_conversation_updates_by_email: Some(false),
            receive_similar_conversation_updates_by_notification: Some(false),
        },
    )
    .await
}

pub async fn create(
    db: &PgPool,
    preferences: &CreateUserConversationPreferences,
) -> Result<UserConversationPreferences, ComhairleError> {
    let columns = preferences.columns();
    let values = preferences.values();

    let (sql, values) = Query::insert()
        .into_table(UserConversationPreferencesIden::Table)
        .columns(columns)
        .values(values)
        .unwrap()
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let preferences = sqlx::query_as_with::<_, UserConversationPreferences, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(preferences)
}

pub async fn get_by_user_and_conversation(
    db: &PgPool,
    user_id: &Uuid,
    conversation_id: &Uuid,
) -> Result<UserConversationPreferences, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(UserConversationPreferencesIden::Table)
        .and_where(Expr::col(UserConversationPreferencesIden::UserId).eq(user_id.to_owned()))
        .and_where(
            Expr::col(UserConversationPreferencesIden::ConversationId)
                .eq(conversation_id.to_owned()),
        )
        .build_sqlx(PostgresQueryBuilder);

    let preferences = sqlx::query_as_with::<_, UserConversationPreferences, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("UserConversationPreferences".into()))?;

    Ok(preferences)
}

pub async fn get_by_user(
    db: &PgPool,
    user_id: &Uuid,
) -> Result<Vec<UserConversationPreferences>, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(UserConversationPreferencesIden::Table)
        .and_where(Expr::col(UserConversationPreferencesIden::UserId).eq(user_id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let preferences = sqlx::query_as_with::<_, UserConversationPreferences, _>(&sql, values)
        .fetch_all(db)
        .await?;

    Ok(preferences)
}

pub async fn update(
    db: &PgPool,
    user_id: &Uuid,
    conversation_id: &Uuid,
    receive_updates_by_notification: Option<bool>,
    receive_updates_by_email: Option<bool>,
    receive_similar_conversation_updates_by_email: Option<bool>,
    receive_similar_conversation_updates_by_notification: Option<bool>,
) -> Result<UserConversationPreferences, ComhairleError> {
    let mut query = Query::update()
        .table(UserConversationPreferencesIden::Table)
        .and_where(Expr::col(UserConversationPreferencesIden::UserId).eq(user_id.to_owned()))
        .and_where(
            Expr::col(UserConversationPreferencesIden::ConversationId)
                .eq(conversation_id.to_owned()),
        )
        .to_owned();

    if let Some(value) = receive_updates_by_notification {
        query = query
            .value(
                UserConversationPreferencesIden::ReceiveUpdatesByNotification,
                value,
            )
            .to_owned();
    }
    if let Some(value) = receive_updates_by_email {
        query = query
            .value(
                UserConversationPreferencesIden::ReceiveUpdatesByEmail,
                value,
            )
            .to_owned();
    }
    if let Some(value) = receive_similar_conversation_updates_by_email {
        query = query
            .value(
                UserConversationPreferencesIden::ReceiveSimilarConversationUpdatesByEmail,
                value,
            )
            .to_owned();
    }
    if let Some(value) = receive_similar_conversation_updates_by_notification {
        query = query
            .value(
                UserConversationPreferencesIden::ReceiveSimilarConversationUpdatesByNotification,
                value,
            )
            .to_owned();
    }

    let (sql, values) = query
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let preferences = sqlx::query_as_with::<_, UserConversationPreferences, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(preferences)
}

pub async fn delete(
    db: &PgPool,
    user_id: &Uuid,
    conversation_id: &Uuid,
) -> Result<UserConversationPreferences, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(UserConversationPreferencesIden::Table)
        .and_where(Expr::col(UserConversationPreferencesIden::UserId).eq(user_id.to_owned()))
        .and_where(
            Expr::col(UserConversationPreferencesIden::ConversationId)
                .eq(conversation_id.to_owned()),
        )
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let preferences = sqlx::query_as_with::<_, UserConversationPreferences, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("UserConversationPreferences".into()))?;

    Ok(preferences)
}
