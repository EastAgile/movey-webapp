use std::env;
use std::future::Future;
use std::pin::Pin;

use jelly::anyhow::{anyhow, Error};
use jelly::email::Email;
use jelly::jobs::{Job, JobState, DEFAULT_QUEUE};
use jelly::serde::{Deserialize, Serialize};
use jelly::tera::Context;

use crate::accounts::Account;

#[derive(Debug, Serialize, Deserialize)]
pub struct SendCollaboratorInvitationEmail {
    pub to: String,
    pub package_name: String,
    pub token: String,
}

pub fn build_invite_collaborator_context(verify_url: &str) -> Context {
    let mut context = Context::new();
    context.insert("action_url", verify_url);
    context
}

impl Job for SendCollaboratorInvitationEmail {
    type State = JobState;
    type Future = Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;

    const NAME: &'static str = "SendCollaboratorInvitationEmail";
    const QUEUE: &'static str = DEFAULT_QUEUE;

    fn run(self, state: JobState) -> Self::Future {
        Box::pin(async move {
            let account = Account::get_by_email(&self.to, &state.pool)
                .await
                .map_err(|e| anyhow!("Error fetching account for collaborator invitation: {:?}", e))?;

            let domain = env::var("JELLY_DOMAIN").expect("No JELLY_DOMAIN value set!");

            let invitation_url = format!(
                "{}/owner_invitations/accept/{}",
                domain,
                self.token
            );

            let email = Email::new(
                "email/reset-password",
                &[account.email],
                &format!("You have been invited to collaborate on {}", self.package_name),
                build_invite_collaborator_context(&invitation_url),
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
}

impl Job for SendRegisterToCollabEmail {
    type State = JobState;
    type Future = Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;

    const NAME: &'static str = "SendRegisterToCollabEmailJob";
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
