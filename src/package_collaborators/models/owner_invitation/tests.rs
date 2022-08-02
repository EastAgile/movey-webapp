use std::env;
use crate::package_collaborators::models::owner_invitation::OwnerInvitation;
use crate::packages::Package;
use crate::test::{DatabaseTestContext, DB_POOL};
use crate::utils::tests::setup_user;
use jelly::prelude::*;
use crate::utils::token::TOKEN_LENGTH;

async fn setup_invitation() -> OwnerInvitation {
    let invited_uid = setup_user(None, None).await;
    let invited_by_uid = setup_user(Some("email1@mail.com".to_string()), None).await;
    let pid = Package::create_test_package(
        &"package1".to_string(),
        &"".to_string(),
        &"".to_string(),
        &"1.0.0".to_string(),
        &"".to_string(),
        &"".to_string(),
        -1,
        -1,
        Some(invited_uid),
        &DB_POOL,
    )
    .await
    .unwrap();
    OwnerInvitation::create(invited_uid, invited_by_uid, pid, &DB_POOL.get().unwrap()).unwrap()
}
#[actix_rt::test]
async fn find_by_token_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let conn = DB_POOL.get().unwrap();

    let owner_invitation1 = setup_invitation().await;
    let owner_invitation2 =
        OwnerInvitation::find_by_token(&owner_invitation1.token, &conn).unwrap();
    assert_eq!(owner_invitation1, owner_invitation2);
    let not_found = OwnerInvitation::find_by_token("test", &conn);
    assert!(not_found.is_err());
    if let Err(Error::Database(diesel::NotFound)) = not_found {
    } else {
        panic!()
    }
}

#[actix_rt::test]
async fn find_by_id_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let conn = DB_POOL.get().unwrap();

    let owner_invitation1 = setup_invitation().await;
    let owner_invitation2 = OwnerInvitation::find_by_id(
        owner_invitation1.invited_user_id,
        owner_invitation1.package_id,
        &conn
    ).unwrap();
    assert_eq!(owner_invitation1, owner_invitation2);
    let not_found = OwnerInvitation::find_by_id(
        owner_invitation1.invited_by_user_id,
        owner_invitation1.package_id,
        &conn
    );
    assert!(not_found.is_err());
    if let Err(Error::Database(diesel::NotFound)) = not_found {
    } else {
        panic!()
    }
}

#[actix_rt::test]
async fn delete_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let conn = DB_POOL.get().unwrap();

    let owner_invitation = setup_invitation().await;
    owner_invitation.delete(&conn).unwrap();
    let not_found = OwnerInvitation::find_by_token(&owner_invitation.token, &conn);
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

    let owner_invitation = setup_invitation().await;
    env::set_var("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS", "1");
    assert!(!owner_invitation.is_expired());

    env::set_var("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS", "0");
    assert!(owner_invitation.is_expired());
}

#[actix_rt::test]
#[should_panic]
async fn is_expired_panics_if_expiration_days_is_less_than_0() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let owner_invitation = setup_invitation().await;
    env::set_var("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS", "-1");
    owner_invitation.is_expired();
}

#[actix_rt::test]
#[should_panic]
async fn is_expired_panics_if_expiration_days_is_not_an_integer() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let owner_invitation = setup_invitation().await;
    env::set_var("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS", "invalid-integer");
    owner_invitation.is_expired();
}

#[actix_rt::test]
async fn create_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let conn = DB_POOL.get().unwrap();

    let owner_invitation1 = setup_invitation().await;
    let owner_invitation2 = OwnerInvitation::find_by_id(
        owner_invitation1.invited_user_id,
        owner_invitation1.package_id,
        &conn
    ).unwrap();
    assert_eq!(owner_invitation1, owner_invitation2);
    assert_eq!(owner_invitation1.token.len(), TOKEN_LENGTH)
}

#[actix_rt::test]
async fn create_new_invitation_if_existing_one_is_expired() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let conn = DB_POOL.get().unwrap();

    let owner_invitation = setup_invitation().await;
    let token = owner_invitation.token;
    let created_at = owner_invitation.created_at;

    env::set_var("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS", "0");
    let owner_invitation = OwnerInvitation::create(
        owner_invitation.invited_user_id,
        owner_invitation.invited_by_user_id,
        owner_invitation.package_id,
        &conn
    ).unwrap();
    assert_ne!(token, owner_invitation.token);
    assert_ne!(created_at, owner_invitation.created_at);
}

#[actix_rt::test]
#[should_panic]
async fn not_create_new_invitation_if_it_already_exists() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let conn = DB_POOL.get().unwrap();

    let owner_invitation = setup_invitation().await;
    env::set_var("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS", "1");
    OwnerInvitation::create(
        owner_invitation.invited_user_id,
        owner_invitation.invited_by_user_id,
        owner_invitation.package_id,
        &conn
    ).unwrap();
}