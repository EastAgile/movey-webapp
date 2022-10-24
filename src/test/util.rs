use crate::accounts::forms::NewAccountForm;
use crate::accounts::Account;
use crate::packages::Package;
use crate::settings::models::token::ApiToken;
use crate::test::DB_POOL;
use jelly::forms::{EmailField, PasswordField};

fn new_account_form(custom_email: Option<&str>) -> NewAccountForm {
    NewAccountForm {
        email: EmailField {
            value: custom_email.unwrap_or("email@host.com").to_string(),
            errors: vec![],
        },
        password: PasswordField {
            value: "So$trongpas0word!".to_string(),
            errors: vec![],
            hints: vec![],
        },
    }
}

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
    let uid = Account::register(&form, &DB_POOL).unwrap();
    let _ = Account::mark_verified(uid, &DB_POOL);
    uid
}

pub fn create_test_token() -> String {
    let form = new_account_form(None);
    let uid = Account::register(&form, &DB_POOL).unwrap();
    let account = Account::get(uid, &DB_POOL).unwrap();
    ApiToken::insert(&account, "test_key", &DB_POOL)
        .unwrap()
        .plaintext
}

pub fn create_stub_packages(account_id: i32, num_of_packages: i32) {
    for idx in 0..num_of_packages {
        Package::create_test_package(
            &format!("package_{}_{}", idx, account_id),
            &"repo_url".to_string(),
            &"package_description".to_string(),
            &"0.0.0".to_string(),
            &"".to_string(),
            &"".to_string(),
            &"".to_string(),
            10,
            200,
            Some(account_id),
            &DB_POOL,
        )
        .unwrap();
    }
}
