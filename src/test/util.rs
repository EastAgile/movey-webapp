use jelly::forms::{EmailField, PasswordField};
use crate::accounts::Account;
use crate::accounts::forms::NewAccountForm;
use crate::packages::Package;
use crate::settings::models::token::ApiToken;
use crate::test::DB_POOL;

fn new_account_form() -> NewAccountForm {
    NewAccountForm {
        email: EmailField {
            value: "email@host.com".to_string(),
            errors: vec![],
        },
        password: PasswordField {
            value: "So$trongpas0word!".to_string(),
            errors: vec![],
            hints: vec![],
        },
    }
}

pub async fn setup_user() -> i32 {
    let form = new_account_form();
    let uid = Account::register(&form, &DB_POOL).await.unwrap();
    let _ = Account::mark_verified(uid, &DB_POOL).await;
    uid
}

pub async fn create_test_token() -> String {
    let form = new_account_form();
    let uid = Account::register(&form, &DB_POOL).await.unwrap();
    let account = Account::get(uid, &DB_POOL).await.unwrap();
    ApiToken::insert(&account, "test_key", &DB_POOL)
        .await
        .unwrap()
        .plaintext
}

pub async fn create_stub_packages(account_id: i32, num_of_packages: i32) {
    for idx in 0..num_of_packages {
        Package::create_test_package(
            &format!("package_{}_{}", idx, account_id),
            &"repo_url".to_string(),
            &"package_description".to_string(),
            &"0.0.0".to_string(),
            &"".to_string(),
            &"".to_string(),
            10,
            200,
            Some(account_id),
            &DB_POOL)
            .await.unwrap();
    }
}
