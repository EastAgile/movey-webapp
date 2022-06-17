use std::env;
use std::future::Future;
use std::pin::Pin;

use jelly::accounts::OneTimeUseTokenGenerator;
use jelly::anyhow::{anyhow, Error};
use jelly::email::Email;
use jelly::jobs::{Job, JobState, DEFAULT_QUEUE};
use jelly::serde::{Deserialize, Serialize};
use jelly::tera::Context;



#[derive(Debug, Serialize, Deserialize)]
pub struct SendContactRequestEmail{
    pub to: String,
}

impl Job for SendContactRequestEmail {
    type State = JobState;
    type Future = Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;

    const NAME: &'static str = "SendContactRequestEmail";
    const QUEUE: &'static str = DEFAULT_QUEUE;

    fn run(self, state: JobState) -> Self::Future {
        Box::pin(async move {
            let email = Email::new(
                "email/contact-request",
                &[self.to],
                "New Contact Request",
                Context::new(),
                state.templates,
            );

            email?.send()?;

            Ok(())
        })
    }
}