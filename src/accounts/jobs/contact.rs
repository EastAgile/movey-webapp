use std::env;
use std::future::Future;
use std::pin::Pin;

use jelly::anyhow::Error;
use jelly::email::Email;
use jelly::jobs::{Job, JobState, DEFAULT_QUEUE};
use jelly::serde::{Deserialize, Serialize};
use jelly::tera::Context;

#[derive(Debug, Serialize, Deserialize)]
pub struct SendContactRequestEmail {
    pub to: String,
    pub name: String,
    pub email: String,
    pub description: String,
    pub category: String,
}
// Send mail with contact detail to administrators
impl Job for SendContactRequestEmail {
    type State = JobState;
    type Future = Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    const NAME: &'static str = "SendContactRequestEmailJob";
    const QUEUE: &'static str = DEFAULT_QUEUE;

    fn run(self, state: JobState) -> Self::Future {
        Box::pin(async move {
            let environment =
                env::var("SENTRY_ALERT_ENVIRONMENT").unwrap_or_else(|_| "".to_string());
            let subject = format!("[{}] New Contact Request", environment);

            let email = Email::new(
                "email/contact-request-to-Movey",
                &[self.to],
                &subject,
                {
                    let mut context = Context::new();
                    context.insert("email", &self.email);
                    context.insert("name", &self.name);
                    context.insert("category", &self.category);
                    context.insert("description", &self.description);
                    context
                },
                state.templates,
            );
            email?.send()?;

            Ok(())
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendContactEmail {
    pub to: String,
}

// Send confirm and thank user for sending contact request
impl Job for SendContactEmail {
    type State = JobState;
    type Future = Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;

    const NAME: &'static str = "SendContactEmail";
    const QUEUE: &'static str = DEFAULT_QUEUE;

    fn run(self, state: JobState) -> Self::Future {
        Box::pin(async move {
            let email = Email::new(
                "email/contact-request",
                &[self.to],
                "Thank you for contacting us",
                Context::new(),
                state.templates,
            );
            email?.send()?;

            Ok(())
        })
    }
}
