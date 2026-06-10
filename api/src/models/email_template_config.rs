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

use crate::{error::ComhairleError, models::users};

#[derive(Serialize, Deserialize, Debug, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "email_template_config")]
pub struct EmailTemplateConfig {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub organization_id: Option<Uuid>,
    pub email_type: String,
    pub slots: EmailTemplateSlots,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EmailTemplateSlots {
    ConversationInvite(DefaultEmailSlots),
    EventRegistrationConfirmation(DefaultEmailSlots),
}

impl EmailTemplateSlots {
    fn to_template(&self) -> &str {
        match self {
            EmailTemplateSlots::ConversationInvite(_) => "conversation_invite.html",
            EmailTemplateSlots::EventRegistrationConfirmation(_) => "event_confirmation.html",
        }
    }
}

impl std::fmt::Display for EmailTemplateSlots {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            EmailTemplateSlots::ConversationInvite(_) => "conversation_invite",
            EmailTemplateSlots::EventRegistrationConfirmation(_) => {
                "event_registration_confirmation"
            }
        };
        write!(f, "{}", value)
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct DefaultEmailSlots {
    pub heading: String,
    pub intro: String,
    pub body: String,
    pub footer: String,
}

impl Type<Postgres> for EmailTemplateSlots {
    fn type_info() -> PgTypeInfo {
        <serde_json::Value as Type<Postgres>>::type_info()
    }
}

impl<'q> Encode<'q, Postgres> for EmailTemplateSlots {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<IsNull, sqlx::error::BoxDynError> {
        let json = serde_json::to_value(self)?;
        <serde_json::Value as Encode<Postgres>>::encode(json, buf)
    }
}

impl<'r> Decode<'r, Postgres> for EmailTemplateSlots {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let json: serde_json::Value = Decode::<Postgres>::decode(value)?;
        Ok(serde_json::from_value(json)?)
    }
}

const DEFAULT_COLUMNS: [EmailTemplateConfigIden; 7] = [
    EmailTemplateConfigIden::Id,
    EmailTemplateConfigIden::OwnerId,
    EmailTemplateConfigIden::OrganizationId,
    EmailTemplateConfigIden::EmailType,
    EmailTemplateConfigIden::Slots,
    EmailTemplateConfigIden::CreatedAt,
    EmailTemplateConfigIden::UpdatedAt,
];

#[derive(Deserialize, JsonSchema, Debug)]
pub struct CreateEmailTemplateConfig {
    pub slots: EmailTemplateSlots,
}

#[instrument(err(Debug))]
pub async fn create(
    db: &PgPool,
    user_id: &Uuid,
    params: &CreateEmailTemplateConfig,
) -> Result<EmailTemplateConfig, ComhairleError> {
    let slots_json = serde_json::to_value(&params.slots)?;

    let mut columns = vec![
        EmailTemplateConfigIden::Slots,
        EmailTemplateConfigIden::OwnerId,
    ];
    let mut values = vec![slots_json.into(), (*user_id).into()];

    columns.push(EmailTemplateConfigIden::EmailType);
    values.push(params.slots.to_string().into());

    let user = users::get_user_by_id(user_id, db).await?;

    if let Some(organization_id) = user.organization_id {
        columns.push(EmailTemplateConfigIden::OrganizationId);
        values.push(organization_id.into());
    }

    let (sql, values) = Query::insert()
        .into_table(EmailTemplateConfigIden::Table)
        .columns(columns)
        .values(values)?
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let email_config = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(email_config)
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct UpdateEmailTemplateConfig {
    slots: Option<EmailTemplateSlots>,
}

impl UpdateEmailTemplateConfig {
    fn try_to_values(&self) -> Result<Vec<(EmailTemplateConfigIden, SimpleExpr)>, ComhairleError> {
        let mut values = vec![];
        if let Some(value) = &self.slots {
            let slots_json = serde_json::to_value(value)?;
            values.push((EmailTemplateConfigIden::Slots, slots_json.into()));
        }

        Ok(values)
    }
}

#[instrument(err(Debug))]
pub async fn update(
    db: &PgPool,
    id: Uuid,
    params: &UpdateEmailTemplateConfig,
) -> Result<EmailTemplateConfig, ComhairleError> {
    let values = params.try_to_values()?;

    if values.is_empty() {
        return Err(ComhairleError::NoValidUpdates);
    }

    let (sql, values) = Query::update()
        .table(EmailTemplateConfigIden::Table)
        .values(values)
        .and_where(Expr::col(EmailTemplateConfigIden::Id).eq(id))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let email_config = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(email_config)
}

#[instrument(err(Debug))]
pub async fn get_by_id(db: &PgPool, id: Uuid) -> Result<EmailTemplateConfig, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(EmailTemplateConfigIden::Table)
        .and_where(Expr::col(EmailTemplateConfigIden::Id).eq(id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let email_config = sqlx::query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                ComhairleError::ResourceNotFound("Email template config".into())
            }
            other => ComhairleError::DatabaseError(other),
        })?;

    Ok(email_config)
}

#[derive(Deserialize, Debug, JsonSchema, Default)]
pub struct EmailTemplateConfigFilterOptions {
    pub owner_id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
}

impl EmailTemplateConfigFilterOptions {
    fn apply(&self, mut query: SelectStatement) -> SelectStatement {
        if let Some(owner_id) = self.owner_id {
            query = query
                .and_where(Expr::col(EmailTemplateConfigIden::OwnerId).eq(owner_id))
                .to_owned();
        }
        if let Some(organization_id) = self.organization_id {
            query = query
                .and_where(Expr::col(EmailTemplateConfigIden::OrganizationId).eq(organization_id))
                .to_owned();
        }

        query
    }
}

#[instrument(err(Debug))]
pub async fn list(
    db: &PgPool,
    filter_options: EmailTemplateConfigFilterOptions,
) -> Result<Vec<EmailTemplateConfig>, ComhairleError> {
    let query = Query::select()
        .from(EmailTemplateConfigIden::Table)
        .columns(DEFAULT_COLUMNS)
        .to_owned();

    let query = filter_options.apply(query);

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let email_configs = query_as_with(&sql, values).fetch_all(db).await?;

    Ok(email_configs)
}

#[instrument(err(Debug))]
pub async fn delete(db: &PgPool, id: Uuid) -> Result<EmailTemplateConfig, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(EmailTemplateConfigIden::Table)
        .and_where(Expr::col(EmailTemplateConfigIden::Id).eq(id))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let email_config = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(email_config)
}

#[cfg(test)]
mod tests {
    use crate::models::model_test_helpers::setup_default_app_and_session;

    use super::*;

    use std::error::Error;

    #[sqlx::test]
    async fn should_create_email_config(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, current_user, _) = session.current_user(&app).await?;

        let params = CreateEmailTemplateConfig {
            slots: EmailTemplateSlots::ConversationInvite(DefaultEmailSlots {
                heading: "<h1>You're invite to a conversation</h1>".to_string(),
                intro: "<p>You have been selected to take part in a public engagement</p>"
                    .to_string(),
                body: "<p>Test body content</p>".to_string(),
                footer: "<p>Thank you for your time</p>".to_string(),
            }),
        };

        let email_config = create(&pool, &current_user.id, &params).await?;

        assert_eq!(
            email_config.owner_id, current_user.id,
            "owner_id doesn't match"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_update_email_config(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, current_user, _) = session.current_user(&app).await?;

        let create_slots = EmailTemplateSlots::ConversationInvite(DefaultEmailSlots {
            heading: "<h1>You're invite to a conversation</h1>".to_string(),
            intro: "<p>You have been selected to take part in a public engagement</p>".to_string(),
            body: "<p>Test body content</p>".to_string(),
            footer: "<p>Thank you for your time</p>".to_string(),
        });

        let params = CreateEmailTemplateConfig {
            slots: create_slots.clone(),
        };

        let new_email_config = create(&pool, &current_user.id, &params).await?;

        assert_eq!(
            new_email_config.slots, create_slots,
            "incorrect slots before update"
        );

        let update_slots = EmailTemplateSlots::ConversationInvite(DefaultEmailSlots {
            heading: "<h1>You're not invite to a conversation</h1>".to_string(),
            intro: "<p>You have not been selected to take part in a public engagement</p>"
                .to_string(),
            body: "<p>Test body content</p>".to_string(),
            footer: "<p>Thank you for your time</p>".to_string(),
        });

        let email_config = update(
            &pool,
            new_email_config.id,
            &UpdateEmailTemplateConfig {
                slots: Some(update_slots.clone()),
            },
        )
        .await?;

        assert_eq!(
            email_config.slots, update_slots,
            "incorrect slots after update"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_get_email_config_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, current_user, _) = session.current_user(&app).await?;

        let params = CreateEmailTemplateConfig {
            slots: EmailTemplateSlots::ConversationInvite(DefaultEmailSlots {
                heading: "<h1>You're invite to a conversation</h1>".to_string(),
                intro: "<p>You have been selected to take part in a public engagement</p>"
                    .to_string(),
                body: "<p>Test body content</p>".to_string(),
                footer: "<p>Thank you for your time</p>".to_string(),
            }),
        };

        let new_email_config = create(&pool, &current_user.id, &params).await?;

        let email_config = get_by_id(&pool, new_email_config.id).await?;

        assert_eq!(new_email_config.id, email_config.id, "ids don't match");

        Ok(())
    }

    #[sqlx::test]
    async fn should_list_email_configs(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, user_a, _) = session.current_user(&app).await?;

        let user_b = users::create_annon_user(&pool).await?;

        let default_slots = DefaultEmailSlots {
            heading: "<h1>Test heading</h1>".to_string(),
            intro: "<p>Test intro</p>".to_string(),
            body: "<p>Test body</p>".to_string(),
            footer: "<p>Test footer</p>".to_string(),
        };

        let params_a = CreateEmailTemplateConfig {
            slots: EmailTemplateSlots::EventRegistrationConfirmation(default_slots.clone()),
        };
        create(&pool, &user_a.id, &params_a).await?;
        let params_b = CreateEmailTemplateConfig {
            slots: EmailTemplateSlots::ConversationInvite(default_slots.clone()),
        };
        create(&pool, &user_b.id, &params_b).await?;

        let filter_options = EmailTemplateConfigFilterOptions {
            owner_id: Some(user_b.id),
            ..Default::default()
        };
        let email_configs = list(&pool, filter_options).await?;

        assert_eq!(email_configs.len(), 1, "incorrect total");
        assert!(
            !email_configs.iter().any(|c| c.owner_id == user_a.id),
            "user_a incorrectly included"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_delete_email_config(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, current_user, _) = session.current_user(&app).await?;

        let params = CreateEmailTemplateConfig {
            slots: EmailTemplateSlots::ConversationInvite(DefaultEmailSlots {
                heading: "<h1>You're invite to a conversation</h1>".to_string(),
                intro: "<p>You have been selected to take part in a public engagement</p>"
                    .to_string(),
                body: "<p>Test body content</p>".to_string(),
                footer: "<p>Thank you for your time</p>".to_string(),
            }),
        };

        let email_config = create(&pool, &current_user.id, &params).await?;

        delete(&pool, email_config.id).await?;

        let err = get_by_id(&pool, email_config.id).await.unwrap_err();

        match err {
            ComhairleError::ResourceNotFound(e) => {
                assert_eq!(
                    e,
                    "Email template config".to_string(),
                    "incorrect error message"
                );
            }
            _ => panic!("Expected ResourceNotFound error"),
        }

        Ok(())
    }
}
