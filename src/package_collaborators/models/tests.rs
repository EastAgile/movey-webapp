use super::*;
use crate::{
    accounts::forms::NewAccountForm,
    test::{DatabaseTestContext, DB_POOL},
};
use diesel::dsl::count;
use jelly::forms::{EmailField, PasswordField};

async fn setup_user() -> i32 {
    let form = NewAccountForm {
        email: EmailField {
            value: "email@host.com".to_string(),
            errors: vec![],
        },
        password: PasswordField {
            value: "So$trongpas0word!".to_string(),
            errors: vec![],
            hints: vec![],
        },
    };
    Account::register(&form, &DB_POOL).await.unwrap()
}

#[actix_rt::test]
async fn new_collaborator_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let uid = setup_user().await;
    let pid = Package::create_test_package(
        &"package1".to_string(),
        &"".to_string(),
        &"".to_string(),
        &"1.0.0".to_string(),
        &"".to_string(),
        &"".to_string(),
        -1,
        -1,
        Some(uid),
        &DB_POOL,
    )
    .await
    .unwrap();

    PackageCollaborator::new_collaborator(pid, uid, uid, &DB_POOL)
        .await
        .unwrap();

    let rel = PackageCollaborator::get(pid, uid, &DB_POOL).await;
    assert!(rel.is_ok());
    assert_eq!(rel.as_ref().unwrap().package_id, pid);
    assert_eq!(rel.unwrap().account_id, uid);
}
