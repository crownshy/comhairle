use crate::models::SqlxResultExt;
use crate::models::error::InviteError;
use crate::models::error::ModelError;
use chrono::{DateTime, Utc};
use comhairle_macros::{DbJsonBEnum, DbStringEnum};
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{Expr, Order, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow};
use tracing::instrument;
use uuid::Uuid;

use super::{invite_response, users::User};

#[derive(Partial, Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "invite")]
#[partially(derive(Deserialize, Debug, JsonSchema, FromRow))]
pub struct Invite {
    #[partially(omit)]
    pub id: Uuid,
    pub invite_type: InviteType,
    #[partially(omit)]
    pub created_by: Option<Uuid>,
    pub status: InviteStatus,
    pub expires_at: Option<DateTime<Utc>>,
    pub conversation_id: Uuid,
    #[partially(transparent)]
    pub event_id: Option<Uuid>,
    pub workflow_id: Option<Uuid>,
    pub workflow_step_id: Option<Uuid>,
    pub login_behaviour: LoginBehaviour,
    pub tags: Vec<String>,
    pub label: Option<String>,
    #[partially(omit)]
    pub created_at: DateTime<Utc>,
    #[partially(omit)]
    pub updated_at: DateTime<Utc>,
    pub accept_count: i32,
}

impl Invite {
    #[instrument(err(Debug))]
    pub fn is_still_valid(&self) -> Result<(), ModelError> {
        // If the invite is accepted we still want to return it
        if self.status == InviteStatus::Accepted {
            return Ok(());
        }

        if !(self.status == InviteStatus::Pending || self.status == InviteStatus::Open) {
            return Err(InviteError::InviteExpired.into());
        }

        if let Some(expiry) = self.expires_at {
            if Utc::now() >= expiry {
                Err(InviteError::InviteExpired.into())
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    #[instrument(err(Debug), skip(db))]
    pub async fn accept(&self, db: &PgPool, user: &User) -> Result<Invite, ModelError> {
        let new_status = if self.status == InviteStatus::Open {
            InviteStatus::Open
        } else {
            InviteStatus::Accepted
        };

        invite_response::create(db, &user.id, &self.id, invite_response::Response::Accept)
            .await
            .map_err(|_| InviteError::InviteResponseAlreadyCreated)?;

        let (sql, values) = Query::update()
            .table(InviteIden::Table)
            .values([
                (InviteIden::Status, new_status.into()),
                (InviteIden::AcceptCount, (self.accept_count + 1).into()),
            ])
            .and_where(Expr::col(InviteIden::Id).eq(self.id.to_owned()))
            .returning(Query::returning().columns(DEFAULT_COLUMNS))
            .build_sqlx(PostgresQueryBuilder);

        let invite = sqlx::query_as_with::<_, Invite, _>(&sql, values)
            .fetch_one(db)
            .await?;

        Ok(invite)
    }

    #[instrument(err(Debug), skip(db))]
    pub async fn reject(&self, db: &PgPool, user: &User) -> Result<Invite, ModelError> {
        let new_status = if self.status == InviteStatus::Pending {
            InviteStatus::Rejected
        } else {
            self.status.clone()
        };

        let (sql, values) = Query::update()
            .table(InviteIden::Table)
            .values([(InviteIden::Status, new_status.into())])
            .and_where(Expr::col(InviteIden::Id).eq(self.id.to_owned()))
            .returning(Query::returning().columns(DEFAULT_COLUMNS))
            .build_sqlx(PostgresQueryBuilder);

        let invite = sqlx::query_as_with::<_, Invite, _>(&sql, values)
            .fetch_one(db)
            .await?;

        invite_response::create(db, &user.id, &invite.id, invite_response::Response::Reject)
            .await?;
        Ok(invite)
    }

    #[instrument(err(Debug))]
    pub fn is_for_user(&self, user: &User) -> Result<(), ModelError> {
        match &self.invite_type {
            InviteType::Email(email) => {
                if let Some(user_email) = user.email.as_ref() {
                    if email.to_lowercase() == user_email.to_lowercase() {
                        Ok(())
                    } else {
                        Err(InviteError::InviteDoesNotMatchUser.into())
                    }
                } else {
                    Err(InviteError::InviteDoesNotMatchUser.into())
                }
            }
            InviteType::User(uuid) => {
                if *uuid == user.id {
                    Ok(())
                } else {
                    Err(InviteError::InviteDoesNotMatchUser.into())
                }
            }
            InviteType::Open => Ok(()),
            InviteType::SingleUse => Ok(()),
        }
    }
}

/// Determines the type of invite that is being sent
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, DbJsonBEnum, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InviteType {
    /// Send an invite by email to a specific person
    Email(String),
    /// Send an invite to an existing user on the platfrom
    User(Uuid),
    /// Create an invite that anyone can use but it can only be accepted once
    SingleUse,
    /// Create an invite that is open and multiple people can access
    Open,
}

/// Tracks the status of an invite
#[derive(Deserialize, Serialize, Clone, JsonSchema, Debug, DbStringEnum, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum InviteStatus {
    /// The invite has been issued but not accepted
    Pending,
    /// The invite is open for anyone to accept (only applies to Open type invites)
    Open,
    /// The invite has been accepted and is no longer valid (used for email, user and single use
    /// invites)
    Accepted,
    /// The invite was rejected and is no longer valid (used for email, user and single use invites )
    Rejected,
    /// The invite has expired or has been closed, it is no longer valid.
    Expired,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct CreateInviteDTO {
    pub invite_type: InviteType,
    #[serde(default)]
    pub login_behaviour: LoginBehaviour,
    pub expires_at: Option<DateTime<Utc>>,
    pub label: Option<String>,
    pub event_id: Option<Uuid>,
}

const DEFAULT_COLUMNS: [InviteIden; 15] = [
    InviteIden::Id,
    InviteIden::InviteType,
    InviteIden::CreatedBy,
    InviteIden::Status,
    InviteIden::ExpiresAt,
    InviteIden::ConversationId,
    InviteIden::EventId,
    InviteIden::WorkflowId,
    InviteIden::WorkflowStepId,
    InviteIden::LoginBehaviour,
    InviteIden::Tags,
    InviteIden::Label,
    InviteIden::CreatedAt,
    InviteIden::UpdatedAt,
    InviteIden::AcceptCount,
];

/// Dictates login behaviour on invite accept.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, DbStringEnum, Default)]
#[serde(rename_all = "snake_case")]
pub enum LoginBehaviour {
    /// If the user is logged out, then direct them to the login page
    /// to finish the login
    #[default]
    Manual,
    /// If the user is logged out, automatically create a guest
    /// account to let them access the system  
    AutoCreateGuest,
}

#[instrument(err(Debug), skip(db))]
pub async fn list_for_conversation(
    db: &PgPool,
    conversation_id: &Uuid,
) -> Result<Vec<Invite>, ModelError> {
    let query = Query::select()
        .from(InviteIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(Expr::col(InviteIden::ConversationId).eq(*conversation_id))
        .order_by(InviteIden::CreatedAt, Order::Desc)
        .to_owned();

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let invites = sqlx::query_as_with::<_, Invite, _>(&sql, values)
        .fetch_all(db)
        .await?;
    Ok(invites)
}

#[instrument(err(Debug), skip(db))]
pub async fn list_for_event(db: &PgPool, event_id: &Uuid) -> Result<Vec<Invite>, ModelError> {
    let query = Query::select()
        .from(InviteIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(Expr::col(InviteIden::EventId).eq(*event_id))
        .order_by(InviteIden::CreatedAt, Order::Desc)
        .to_owned();

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let invites = sqlx::query_as_with::<_, Invite, _>(&sql, values)
        .fetch_all(db)
        .await?;

    Ok(invites)
}

#[instrument(err(Debug), skip(db))]
pub async fn get_by_id(db: &PgPool, invite_id: &Uuid) -> Result<Invite, ModelError> {
    let (sql, values) = Query::select()
        .from(InviteIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(Expr::col(InviteIden::Id).eq(*invite_id))
        .build_sqlx(PostgresQueryBuilder);

    let invite = sqlx::query_as_with::<_, Invite, _>(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Invite")?;

    Ok(invite)
}

#[instrument(err(Debug), skip(db))]
pub async fn update(
    db: &PgPool,
    id: &Uuid,
    partial_invite: PartialInvite,
) -> Result<Invite, ModelError> {
    let mut query = Query::update();
    query.table(InviteIden::Table);

    if let Some(label) = partial_invite.label {
        query.value(InviteIden::Label, label);
    }

    let (sql, values) = query
        .and_where(Expr::col(InviteIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let invite = sqlx::query_as_with::<_, Invite, _>(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Invite")?;

    Ok(invite)
}

#[instrument(err(Debug), skip(db))]
pub async fn delete(db: &PgPool, id: &Uuid) -> Result<Invite, ModelError> {
    let (sql, values) = Query::delete()
        .from_table(InviteIden::Table)
        .and_where(Expr::col(InviteIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let invite = sqlx::query_as_with::<_, Invite, _>(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Invite")?;

    Ok(invite)
}

#[instrument(err(Debug), skip(db))]
pub async fn create(
    db: &PgPool,
    create_invite: CreateInviteDTO,
    conversation_id: &Uuid,
    user_id: Option<Uuid>,
) -> Result<Invite, ModelError> {
    let starting_status = match create_invite.invite_type {
        InviteType::Email(_) | InviteType::User(_) | InviteType::SingleUse => InviteStatus::Pending,
        InviteType::Open => InviteStatus::Open,
    };

    let mut columns = vec![
        InviteIden::ConversationId,
        InviteIden::InviteType,
        InviteIden::LoginBehaviour,
        InviteIden::Status,
        InviteIden::ExpiresAt,
        InviteIden::Label,
        InviteIden::EventId,
    ];
    let mut values = vec![
        conversation_id.to_owned().into(),
        create_invite.invite_type.into(),
        create_invite.login_behaviour.into(),
        starting_status.into(),
        create_invite.expires_at.into(),
        create_invite.label.into(),
        create_invite.event_id.into(),
    ];

    if let Some(user_id) = user_id {
        columns.push(InviteIden::CreatedBy);
        values.push(user_id.into());
    }

    let (sql, values) = Query::insert()
        .into_table(InviteIden::Table)
        .columns(columns)
        .values(values)
        .unwrap()
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_as_with::<_, Invite, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|e| InviteError::FailedToCreateInvite(e).into())
}

#[derive(FromRow, Serialize, Deserialize, JsonSchema, Debug, PartialEq, Eq)]
pub struct DailyResponseStats {
    pub day: DateTime<Utc>,
    pub accept: i32,
    pub reject: i32,
}

#[instrument(err(Debug), skip(db))]
pub async fn get_stats_for_invite(
    db: &PgPool,
    invite_id: &Uuid,
) -> Result<Vec<DailyResponseStats>, ModelError> {
    let result = sqlx::query_as::<_, DailyResponseStats>(
        r#"
        SELECT date_trunc('day', created_at) as day,
        SUM(CASE WHEN response = 'accept' THEN 1 ELSE 0 END)::INT  as accept,
        SUM(CASE WHEN response = 'reject' THEN 1 ELSE 0 END)::INT  as reject
        FROM invite_response
        where invite_id = $1
        GROUP BY date_trunc('day', created_at)
        ORDER BY date_trunc('day', created_at) ASC;
    "#,
    )
    .bind(invite_id)
    .fetch_all(db)
    .await
    .map_err(InviteError::InviteStatsAggregationError)?;

    Ok(result)
}
