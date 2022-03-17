use jelly::forms::{EmailField, PasswordField, TextField, Validation};
use serde::{Deserialize, Serialize};
use diesel::{Insertable};

use crate::schema::accounts;

fn default_redirect_path() -> String {
    "/".into()
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct LoginForm {
    pub email: EmailField,
    pub password: PasswordField,

    #[serde(default = "default_redirect_path")]
    pub redirect: String,
}

impl Validation for LoginForm {
    fn is_valid(&mut self) -> bool {
        self.email.is_valid() && !self.password.value.is_empty()
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct NewAccountForm {
    pub name: TextField,
    pub email: EmailField,
    pub password: PasswordField,
}

impl Validation for NewAccountForm {
    fn is_valid(&mut self) -> bool {
        self.name.is_valid()
            && self.email.is_valid()
            && self.password.validate_with(&[&self.name, &self.email])
    }
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct EmailForm {
    pub email: EmailField,
}

impl Validation for EmailForm {
    fn is_valid(&mut self) -> bool {
        self.email.is_valid()
    }
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct ChangePasswordForm {
    // Unused in rendering, but stored here to enable password
    // checking with relative values.
    pub name: Option<String>,
    pub email: Option<String>,

    pub password: PasswordField,
    pub password_confirm: PasswordField,
}

impl Validation for ChangePasswordForm {
    fn is_valid(&mut self) -> bool {
        if !self.password.is_valid() || !self.password_confirm.is_valid() {
            return false;
        }

        if self.password.value != self.password_confirm.value {
            self.password
                .errors
                .push("Passwords must match.".to_string());
            return false;
        }

        self.password
            .validate_with(&[&self.name.as_ref().unwrap(), &self.email.as_ref().unwrap()])
    }
}
