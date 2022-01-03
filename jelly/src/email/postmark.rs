//! Provides a very, very basic `Email` struct that can send via Postmark.
//! This is designed for transactional emails - if you need otherwise,
//! you're free to import Lettre or whatever.
//!
//! If you prefer a different provider than Postmark, you can swap the
//! send implementation in here.
use std::env::var;
use anyhow::{anyhow, Context, Result};

use super::common::env_exists_and_not_empty;
pub use super::common::Email;

/// Check that all needed environment variables are set and not empty.
pub fn check_conf() {
    ["POSTMARK_API_KEY", "POSTMARK_MESSAGE_STREAM"]
        .iter()
        .for_each(|env| env_exists_and_not_empty(env));
}

impl Email {
    /// Send the email. Relies on you ensuring that `POSTMARK_API_KEY`
    /// is set in your `.env`.
    pub fn send_via_postmark(&self, base_url_api: &str) -> Result<(), anyhow::Error> {
        let api_key = var("POSTMARK_API_KEY").expect("POSTMARK_API_KEY not set!");

        let resp = minreq::post(base_url_api.to_owned() + "/email")
            .with_header("X-Postmark-Server-Token", api_key)
            .with_json(&self)?
            .send()
            .context("Posting mail vias postmark API")?;

        if resp.status_code == 200 {
            debug!("Mail sent to {} via postmark.", &self.to);
            Ok(())
        } else {
            Err(anyhow!(
                "Sending mail to {} via postmark failed. API call returns code {} : {} \n {} ",
                &self.to,
                resp.status_code,
                resp.reason_phrase,
                resp.as_str()?
            ))
        }
    }
}
