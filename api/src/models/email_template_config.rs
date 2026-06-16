use std::collections::HashMap;

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

use crate::{
    error::ComhairleError,
    models::{users, SqlxResultExt},
};

/// A client-configured email template, persisted to the `email_template_config` table.
///
/// Each record belongs to an owner and optionally an organization, and holds
/// the configured slot content for a particular email type via [`EmailTemplateSlots`].
/// The `slots` field determines which email template will be rendered and what
/// content will be injected into it.
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

/// The configurable slot content for a particular email template type.
///
/// Each variant corresponds to a distinct email template and carries the slot
/// content that will be injected into it at send time. Serialised as a tagged
/// JSON object (e.g. `{ "type": "conversation_invite", "heading": "...", ... }`)
/// for storage in the `slots` JSONB column on [`EmailTemplateConfig`].
///
/// # Adding a new email type
///
/// 1. Add a variant here with its slot struct.
/// 2. Add the corresponding HTML template.
/// 3. Update [`EmailTemplateSlots::schemas`] with the new variant's schema.
/// 4. Update [`EmailTemplateSlots::to_template`] to return the template filename.
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EmailTemplateSlots {
    ConversationInvite(DefaultEmailSlots),
    EventRegistrationInvite(DefaultEmailSlots),
    EventRegistrationConfirmation(DefaultEmailSlots),
}

pub const TYPE_CONVERSATION_INVITE: &str = "conversation_invite";
pub const TYPE_EVENT_REGISTRATION_INVITE: &str = "event_registration_invite";
pub const TYPE_EVENT_REGISTRATION_CONFIRMATION: &str = "event_registration_confirmation";

impl EmailTemplateSlots {
    /// Returns the schema for every email template type.
    ///
    /// Each entry describes a variant of [`EmailTemplateSlots`], pairing the
    /// variant's string identifier with the [`SlotDefinition`]s that define its
    /// configurable slots. This is intended to be served to the frontend so it
    /// can render the correct form fields when a user selects an email type to
    /// configure.
    ///
    /// # Adding a new email type
    ///
    /// When a new variant is added to [`EmailTemplateSlots`], a corresponding
    /// [`EmailTypeSchema`] must be added here manually. There is no compiler
    /// enforcement for this.
    pub fn schemas() -> &'static [EmailTypeSchema] {
        &[
            EmailTypeSchema {
                email_type: TYPE_CONVERSATION_INVITE,
                slots: DEFAULT_SLOTS_SCHEMA,
            },
            EmailTypeSchema {
                email_type: TYPE_EVENT_REGISTRATION_INVITE,
                slots: DEFAULT_SLOTS_SCHEMA,
            },
            EmailTypeSchema {
                email_type: TYPE_EVENT_REGISTRATION_CONFIRMATION,
                slots: DEFAULT_SLOTS_SCHEMA,
            },
        ]
    }

    pub fn to_template(&self) -> &str {
        match self {
            EmailTemplateSlots::ConversationInvite(_) => "conversation_invite.html",
            EmailTemplateSlots::EventRegistrationInvite(_) => "event_registration_invite.html",
            EmailTemplateSlots::EventRegistrationConfirmation(_) => "event_confirmation.html",
        }
    }

    pub fn schema(&self) -> &'static [SlotSchemaDefinition] {
        match self {
            EmailTemplateSlots::ConversationInvite(_) => DEFAULT_SLOTS_SCHEMA,
            EmailTemplateSlots::EventRegistrationInvite(_) => DEFAULT_SLOTS_SCHEMA,
            EmailTemplateSlots::EventRegistrationConfirmation(_) => DEFAULT_SLOTS_SCHEMA,
        }
    }

    /// Converts the email template slots into a `HashMap` of key/value pairs
    /// suitable for use as a minijinja template context.
    ///
    /// The returned map contains the slot field names (e.g. `heading`, `intro`,
    /// `body`, `footer`) and their corresponding values, which are used to
    /// populate the variables referenced in the email's minijinja template.
    ///
    /// Because `minijinja::context!` supports spreading a `HashMap` with `..`,
    /// the map returned here can be combined with additional ad-hoc context
    /// variables that aren't stored as part of the email config (e.g. dynamic
    /// links or IDs generated at send time). For example:
    ///
    /// ```ignore
    /// let base = email_config.slots.to_mailer_map();
    /// let context = minijinja::context! { invite_link => "foo@bar.com", ..base };
    /// ```
    pub fn to_mailer_map(&self) -> HashMap<&str, String> {
        match self {
            EmailTemplateSlots::ConversationInvite(slots) => slots.to_mailer_map(),
            EmailTemplateSlots::EventRegistrationInvite(slots) => slots.to_mailer_map(),
            EmailTemplateSlots::EventRegistrationConfirmation(slots) => slots.to_mailer_map(),
        }
    }
}

impl std::fmt::Display for EmailTemplateSlots {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            EmailTemplateSlots::ConversationInvite(_) => TYPE_CONVERSATION_INVITE,
            EmailTemplateSlots::EventRegistrationInvite(_) => TYPE_EVENT_REGISTRATION_INVITE,
            EmailTemplateSlots::EventRegistrationConfirmation(_) => {
                TYPE_EVENT_REGISTRATION_CONFIRMATION
            }
        };
        write!(f, "{}", value)
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Default)]
pub struct DefaultEmailSlots {
    pub heading: String,
    pub intro: String,
    pub body: String,
    pub footer: String,
}

impl DefaultEmailSlots {
    fn to_mailer_map(&self) -> HashMap<&str, String> {
        HashMap::from([
            ("heading", self.heading.clone()),
            ("intro", self.intro.clone()),
            ("body", self.body.clone()),
            ("footer", self.footer.clone()),
        ])
    }
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

/// Metadata describing a single configurable slot in an email template.
///
/// `SlotSchemaDefinition` is used to communicate the structure of an email template
/// to the frontend, allowing it to render the correct form fields when a user
/// is configuring a particular email type. It is the UI-facing counterpart to
/// the typed fields on structs like [`DefaultEmailSlots`].
#[derive(Serialize, JsonSchema, Debug, Clone)]
pub struct SlotSchemaDefinition {
    /// The key used to identify this slot, matching the field name on the
    /// corresponding slots struct (e.g. `"heading"`, `"body"`).
    pub key: &'static str,
    /// A human-readable label for display in the configuration UI.
    pub label: &'static str,
    /// A short description shown to the user explaining what this slot
    /// controls and any guidance on what to write.
    pub hint: &'static str,
    /// Whether this slot must be populated before the config can be saved.
    pub required: bool,
    /// An optional upper bound on the number of characters allowed in this
    /// slot. Used for both frontend validation and server-side validation
    /// before persisting. `None` means no limit is enforced.
    pub max_chars: Option<usize>,
}

#[derive(Serialize, JsonSchema, Debug, Clone)]
pub struct EmailTypeSchema {
    pub email_type: &'static str,
    pub slots: &'static [SlotSchemaDefinition],
}

const DEFAULT_SLOTS_SCHEMA: &[SlotSchemaDefinition] = &[
    SlotSchemaDefinition {
        key: "heading",
        label: "Heading",
        hint: "The email heading",
        required: true,
        max_chars: Some(100),
    },
    SlotSchemaDefinition {
        key: "intro",
        label: "Intro",
        hint: "Opening paragraph",
        required: true,
        max_chars: Some(100),
    },
    SlotSchemaDefinition {
        key: "body",
        label: "Body",
        hint: "Main email content",
        required: true,
        max_chars: None,
    },
    SlotSchemaDefinition {
        key: "footer",
        label: "Footer",
        hint: "Closing line",
        required: false,
        max_chars: Some(100),
    },
];

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
    user_id: Uuid,
    params: &CreateEmailTemplateConfig,
) -> Result<EmailTemplateConfig, ComhairleError> {
    let slots_json = serde_json::to_value(&params.slots)?;

    let mut columns = vec![
        EmailTemplateConfigIden::Slots,
        EmailTemplateConfigIden::OwnerId,
    ];
    let mut values = vec![slots_json.into(), user_id.into()];

    columns.push(EmailTemplateConfigIden::EmailType);
    values.push(params.slots.to_string().into());

    let user = users::get_user_by_id(&user_id, db).await?;

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
        .not_found_as("Email template config")?;

    Ok(email_config)
}

#[instrument(err(Debug))]
pub async fn get_by_type_user(
    db: &PgPool,
    user_id: Uuid,
    email_type: &str,
) -> Result<EmailTemplateConfig, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(EmailTemplateConfigIden::Table)
        .and_where(Expr::col(EmailTemplateConfigIden::OwnerId).eq(user_id))
        .and_where(Expr::col(EmailTemplateConfigIden::EmailType).eq(email_type.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let email_config = query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .not_found_as("Email template config")?;

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

        let email_config = create(&pool, current_user.id, &params).await?;

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

        let new_email_config = create(&pool, current_user.id, &params).await?;

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

        let new_email_config = create(&pool, current_user.id, &params).await?;

        let email_config = get_by_id(&pool, new_email_config.id).await?;

        assert_eq!(new_email_config.id, email_config.id, "ids don't match");

        Ok(())
    }

    #[sqlx::test]
    async fn should_get_email_config_user_and_email_type(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
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

        let new_email_config = create(&pool, current_user.id, &params).await?;

        let email_config =
            get_by_type_user(&pool, current_user.id, TYPE_CONVERSATION_INVITE).await?;

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
        create(&pool, user_a.id, &params_a).await?;
        let params_b = CreateEmailTemplateConfig {
            slots: EmailTemplateSlots::ConversationInvite(default_slots.clone()),
        };
        create(&pool, user_b.id, &params_b).await?;

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

        let email_config = create(&pool, current_user.id, &params).await?;

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
