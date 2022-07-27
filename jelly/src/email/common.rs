use std::env;
use std::env::var;
use std::sync::{Arc, RwLock};
use tera::{Context, Tera};

use anyhow::{anyhow, Error, Result};
use chrono::{Datelike, Utc};
use serde::Serialize;

pub trait Configurable {
    /// Check that configuration is complete.
    /// This function shall be used at start up to detect misconfiguration as soon as possible
    /// It panics if configuration is incorrect.
    fn check_conf();
}

/// Check that environment variable exists and is not empty else panic.
#[allow(dead_code)]
pub fn env_exists_and_not_empty(env: &str) {
    if var(env).expect(&format!("{} not set!", env)).is_empty() {
        panic!("{} is empty", env)
    }
}

#[derive(Debug, Default, Serialize)]
pub struct Email {
    /// Who's sending this.
    #[serde(rename = "From")]
    pub from: String,

    /// Who to send to. Comma-delimited.
    #[serde(rename = "To")]
    pub to: String,

    /// Who to send to. Comma-delimited.
    #[serde(rename = "Subject")]
    pub subject: String,

    /// What to send (plaintext)
    #[serde(rename = "TextBody")]
    pub body: String,

    /// What to send (HTML)
    #[serde(rename = "HtmlBody")]
    pub body_html: String,

    /// Postmark stream to use
    #[serde(rename = "MessageStream")]
    pub postmark_message_stream: String,
}

impl Email {
    /// Construct a new `Email`.
    ///
    /// * [`template_name`] : the template name to be used
    /// * [`to`] : an array of destinationemail addresses
    /// * [`subject`] : the mail subject line
    /// * [`context`] : the [`Context`] used to render the template
    /// * [`templates`] : the tera templates
    pub fn new(
        template_name: &str,
        to: &[String],
        subject: &str,
        mut context: Context,
        templates: Arc<RwLock<Tera>>,
    ) -> Result<Self, anyhow::Error> {
        let engine = templates
            .read()
            .map_err(|e| anyhow!("Error acquiring template read lock: {:?}", e))?;

        let now = Utc::now();
        let year = now.year();
        context.insert("year", &year.to_string());
        context.insert("subject", &subject);

        for (k, v) in env::vars() {
            if k.starts_with("JELLY_") {
                context.insert(k, &v);
            }
        }

        debug!("Context for template {} : {:?}", template_name, &context);

        let body_html = engine
            .render(&(template_name.to_owned() + ".html"), &context)
            .map_err(Error::msg)?;
        let body = engine
            .render(&(template_name.to_owned() + ".txt"), &context)
            .map_err(Error::msg)?;

        Ok(Email {
            to: to.join(","),
            from: var("EMAIL_DEFAULT_FROM").expect("EMAIL_DEFAULT_FROM not set!"),
            body_html,
            body,
            subject: subject.to_string(),
            #[cfg(not(feature = "email-postmark"))]
            postmark_message_stream: "".to_string(),
            #[cfg(feature = "email-postmark")]
            postmark_message_stream: var("POSTMARK_MESSAGE_STREAM")
                .expect("POSTMARK_MESSAGE_STREAM not set!"),
        })
    }
}
