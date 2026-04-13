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

#[derive(Debug, Serialize, FromRow)]
pub struct ExportedContact {
    pub email: String,
    pub user_type: String,
    pub conversation_updates: bool,
    pub similar_conversations_updates: bool,
    pub signup_date: DateTime<Utc>,
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

pub async fn get_contacts_for_export(
    db: &PgPool,
    conversation_id: &Uuid,
) -> Result<Vec<ExportedContact>, ComhairleError> {
    // Query for authenticated users who opted in to email updates
    let authenticated_query = r#"
        SELECT
            u.email as email,
            'Authenticated' as user_type,
            ucp.receive_updates_by_email as conversation_updates,
            ucp.receive_similar_conversation_updates_by_email as similar_conversations_updates,
            ucp.created_at as signup_date
        FROM user_conversation_preferences ucp
        INNER JOIN comhairle_user u ON ucp.user_id = u.id
        WHERE ucp.conversation_id = $1
          AND u.email IS NOT NULL
          AND (ucp.receive_updates_by_email = true OR ucp.receive_similar_conversation_updates_by_email = true)
    "#;

    // Query for anonymous users who opted in to email updates
    let anonymous_query = r#"
        SELECT
            email,
            'Anonymous' as user_type,
            receive_updates_by_email as conversation_updates,
            receive_similar_conversation_updates_by_email as similar_conversations_updates,
            created_at as signup_date
        FROM conversation_email_notification_recipients
        WHERE conversation_id = $1
          AND (receive_updates_by_email = true OR receive_similar_conversation_updates_by_email = true)
    "#;

    // Combine both queries using UNION
    let combined_query = format!(
        "{} UNION ALL {} ORDER BY signup_date DESC",
        authenticated_query, anonymous_query
    );

    let contacts = sqlx::query_as::<_, ExportedContact>(&combined_query)
        .bind(conversation_id)
        .bind(conversation_id)
        .fetch_all(db)
        .await?;

    Ok(contacts)
}
