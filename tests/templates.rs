#[macro_use]
extern crate lazy_static;

use jelly::tera::Tera;
use std::env;

// Load templates once for the tests
lazy_static! {
    static ref TEMPLATES: Tera = {
        dotenv::dotenv().ok();
        let templates_glob = env::var("TEMPLATES_GLOB").expect("TEMPLATES_GLOB not set!");

        Tera::new(&templates_glob).expect("Unable to compile templates!")
    };
}

mod template_should_work_for {
    use super::*;
    /// Test that email templates render correctly with current .env.
    /// You should adapt the test to follow the settings in your .env and
    /// the template your use.

    #[allow(unused_imports)]
    use anyhow::{self, bail};
    use jelly::tera::escape_html;
    use log::debug;
    use mainlib::accounts::jobs;
    use std::env;
    use std::sync::{Arc, RwLock};
    use test_log::test;

    #[test]
    fn odd_registration_attempt() -> Result<(), anyhow::Error> {
        dotenv::dotenv().ok();
        let email = jelly::email::Email::new(
            "email/odd-registration-attempt",
            &["Erby Doe <test@example.com>".to_string()],
            "Test subject",
            jobs::build_odd_registration_attempt_context("John Doe"),
            Arc::new(RwLock::new(TEMPLATES.clone())),
        )?;

        assert_eq!(email.from, env::var("EMAIL_DEFAULT_FROM")?);
        assert_eq!(email.to, "Erby Doe <test@example.com>");
        assert_eq!(email.subject, "Test subject");
        debug!("{}", email.body);
        assert!(email.body.contains("accounts/reset"));
        debug!("{}", email.body_html);
        assert!(email.body_html.contains(&escape_html("accounts/reset")));
        Ok(())
    }

    #[test]
    fn reset_password() -> Result<(), anyhow::Error> {
        dotenv::dotenv().ok();
        let email = jelly::email::Email::new(
            "email/reset-password",
            &["Erby Doe <test@example.com>".to_string()],
            "Test subject",
            jobs::build_reset_password_context("/verify/xxxx"),
            Arc::new(RwLock::new(TEMPLATES.clone())),
        )?;

        assert_eq!(email.from, env::var("EMAIL_DEFAULT_FROM")?);
        assert_eq!(email.to, "Erby Doe <test@example.com>");
        assert_eq!(email.subject, "Test subject");
        debug!("{}", email.body);
        assert!(email.body.contains("/verify/xxxx"));
        debug!("{}", email.body_html);
        assert!(email.body_html.contains(&escape_html("/verify/xxxx")));
        Ok(())
    }

    #[test]
    fn verify_account() -> Result<(), anyhow::Error> {
        dotenv::dotenv().ok();
        let email = jelly::email::Email::new(
            "email/verify-account",
            &["Erby Doe <test@example.com>".to_string()],
            "Test subject",
            jobs::build_verify_context("/verify/account"),
            Arc::new(RwLock::new(TEMPLATES.clone())),
        )?;

        assert_eq!(email.from, env::var("EMAIL_DEFAULT_FROM")?);
        assert_eq!(email.to, "Erby Doe <test@example.com>");
        assert_eq!(email.subject, "Test subject");
        debug!("{}", email.body);
        assert!(email.body.contains("/verify/account"));
        debug!("{}", email.body_html);
        assert!(email.body_html.contains(&escape_html("/verify/account")));
        Ok(())
    }
}
