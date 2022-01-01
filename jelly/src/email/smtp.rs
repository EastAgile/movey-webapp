use std::env::var;

use anyhow::Result;

use super::common::{env_exists_and_not_empty, Email};
use lettre::message::MultiPart;
use lettre::transport::smtp::{authentication::Credentials, client::Tls};
use lettre::{Message, SmtpTransport, Transport};

/// Check that all needed environment variables are set and not empty.
pub fn check_conf() {
    [
        "EMAIL_DEFAULT_FROM",
        "EMAIL_SMTP_HOST",
        "EMAIL_SMTP_PORT",
        "EMAIL_SMTP_USERNAME",
        "EMAIL_SMTP_PASSWORD",
    ]
    .iter()
    .for_each(|env| env_exists_and_not_empty(env));
}

impl Email {
    /// Send the email. Relies on you ensuring that `EMAIL_DEFAULT_FROM`,
    /// `EMAIL_SMTP_HOST`, `EMAIL_SMTP_USERNAME`, and `EMAIL_SMTP_PASSWORD`
    /// are set in your `.env`.
    pub fn send_via_smtp(&self) -> Result<(), anyhow::Error> {
        let host = var("EMAIL_SMTP_HOST").expect("EMAIL_SMTP_HOST not set!");
        let port = var("EMAIL_SMTP_PORT").expect("EMAIL_SMTP_PORT not set!");
        let username = var("EMAIL_SMTP_USERNAME").expect("EMAIL_SMTP_USERNAME not set!");
        let password = var("EMAIL_SMTP_PASSWORD").expect("EMAIL_SMTP_PASSWORD not set!");
        let reply_to = var("JELLY_SUPPORT_EMAIL")
            .or_else::<anyhow::Error, _>(|_v| Ok(self.from.clone()))
            .unwrap();

        let email = Message::builder()
            .from(self.from.parse()?)
            .reply_to(reply_to.parse()?)
            .to(self.to.parse()?)
            .subject(&self.subject)
            .multipart(MultiPart::alternative_plain_html(
                self.body.clone(),
                self.body_html.clone(),
            ))?;

        let creds = Credentials::new(username, password);

        // Open a remote connection to EMAIL_SMTP_HOST
        let mut mailer_builder = SmtpTransport::relay(&host)?
            .port(port.parse()?)
            .credentials(creds);
        if let Ok(notls) = var("EMAIL_SMTP_NOTLS").and_then(|v| Ok(v == "1" || v == "true")) {
            if notls {
                mailer_builder = mailer_builder.tls(Tls::None);
                info!("Send email with no TLS");
            }
        }

        let mailer = mailer_builder.build();
        mailer.send(&email)?;
        debug!("Mail sent to {} via smtp.", &self.to);

        Ok(())
    }
}
