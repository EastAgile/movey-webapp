use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
use std::ops::Deref;
use zxcvbn::zxcvbn;

use super::Validation;

/// A field for validating password strength. Will also include
/// hints on how to make a better password.
#[derive(Debug, Default, Serialize)]
pub struct PasswordField {
    pub value: String,
    pub errors: Vec<String>,
    pub hints: Vec<String>,
}

impl PasswordField {
    pub fn validate_with(&mut self, user_inputs: &[&str]) -> bool {
        if self.value == "" {
            self.errors.push("Password cannot be blank.".to_string());
            return false;
        }

        // The unwrap is safe, as it only errors if the
        // password is blank, which we already
        // handle above.
        let estimate = zxcvbn(&self.value, user_inputs).unwrap();
        if estimate.score() <= 2 {
            if let Some(feedback) = estimate.feedback() {
                if let Some(warning) = feedback.warning() {
                    self.errors.push(format!("{}", warning));
                } else {
                    self.errors.push(format!("{}", "Password not strong enough."));
                }

                self.hints = feedback
                    .suggestions()
                    .iter()
                    .map(|s| format!("{}", s))
                    .collect();
            }

            return false;
        }

        true
    }
}

impl fmt::Display for PasswordField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<'de> Deserialize<'de> for PasswordField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|t| PasswordField {
            value: t,
            errors: Vec::new(),
            hints: Vec::new(),
        })
    }
}

impl Deref for PasswordField {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Validation for PasswordField {
    fn is_valid(&mut self) -> bool {
        self.validate_with(&[])
    }
}
