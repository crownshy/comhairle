use crate::error::ComhairleError;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{enum_def, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "conversation_email_notification_recipients")]
pub struct ConversationEmailNotificationRecipients {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub email: String,
    pub receive_updates_by_email: bool,
    pub receive_similar_conversation_updates_by_email: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct CreateConversationEmailNotificationRecipients {
    pub conversation_id: Uuid,
    pub email: String,
    pub receive_updates_by_email: bool,
    pub receive_similar_conversation_updates_by_email: bool,
}

const DEFAULT_COLUMNS: [ConversationEmailNotificationRecipientsIden; 7] = [
    ConversationEmailNotificationRecipientsIden::Id,
    ConversationEmailNotificationRecipientsIden::ConversationId,
    ConversationEmailNotificationRecipientsIden::Email,
    ConversationEmailNotificationRecipientsIden::ReceiveUpdatesByEmail,
    ConversationEmailNotificationRecipientsIden::ReceiveSimilarConversationUpdatesByEmail,
    ConversationEmailNotificationRecipientsIden::CreatedAt,
    ConversationEmailNotificationRecipientsIden::UpdatedAt,
];

impl CreateConversationEmailNotificationRecipients {
    pub fn columns(&self) -> Vec<ConversationEmailNotificationRecipientsIden> {
        vec![
            ConversationEmailNotificationRecipientsIden::ConversationId,
            ConversationEmailNotificationRecipientsIden::Email,
            ConversationEmailNotificationRecipientsIden::ReceiveUpdatesByEmail,
            ConversationEmailNotificationRecipientsIden::ReceiveSimilarConversationUpdatesByEmail,
        ]
    }

    pub fn values(&self) -> Vec<sea_query::SimpleExpr> {
        vec![
            self.conversation_id.into(),
            self.email.clone().into(),
            self.receive_updates_by_email.into(),
            self.receive_similar_conversation_updates_by_email.into(),
        ]
    }
}

pub async fn create(
    db: &PgPool,
    recipient: &CreateConversationEmailNotificationRecipients,
) -> Result<ConversationEmailNotificationRecipients, ComhairleError> {
    let columns = recipient.columns();
    let values = recipient.values();

    let (sql, values) = Query::insert()
        .into_table(ConversationEmailNotificationRecipientsIden::Table)
        .columns(columns)
        .values(values)
        .unwrap()
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let recipient = sqlx::query_as_with::<_, ConversationEmailNotificationRecipients, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(recipient)
}

pub async fn get_by_conversation_and_email(
    db: &PgPool,
    conversation_id: &Uuid,
    email: &str,
) -> Result<ConversationEmailNotificationRecipients, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(ConversationEmailNotificationRecipientsIden::Table)
        .and_where(
            Expr::col(ConversationEmailNotificationRecipientsIden::ConversationId)
                .eq(conversation_id.to_owned()),
        )
        .and_where(
            Expr::col(ConversationEmailNotificationRecipientsIden::Email)
                .eq(email.to_owned()),
        )
        .build_sqlx(PostgresQueryBuilder);

    let recipient = sqlx::query_as_with::<_, ConversationEmailNotificationRecipients, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("ConversationEmailNotificationRecipients".into()))?;

    Ok(recipient)
}

pub async fn get_by_conversation(
    db: &PgPool,
    conversation_id: &Uuid,
) -> Result<Vec<ConversationEmailNotificationRecipients>, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(ConversationEmailNotificationRecipientsIden::Table)
        .and_where(
            Expr::col(ConversationEmailNotificationRecipientsIden::ConversationId)
                .eq(conversation_id.to_owned()),
        )
        .build_sqlx(PostgresQueryBuilder);

    let recipients = sqlx::query_as_with::<_, ConversationEmailNotificationRecipients, _>(&sql, values)
        .fetch_all(db)
        .await?;

    Ok(recipients)
}

pub async fn delete_by_conversation_and_email(
    db: &PgPool,
    conversation_id: &Uuid,
    email: &str,
) -> Result<ConversationEmailNotificationRecipients, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(ConversationEmailNotificationRecipientsIden::Table)
        .and_where(
            Expr::col(ConversationEmailNotificationRecipientsIden::ConversationId)
                .eq(conversation_id.to_owned()),
        )
        .and_where(
            Expr::col(ConversationEmailNotificationRecipientsIden::Email)
                .eq(email.to_owned()),
        )
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let recipient = sqlx::query_as_with::<_, ConversationEmailNotificationRecipients, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("ConversationEmailNotificationRecipients".into()))?;

    Ok(recipient)
}