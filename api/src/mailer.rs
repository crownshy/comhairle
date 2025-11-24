use crate::error::ComhairleError;
use crate::models::users::User;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use minijinja::{context, Environment, Value};
use tracing::instrument;

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
    ) -> Result<(), ComhairleError>;

    fn send_welcome_email(&self, user: &User, verify_link: String) -> Result<(), ComhairleError>;

    fn send_password_reset_email(
        &self,
        to: String,
        user: User,
        token: &str,
    ) -> Result<(), ComhairleError>;

    fn send_verification_email(
        &self,
        user: &User,
        verify_link: String,
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
        mailer.expect_send_email().returning(|_, _, _, _| Ok(()));
        mailer
            .expect_send_password_reset_email()
            .returning(|_, _, _| Ok(()));

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
    #[instrument(err(Debug))]
    fn send_email(
        &self,
        to: &str,
        subject: &str,
        template: &str,
        context: Value,
    ) -> Result<(), ComhairleError> {
        let template = self
            .template_engine
            .get_template(template)
            .expect("template to exist");

        let content = template
            .render(context)
            .expect("Template to render properly");

        let email = Message::builder()
            .from("noreply@comhairle.scot".parse().unwrap())
            .reply_to("invites@comhairle.scot".parse().unwrap())
            .to(to.parse().unwrap())
            .header(lettre::message::header::ContentType::TEXT_HTML)
            .subject(subject)
            .body(content)
            .unwrap();

        let mailer = SmtpTransport::relay(&self.host)
            .unwrap()
            .credentials(self.creds.clone())
            .build();

        mailer.send(&email)?;
        Ok(())
    }

    fn send_welcome_email(&self, user: &User, verify_link: String) -> Result<(), ComhairleError> {
        if let Some(email) = &user.email {
            self.send_email(
                email,
                "Welcome to Comhairle",
                "welcome.html",
                context! {user => user, subject=>"Welcome to Comhairle", verify_link},
            )
        } else {
            Err(ComhairleError::WrongUserType)
        }
    }

    fn send_verification_email(
        &self,
        user: &User,
        verify_link: String,
    ) -> Result<(), ComhairleError> {
        if let Some(email) = &user.email {
            self.send_email(
                email,
                "Confirm your email address",
                "verify_email.html",
                context! { user, verify_link }
            )
        } else {
            Err(ComhairleError::WrongUserType)
        }
    }

    fn send_password_reset_email(
        &self,
        to: String,
        user: User,
        token: &str,
    ) -> Result<(), ComhairleError> {
        Ok(())
    }
}
