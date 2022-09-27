use std::env;
use std::future::Future;
use std::pin::Pin;

use jelly::accounts::OneTimeUseTokenGenerator;
use jelly::anyhow::{anyhow, Error};
use jelly::email::Email;
use jelly::jobs::{Job, JobState};
use jelly::serde::{Deserialize, Serialize};
use jelly::tera::Context;

use crate::accounts::Account;

#[derive(Debug, Serialize, Deserialize)]
pub struct SendVerifyAccountEmail {
    pub to: i32,
}

pub fn build_context(username: &str, verify_url: &str) -> Context {
    let mut context = Context::new();
    context.insert("username", &username);
    context.insert("action_url", &verify_url);
    context
}

impl Job for SendVerifyAccountEmail {
    type State = JobState;
    type Future = Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;

    const NAME: &'static str = "SendVerifyAccountEmailJob";

    fn run(self, state: JobState) -> Self::Future {
        Box::pin(async move {
            let account = Account::get(self.to, &state.pool)
                .map_err(|e| anyhow!("Error fetching account for verification: {:?}", e))?;

            let domain = env::var("JELLY_DOMAIN").expect("No JELLY_DOMAIN value set!");

            let verify_url = format!(
                "{}/accounts/verify/{}-{}/",
                domain,
                base64_url::encode(&format!("{}", account.id)),
                account
                    .create_reset_token()
                    .map_err(|e| { anyhow!("Error creating verification token: {:?}", e) })?
            );

            let email = Email::new(
                "email/verify-account",
                &[account.email],
                "Verify your new Movey account",
                build_context(&account.name, &verify_url),
                state.templates,
            );

            email?.send()?;

            Ok(())
        })
    }
}
