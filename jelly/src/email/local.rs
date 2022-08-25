use std::env::var;

use anyhow::Result;

#[allow(unused_imports)]
use super::common::{env_exists_and_not_empty, Email};
use lettre::message::MultiPart;
use lettre::{Message, Transport, FileTransport};

/// Check that all needed environment variables are set and not empty.
pub fn check_conf() {
}

impl Email {
    /// Send the email. Relies on you ensuring that `EMAIL_DEFAULT_FROM`,
    /// `EMAIL_SMTP_HOST`, `EMAIL_SMTP_USERNAME`, and `EMAIL_SMTP_PASSWORD`
    /// are set in your `.env`.
    pub fn send_locally(&self) -> Result<(), anyhow::Error> {
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

        std::fs::create_dir_all("./emails/")?;
        let mailer = FileTransport::new("./emails/");
        mailer.send(&email)?;
        debug!("Mail sent to {} locally, stored at ./emails/", &self.to);

        Ok(())
    }
}
