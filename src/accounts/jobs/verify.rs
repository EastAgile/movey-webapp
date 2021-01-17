use std::collections::HashMap;
use std::env;
use std::future::Future;
use std::pin::Pin;

use jelly::accounts::OneTimeUseTokenGenerator;
use jelly::serde::{Deserialize, Serialize};
use jelly::anyhow::{anyhow, Error};
use jelly::email::Email;
use jelly::jobs::{DEFAULT_QUEUE, Job, JobState};

use crate::accounts::Account;

#[derive(Debug, Serialize, Deserialize)]
pub struct SendVerifyAccountEmail {
    pub to: i32
}

impl Job for SendVerifyAccountEmail {
    type State = JobState;
    type Future = Pin<Box<dyn Future<Output=Result<(), Error>> + Send>>;

    const NAME: &'static str = "SendVerifyAccountEmailJob";
    const QUEUE: &'static str = DEFAULT_QUEUE;

    fn run(self, state: JobState) -> Self::Future {
        Box::pin(async move {
            let account = Account::get(self.to, &state.pool).await.map_err(|e| {
                anyhow!("Error fetching account for verification: {:?}", e)
            })?;

            let domain = env::var("DOMAIN")
                .expect("No DOMAIN value set!");

            let verify_url = format!(
                "{}/accounts/verify/{}-{}/",
                domain,
                base64_url::encode(&format!("{}", account.id)),
                account.create_reset_token().map_err(|e| {
                    anyhow!("Error creating verification token: {:?}", e)
                })?
            );

            let email = Email::new("verify-account", &[account.email], {
                let mut model = HashMap::new();
                model.insert("preview", "Verify your new account".into());
                model.insert("action_url", verify_url);
                model
            });
            
            email.send()?;
            
            Ok(())
        })
    }
}

