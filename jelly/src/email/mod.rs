pub (crate) use common::Configurable;
pub use common::Email;
pub use tera::Context;

use anyhow::anyhow;

pub (crate) mod common;
#[cfg(feature = "email-postmark")]
pub mod postmark;
#[cfg(feature = "email-sendgrid")]
pub mod sendgrid;
#[cfg(feature = "email-smtp")]
pub mod smtp;
#[cfg(feature = "email-local")]
pub mod local;

impl Configurable for Email {
    fn check_conf() {
        #[cfg(feature = "email-postmark")]
        postmark::check_conf();
        #[cfg(feature = "email-smtp")]
        smtp::check_conf();
        #[cfg(feature = "email-sendgrid")]
        sendgrid::check_conf();
        #[cfg(feature = "email-local")]
        local::check_conf();
    }
}

impl Email {
    pub fn send(self) -> Result<(), anyhow::Error> {
       #[allow(unused_mut)]
       let mut res = Result::Err(anyhow!("No email provider configured"));
        #[cfg(feature = "email-postmark")]
        if res.is_err() {
            res = Email::send_via_postmark(&self, "https://api.postmarkapp.com" );
        }
        #[cfg(feature = "email-sendgrid")]
        if res.is_err() {
            res = Email::send_via_sendgrid(&self, "https://api.sendgrid.com");
        }
        #[cfg(feature = "email-smtp")]
        if res.is_err() {
            res = Email::send_via_smtp(&self);
        }
        #[cfg(feature = "email-local")]
        if res.is_err() {
            res = Email::send_locally(&self);
        }
        res
    }
}
