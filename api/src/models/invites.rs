use crate::error::ComhairleError;
use chrono::{DateTime, Utc};
use comhairle_macros::{DbJsonBEnum, DbStringEnum};
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{enum_def, Alias, Expr, Func, Order, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use tracing::instrument;
use uuid::Uuid;

use super::{
    invite_response::{self, InviteResponseIden},
    users::User,
};

#[derive(Partial, Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "invite")]
#[partially(derive(Deserialize, Debug, JsonSchema, FromRow))]
pub struct Invite {
    #[partially(omit)]
    pub id: Uuid,
    pub invite_type: InviteType,
    pub created_by: Uuid,
    pub status: InviteStatus,
    pub expires_at: Option<DateTime<Utc>>,
    pub conversation_id: Uuid,
    pub workflow_id: Option<Uuid>,
    pub workflow_step_id: Option<Uuid>,
    pub login_behaviour: LoginBehaviour,
    pub tags: Vec<String>,
    #[partially(omit)]
    pub created_at: DateTime<Utc>,
    #[partially(omit)]
    pub updated_at: DateTime<Utc>,
    pub accept_count: i32,
}

impl Invite {
    #[instrument(err(Debug))]
    pub fn is_still_valid(&self) -> Result<(), ComhairleError> {
        if !(self.status == InviteStatus::Pending || self.status == InviteStatus::Open) {
            return Err(ComhairleError::InviteExpired);
        }

        if let Some(expiry) = self.expires_at {
            if Utc::now() >= expiry {
                Err(ComhairleError::InviteExpired)
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    #[instrument(err(Debug))]
    pub async fn accept(&self, db: &PgPool, user: &User) -> Result<Invite, ComhairleError> {
        let new_status = if self.status == InviteStatus::Open {
            InviteStatus::Open
        } else {
            InviteStatus::Accepted
        };

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

        invite_response::create(&db, &user.id, &invite.id, invite_response::Response::Accept)
            .await?;
        Ok(invite)
    }

    #[instrument(err(Debug))]
    pub async fn reject(&self, db: &PgPool, user: &User) -> Result<Invite, ComhairleError> {
        let (sql, values) = Query::update()
            .table(InviteIden::Table)
            .values([(InviteIden::Status, InviteStatus::Rejected.into())])
            .and_where(Expr::col(InviteIden::Id).eq(self.id.to_owned()))
            .returning(Query::returning().columns(DEFAULT_COLUMNS))
            .build_sqlx(PostgresQueryBuilder);

        let invite = sqlx::query_as_with::<_, Invite, _>(&sql, values)
            .fetch_one(db)
            .await?;

        invite_response::create(&db, &user.id, &invite.id, invite_response::Response::Reject)
            .await?;
        Ok(invite)
    }

    #[instrument(err(Debug))]
    pub fn is_for_user(&self, user: &User) -> Result<(), ComhairleError> {
        match &self.invite_type {
            InviteType::Email(email) => {
                if Some(email) == user.email.as_ref() {
                    Ok(())
                } else {
                    Err(ComhairleError::InviteDoesNotMatchUser)
                }
            }
            InviteType::User(uuid) => {
                if *uuid == user.id {
                    Ok(())
                } else {
                    Err(ComhairleError::InviteDoesNotMatchUser)
                }
            }
            InviteType::Open => Ok(()),
            InviteType::SingleUse => Ok(()),
        }
    }
}

/// Determines the type of invite that is being sent
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, DbJsonBEnum)]
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
    invite_type: InviteType,
    #[serde(default)]
    login_behaviour: LoginBehaviour,
    expires_at: Option<DateTime<Utc>>,
}

const DEFAULT_COLUMNS: [InviteIden; 13] = [
    InviteIden::Id,
    InviteIden::InviteType,
    InviteIden::CreatedBy,
    InviteIden::Status,
    InviteIden::ExpiresAt,
    InviteIden::ConversationId,
    InviteIden::WorkflowId,
    InviteIden::WorkflowStepId,
    InviteIden::LoginBehaviour,
    InviteIden::Tags,
    InviteIden::CreatedAt,
    InviteIden::UpdatedAt,
    InviteIden::AcceptCount,
];

/// Dictates login behaviour on invite accept.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, DbStringEnum)]
#[serde(rename_all = "snake_case")]
pub enum LoginBehaviour {
    /// If the user is logged out, then direct them to the login page
    /// to finish the login
    Manual,
    /// If the user is logged out, automatically create an annon
    /// account to let them access the system  
    AutoCreateAnnon,
}

impl Default for LoginBehaviour {
    fn default() -> Self {
        Self::Manual
    }
}

#[instrument(err(Debug))]
pub async fn list_for_conversation(
    db: &PgPool,
    conversation_id: &Uuid,
) -> Result<Vec<Invite>, ComhairleError> {
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

#[instrument(err(Debug))]
pub async fn get(db: &PgPool, invite_id: &Uuid) -> Result<Invite, ComhairleError> {
    let (sql, values) = Query::select()
        .from(InviteIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(Expr::col(InviteIden::Id).eq(*invite_id))
        .build_sqlx(PostgresQueryBuilder);

    let invite = sqlx::query_as_with::<_, Invite, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|e| {
            println!("{e:#?}");
            ComhairleError::ResourceNotFound("Invite".into())
        })?;

    Ok(invite)
}

#[instrument(err(Debug))]
pub async fn delete(db: &PgPool, id: &Uuid) -> Result<Invite, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(InviteIden::Table)
        .and_where(Expr::col(InviteIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let invite = sqlx::query_as_with::<_, Invite, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("Invite".into()))?;

    Ok(invite)
}

#[instrument(err(Debug))]
pub async fn create(
    db: &PgPool,
    create_invite: CreateInviteDTO,
    conversation_id: &Uuid,
    user_id: &Uuid,
) -> Result<Invite, ComhairleError> {
    let starting_status = match create_invite.invite_type {
        InviteType::Email(_) | InviteType::User(_) | InviteType::SingleUse => InviteStatus::Pending,
        InviteType::Open => InviteStatus::Open,
    };

    let (sql, values) = Query::insert()
        .into_table(InviteIden::Table)
        .columns(vec![
            InviteIden::CreatedBy,
            InviteIden::ConversationId,
            InviteIden::InviteType,
            InviteIden::LoginBehaviour,
            InviteIden::Status,
            InviteIden::ExpiresAt,
        ])
        .values(vec![
            user_id.to_owned().into(),
            conversation_id.to_owned().into(),
            create_invite.invite_type.into(),
            create_invite.login_behaviour.into(),
            starting_status.into(),
            create_invite.expires_at.into(),
        ])
        .unwrap()
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_as_with::<_, Invite, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|e| ComhairleError::FailedToCreateInvite(e))
}

#[derive(FromRow, Serialize, Deserialize, JsonSchema, Debug, PartialEq, Eq)]
pub struct DailyResponseStats {
    day: DateTime<Utc>,
    accept: i32,
    reject: i32,
}

pub async fn get_stats_for_invite(
    db: &PgPool,
    invite_id: &Uuid,
) -> Result<Vec<DailyResponseStats>, ComhairleError> {
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
    .map_err(|e| ComhairleError::InviteStatsAggregationError(e))?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::models::{
        conversation::{self, CreateConversation, PartialConversation},
        users,
        workflow::{self, CreateWorkflow},
    };

    use super::*;
    use fake::{Fake, Faker};
    use sqlx::PgPool;
    use std::error::Error;

    #[sqlx::test]
    async fn should_get_correct_stats_for_invite(db: PgPool) -> Result<(), Box<dyn Error>> {
        let user1 = users::create_user(&Faker.fake(), &db).await?;
        let user2 = users::create_user(&Faker.fake(), &db).await?;
        let user3 = users::create_user(&Faker.fake(), &db).await?;
        let user4 = users::create_user(&Faker.fake(), &db).await?;

        let conversation = conversation::create(
            &db,
            &CreateConversation {
                is_public: true,
                is_invite_only: true,
                ..Faker.fake()
            },
            user1.id,
        )
        .await?;

        println!("conversation {conversation:#?}");

        let workflow = workflow::create(
            &db,
            &CreateWorkflow { ..Faker.fake() },
            conversation.id,
            user1.id,
        )
        .await?;
        println!("workflow {workflow:#?}");

        let conversation = conversation::update(
            &db,
            &conversation.id,
            &PartialConversation {
                default_workflow_id: Some(workflow.id),
                ..Default::default()
            },
        )
        .await?;

        println!("conversation with default workflow {conversation:#?}");

        let invite = create(
            &db,
            CreateInviteDTO {
                invite_type: InviteType::Open,
                login_behaviour: LoginBehaviour::Manual,
                expires_at: None,
            },
            &conversation.id,
            &user1.id,
        )
        .await?;

        invite.accept(&db, &user2).await?;
        invite.reject(&db, &user3).await?;
        invite.accept(&db, &user4).await?;

        let stats = get_stats_for_invite(&db, &invite.id).await?;

        assert_eq!(
            stats,
            vec![DailyResponseStats {
                day: Utc::now()
                    .date_naive()
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_utc(),
                accept: 2,
                reject: 1
            }],
            "should get the correct daily stats"
        );

        Ok(())
    }
}
