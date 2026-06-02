use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{enum_def, Expr, PostgresQueryBuilder, Query, SelectStatement, SimpleExpr};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{
    encode::IsNull,
    prelude::{FromRow, Type},
    query_as_with, Decode, Encode, PgPool, Postgres,
};
use sqlx_postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef};
use tracing::instrument;
use uuid::Uuid;

#[cfg(test)]
use fake::Dummy;

use crate::{
    error::ComhairleError,
    models::pagination::{Order, PageOptions, PaginatedResults},
};

#[derive(Serialize, Deserialize, Debug, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "scheduled_email")]
pub struct ScheduledEmail {
    pub id: Uuid,
    pub user_email: String,
    pub email_config: ScheduledEmailConfig,
    pub status: EmailStatus,
    pub send_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct ScheduledEmailConfig {
    pub template: EmailTemplate,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EmailTemplate {
    // Extend with other templates relevant to scheduling
    EventReminder {
        event_name: String,
        event_time: String,
        event_link: String,
        organization_name: String,
        organization_email: Option<String>,
    },
}

impl Type<Postgres> for ScheduledEmailConfig {
    fn type_info() -> PgTypeInfo {
        <serde_json::Value as Type<Postgres>>::type_info()
    }
}

impl<'q> Encode<'q, Postgres> for EmailTemplate {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<IsNull, sqlx::error::BoxDynError> {
        let json = serde_json::to_value(self)?;
        <serde_json::Value as Encode<Postgres>>::encode(json, buf)
    }
}

impl<'r> Decode<'r, Postgres> for ScheduledEmailConfig {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let json: serde_json::Value = Decode::<Postgres>::decode(value)?;
        Ok(serde_json::from_value(json)?)
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, sqlx::Type, Clone, JsonSchema)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(Dummy))]
pub enum EmailStatus {
    #[sqlx(rename = "pending")]
    Pending,
    #[sqlx(rename = "sent")]
    Sent,
    #[sqlx(rename = "failed")]
    Failed,
}

impl From<EmailStatus> for sea_query::Value {
    fn from(val: EmailStatus) -> Self {
        sea_query::Value::String(Some(Box::new(val.to_string())))
    }
}

impl std::fmt::Display for EmailStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            EmailStatus::Pending => "pending",
            EmailStatus::Sent => "sent",
            EmailStatus::Failed => "failed",
        };
        write!(f, "{}", value)
    }
}

const DEFAULT_COLUMNS: [ScheduledEmailIden; 7] = [
    ScheduledEmailIden::Id,
    ScheduledEmailIden::UserEmail,
    ScheduledEmailIden::EmailConfig,
    ScheduledEmailIden::Status,
    ScheduledEmailIden::SendAt,
    ScheduledEmailIden::CreatedAt,
    ScheduledEmailIden::UpdatedAt,
];

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct CreateScheduledEmail {
    pub user_email: String,
    pub send_at: DateTime<Utc>,
    pub email_config: ScheduledEmailConfig,
}

#[instrument(err(Debug))]
pub async fn create(
    db: &PgPool,
    email: CreateScheduledEmail,
) -> Result<ScheduledEmail, ComhairleError> {
    let mut columns = vec![ScheduledEmailIden::UserEmail, ScheduledEmailIden::SendAt];
    let mut values: Vec<SimpleExpr> = vec![email.user_email.into(), email.send_at.into()];

    let email_config = serde_json::to_value(email.email_config)?;
    columns.push(ScheduledEmailIden::EmailConfig);
    values.push(email_config.into());

    let (sql, values) = Query::insert()
        .into_table(ScheduledEmailIden::Table)
        .columns(columns)
        .values(values)?
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let scheduled_email = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(scheduled_email)
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct UpdateScheduledEmail {
    status: Option<EmailStatus>,
}

impl UpdateScheduledEmail {
    fn to_values(&self) -> Vec<(ScheduledEmailIden, SimpleExpr)> {
        let mut values = vec![];

        if let Some(value) = &self.status {
            values.push((ScheduledEmailIden::Status, value.clone().into()))
        }

        values
    }
}

#[instrument(err(Debug))]
pub async fn update(
    db: &PgPool,
    id: Uuid,
    update_email: UpdateScheduledEmail,
) -> Result<ScheduledEmail, ComhairleError> {
    let values = update_email.to_values();

    if values.is_empty() {
        return Err(ComhairleError::NoValidUpdates);
    }

    let (sql, values) = Query::update()
        .table(ScheduledEmailIden::Table)
        .values(values)
        .and_where(Expr::col(ScheduledEmailIden::Id).eq(id))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let email = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(email)
}

#[instrument(err(Debug))]
pub async fn get_by_id(db: &PgPool, id: Uuid) -> Result<ScheduledEmail, ComhairleError> {
    let (sql, values) = Query::select()
        .from(ScheduledEmailIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(Expr::col(ScheduledEmailIden::Id).eq(id))
        .build_sqlx(PostgresQueryBuilder);

    let email = query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ComhairleError::ResourceNotFound("Scheduled email".into()),
            other => ComhairleError::DatabaseError(other),
        })?;

    Ok(email)
}

#[derive(Deserialize, Debug, Default, JsonSchema)]
pub struct ScheduledEmailFilterOptions {
    user_email: Option<String>,
}

impl ScheduledEmailFilterOptions {
    fn apply(&self, mut query: SelectStatement) -> SelectStatement {
        if let Some(user_email) = &self.user_email {
            query = query
                .and_where(
                    Expr::col((ScheduledEmailIden::Table, ScheduledEmailIden::UserEmail))
                        .eq(user_email.to_owned()),
                )
                .to_owned();
        }

        query
    }
}

#[derive(Deserialize, Debug, Default, JsonSchema)]
pub struct ScheduledEmailOrderOptions {
    created_at: Option<Order>,
    send_at: Option<Order>,
}

impl ScheduledEmailOrderOptions {
    fn apply(&self, mut query: SelectStatement) -> SelectStatement {
        if let Some(order) = &self.created_at {
            query = query
                .order_by(
                    (ScheduledEmailIden::Table, ScheduledEmailIden::CreatedAt),
                    order.into(),
                )
                .to_owned();
        }
        if let Some(order) = &self.send_at {
            query = query
                .order_by(
                    (ScheduledEmailIden::Table, ScheduledEmailIden::SendAt),
                    order.into(),
                )
                .to_owned();
        }

        query
    }
}

#[instrument(err(Debug))]
pub async fn list(
    db: &PgPool,
    page_options: PageOptions,
    filter_options: ScheduledEmailFilterOptions,
    order_options: ScheduledEmailOrderOptions,
) -> Result<PaginatedResults<ScheduledEmail>, ComhairleError> {
    let query = Query::select()
        .from(ScheduledEmailIden::Table)
        .columns(DEFAULT_COLUMNS.map(|col| (ScheduledEmailIden::Table, col)))
        .to_owned();

    let query = filter_options.apply(query);
    let query = order_options.apply(query);

    let emails = page_options.fetch_paginated_results(db, query).await?;

    Ok(emails)
}

#[instrument(err(Debug))]
pub async fn list_upcoming_emails(
    db: &PgPool,
    upcoming_duration: chrono::Duration,
) -> Result<Vec<ScheduledEmail>, ComhairleError> {
    let (sql, values) = Query::select()
        .from(ScheduledEmailIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(Expr::col(ScheduledEmailIden::Status).eq(EmailStatus::Pending.to_string()))
        .and_where(Expr::col(ScheduledEmailIden::SendAt).gt(Utc::now()))
        .and_where(Expr::col(ScheduledEmailIden::SendAt).lte(Utc::now() + upcoming_duration))
        .build_sqlx(PostgresQueryBuilder);

    let scheduled_emails = query_as_with(&sql, values).fetch_all(db).await?;

    Ok(scheduled_emails)
}

#[instrument(err(Debug))]
pub async fn delete(db: &PgPool, id: Uuid) -> Result<ScheduledEmail, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(ScheduledEmailIden::Table)
        .and_where(Expr::col(ScheduledEmailIden::Id).eq(id))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let email = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(email)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error::Error;

    #[sqlx::test]
    async fn should_create_scheduled_email(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let params = CreateScheduledEmail {
            user_email: "test@test.com".to_string(),
            send_at: Utc::now() + chrono::Duration::days(1),
            email_config: ScheduledEmailConfig {
                template: EmailTemplate::EventReminder {
                    event_name: "test_event".to_string(),
                    event_time: (Utc::now() + chrono::Duration::days(1)).to_string(),
                    event_link: "https://test.com".to_string(),
                    organization_name: "Test org".to_string(),
                    organization_email: None,
                },
            },
        };

        let email = create(&pool, params).await?;

        assert_eq!(
            email.user_email,
            "test@test.com".to_string(),
            "incorrect user_email"
        );
        assert!(email.send_at > Utc::now(), "send_at not in the future");

        Ok(())
    }

    #[sqlx::test]
    async fn should_update_scheduled_email(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let params = CreateScheduledEmail {
            user_email: "test@test.com".to_string(),
            send_at: Utc::now() + chrono::Duration::days(1),
            email_config: ScheduledEmailConfig {
                template: EmailTemplate::EventReminder {
                    event_name: "test_event".to_string(),
                    event_time: (Utc::now() + chrono::Duration::days(1)).to_string(),
                    event_link: "https://test.com".to_string(),
                    organization_name: "Test org".to_string(),
                    organization_email: None,
                },
            },
        };

        let email = create(&pool, params).await?;

        assert_eq!(
            email.status,
            EmailStatus::Pending,
            "incorrect status before update"
        );

        let update_params = UpdateScheduledEmail {
            status: Some(EmailStatus::Sent),
        };
        let email = update(&pool, email.id, update_params).await?;

        assert_eq!(
            email.status,
            EmailStatus::Sent,
            "incorrect status after update"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_get_scheduled_email_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let params = CreateScheduledEmail {
            user_email: "test@test.com".to_string(),
            send_at: Utc::now() + chrono::Duration::days(1),
            email_config: ScheduledEmailConfig {
                template: EmailTemplate::EventReminder {
                    event_name: "test_event".to_string(),
                    event_time: (Utc::now() + chrono::Duration::days(1)).to_string(),
                    event_link: "https://test.com".to_string(),
                    organization_name: "Test org".to_string(),
                    organization_email: None,
                },
            },
        };

        let created_email = create(&pool, params).await?;

        let email = get_by_id(&pool, created_email.id).await?;

        assert_eq!(
            email.user_email,
            "test@test.com".to_string(),
            "incorrect user_email"
        );
        assert!(email.send_at > Utc::now(), "send_at not in the future");

        Ok(())
    }

    #[sqlx::test]
    async fn should_list_scheduled_emails(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let params_1 = CreateScheduledEmail {
            user_email: "user-1@test.com".to_string(),
            send_at: Utc::now() + chrono::Duration::days(1),
            email_config: ScheduledEmailConfig {
                template: EmailTemplate::EventReminder {
                    event_name: "test_event".to_string(),
                    event_time: (Utc::now() + chrono::Duration::days(1)).to_string(),
                    event_link: "https://test.com".to_string(),
                    organization_name: "Test org".to_string(),
                    organization_email: None,
                },
            },
        };
        let params_2 = CreateScheduledEmail {
            user_email: "user-2@test.com".to_string(),
            send_at: Utc::now() + chrono::Duration::days(2),
            email_config: ScheduledEmailConfig {
                template: EmailTemplate::EventReminder {
                    event_name: "test_event".to_string(),
                    event_time: (Utc::now() + chrono::Duration::days(1)).to_string(),
                    event_link: "https://test.com".to_string(),
                    organization_name: "Test org".to_string(),
                    organization_email: None,
                },
            },
        };
        let params_3 = CreateScheduledEmail {
            user_email: "user-1@test.com".to_string(),
            send_at: Utc::now() + chrono::Duration::days(2),
            email_config: ScheduledEmailConfig {
                template: EmailTemplate::EventReminder {
                    event_name: "test_event".to_string(),
                    event_time: (Utc::now() + chrono::Duration::days(1)).to_string(),
                    event_link: "https://test.com".to_string(),
                    organization_name: "Test org".to_string(),
                    organization_email: None,
                },
            },
        };

        let email_1 = create(&pool, params_1).await?;
        create(&pool, params_2).await?;
        let email_3 = create(&pool, params_3).await?;

        let page_options = PageOptions {
            offset: None,
            limit: None,
        };
        let filter_options = ScheduledEmailFilterOptions {
            user_email: Some("user-1@test.com".to_string()),
        };
        let order_options = ScheduledEmailOrderOptions {
            ..Default::default()
        };
        let results = list(&pool, page_options, filter_options, order_options).await?;

        assert_eq!(results.total, 2, "incorrect total");
        assert!(
            results.records.iter().any(|e| e.id == email_1.id),
            "missing first email"
        );
        assert!(
            results.records.iter().any(|e| e.id == email_3.id),
            "missing third email"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_list_scheduled_emails_upcoming_2_hours(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let params_23_hours = CreateScheduledEmail {
            user_email: "user-1@test.com".to_string(),
            send_at: Utc::now() + chrono::Duration::hours(23),
            email_config: ScheduledEmailConfig {
                template: EmailTemplate::EventReminder {
                    event_name: "test_event".to_string(),
                    event_time: (Utc::now() + chrono::Duration::days(1)).to_string(),
                    event_link: "https://test.com".to_string(),
                    organization_name: "Test org".to_string(),
                    organization_email: None,
                },
            },
        };
        let params_2_days = CreateScheduledEmail {
            user_email: "user-2@test.com".to_string(),
            send_at: Utc::now() + chrono::Duration::days(2),
            email_config: ScheduledEmailConfig {
                template: EmailTemplate::EventReminder {
                    event_name: "test_event".to_string(),
                    event_time: (Utc::now() + chrono::Duration::days(1)).to_string(),
                    event_link: "https://test.com".to_string(),
                    organization_name: "Test org".to_string(),
                    organization_email: None,
                },
            },
        };
        let params_past = CreateScheduledEmail {
            user_email: "user-1@test.com".to_string(),
            send_at: Utc::now() - chrono::Duration::days(1),
            email_config: ScheduledEmailConfig {
                template: EmailTemplate::EventReminder {
                    event_name: "test_event".to_string(),
                    event_time: (Utc::now() + chrono::Duration::days(1)).to_string(),
                    event_link: "https://test.com".to_string(),
                    organization_name: "Test org".to_string(),
                    organization_email: None,
                },
            },
        };
        let params_sent = CreateScheduledEmail {
            user_email: "user-1@test.com".to_string(),
            send_at: Utc::now() + chrono::Duration::minutes(15),
            email_config: ScheduledEmailConfig {
                template: EmailTemplate::EventReminder {
                    event_name: "test_event".to_string(),
                    event_time: (Utc::now() + chrono::Duration::days(1)).to_string(),
                    event_link: "https://test.com".to_string(),
                    organization_name: "Test org".to_string(),
                    organization_email: None,
                },
            },
        };
        let params_30_mins = CreateScheduledEmail {
            user_email: "user-1@test.com".to_string(),
            send_at: Utc::now() + chrono::Duration::minutes(30),
            email_config: ScheduledEmailConfig {
                template: EmailTemplate::EventReminder {
                    event_name: "test_event".to_string(),
                    event_time: (Utc::now() + chrono::Duration::days(1)).to_string(),
                    event_link: "https://test.com".to_string(),
                    organization_name: "Test org".to_string(),
                    organization_email: None,
                },
            },
        };

        create(&pool, params_23_hours).await?;
        create(&pool, params_2_days).await?;
        create(&pool, params_past).await?;
        let email_sent = create(&pool, params_sent).await?;
        let update_params = UpdateScheduledEmail {
            status: Some(EmailStatus::Sent),
        };
        update(&pool, email_sent.id, update_params).await?;

        let email_30_mins = create(&pool, params_30_mins).await?;

        let results_2_hours = list_upcoming_emails(&pool, chrono::Duration::hours(2)).await?;

        assert_eq!(results_2_hours.len(), 1, "incorrect total for 2 hours");
        assert!(
            results_2_hours.iter().all(|e| e.id == email_30_mins.id),
            "incorrect id"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_delete_scheduled_email_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let params = CreateScheduledEmail {
            user_email: "test@test.com".to_string(),
            send_at: Utc::now() + chrono::Duration::days(1),
            email_config: ScheduledEmailConfig {
                template: EmailTemplate::EventReminder {
                    event_name: "test_event".to_string(),
                    event_time: (Utc::now() + chrono::Duration::days(1)).to_string(),
                    event_link: "https://test.com".to_string(),
                    organization_name: "Test org".to_string(),
                    organization_email: None,
                },
            },
        };

        let created_email = create(&pool, params).await?;

        delete(&pool, created_email.id).await?;

        let err = get_by_id(&pool, created_email.id).await.unwrap_err();

        match err {
            ComhairleError::ResourceNotFound(e) => {
                assert_eq!(e, "Scheduled email".to_string(), "incorrect error message");
            }
            _ => panic!("Expected ResourceNotFound error"),
        }

        Ok(())
    }
}
