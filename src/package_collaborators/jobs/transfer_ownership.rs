use std::env;
use std::future::Future;
use std::pin::Pin;

use jelly::anyhow::{anyhow, Error};
use jelly::email::Email;
use jelly::jobs::{Job, JobState};
use jelly::serde::{Deserialize, Serialize};
use jelly::tera::Context;

use crate::accounts::Account;

#[derive(Debug, Serialize, Deserialize)]
pub struct SendOwnershipTransferEmail {
    pub to: String,
    pub package_name: String,
    pub token: String,
}

impl Job for SendOwnershipTransferEmail {
    type State = JobState;
    type Future = Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;

    const NAME: &'static str = "SendOwnershipTransferEmail";

    fn run(self, state: JobState) -> Self::Future {
        Box::pin(async move {
            let account = Account::get_by_email(&self.to, &state.pool)
                .map_err(|e| anyhow!("Error fetching account for transfer invitation: {:?}", e))?;
            let domain = env::var("JELLY_DOMAIN").expect("No JELLY_DOMAIN value set!");

            let accept_transfer_url = format!("{}/collaborators/accept/{}", domain, self.token);

            let email = Email::new(
                "email/transfer-ownership",
                &[account.email],
                &format!(
                    "You have been invited to be the owner of package {}",
                    self.package_name
                ),
                {
                    let mut context = Context::new();
                    context.insert("action_url", &accept_transfer_url);
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
