use std::future::Future;
use std::pin::Pin;

use jelly::anyhow::{ Error};
use jelly::email::Email;
use jelly::jobs::{Job, JobState, DEFAULT_QUEUE};
use jelly::serde::{Deserialize, Serialize};
use jelly::tera::Context;



#[derive(Debug, Serialize, Deserialize)]
pub struct SendContactRequestEmail{
    pub to: String,
    pub name: String,
    pub email: String,
    pub description: String,
    pub category: String
}

impl Job for SendContactRequestEmail {
    type State = JobState;
    type Future = Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;

    const NAME: &'static str = "SendContactRequestEmailJob";
    const QUEUE: &'static str = DEFAULT_QUEUE;

    fn run(self, state: JobState) -> Self::Future {
        Box::pin(async move {

            let email = Email::new(
                "email/contact-request",
                &[self.to],
                "New Contact Request",
                {let mut context = Context::new();
                            context.insert("email", &self.email);
                            context.insert("name", &self.name);
                            context.insert("category", &self.category);
                            context.insert("description", &self.description);
                            context},
                state.templates,
            );

            email?.send()?;

            Ok(())
        })
    }
}
