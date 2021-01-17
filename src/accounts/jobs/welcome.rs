use std::collections::HashMap;
use std::pin::Pin;
use std::future::Future;

use jelly::serde::{Deserialize, Serialize};
use jelly::anyhow::{anyhow, Error};
use jelly::email::Email;
use jelly::jobs::{DEFAULT_QUEUE, Job, JobState};

use crate::accounts::Account;

/// A job for sending a Welcome email, generally dispatched after an account
/// has been verified.
#[derive(Debug, Serialize, Deserialize)]
pub struct SendWelcomeAccountEmail {
    pub to: i32
}

impl Job for SendWelcomeAccountEmail {
    type State = JobState;
    type Future = Pin<Box<dyn Future<Output=Result<(), Error>> + Send>>;

    const NAME: &'static str = "SendWelcomeAccountEmailJob";
    const QUEUE: &'static str = DEFAULT_QUEUE;

    fn run(self, state: JobState) -> Self::Future {
        Box::pin(async move {
            let (name, email) = Account::fetch_email(self.to, &state.pool).await.map_err(|e| {
                anyhow!("Error fetching user name/email: {:?}", e)
            })?;

            let email = Email::new("welcome", &[email], {
                let mut model = HashMap::new();
                model.insert("preview", "Welcome to the service".into());
                model.insert("name", name);
                model
            });
            
            email.send()?;
            
            Ok(())
        })
    }
}

