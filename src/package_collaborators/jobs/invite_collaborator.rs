use std::env;
use std::future::Future;
use std::pin::Pin;

use jelly::anyhow::Error;
use jelly::email::Email;
use jelly::jobs::{Job, JobState};
use jelly::serde::{Deserialize, Serialize};
use jelly::tera::Context;

#[derive(Debug, Serialize, Deserialize)]
pub struct SendCollaboratorInvitationEmail {
    pub to: String,
    pub package_name: String,
    pub token: String,
}

impl Job for SendCollaboratorInvitationEmail {
    type State = JobState;
    type Future = Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;

    const NAME: &'static str = "SendCollaboratorInvitationEmail";

    fn run(self, state: JobState) -> Self::Future {
        Box::pin(async move {
            let domain = env::var("JELLY_DOMAIN").expect("No JELLY_DOMAIN value set!");

            let invitation_url = format!("{}/collaborators/accept/{}", domain, self.token);

            let email = Email::new(
                "email/invite-collaborator",
                &[self.to],
                &format!(
                    "You have been invited to collaborate on {}",
                    self.package_name
                ),
                {
                    let mut context = Context::new();
                    context.insert("action_url", &invitation_url);
                    context.insert("package_name", &self.package_name);
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
pub struct SendRegisterToCollabEmail {
    pub to: String,
    pub package_name: String,
}

impl Job for SendRegisterToCollabEmail {
    type State = JobState;
    type Future = Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;

    const NAME: &'static str = "SendRegisterToCollabEmail";

    fn run(self, state: JobState) -> Self::Future {
        Box::pin(async move {
            let domain = env::var("JELLY_DOMAIN").expect("No JELLY_DOMAIN value set!");
            let register_url = format!("{}/accounts/register?redirect=/profile", domain);

            let email = Email::new(
                "email/register-to-collab",
                &[self.to],
                &format!(
                    "You have been invited to collaborate on {}",
                    self.package_name
                ),
                {
                    let mut context = Context::new();
                    context.insert("action_url", &register_url);
                    context.insert("package_name", &self.package_name);
                    context
                },
                state.templates,
            );

            email?.send()?;

            Ok(())
        })
    }
}
