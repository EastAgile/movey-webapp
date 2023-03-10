use crate::accounts::forms::NewAccountForm;
use crate::accounts::Account;
use crate::test::DB_POOL;
use jelly::forms::{EmailField, PasswordField};

pub fn setup_user(email: Option<String>, password: Option<String>) -> i32 {
    let form = NewAccountForm {
        email: EmailField {
            value: email.unwrap_or_else(|| "email@host.com".to_string()),
            errors: vec![],
        },
        password: PasswordField {
            value: password.unwrap_or_else(|| "So$trongpas0word!".to_string()),
            errors: vec![],
            hints: vec![],
        },
    };
    Account::register(&form, &DB_POOL).unwrap()
}
