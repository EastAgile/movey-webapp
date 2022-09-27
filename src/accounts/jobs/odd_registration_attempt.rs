use std::env::var;
use std::future::Future;
use std::pin::Pin;

use jelly::anyhow::{anyhow, Error};
use jelly::email::Email;
use jelly::jobs::{Job, JobState};
use jelly::serde::{Deserialize, Serialize};
use jelly::tera::Context;

use crate::accounts::Account;

/// An email that gets sent if a user attempts to register
/// under an already registered email. We don't want to say
/// "this email exists already", as that reveals that a user has
/// registered for this service.
///
/// Instead we'll just send the registered account an email asking
/// if they meant to reset their password, and display to the user
/// registering the standard "verify" flow.
#[derive(Debug, Serialize, Deserialize)]
pub struct SendAccountOddRegisterAttemptEmail {
    pub to: String,
}

pub fn build_context(name: &str) -> Context {
    let mut context = Context::new();
    context.insert("name", name);
    context.insert(
        "action_url",
        &format!(
            "{}/accounts/reset/",
            var("JELLY_DOMAIN").expect("JELLY_DOMAIN not set?")
        ),
    );
    context
}

impl Job for SendAccountOddRegisterAttemptEmail {
    type State = JobState;
    type Future = Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;

    const NAME: &'static str = "SendAccountOddRegisterAttemptEmailJob";

    fn run(self, state: JobState) -> Self::Future {
        Box::pin(async move {
            let name = Account::fetch_name_from_email(&self.to, &state.pool).map_err(|e| {
                anyhow!(
                    "Error fetching user name for odd registration attempt: {:?}",
                    e
                )
            })?;

            let email = Email::new(
                "email/odd-registration-attempt",
                &[self.to],
                "Did you want to reset your Movey password?",
                build_context(&name),
                state.templates,
            );

            email?.send()?;

            Ok(())
        })
    }
}
