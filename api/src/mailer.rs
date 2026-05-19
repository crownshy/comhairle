use crate::error::ComhairleError;
use crate::models::event::LocalizedEvent;
use crate::models::users::User;

use chrono::{DateTime, Utc};
use chrono_tz::US::Pacific;
use icalendar::{self as ical, Component, EventLike};
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{
    message::{header::ContentType, Attachment, Body, MultiPart, SinglePart},
    Message, SmtpTransport, Transport,
};
use minijinja::{context, Environment, Value};
use std::str::FromStr;
use tracing::{instrument, warn};

#[cfg(test)]
use mockall::{automock, predicate::*};

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

    fn send_event_registration_email(
        &self,
        email: String,
        event_name: String,
        event_time: String,
        invite_link: String,
        organization_name: String,
        organization_email: Option<String>,
    ) -> Result<(), ComhairleError>;

    fn send_event_confirmation_email(
        &self,
        email: String,
        event_name: String,
        event_time: String,
        event_link: String,
        organization_name: String,
        organization_email: Option<String>,
    ) -> Result<(), ComhairleError>;

    fn send_event_reminder(
        &self,
        email: String,
        event_name: String,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        link_href: String,
        organization_name: String,
        organization_email: Option<String>,
    ) -> Result<(), ComhairleError>;
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
            .expect_send_otp_email()
            .returning(|_, _, _, _| Ok(()));
        mailer
            .expect_send_password_reset_email()
            .returning(|_, _, _| Ok(()));
        mailer
            .expect_send_event_registration_email()
            .returning(|_, _, _, _, _, _| Ok(()));
        mailer
            .expect_send_event_confirmation_email()
            .returning(|_, _, _, _, _, _| Ok(()));
        mailer
            .expect_send_event_reminder()
            .returning(|_, _, _, _, _, _| Ok(()));

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
}

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

        if let Err(e) = mailer.send(&email) {
            warn!("Mailer error: {e}");
        }

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

    fn send_event_registration_email(
        &self,
        email: String,
        event_name: String,
        event_time: String,
        invite_link: String,
        organization_name: String,
        organization_email: Option<String>,
    ) -> Result<(), ComhairleError> {
        self.send_email(
            &email,
            "Invitation to take part in an event",
            "event_registration_invite.html",
            context! {
                event_name => event_name,
                event_time => event_time,
                organization_name => organization_name,
                organization_email => organization_email,
                invite_link => invite_link,
            },
            None,
        )
    }

    fn send_event_confirmation_email(
        &self,
        email: String,
        event_name: String,
        event_time: String,
        event_link: String,
        organization_name: String,
        organization_email: Option<String>,
    ) -> Result<(), ComhairleError> {
        self.send_email(
            &email,
            "Event registration confirmation",
            "event_confirmation.html",
            context! {
                event_name => event_name,
                event_time => event_time,
                organization_name => organization_name,
                organization_email => organization_email,
                event_link => event_link,
            },
            None,
        )
    }

    fn send_event_reminder(
        &self,
        email: String,
        event_name: String,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        link_href: String,
        organization_name: String,
        organization_email: Option<String>,
    ) -> Result<(), ComhairleError> {
        let calendar_invite =
            create_calendar_invite(&event_name, &event_name, start_time, end_time)?;

        let formatted_date = start_time
            .with_timezone(&Pacific) // TODO: find a way to make this configurable or dynamic
            .format("%B %d, %Y at %H:%M %Z")
            .to_string();

        self.send_email(
            &email,
            "Upcoming event reminder",
            "event_reminder.html",
            context! {
                event_name => event_name,
                event_time => formatted_date,
                organization_name => organization_name,
                organization_email => organization_email,
                event_link => link_href,
            },
            Some(calendar_invite),
        )
    }
}

fn create_calendar_invite(
    name: &str,
    description: &str,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<SinglePart, ComhairleError> {
    let calendar_invite = ical::Calendar::new()
        .name(name)
        .push(
            ical::Event::new()
                .summary(name)
                .description(description)
                .starts(start)
                .ends(end),
        )
        .done();

    let invite_body = Body::new(calendar_invite.to_string());
    let content_type = ContentType::from_str("text/calendar")?;

    let attachment =
        Attachment::new_inline("Calendar invite".to_string()).body(invite_body, content_type);

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
