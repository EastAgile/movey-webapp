use crate::package_collaborators::models::external_invitation::ExternalInvitation;
use crate::packages::Package;
use crate::test::{DatabaseTestContext, DB_POOL};
use crate::utils::tests::setup_user;
use jelly::prelude::*;
use std::env;

fn setup_external_invitation() -> ExternalInvitation {
    let outside_email = String::from("email@not_in_db.com");
    let uid = setup_user(Some("email1@mail.com".to_string()), None);
    let pid = Package::create_test_package(
        &"package1".to_string(),
        &"".to_string(),
        &"".to_string(),
        &"1.0.0".to_string(),
        &"".to_string(),
        &"".to_string(),
        &"".to_string(),
        -1,
        -1,
        Some(uid),
        &DB_POOL,
    )
    .unwrap();
    ExternalInvitation::create(&outside_email, uid, pid, &DB_POOL.get().unwrap()).unwrap()
}

#[actix_rt::test]
async fn external_invitation_find_by_id_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let db = &DB_POOL;
    let conn = db.get().unwrap();

    let external_1 = setup_external_invitation();
    let external_2 = ExternalInvitation::find_by_id(
        &external_1.external_user_email,
        external_1.package_id,
        &conn,
    )
    .unwrap();
    assert_eq!(external_1, external_2);
    let not_found =
        ExternalInvitation::find_by_id("some@random_email", external_1.package_id, &conn);
    assert!(not_found.is_err());
    if let Err(Error::Database(diesel::NotFound)) = not_found {
    } else {
        panic!()
    }
}

#[actix_rt::test]
async fn external_invitation_delete_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let db = &DB_POOL;
    let conn = db.get().unwrap();
    let external_invitation = setup_external_invitation();

    external_invitation.delete(&conn).unwrap();
    let not_found = ExternalInvitation::find_by_id(
        &external_invitation.external_user_email,
        external_invitation.package_id,
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

    let external_invitation = setup_external_invitation();
    env::set_var("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS", "1");
    assert!(!external_invitation.is_expired());

    env::set_var("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS", "0");
    assert!(external_invitation.is_expired());
}

#[actix_rt::test]
#[should_panic]
async fn is_expired_panics_if_expiration_days_is_less_than_0() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let external_invitation = setup_external_invitation();
    env::set_var("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS", "-1");
    external_invitation.is_expired();
}

#[actix_rt::test]
#[should_panic]
async fn is_expired_panics_if_expiration_days_is_not_an_integer() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let external_invitation = setup_external_invitation();
    env::set_var("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS", "invalid-integer");
    external_invitation.is_expired();
}

#[actix_rt::test]
async fn create_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let conn = DB_POOL.get().unwrap();

    let external1 = setup_external_invitation();
    let external2 =
        ExternalInvitation::find_by_id(&external1.external_user_email, external1.package_id, &conn)
            .unwrap();
    assert_eq!(external1, external2);
}

#[actix_rt::test]
async fn create_new_invitation_if_existing_one_is_expired() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let conn = DB_POOL.get().unwrap();

    let external1 = setup_external_invitation();
    let created_at = external1.created_at;

    env::set_var("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS", "0");
    let external2 = ExternalInvitation::create(
        &external1.external_user_email,
        external1.invited_by_user_id,
        external1.package_id,
        &conn,
    )
    .unwrap();
    assert_ne!(created_at, external2.created_at);
}

#[actix_rt::test]
#[should_panic]
async fn not_create_new_invitation_if_it_already_exists() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let conn = DB_POOL.get().unwrap();

    let external = setup_external_invitation();
    env::set_var("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS", "1");
    ExternalInvitation::create(
        &external.external_user_email,
        external.invited_by_user_id,
        external.package_id,
        &conn,
    )
    .unwrap();
}

#[actix_rt::test]
async fn external_invitation_find_by_email_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let db = &DB_POOL;
    let conn = db.get().unwrap();

    let external_1 = setup_external_invitation();
    let external_2 =
        ExternalInvitation::find_by_email(&external_1.external_user_email, &conn).unwrap();
    assert_eq!(external_2.len(), 1);
    assert_eq!(external_1, external_2[0]);
    let not_found = ExternalInvitation::find_by_email("not_existed@random_email", &conn);
    assert!(not_found.unwrap().is_empty());
}

#[actix_rt::test]
async fn external_invitation_delete_by_id_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let db = &DB_POOL;
    let conn = db.get().unwrap();

    let res = ExternalInvitation::delete_by_id("not_existed@mail.com", 1, &conn).unwrap();
    assert_eq!(res, 0);

    let external_invitation = setup_external_invitation();

    let res = ExternalInvitation::delete_by_id(
        &external_invitation.external_user_email,
        external_invitation.package_id,
        &conn,
    )
    .unwrap();
    assert_eq!(res, 1);
    let not_found = ExternalInvitation::find_by_id(
        &external_invitation.external_user_email,
        external_invitation.package_id,
        &conn,
    );
    assert!(not_found.is_err());
    if let Err(Error::Database(diesel::NotFound)) = not_found {
    } else {
        panic!()
    }
}
