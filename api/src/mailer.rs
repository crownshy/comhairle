use crate::models::conversation;
use crate::models::email_template_config::{
    self, MailerContextMap, SCHEMA_CONVERSATION_INVITE, SCHEMA_EVENT_REGISTRATION_CONFIRMATION,
    SCHEMA_EVENT_REGISTRATION_INVITE,
};
use crate::models::event::{self, LocalizedEvent, ResolveTimeZone};
use crate::models::organization::Organization;
use crate::models::users::User;
use crate::{ComhairleState, error::ComhairleError};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use icalendar::{self as ical, Component, EventLike};
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{
    Message, SmtpTransport, Transport,
    message::{Attachment, Body, MultiPart, SinglePart, header::ContentType},
};
use minijinja::{Environment, Value, context};
use std::collections::HashMap;
use std::{str::FromStr, sync::Arc};
use tracing::{instrument, warn};
use uuid::Uuid;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[async_trait]
#[cfg_attr(test, automock)]
pub trait ComhairleMailer: Send + Sync {
    fn send_email(
        &self,
        to: &str,
        subject: &str,
        template: &str,
        context: Value,
        attachment: Option<SinglePart>,
    ) -> Result<(), ComhairleError>;

    async fn send_conversation_invite_email(
        &self,
        state: &Arc<ComhairleState>,
        email: &str,
        conversation_id: Uuid,
        user_id: Uuid,
        invite_id: Uuid,
        locale: &str,
    ) -> Result<(), ComhairleError>;

    fn send_welcome_email(&self, user: &User, verify_link: String) -> Result<(), ComhairleError>;

    fn send_password_reset_email(
        &self,
        to: &Option<String>,
        username: &Option<String>,
        reset_link: String,
    ) -> Result<(), ComhairleError>;

    fn send_verification_email(
        &self,
        username: &Option<String>,
        email: &Option<String>,
        verify_link: String,
    ) -> Result<(), ComhairleError>;

    fn send_otp_email(
        &self,
        username: &Option<String>,
        email: &Option<String>,
        passcode: String,
        passcode_link: Option<String>,
    ) -> Result<(), ComhairleError>;

    async fn send_event_registration_email(
        &self,
        state: &Arc<ComhairleState>,
        email: &str,
        event_id: Uuid,
        user_id: Uuid,
        invite_id: Uuid,
        locale: &str,
    ) -> Result<(), ComhairleError>;

    async fn send_event_confirmation_email(
        &self,
        state: &Arc<ComhairleState>,
        email: &str,
        event_id: Uuid,
        user_id: Uuid,
        locale: &str,
    ) -> Result<(), ComhairleError>;

    fn send_event_reminder(
        &self,
        email: String,
        event: &LocalizedEvent,
        organization: &Option<Organization>,
        link_href: String,
    ) -> Result<(), ComhairleError>;

    fn send_conversation_broadcast_email(
        &self,
        email: &str,
        subject: &str,
        html_body: &str,
    ) -> Result<(), ComhairleError>;

    fn preview_email(
        &self,
        template: &str,
        slots_map: HashMap<String, String>,
        variables_map: Option<HashMap<String, String>>,
    ) -> Result<String, ComhairleError>;
}

#[derive(Debug)]
pub struct Mailer {
    host: String,
    creds: Credentials,
    template_engine: Environment<'static>,
}

#[cfg(test)]
impl MockComhairleMailer {
    pub fn base() -> MockComhairleMailer {
        let mut mailer = MockComhairleMailer::new();

        mailer.expect_send_welcome_email().returning(|_, _| Ok(()));
        mailer
            .expect_send_verification_email()
            .returning(|_, _, _| Ok(()));
        mailer.expect_send_email().returning(|_, _, _, _, _| Ok(()));
        mailer
            .expect_send_conversation_invite_email()
            .returning(|_, _, _, _, _, _| Box::pin(async move { Ok(()) }));
        mailer
            .expect_send_otp_email()
            .returning(|_, _, _, _| Ok(()));
        mailer
            .expect_send_password_reset_email()
            .returning(|_, _, _| Ok(()));
        mailer
            .expect_send_event_registration_email()
            .returning(|_, _, _, _, _, _| Box::pin(async move { Ok(()) }));
        mailer
            .expect_send_event_confirmation_email()
            .returning(|_, _, _, _, _| Box::pin(async move { Ok(()) }));
        mailer
            .expect_send_event_reminder()
            .returning(|_, _, _, _| Ok(()));
        mailer
            .expect_send_conversation_broadcast_email()
            .returning(|_, _, _| Ok(()));
        mailer
            .expect_preview_email()
            .returning(|_, _, _| Ok(String::new()));

        mailer
    }
}

impl Mailer {
    pub fn new(host: &str, user: &str, password: &str) -> Self {
        let creds = Credentials::new(user.to_string(), password.to_string());
        let mut env = minijinja::Environment::new();
        minijinja_embed::load_templates!(&mut env);
        Self {
            host: host.into(),
            creds,
            template_engine: env,
        }
    }

    /// Resolves dynamic placeholders within email template slot values.
    ///
    /// Each value in `slots_map` (e.g. `heading`, `intro`, `body`, `footer`)
    /// may itself contain minijinja syntax such as `{{ conversation_title }}`,
    /// allowing users to embed dynamic, runtime-provided data within the
    /// content they configure for an email template.
    ///
    /// This method renders each slot value as its own minijinja template using
    /// the supplied `context`, returning a new map with the resolved values.
    /// The result can then be merged into the final context used to render the
    /// outer email template, e.g.:
    ///
    /// ```ignore
    /// let resolved = self.resolve_slots_map(&context, &slots_map)?;
    /// let final_context = minijinja::context! { ..resolved };
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if any slot value fails to render (e.g. invalid
    /// minijinja syntax).
    fn resolve_slots_map(
        &self,
        context: &minijinja::Value,
        slots_map: &HashMap<String, String>,
    ) -> Result<HashMap<String, String>, ComhairleError> {
        let mut rendered_map: HashMap<String, String> = HashMap::new();

        for (key, value) in slots_map {
            let rendered = self.template_engine.render_str(value, context)?;
            rendered_map.insert(key.to_string(), rendered);
        }

        Ok(rendered_map)
    }
}

#[async_trait]
impl ComhairleMailer for Mailer {
    #[instrument(err(Debug), skip(attachment))]
    fn send_email(
        &self,
        to: &str,
        subject: &str,
        template: &str,
        context: Value,
        attachment: Option<SinglePart>,
    ) -> Result<(), ComhairleError> {
        let template = self
            .template_engine
            .get_template(template)
            .expect("template to exist");

        let html = template
            .render(context)
            .expect("Template to render properly");
        let html_inline_styles = css_inline::inline(&html)?;

        let to = Mailbox::from_str(to)?;
        let message_builder = Message::builder()
            .from("noreply@comhairle.scot".parse().unwrap())
            .reply_to("invites@comhairle.scot".parse().unwrap())
            .to(to)
            .subject(subject);

        let mut content = MultiPart::mixed().singlepart(
            SinglePart::builder()
                .header(ContentType::TEXT_HTML)
                .body(html_inline_styles),
        );

        if let Some(attachment) = attachment {
            content = content.singlepart(attachment);
        }

        let email = message_builder.multipart(content)?;

        let mailer = SmtpTransport::relay(&self.host)?
            .credentials(self.creds.clone())
            .build();

        mailer.send(&email).map_err(|e| {
            warn!("Mailer error: {e}");
            e
        })?;

        Ok(())
    }

    async fn send_conversation_invite_email(
        &self,
        state: &Arc<ComhairleState>,
        to: &str,
        conversation_id: Uuid,
        user_id: Uuid,
        invite_id: Uuid,
        locale: &str,
    ) -> Result<(), ComhairleError> {
        let conversation =
            conversation::get_localised_by_id(&state.db, &conversation_id, locale).await?;

        let invite_link = format!(
            "{}/conversations/{}/invite/{}",
            state.config.domain,
            conversation
                .slug
                .unwrap_or_else(|| conversation.id.to_string()),
            invite_id
        );

        let conversation_context =
            context! { conversation_title => conversation.title, invite_link };

        let email_config = email_template_config::get_by_type_user(
            &state.db,
            user_id,
            &SCHEMA_CONVERSATION_INVITE.email_type,
        )
        .await?;

        let subject = email_config
            .as_ref()
            .and_then(|ec| ec.subject.as_deref())
            .unwrap_or(SCHEMA_CONVERSATION_INVITE.default_subject);
        let subject = self
            .template_engine
            .render_str(subject, &conversation_context)?;

        let slots_map = email_config
            .as_ref()
            .map(|ec| ec.slots.mailer_context_map())
            .unwrap_or(SCHEMA_CONVERSATION_INVITE.slots.mailer_context_map());

        let rendered_map = self.resolve_slots_map(&conversation_context, &slots_map)?;

        let context = context! { invite_link, domain => state.config.domain, ..rendered_map };

        self.send_email(
            to,
            &subject,
            SCHEMA_CONVERSATION_INVITE.template,
            context,
            None,
        )?;

        Ok(())
    }

    fn send_welcome_email(&self, user: &User, verify_link: String) -> Result<(), ComhairleError> {
        if let Some(email) = &user.email {
            self.send_email(
                email,
                "Welcome to Comhairle",
                "welcome.html",
                context! {user => user, subject=>"Welcome to Comhairle", verify_link},
                None,
            )
        } else {
            Err(ComhairleError::WrongUserType)
        }
    }

    fn send_verification_email(
        &self,
        username: &Option<String>,
        email: &Option<String>,
        verify_link: String,
    ) -> Result<(), ComhairleError> {
        if let Some(email) = email {
            self.send_email(
                email,
                "Confirm your email address",
                "verify_email.html",
                context! { username, verify_link },
                None,
            )
        } else {
            Err(ComhairleError::WrongUserType)
        }
    }

    fn send_otp_email(
        &self,
        username: &Option<String>,
        email: &Option<String>,
        passcode: String,
        passcode_link: Option<String>,
    ) -> Result<(), ComhairleError> {
        if let Some(email) = email {
            self.send_email(
                email,
                "Your Comhairle one-time-passcode",
                "one_time_passcode.html",
                context! { username, passcode, passcode_link },
                None,
            )
        } else {
            Err(ComhairleError::WrongUserType)
        }
    }

    fn send_password_reset_email(
        &self,
        to: &Option<String>,
        username: &Option<String>,
        reset_link: String,
    ) -> Result<(), ComhairleError> {
        if let Some(email) = to {
            self.send_email(
                email,
                "Reset your Comhairle password",
                "password_reset.html",
                context! { username, reset_link },
                None,
            )
        } else {
            Err(ComhairleError::WrongUserType)
        }
    }

    async fn send_event_registration_email(
        &self,
        state: &Arc<ComhairleState>,
        email: &str,
        event_id: Uuid,
        user_id: Uuid,
        invite_id: Uuid,
        locale: &str,
    ) -> Result<(), ComhairleError> {
        let event = event::get_localized_by_id(&state.db, &event_id, locale).await?;

        let invite_link = format!(
            "{}/conversations/{}/events/{}/invite/{}",
            state.config.domain, event.conversation_id, event.id, invite_id
        );

        let event_context = context! {
            event_name => event.name,
            event_time => event.format_date_with_time_zone(event.start_time, None),
            invite_link,
        };

        let email_config = email_template_config::get_by_type_user(
            &state.db,
            user_id,
            &SCHEMA_EVENT_REGISTRATION_INVITE.email_type,
        )
        .await?;

        let subject = email_config
            .as_ref()
            .and_then(|ec| ec.subject.as_deref())
            .unwrap_or(SCHEMA_EVENT_REGISTRATION_INVITE.default_subject);
        let subject = self.template_engine.render_str(subject, &event_context)?;

        let slots_map = email_config
            .as_ref()
            .map(|ec| ec.slots.mailer_context_map())
            .unwrap_or(SCHEMA_EVENT_REGISTRATION_INVITE.slots.mailer_context_map());

        let rendered_map = self.resolve_slots_map(&event_context, &slots_map)?;

        let context = context! {
            event_name => event.name,
            event_time => event.format_date_with_time_zone(event.start_time, None),
            invite_link,
            ..rendered_map
        };

        self.send_email(
            email,
            &subject,
            "event_registration_invite.html",
            context,
            None,
        )
    }

    async fn send_event_confirmation_email(
        &self,
        state: &Arc<ComhairleState>,
        email: &str,
        event_id: Uuid,
        user_id: Uuid,
        locale: &str,
    ) -> Result<(), ComhairleError> {
        let event = event::get_localized_by_id(&state.db, &event_id, locale).await?;

        let event_link = format!(
            "{}/conversations/{}/events/{}",
            state.config.domain, event.conversation_id, event.id
        );
        let calendar_invite = create_calendar_invite_attachment(
            &event.name,
            &event.description,
            event.start_time,
            event.end_time,
        )?;

        let event_context = context! {
            event_name => event.name,
            event_time => event.format_date_with_time_zone(event.start_time, None),
            event_link,
        };

        let email_config = email_template_config::get_by_type_user(
            &state.db,
            user_id,
            &SCHEMA_EVENT_REGISTRATION_CONFIRMATION.email_type,
        )
        .await?;

        let subject = email_config
            .as_ref()
            .and_then(|ec| ec.subject.as_deref())
            .unwrap_or(SCHEMA_EVENT_REGISTRATION_CONFIRMATION.default_subject);
        let subject = self.template_engine.render_str(subject, &event_context)?;

        let slots_map = email_config
            .as_ref()
            .map(|ec| ec.slots.mailer_context_map())
            .unwrap_or(
                SCHEMA_EVENT_REGISTRATION_CONFIRMATION
                    .slots
                    .mailer_context_map(),
            );

        let rendered_map = self.resolve_slots_map(&event_context, &slots_map)?;

        let context = context! {
            event_name => event.name,
            event_time => event.format_date_with_time_zone(event.start_time, None),
            event_link,
            ..rendered_map
        };

        self.send_email(
            email,
            &subject,
            "event_confirmation.html",
            context,
            Some(calendar_invite),
        )
    }

    fn send_event_reminder(
        &self,
        email: String,
        event: &LocalizedEvent,
        _organization: &Option<Organization>,
        link_href: String,
    ) -> Result<(), ComhairleError> {
        let calendar_invite = create_calendar_invite_attachment(
            &event.name,
            &event.description,
            event.start_time,
            event.end_time,
        )?;

        self.send_email(
            &email,
            "Upcoming event reminder",
            "event_reminder.html",
            context! {
                event_name => event.name,
                event_time => event.format_date_with_time_zone(event.start_time, None),
                organization_name => "Bloom", // TODO:
                // organization_email => None,
                event_link => link_href,
            },
            Some(calendar_invite),
        )
    }

    fn send_conversation_broadcast_email(
        &self,
        email: &str,
        subject: &str,
        html_body: &str,
    ) -> Result<(), ComhairleError> {
        self.send_email(
            email,
            subject,
            "conversation_broadcast.html",
            context! { subject, body => html_body },
            None,
        )
    }

    /// Renders an email template to HTML for preview purposes.
    ///
    /// Intended to be called from the frontend to allow users to preview how a
    /// customised [`EmailTemplateConfig`] will appear as they are editing it,
    /// before any emails are actually sent.
    ///
    /// # Errors
    ///
    /// Returns a [`ComhairleError`] if:
    /// - The named template does not exist ([`ComhairleError::MissingEmailTemplate`])
    /// - The template fails to render (e.g. missing required variables or invalid syntax)
    /// - CSS inlining fails
    fn preview_email(
        &self,
        template: &str,
        slots_map: HashMap<String, String>,
        variables_map: Option<HashMap<String, String>>,
    ) -> Result<String, ComhairleError> {
        let context = match variables_map {
            Some(var_map) => {
                let local_context = context! { ..var_map };
                let rendered_map = self.resolve_slots_map(&local_context, &slots_map)?;

                context! { ..rendered_map }
            }
            None => context! { ..slots_map },
        };

        let template = self
            .template_engine
            .get_template(template)
            .map_err(|_| ComhairleError::MissingEmailTemplate(template.to_string()))?;

        let html = template.render(context)?;
        let html_inline_styles = css_inline::inline(&html)?;

        Ok(html_inline_styles)
    }
}

fn create_calendar_invite_attachment(
    name: &str,
    description: &str,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<SinglePart, ComhairleError> {
    let calendar_invite = build_calendar_invite(name, description, start, end);

    build_invite_attachment(calendar_invite.to_string())
}

pub fn build_calendar_invite(
    name: &str,
    description: &str,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> ical::Calendar {
    ical::Calendar::new()
        .name(name)
        .push(
            ical::Event::new()
                .summary(name)
                .description(description)
                .starts(start)
                .ends(end),
        )
        .done()
}

pub fn build_invite_attachment(ics_body: String) -> Result<SinglePart, ComhairleError> {
    let invite_body = Body::new(ics_body);
    let content_type =
        ContentType::from_str("text/calendar; charset=utf-8; method=REQUEST; name=\"invite.ics\"")?;

    let attachment =
        Attachment::new_inline("calendar-invite".to_string()).body(invite_body, content_type);

    Ok(attachment)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_load_email_template() {
        let mailer = Mailer::new("test_host", "test_user", "test_password");

        let template = mailer.template_engine.get_template("welcome.html");
        assert!(template.is_ok(), "error retrieving welcome template");

        let template = mailer
            .template_engine
            .get_template("conversation_invite.html");
        assert!(template.is_ok(), "error retrieving conversation template");

        let template = mailer.template_engine.get_template("email_layout.html");
        assert!(template.is_ok(), "error retrieving layout template");
    }
}
