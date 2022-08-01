use crate::package_collaborators::models::pending_invitation::PendingInvitation;
use crate::packages::Package;
use crate::test::{DatabaseTestContext, DB_POOL};
use crate::utils::tests::setup_user;
use jelly::prelude::*;

async fn setup_user_package() -> (String, i32, i32) {
    let outside_email = String::from("email@not_in_db.com");
    let uid = setup_user(Some("email1@mail.com".to_string()), None).await;
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
    (outside_email, uid, pid)
}
#[actix_rt::test]
async fn pending_invitation_find_by_token_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let (email, uid, pid) = setup_user_package().await;
    let db = &DB_POOL;
    let conn = db.get().unwrap();
    let pending_1 = PendingInvitation::create(&email, uid, pid, &conn).unwrap();

    let pending_2 = PendingInvitation::find_by_token(&pending_1.token, &conn).unwrap();
    assert_eq!(pending_1, pending_2);
    let not_found = PendingInvitation::find_by_token("test", &conn);
    assert!(not_found.is_err());
    if let Err(Error::Database(diesel::NotFound)) = not_found {
    } else {
        panic!()
    }
}

#[actix_rt::test]
async fn pending_invitation_find_by_id_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let (email, uid, pid) = setup_user_package().await;
    let db = &DB_POOL;
    let conn = db.get().unwrap();
    let pending_1 = PendingInvitation::create(&email, uid, pid, &conn).unwrap();
    let pending_2 = PendingInvitation::find_by_id(&email, pid, &conn).unwrap();
    assert_eq!(pending_1, pending_2);
    let not_found = PendingInvitation::find_by_id("some@random_email", pid, &conn);
    assert!(not_found.is_err());
    if let Err(Error::Database(diesel::NotFound)) = not_found {
    } else {
        panic!()
    }
}

#[actix_rt::test]
async fn pending_invitation_delete_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let (email, uid, pid) = setup_user_package().await;
    let db = &DB_POOL;
    let conn = db.get().unwrap();
    let owner_invitation = PendingInvitation::create(&email, uid, pid, &conn).unwrap();

    owner_invitation.delete(&conn).unwrap();
    let not_found = PendingInvitation::find_by_token(&owner_invitation.token, &conn);
    assert!(not_found.is_err());
    if let Err(Error::Database(diesel::NotFound)) = not_found {
    } else {
        panic!()
    }
}
