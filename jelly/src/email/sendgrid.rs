use super::common::env_exists_and_not_empty;
pub use super::common::Email;
use anyhow::{anyhow, Context, Result};
use serde::Serialize;
use std::env::var;

#[derive(Serialize, Debug)]
struct EmailAddress<'a> {
    email: &'a String,
}

#[derive(Serialize, Debug)]
struct Personalization<'a> {
    to: Vec<EmailAddress<'a>>,
}

#[derive(Serialize, Debug)]
struct Content<'a> {
    r#type: &'a String,
    value: &'a String,
}

#[derive(Serialize, Debug)]
struct SendgridV3Data<'a> {
    personalizations: Vec<Personalization<'a>>,
    from: EmailAddress<'a>,
    subject: &'a String,
    content: Vec<Content<'a>>,
}

/// Check that all needed environment variables are set and not empty.
pub fn check_conf() {
    ["SENDGRID_API_KEY"]
        .iter()
        .for_each(|env| env_exists_and_not_empty(env));
}

impl Email {
    /// Send the email.
    pub fn send_via_sendgrid(&self, base_api_url: &str) -> Result<(), anyhow::Error> {
        let text_plain = "text/plain".to_string();
        let text_html = "text/html".to_string();
        let data = SendgridV3Data {
            personalizations: vec![Personalization {
                to: vec![EmailAddress { email: &self.to }],
            }],
            from: EmailAddress { email: &self.from },
            subject: &self.subject,
            content: vec![
                Content {
                    r#type: &text_plain,
                    value: &self.body,
                },
                Content {
                    r#type: &text_html,
                    value: &self.body_html,
                },
            ],
        };
        debug!("sendgrid payload: {}", serde_json::to_string(&data)?);

        let api_key = var("SENDGRID_API_KEY").expect("SENDGRID_API_KEY not set!");
        let resp = minreq::post(base_api_url.to_owned() + "/v3/mail/send") // TODO use external server  for test
            .with_header("Authorization", format!("{} {}", "Bearer", api_key))
            .with_json(&data)?
            .with_timeout(30)
            .send()
            .context("Posting mail vias sendgrid API")?;

        if resp.status_code >= 200 && resp.status_code <= 299 {
            debug!("Mail sent to {} via sendgrid.", &self.to);
            Ok(())
        } else {
            Err(anyhow!(
                "Sending mail to {} via sendgrid failed. API call returns code {} : {} \n {} ",
                &self.to,
                resp.status_code,
                resp.reason_phrase,
                resp.as_str()?
            ))
        }
    }
}
