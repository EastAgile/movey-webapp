use crate::accounts::forms::NewAccountForm;
use crate::accounts::Account;
use crate::test::{DatabaseTestContext, DB_POOL};
use jelly::forms::{EmailField, PasswordField};

// Sample unit test
#[actix_rt::test]
async fn test_register() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let form = NewAccountForm {
        email: EmailField {
            value: "email@host.com".to_string(),
            errors: vec![],
        },
        password: PasswordField {
            value: "xxyyzz".to_string(),
            errors: vec![],
            hints: vec![],
        },
    };
    let uid = Account::register(&form, &DB_POOL).unwrap();
    let account = Account::get(uid, &DB_POOL).unwrap();
    assert_eq!(account.email, "email@host.com");
}
