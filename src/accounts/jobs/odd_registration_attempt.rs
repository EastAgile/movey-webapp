use std::collections::HashMap;
use std::env::var;
use std::pin::Pin;
use std::future::Future;

use jelly::serde::{Deserialize, Serialize};
use jelly::anyhow::{anyhow, Error};
use jelly::email::Email;
use jelly::jobs::{DEFAULT_QUEUE, Job, JobState};

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
    pub to: String
}

impl Job for SendAccountOddRegisterAttemptEmail {
    type State = JobState;
    type Future = Pin<Box<dyn Future<Output=Result<(), Error>> + Send>>;

    const NAME: &'static str = "SendAccountOddRegisterAttemptEmailJob";
    const QUEUE: &'static str = DEFAULT_QUEUE;

    fn run(self, state: JobState) -> Self::Future {
        Box::pin(async move {
            let name = Account::fetch_name_from_email(&self.to, &state.pool).await.map_err(|e| {
                anyhow!("Error fetching user name/email for odd registration attempt: {:?}", e)
            })?;

            let email = Email::new("odd-registration-attempt", &[self.to], {
                let mut model = HashMap::new();
                model.insert("preview", "Did you want to reset your password?".into());
                model.insert("name", name);
                model.insert("action_url", format!("{}/accounts/reset/", var("DOMAIN")
                        .expect("DOMAIN not set?")
                ));
                model
            });
            
            email.send()?;
            
            Ok(())
        })
    }
}

