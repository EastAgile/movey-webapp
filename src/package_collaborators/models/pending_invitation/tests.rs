use crate::package_collaborators::models::pending_invitation::PendingInvitation;
use crate::packages::Package;
use crate::test::{DatabaseTestContext, DB_POOL};
use crate::utils::tests::setup_user;
use jelly::prelude::*;
use std::env;

async fn setup_pending_invitation() -> PendingInvitation {
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
    PendingInvitation::create(&outside_email, uid, pid, &DB_POOL.get().unwrap()).unwrap()
}

#[actix_rt::test]
async fn pending_invitation_find_by_id_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let db = &DB_POOL;
    let conn = db.get().unwrap();

    let pending_1 = setup_pending_invitation().await;
    let pending_2 =
        PendingInvitation::find_by_id(&pending_1.pending_user_email, pending_1.package_id, &conn)
            .unwrap();
    assert_eq!(pending_1, pending_2);
    let not_found = PendingInvitation::find_by_id("some@random_email", pending_1.package_id, &conn);
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

    let db = &DB_POOL;
    let conn = db.get().unwrap();
    let pending_invitation = setup_pending_invitation().await;

    pending_invitation.delete(&conn).unwrap();
    let not_found = PendingInvitation::find_by_id(
        &pending_invitation.pending_user_email,
        pending_invitation.package_id,
        &conn,
    );
    assert!(not_found.is_err());
    if let Err(Error::Database(diesel::NotFound)) = not_found {
    } else {
        panic!()
    }
}

#[actix_rt::test]
async fn is_expired_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let pending_invitation = setup_pending_invitation().await;
    env::set_var("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS", "1");
    assert!(!pending_invitation.is_expired());

    env::set_var("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS", "0");
    assert!(pending_invitation.is_expired());
}

#[actix_rt::test]
#[should_panic]
async fn is_expired_panics_if_expiration_days_is_less_than_0() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let pending_invitation = setup_pending_invitation().await;
    env::set_var("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS", "-1");
    pending_invitation.is_expired();
}

#[actix_rt::test]
#[should_panic]
async fn is_expired_panics_if_expiration_days_is_not_an_integer() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let pending_invitation = setup_pending_invitation().await;
    env::set_var("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS", "invalid-integer");
    pending_invitation.is_expired();
}

#[actix_rt::test]
async fn create_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let conn = DB_POOL.get().unwrap();

    let pending1 = setup_pending_invitation().await;
    let pending2 =
        PendingInvitation::find_by_id(&pending1.pending_user_email, pending1.package_id, &conn)
            .unwrap();
    assert_eq!(pending1, pending2);
}

#[actix_rt::test]
async fn create_new_invitation_if_existing_one_is_expired() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let conn = DB_POOL.get().unwrap();

    let pending1 = setup_pending_invitation().await;
    let created_at = pending1.created_at;

    env::set_var("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS", "0");
    let pending2 = PendingInvitation::create(
        &pending1.pending_user_email,
        pending1.invited_by_user_id,
        pending1.package_id,
        &conn,
    )
    .unwrap();
    assert_ne!(created_at, pending2.created_at);
}

#[actix_rt::test]
#[should_panic]
async fn not_create_new_invitation_if_it_already_exists() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let conn = DB_POOL.get().unwrap();

    let pending = setup_pending_invitation().await;
    env::set_var("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS", "1");
    PendingInvitation::create(
        &pending.pending_user_email,
        pending.invited_by_user_id,
        pending.package_id,
        &conn,
    )
    .unwrap();
}

#[actix_rt::test]
async fn pending_invitation_find_by_email_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let db = &DB_POOL;
    let conn = db.get().unwrap();

    let pending_1 = setup_pending_invitation().await;
    let pending_2 =
        PendingInvitation::find_by_email(&pending_1.pending_user_email, &conn).unwrap();
    assert_eq!(pending_2.len(), 1);
    assert_eq!(pending_1, pending_2[0]);
    let not_found = PendingInvitation::find_by_email("not_existed@random_email", &conn);
    assert!(not_found.unwrap().is_empty());
}
