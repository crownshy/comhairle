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

    fn send_welcome_email(&self, to: &str, user: &User) -> Result<(), ComhairleError>;

    fn send_password_reset_email(
        &self,
        to: String,
        user: User,
        token: &str,
    ) -> Result<(), ComhairleError>;
}

#[derive(Debug)]
pub struct Mailer {
    host: String,
    creds: Credentials,
    template_engine: Environment<'static>,
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
        let templates = self.template_engine.templates();
        println!("Printing templates");
        for t in templates {
            println!("{t:#?}");
        }
        println!("done");

        let template = self
            .template_engine
            .get_template(template)
            .expect("template to exist");

        let content = template
            .render(context)
            .expect("Template to render properly");
        println!("Printing content {content}");

        let email = Message::builder()
            .from("noreply@comhairle.scot".parse().unwrap())
            .reply_to("noreply@comhairle.scot".parse().unwrap())
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

    fn send_welcome_email(&self, to: &str, user: &User) -> Result<(), ComhairleError> {
        if let Some(email) = &user.email {
            self.send_email(
                email,
                "Welcome to Comhairle".into(),
                "welcome.html",
                context! {user => user, subject=>"Welcome to Comhairle"},
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
