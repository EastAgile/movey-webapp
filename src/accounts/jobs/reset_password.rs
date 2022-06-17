use std::env;
use std::future::Future;
use std::pin::Pin;

use jelly::accounts::OneTimeUseTokenGenerator;
use jelly::anyhow::{anyhow, Error};
use jelly::email::Email;
use jelly::jobs::{Job, JobState, DEFAULT_QUEUE};
use jelly::serde::{Deserialize, Serialize};
use jelly::tera::Context;

use crate::accounts::Account;

#[derive(Debug, Serialize, Deserialize)]
pub struct SendResetPasswordEmail {
    pub to: String,
}

pub fn build_context(verify_url: &str) -> Context {
    let mut context = Context::new();
    context.insert("action_url", verify_url);
    context
}

impl Job for SendResetPasswordEmail {
    type State = JobState;
    type Future = Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;

    const NAME: &'static str = "SendResetPasswordEmailJob";
    const QUEUE: &'static str = DEFAULT_QUEUE;

    fn run(self, state: JobState) -> Self::Future {
        Box::pin(async move {
            let account = Account::get_by_email(&self.to, &state.pool)
                .await
                .map_err(|e| anyhow!("Error fetching account for password reset: {:?}", e))?;

            let domain = env::var("JELLY_DOMAIN").expect("No JELLY_DOMAIN value set!");

            let verify_url = format!(
                "{}/accounts/reset/{}-{}/",
                domain,
                base64_url::encode(&format!("{}", account.id)),
                account
                    .create_reset_token()
                    .map_err(|e| { anyhow!("Error creating verification token: {:?}", e) })?
            );

            let email = Email::new(
                "email/reset-password",
                &[account.email],
                "Reset your Movey account password",
                build_context(&verify_url),
                state.templates,
            );

            email?.send()?;

            Ok(())
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendPasswordWasResetEmail {
    pub to: String,
}

impl Job for SendPasswordWasResetEmail {
    type State = JobState;
    type Future = Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;

    const NAME: &'static str = "SendPasswordWasResetEmailJob";
    const QUEUE: &'static str = DEFAULT_QUEUE;

    fn run(self, state: JobState) -> Self::Future {
        Box::pin(async move {
            let email = Email::new(
                "email/password-was-reset",
                &[self.to],
                "Your Movey password was reset",
                Context::new(),
                state.templates,
            );

            email?.send()?;

            Ok(())
        })
    }
}


