use std::collections::HashMap;

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{
    Alias, Cond, Expr, Func, JoinType, Order, PostgresQueryBuilder, Query, SelectStatement,
    SimpleExpr, enum_def,
};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::encode::IsNull;
use sqlx::prelude::{FromRow, Type};
use sqlx::{Decode, Encode, PgPool, Postgres, query_as_with};
use sqlx_postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef};
use strum::EnumCount as _;
use strum_macros::EnumCount;
use tracing::instrument;
use uuid::Uuid;

use crate::error::ComhairleError;
use crate::models::conversation::ConversationIden;
use crate::models::translations::{TextContentIden, TextTranslationIden};
use crate::models::{SqlxResultExt, users};

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
    pub email_type: EmailType,
    pub slots: EmailTemplateSlots,
    pub subject: Option<String>,
    pub conversation_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone, PartialEq, PartialOrd, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum EmailType {
    ConversationInvite,
    EventRegistrationInvite,
    EventRegistrationConfirmation,
    EventReminder,
}

impl From<EmailType> for sea_query::Value {
    fn from(val: EmailType) -> Self {
        sea_query::Value::String(Some(Box::new(val.to_string())))
    }
}

impl std::fmt::Display for EmailType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            EmailType::ConversationInvite => "conversation_invite",
            EmailType::EventRegistrationInvite => "event_registration_invite",
            EmailType::EventRegistrationConfirmation => "event_registration_confirmation",
            EmailType::EventReminder => "event_reminder",
        };
        write!(f, "{}", value)
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

const DEFAULT_COLUMNS: [EmailTemplateConfigIden; 9] = [
    EmailTemplateConfigIden::Id,
    EmailTemplateConfigIden::OwnerId,
    EmailTemplateConfigIden::OrganizationId,
    EmailTemplateConfigIden::EmailType,
    EmailTemplateConfigIden::Slots,
    EmailTemplateConfigIden::Subject,
    EmailTemplateConfigIden::ConversationId,
    EmailTemplateConfigIden::CreatedAt,
    EmailTemplateConfigIden::UpdatedAt,
];

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
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, EnumCount)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EmailTemplateSlots {
    ConversationInvite(DefaultEmailSlots),
    EventRegistrationInvite(DefaultEmailSlots),
    EventRegistrationConfirmation(DefaultEmailSlots),
    EventReminder(DefaultEmailSlots),
}

impl EmailTemplateSlots {
    /// Returns the schema for every email template type.
    ///
    /// Each entry describes a variant of [`EmailTemplateSlots`], pairing the
    /// variant's string identifier with the [`SlotDefinition`]s that define its
    /// configurable slots. This is intended to be served to the frontend so it
    /// can render the correct form fields and default content when a user
    /// selects an email type to configure.
    pub fn schemas() -> [EmailTypeSchema; EmailTemplateSlots::COUNT] {
        [
            EmailTemplateSlots::ConversationInvite(DefaultEmailSlots::default()).schema(),
            EmailTemplateSlots::EventRegistrationInvite(DefaultEmailSlots::default()).schema(),
            EmailTemplateSlots::EventRegistrationConfirmation(DefaultEmailSlots::default())
                .schema(),
            EmailTemplateSlots::EventReminder(DefaultEmailSlots::default()).schema(),
        ]
    }

    /// Returns the schema for the associated email template type.
    pub fn schema(&self) -> EmailTypeSchema {
        match self {
            EmailTemplateSlots::ConversationInvite(_) => SCHEMA_CONVERSATION_INVITE,
            EmailTemplateSlots::EventRegistrationInvite(_) => SCHEMA_EVENT_REGISTRATION_INVITE,
            EmailTemplateSlots::EventRegistrationConfirmation(_) => {
                SCHEMA_EVENT_REGISTRATION_CONFIRMATION
            }
            EmailTemplateSlots::EventReminder(_) => SCHEMA_EVENT_REMINDER,
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
    /// let base = email_config.slots.mailer_context_map();
    /// let context = minijinja::context! { invite_link => "foo@bar.com", ..base };
    /// ```
    pub fn mailer_context_map(&self) -> HashMap<String, String> {
        match self {
            EmailTemplateSlots::ConversationInvite(slots) => slots.mailer_context_map(),
            EmailTemplateSlots::EventRegistrationInvite(slots) => slots.mailer_context_map(),
            EmailTemplateSlots::EventRegistrationConfirmation(slots) => slots.mailer_context_map(),
            EmailTemplateSlots::EventReminder(slots) => slots.mailer_context_map(),
        }
    }

    /// Returns a map of placeholder values for runtime template variables,
    /// used when previewing a customised [`EmailTemplateConfig`] in the frontend.
    ///
    /// When users compose email content they can embed dynamic variables (e.g.
    /// `{{ conversation_title }}`) that are only available at the point an email
    /// is actually sent. This method provides realistic example values for those
    /// variables so that previews render meaningfully rather than showing empty
    /// or broken output.
    ///
    /// The returned map is specific to each [`EmailTemplateSlots`] variant, as
    /// different email types expose different runtime variables. The values are
    /// illustrative only and are never used in real email sends.
    pub fn preview_variables_map(&self) -> HashMap<String, String> {
        match self {
            EmailTemplateSlots::ConversationInvite(_) => HashMap::from([(
                "conversation_title".to_string(),
                "Renewable energy in rural areas".to_string(),
            )]),
            EmailTemplateSlots::EventRegistrationConfirmation(_) => HashMap::from([
                (
                    "event_name".to_string(),
                    "Prioritising accessibility in websites".to_string(),
                ),
                ("event_time".to_string(), "24 May, 2026".to_string()),
                (
                    "invite_link".to_string(),
                    "https://crown-shy.com/invite".to_string(),
                ),
            ]),
            EmailTemplateSlots::EventRegistrationInvite(_) => HashMap::from([
                (
                    "event_name".to_string(),
                    "Prioritising accessibility in websites".to_string(),
                ),
                ("event_time".to_string(), "24 May, 2026".to_string()),
                (
                    "event_link".to_string(),
                    "https://crown-shy.com/event".to_string(),
                ),
            ]),
            EmailTemplateSlots::EventReminder(_) => HashMap::from([
                (
                    "event_name".to_string(),
                    "Prioritising accessibility in websites".to_string(),
                ),
                ("event_time".to_string(), "24 May, 2026".to_string()),
                (
                    "event_link".to_string(),
                    "https://crown-shy.com/event".to_string(),
                ),
            ]),
        }
    }

    /// Helper method to return HTML template file name for associated email type
    pub fn email_template(&self) -> &str {
        self.schema().template
    }
}

impl std::fmt::Display for EmailTemplateSlots {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.schema().email_type)
    }
}

// ===
//
// Schemas
//
// ===

#[derive(Serialize, JsonSchema, Debug, Clone)]
pub struct EmailTypeSchema {
    pub email_type: EmailType,
    pub template: &'static str,
    pub default_subject: &'static str,
    pub variables: &'static [&'static str],
    pub slots: &'static [SlotSchemaDefinition],
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
    /// Default html used as a starting point for a slot as well as a fallback
    /// if an [`EmailTemplateConfig`] does not exist for this user / email_type.
    pub default_content: &'static str,
    /// Defines whether a slot requires rich text HTML editing or plain text
    /// inserted into an existing HTML tag and is used on the frontend to render
    /// the appropriate input type.
    content_type: ContentType,
}

#[derive(Serialize, JsonSchema, Debug, Clone)]
#[serde(rename_all = "snake_case")]
enum ContentType {
    PlainText,
    RichText,
}

pub const SCHEMA_CONVERSATION_INVITE: EmailTypeSchema = EmailTypeSchema {
    email_type: EmailType::ConversationInvite,
    template: "conversation_invite.html",
    default_subject: "Invitation to take part in a public consultation",
    variables: &["conversation_title"],
    slots: &[
        SlotSchemaDefinition {
            key: "heading",
            label: "Heading",
            hint: "The email heading",
            default_content: "Dear Invited Participant,",
            content_type: ContentType::PlainText,
        },
        SlotSchemaDefinition {
            key: "intro",
            label: "Intro",
            hint: "Opening paragraph",
            default_content: "<p>You have been selected to take part in a public engagement, <strong>{{conversation_title}}</strong>, hosted on the <strong>Comhairle</strong> platform by <strong>CrownShy</strong>.<p />",
            content_type: ContentType::RichText,
        },
        SlotSchemaDefinition {
            key: "body",
            label: "Body",
            hint: "Main email content",
            default_content: "<p>We’re inviting you to complete the online engagement.</p><p>We’re keen to hear your real views and reflections. If you choose to take part, we ask that you:</p><ul><li>Read and consider each question carefully.</li><li>Provide your own honest opinions, not blank or repetitive answers.</li><li>Complete <strong>all sections</strong> of the engagement.</li></ul>",
            content_type: ContentType::RichText,
        },
        SlotSchemaDefinition {
            key: "footer",
            label: "Footer",
            hint: "Closing line",
            default_content: "<p>Thank you very much for your time and contribution to this important process.</p><p>Warm regards, <br /><strong>The CrownShy Team.</strong></p>",
            content_type: ContentType::RichText,
        },
    ],
};

pub const SCHEMA_EVENT_REGISTRATION_INVITE: EmailTypeSchema = EmailTypeSchema {
    email_type: EmailType::EventRegistrationInvite,
    template: "event_registration_invite.html",
    default_subject: "Invitation to take part in an event",
    variables: &["event_name", "event_time", "invite_link"],
    slots: &[
        SlotSchemaDefinition {
            key: "heading",
            label: "Heading",
            hint: "The email heading",
            default_content: "Hello!",
            content_type: ContentType::PlainText,
        },
        SlotSchemaDefinition {
            key: "intro",
            label: "Intro",
            hint: "Opening paragraph",
            default_content: "<p>You are invited to take part in <strong>{{event_name}}</strong> with <strong>CrownShy</strong> to collaborate with them.</p>",
            content_type: ContentType::RichText,
        },
        SlotSchemaDefinition {
            key: "body",
            label: "Body",
            hint: "Main email content",
            default_content: "<p>Click the button below to register.</p>",
            content_type: ContentType::RichText,
        },
        SlotSchemaDefinition {
            key: "footer",
            label: "Footer",
            hint: "Closing line",
            default_content: "<p>For any questions, please contact us at<br /><a href=\"mailto:team@crown-shy.com\">team@crown-shy.com</a></p><p><p>We look forward to your participation!</p><p><strong>CrownShy</strong></p>",
            content_type: ContentType::RichText,
        },
    ],
};

pub const SCHEMA_EVENT_REGISTRATION_CONFIRMATION: EmailTypeSchema = EmailTypeSchema {
    email_type: EmailType::EventRegistrationConfirmation,
    template: "event_confirmation.html",
    default_subject: "Event registration confirmation",
    variables: &["event_name", "event_time", "event_link"],
    slots: &[
        SlotSchemaDefinition {
            key: "heading",
            label: "Heading",
            hint: "The email heading",
            default_content: "You're in!",
            content_type: ContentType::PlainText,
        },
        SlotSchemaDefinition {
            key: "intro",
            label: "Intro",
            hint: "Opening paragraph",
            default_content: "<p>Thank you for registering for <strong>{{event_name}}</strong>.",
            content_type: ContentType::RichText,
        },
        SlotSchemaDefinition {
            key: "body",
            label: "Body",
            hint: "Main email content",
            default_content: "<p>You will be sent a reminder email before the event with instructions on how to join the online meeting.</p>",
            content_type: ContentType::RichText,
        },
        SlotSchemaDefinition {
            key: "footer",
            label: "Footer",
            hint: "Closing line",
            default_content: "<p>For any questions, please contact us at<br /><a href=\"mailto:team@crown-shy.com\">team@crown-shy.com</a></p><p><p>We look forward to seeing you there!</p><p><strong>CrownShy</strong></p>",
            content_type: ContentType::RichText,
        },
    ],
};

pub const SCHEMA_EVENT_REMINDER: EmailTypeSchema = EmailTypeSchema {
    email_type: EmailType::EventReminder,
    template: "event_reminder.html",
    default_subject: "Upcoming event reminder",
    variables: &["event_name", "event_time", "event_link"],
    slots: &[
        SlotSchemaDefinition {
            key: "heading",
            label: "Heading",
            hint: "The email heading",
            default_content: "Reminder!",
            content_type: ContentType::PlainText,
        },
        SlotSchemaDefinition {
            key: "intro",
            label: "Intro",
            hint: "Opening paragraph",
            default_content: "<p>An event you recently registered for is starting soon.</p>",
            content_type: ContentType::RichText,
        },
        SlotSchemaDefinition {
            key: "body",
            label: "Body",
            hint: "Main email content",
            default_content: "<p><strong>{{event_name}}</strong> will begin at {{event_time}}. Click the button below to join the event.</p>",
            content_type: ContentType::RichText,
        },
        SlotSchemaDefinition {
            key: "footer",
            label: "Footer",
            hint: "Closing line",
            default_content: "<p>For any questions, please contact us at<br /><a href=\"mailto:team@crown-shy.com\">team@crown-shy.com</a></p><p>We look forward to seeing you there!</p><p><strong>CrownShy</strong></p>",
            content_type: ContentType::RichText,
        },
    ],
};

/// Converts `self` to [`HashMap`] mapping email template variable keys to their content.
pub trait MailerContextMap {
    fn mailer_context_map(&self) -> HashMap<String, String>;
}

/// Builds the map from each slot's key to its default content.
impl MailerContextMap for [SlotSchemaDefinition] {
    fn mailer_context_map(&self) -> HashMap<String, String> {
        self.iter()
            .map(|slot| (slot.key.to_string(), slot.default_content.to_string()))
            .collect()
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Default)]
pub struct DefaultEmailSlots {
    pub heading: String,
    pub intro: String,
    pub body: String,
    pub footer: String,
}

/// Builds the map from the fixed email slot field names to their current values.
impl MailerContextMap for DefaultEmailSlots {
    fn mailer_context_map(&self) -> HashMap<String, String> {
        HashMap::from([
            ("heading".to_string(), self.heading.clone()),
            ("intro".to_string(), self.intro.clone()),
            ("body".to_string(), self.body.clone()),
            ("footer".to_string(), self.footer.clone()),
        ])
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct CreateEmailTemplateConfig {
    pub slots: EmailTemplateSlots,
    pub subject: Option<String>,
    pub conversation_id: Option<Uuid>,
}

#[instrument(err(Debug), skip(db))]
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

    if let Some(conversation_id) = params.conversation_id {
        columns.push(EmailTemplateConfigIden::ConversationId);
        values.push(conversation_id.into());
    }

    if let Some(subject) = &params.subject {
        columns.push(EmailTemplateConfigIden::Subject);
        values.push(subject.into());
    }

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

    let email_config = query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Email Template Config")?;

    Ok(email_config)
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Default)]
pub struct UpdateEmailTemplateConfig {
    pub slots: Option<EmailTemplateSlots>,
    pub subject: Option<String>,
}

impl UpdateEmailTemplateConfig {
    fn try_to_values(&self) -> Result<Vec<(EmailTemplateConfigIden, SimpleExpr)>, ComhairleError> {
        let mut values = vec![];
        if let Some(value) = &self.slots {
            let slots_json = serde_json::to_value(value)?;
            values.push((EmailTemplateConfigIden::Slots, slots_json.into()));
        }

        if let Some(value) = &self.subject {
            values.push((EmailTemplateConfigIden::Subject, value.into()))
        }

        Ok(values)
    }
}

#[instrument(err(Debug), skip(db))]
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

#[instrument(err(Debug), skip(db))]
pub async fn get_by_id(db: &PgPool, id: Uuid) -> Result<EmailTemplateConfig, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(EmailTemplateConfigIden::Table)
        .and_where(Expr::col(EmailTemplateConfigIden::Id).eq(id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let email_config = sqlx::query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Email Template Config")?;

    Ok(email_config)
}

/// An [`EmailTemplateConfig`] together with display details of the scope it
/// applies to, so templates of the same email type can be told apart.
///
/// `conversation_title` is `None` when the template is not scoped to a
/// conversation.
#[derive(Debug, FromRow)]
pub struct EmailTemplateConfigWithScope {
    #[sqlx(flatten)]
    pub config: EmailTemplateConfig,
    pub conversation_title: Option<String>,
}

/// Selects email template configs joined with the title of the conversation
/// they are scoped to, in `locale` with a fallback to the conversation's
/// primary locale.
fn select_with_scope(locale: &str) -> SelectStatement {
    let tc = Alias::new("tc_title");
    let tt = Alias::new("tt_title");
    let tt_primary = Alias::new("tt_primary_title");

    Query::select()
        .columns(DEFAULT_COLUMNS.map(|col| (EmailTemplateConfigIden::Table, col)))
        .expr_as(
            Func::coalesce([
                Expr::col((tt.clone(), TextTranslationIden::Content)).into(),
                Expr::col((tt_primary.clone(), TextTranslationIden::Content)).into(),
            ]),
            Alias::new("conversation_title"),
        )
        .from(EmailTemplateConfigIden::Table)
        .left_join(
            ConversationIden::Table,
            Expr::col((
                EmailTemplateConfigIden::Table,
                EmailTemplateConfigIden::ConversationId,
            ))
            .equals((ConversationIden::Table, ConversationIden::Id)),
        )
        .join_as(
            JoinType::LeftJoin,
            TextContentIden::Table,
            tc.clone(),
            Expr::col((ConversationIden::Table, ConversationIden::Title))
                .equals((tc.clone(), TextContentIden::Id)),
        )
        .join_as(
            JoinType::LeftJoin,
            TextTranslationIden::Table,
            tt.clone(),
            Expr::col((tc.clone(), TextContentIden::Id))
                .equals((tt.clone(), TextTranslationIden::ContentId))
                .and(Expr::col((tt.clone(), TextTranslationIden::Locale)).eq(locale)),
        )
        .join_as(
            JoinType::LeftJoin,
            TextTranslationIden::Table,
            tt_primary.clone(),
            Expr::col((tc.clone(), TextContentIden::Id))
                .equals((tt_primary.clone(), TextTranslationIden::ContentId))
                .and(
                    Expr::col((tt_primary.clone(), TextTranslationIden::Locale))
                        .equals((tc.clone(), TextContentIden::PrimaryLocale)),
                ),
        )
        .to_owned()
}

/// Gets a single [`EmailTemplateConfig`] by `id`, paired with the title of the
/// conversation it is scoped to.
///
/// The title is returned in `locale` when a translation exists, falling back
/// to the conversation's primary locale, and is `None` for unscoped templates.
///
/// Returns [`ComhairleError::ResourceNotFound`] when no config has this `id`.
#[instrument(err(Debug), skip(db))]
pub async fn get_by_id_with_scope(
    db: &PgPool,
    id: Uuid,
    locale: &str,
) -> Result<EmailTemplateConfigWithScope, ComhairleError> {
    let (sql, values) = select_with_scope(locale)
        .and_where(Expr::col((EmailTemplateConfigIden::Table, EmailTemplateConfigIden::Id)).eq(id))
        .build_sqlx(PostgresQueryBuilder);

    let email_config = query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Email Template Config")?;

    Ok(email_config)
}

/// Finds the [`EmailTemplateConfig`] that should be used when sending an email
/// of `email_type`.
///
/// Resolution falls back from the most specific scope to the least:
///
/// 1. A template scoped to `conversation_id`, regardless of who created it, so
///    every admin sending for that conversation gets the same template.
/// 2. The sender's unscoped default template for `email_type`.
///
/// Returns `None` when neither exists, in which case callers should fall back
/// to the built-in defaults on the email type's [`EmailTypeSchema`].
///
/// Passing `None` for `conversation_id` only considers the sender's default.
#[instrument(err(Debug), skip(db))]
pub async fn get_effective(
    db: &PgPool,
    sender_id: Uuid,
    email_type: &EmailType,
    conversation_id: Option<Uuid>,
) -> Result<Option<EmailTemplateConfig>, ComhairleError> {
    // If not scoped to specific conversation, scope to user
    let owner_default = Cond::all()
        .add(Expr::col(EmailTemplateConfigIden::ConversationId).is_null())
        .add(Expr::col(EmailTemplateConfigIden::OwnerId).eq(sender_id));

    let mut scope = Cond::any().add(owner_default);
    if let Some(conversation_id) = conversation_id {
        scope = scope.add(Expr::col(EmailTemplateConfigIden::ConversationId).eq(conversation_id));
    }

    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(EmailTemplateConfigIden::Table)
        .cond_where(
            Cond::all()
                .add(Expr::col(EmailTemplateConfigIden::EmailType).eq(email_type.to_owned()))
                .add(scope),
        )
        .order_by_expr(
            Expr::col(EmailTemplateConfigIden::ConversationId).is_null(),
            Order::Asc,
        )
        .limit(1)
        .build_sqlx(PostgresQueryBuilder);

    let email_config = query_as_with(&sql, values)
        .fetch_optional(db)
        .await
        .resolve_db_err("Email Template Config")?;

    Ok(email_config)
}

#[derive(Deserialize, Debug, JsonSchema, Default)]
pub struct EmailTemplateConfigFilterOptions {
    pub email_type: Option<EmailType>,
}

impl EmailTemplateConfigFilterOptions {
    fn apply(&self, mut query: SelectStatement) -> SelectStatement {
        if let Some(email_type) = &self.email_type {
            query = query
                .and_where(
                    Expr::col((
                        EmailTemplateConfigIden::Table,
                        EmailTemplateConfigIden::EmailType,
                    ))
                    .eq(email_type.to_owned()),
                )
                .to_owned();
        }

        query
    }
}

#[instrument(err(Debug), skip(db))]
pub async fn list(
    db: &PgPool,
    user_id: &Uuid,
    filter_options: EmailTemplateConfigFilterOptions,
) -> Result<Vec<EmailTemplateConfig>, ComhairleError> {
    let query = Query::select()
        .from(EmailTemplateConfigIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(Expr::col(EmailTemplateConfigIden::OwnerId).eq(user_id.to_owned()))
        .to_owned();

    let query = filter_options.apply(query);

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let email_configs = query_as_with(&sql, values).fetch_all(db).await?;

    Ok(email_configs)
}

/// Lists the email templates owned by `user_id`, each paired with the title of
/// the conversation it is scoped to.
///
/// Unlike [`list`], this joins the conversation and its title translations so
/// the admin list can tell apart several templates of the same email type, such
/// as `conversation_invite` templates scoped to different conversations.
/// Unscoped templates are still returned, with `conversation_title` set to `None`.
///
/// The title is returned in `locale` when a translation exists, falling back to
/// the conversation's primary locale.
///
/// # Arguments
///
/// * `user_id` - Only templates owned by this user are returned.
/// * `filter_options` - Optional filters, such as `email_type`.
/// * `locale` - The preferred locale for `conversation_title`.
#[instrument(err(Debug), skip(db))]
pub async fn list_with_scope(
    db: &PgPool,
    user_id: Uuid,
    filter_options: EmailTemplateConfigFilterOptions,
    locale: &str,
) -> Result<Vec<EmailTemplateConfigWithScope>, ComhairleError> {
    let query = select_with_scope(locale)
        .and_where(
            Expr::col((
                EmailTemplateConfigIden::Table,
                EmailTemplateConfigIden::OwnerId,
            ))
            .eq(user_id),
        )
        .to_owned();

    let query = filter_options.apply(query);

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let email_configs = query_as_with(&sql, values).fetch_all(db).await?;

    Ok(email_configs)
}

#[instrument(err(Debug), skip(db))]
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
    use std::error::Error;

    use fake::{Fake, Faker};

    use super::*;
    use crate::models::conversation;
    use crate::models::model_test_helpers::{
        get_random_conversation_id, setup_default_app_and_session,
    };

    fn conversation_invite_params(conversation_id: Option<Uuid>) -> CreateEmailTemplateConfig {
        CreateEmailTemplateConfig {
            slots: EmailTemplateSlots::ConversationInvite(DefaultEmailSlots::default()),
            subject: None,
            conversation_id,
        }
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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
            subject: None,
            conversation_id: None,
        };

        let email_config = create(&pool, current_user.id, &params).await?;

        assert_eq!(
            email_config.owner_id, current_user.id,
            "owner_id doesn't match"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_conversation_invite_configs_for_different_conversations(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, user, _) = session.current_user(&app).await?;
        let conversation_a = get_random_conversation_id(&app, &mut session).await?;
        let conversation_b = get_random_conversation_id(&app, &mut session).await?;

        let config_a = create(
            &pool,
            user.id,
            &conversation_invite_params(Some(conversation_a)),
        )
        .await?;
        let config_b = create(
            &pool,
            user.id,
            &conversation_invite_params(Some(conversation_b)),
        )
        .await?;

        assert_eq!(
            config_a.conversation_id,
            Some(conversation_a),
            "incorrect conversation_id"
        );
        assert_eq!(
            config_b.conversation_id,
            Some(conversation_b),
            "incorrect conversation_id"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_owner_default_alongside_conversation_scoped_config(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, user, _) = session.current_user(&app).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        create(
            &pool,
            user.id,
            &conversation_invite_params(Some(conversation_id)),
        )
        .await?;
        let default_config = create(&pool, user.id, &conversation_invite_params(None)).await?;

        assert_eq!(
            default_config.conversation_id, None,
            "default should be unscoped"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_reject_duplicate_config_for_same_conversation(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, user, _) = session.current_user(&app).await?;
        let other_user = users::create_user(&Faker.fake(), &pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        create(
            &pool,
            user.id,
            &conversation_invite_params(Some(conversation_id)),
        )
        .await?;
        let err = create(
            &pool,
            other_user.id,
            &conversation_invite_params(Some(conversation_id)),
        )
        .await
        .unwrap_err();

        assert!(
            matches!(err, ComhairleError::Conflict(_)),
            "expected Conflict, got {err:?}"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_reject_duplicate_owner_default_config(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, user, _) = session.current_user(&app).await?;

        create(&pool, user.id, &conversation_invite_params(None)).await?;
        let err = create(&pool, user.id, &conversation_invite_params(None))
            .await
            .unwrap_err();

        assert!(
            matches!(err, ComhairleError::Conflict(_)),
            "expected Conflict, got {err:?}"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_effective_conversation_config_over_owner_default(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, user, _) = session.current_user(&app).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        create(&pool, user.id, &conversation_invite_params(None)).await?;
        let scoped = create(
            &pool,
            user.id,
            &conversation_invite_params(Some(conversation_id)),
        )
        .await?;

        let effective = get_effective(
            &pool,
            user.id,
            &EmailType::ConversationInvite,
            Some(conversation_id),
        )
        .await?
        .ok_or("expected a config")?;

        assert_eq!(
            effective.id, scoped.id,
            "expected conversation scoped config"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_effective_owner_default_for_unscoped_conversation(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, user, _) = session.current_user(&app).await?;
        let scoped_conversation = get_random_conversation_id(&app, &mut session).await?;
        let unscoped_conversation = get_random_conversation_id(&app, &mut session).await?;

        create(
            &pool,
            user.id,
            &conversation_invite_params(Some(scoped_conversation)),
        )
        .await?;
        let default_config = create(&pool, user.id, &conversation_invite_params(None)).await?;

        let effective = get_effective(
            &pool,
            user.id,
            &EmailType::ConversationInvite,
            Some(unscoped_conversation),
        )
        .await?
        .ok_or("expected a config")?;

        assert_eq!(
            effective.id, default_config.id,
            "expected owner default config"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_effective_conversation_config_for_any_sender(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, user, _) = session.current_user(&app).await?;
        let other_user = users::create_user(&Faker.fake(), &pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let scoped = create(
            &pool,
            user.id,
            &conversation_invite_params(Some(conversation_id)),
        )
        .await?;
        create(&pool, other_user.id, &conversation_invite_params(None)).await?;

        let effective = get_effective(
            &pool,
            other_user.id,
            &EmailType::ConversationInvite,
            Some(conversation_id),
        )
        .await?
        .ok_or("expected a config")?;

        assert_eq!(
            effective.id, scoped.id,
            "expected conversation scoped config"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_not_get_effective_another_users_default(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, user, _) = session.current_user(&app).await?;
        let other_user = users::create_user(&Faker.fake(), &pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        create(&pool, other_user.id, &conversation_invite_params(None)).await?;

        let effective = get_effective(
            &pool,
            user.id,
            &EmailType::ConversationInvite,
            Some(conversation_id),
        )
        .await?;

        assert!(effective.is_none(), "should not use another user's default");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_effective_owner_default_without_conversation_id(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, user, _) = session.current_user(&app).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        create(
            &pool,
            user.id,
            &conversation_invite_params(Some(conversation_id)),
        )
        .await?;
        let default_config = create(&pool, user.id, &conversation_invite_params(None)).await?;

        let effective = get_effective(&pool, user.id, &EmailType::ConversationInvite, None)
            .await?
            .ok_or("expected a config")?;

        assert_eq!(
            effective.id, default_config.id,
            "expected owner default config"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_not_get_effective_config_of_another_email_type(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, user, _) = session.current_user(&app).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        create(
            &pool,
            user.id,
            &CreateEmailTemplateConfig {
                slots: EmailTemplateSlots::EventReminder(DefaultEmailSlots::default()),
                subject: None,
                conversation_id: Some(conversation_id),
            },
        )
        .await?;

        let effective = get_effective(
            &pool,
            user.id,
            &EmailType::ConversationInvite,
            Some(conversation_id),
        )
        .await?;

        assert!(
            effective.is_none(),
            "should not match a different email_type"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_effective_none_when_no_configs_exist(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, user, _) = session.current_user(&app).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let effective = get_effective(
            &pool,
            user.id,
            &EmailType::ConversationInvite,
            Some(conversation_id),
        )
        .await?;

        assert!(effective.is_none(), "expected no config");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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
            subject: None,
            conversation_id: None,
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
                ..Default::default()
            },
        )
        .await?;

        assert_eq!(
            email_config.slots, update_slots,
            "incorrect slots after update"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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
            subject: None,
            conversation_id: None,
        };

        let new_email_config = create(&pool, current_user.id, &params).await?;

        let email_config = get_by_id(&pool, new_email_config.id).await?;

        assert_eq!(new_email_config.id, email_config.id, "ids don't match");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_email_config_with_scope(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, user, _) = session.current_user(&app).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let conversation = conversation::get_localised_by_id(&pool, &conversation_id, "en").await?;

        let new_scoped = create(
            &pool,
            user.id,
            &conversation_invite_params(Some(conversation_id)),
        )
        .await?;

        let found_scoped = get_by_id_with_scope(&pool, new_scoped.id, "en").await?;

        assert_eq!(
            found_scoped.conversation_title,
            Some(conversation.title),
            "conversation_title does not match"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_list_email_configs(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, user, _) = session.current_user(&app).await?;

        let default_slots = DefaultEmailSlots {
            heading: "<h1>Test heading</h1>".to_string(),
            intro: "<p>Test intro</p>".to_string(),
            body: "<p>Test body</p>".to_string(),
            footer: "<p>Test footer</p>".to_string(),
        };

        let params_a = CreateEmailTemplateConfig {
            slots: EmailTemplateSlots::EventRegistrationConfirmation(default_slots.clone()),
            subject: None,
            conversation_id: None,
        };
        create(&pool, user.id, &params_a).await?;
        let params_b = CreateEmailTemplateConfig {
            slots: EmailTemplateSlots::ConversationInvite(default_slots.clone()),
            subject: None,
            conversation_id: None,
        };
        create(&pool, user.id, &params_b).await?;

        let filter_options = EmailTemplateConfigFilterOptions {
            email_type: Some(EmailType::EventRegistrationConfirmation),
        };
        let email_configs = list(&pool, &user.id, filter_options).await?;

        assert_eq!(email_configs.len(), 1, "incorrect total");
        assert!(
            !email_configs
                .iter()
                .any(|c| c.email_type == EmailType::ConversationInvite),
            "incorrectly email_type included"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_list_with_scope_conversation_title_and_unscoped_configs(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, user, _) = session.current_user(&app).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let conversation = conversation::get_localised_by_id(&pool, &conversation_id, "en").await?;

        let scoped = create(
            &pool,
            user.id,
            &conversation_invite_params(Some(conversation_id)),
        )
        .await?;
        let unscoped = create(&pool, user.id, &conversation_invite_params(None)).await?;

        let configs = list_with_scope(&pool, user.id, Default::default(), "en").await?;

        assert_eq!(configs.len(), 2, "incorrect total");

        let scoped_result = configs
            .iter()
            .find(|c| c.config.id == scoped.id)
            .ok_or("missing scoped config")?;
        assert_eq!(
            scoped_result.conversation_title.as_deref(),
            Some(conversation.title.as_str()),
            "incorrect conversation_title"
        );

        let unscoped_result = configs
            .iter()
            .find(|c| c.config.id == unscoped.id)
            .ok_or("missing scoped config")?;
        assert_eq!(
            unscoped_result.conversation_title.as_deref(),
            None,
            "unscoped config should have no conversation_title"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_list_with_scope_primary_locale_title_when_locale_missing(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, user, _) = session.current_user(&app).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let conversation = conversation::get_localised_by_id(&pool, &conversation_id, "en").await?;

        create(
            &pool,
            user.id,
            &conversation_invite_params(Some(conversation_id)),
        )
        .await?;

        let configs = list_with_scope(&pool, user.id, Default::default(), "xx").await?;

        assert_eq!(
            configs.len(),
            1,
            "translation joins should not duplicate rows"
        );
        assert_eq!(
            configs[0].conversation_title.as_deref(),
            Some(conversation.title.as_str()),
            "expected primary locale title"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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
            subject: None,
            conversation_id: None,
        };

        let email_config = create(&pool, current_user.id, &params).await?;

        delete(&pool, email_config.id).await?;

        let err = get_by_id(&pool, email_config.id).await.unwrap_err();

        match err {
            ComhairleError::ResourceNotFound(e) => {
                assert_eq!(
                    e,
                    "Email Template Config".to_string(),
                    "incorrect error message"
                );
            }
            _ => panic!("Expected ResourceNotFound error"),
        }

        Ok(())
    }
}
