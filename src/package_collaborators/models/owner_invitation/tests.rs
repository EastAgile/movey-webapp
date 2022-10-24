use crate::package_collaborators::models::owner_invitation::OwnerInvitation;
use crate::package_collaborators::package_collaborator::{PackageCollaborator, Role};
use crate::packages::Package;
use crate::test::util::setup_user;
use crate::test::{DatabaseTestContext, DB_POOL};
use crate::utils::token::TOKEN_LENGTH;
use jelly::prelude::*;
use std::env;

fn setup_invitation(is_transferring: Option<bool>) -> OwnerInvitation {
    let invited_uid = setup_user(None, None);
    let invited_by_uid = setup_user(Some("email1@mail.com".to_string()), None);
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
        0,
        0,
        Some(invited_by_uid),
        &DB_POOL,
    )
    .unwrap();
    OwnerInvitation::create(
        invited_uid,
        invited_by_uid,
        pid,
        is_transferring,
        None,
        &DB_POOL.get().unwrap(),
    )
    .unwrap()
}

#[actix_rt::test]
async fn find_by_token_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let conn = DB_POOL.get().unwrap();

    let owner_invitation1 = setup_invitation(None);
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

    let owner_invitation1 = setup_invitation(None);
    let owner_invitation2 = OwnerInvitation::find_by_id(
        owner_invitation1.invited_user_id,
        owner_invitation1.package_id,
        &conn,
    )
    .unwrap();
    assert_eq!(owner_invitation1, owner_invitation2);
    let not_found = OwnerInvitation::find_by_id(
        owner_invitation1.invited_by_user_id,
        owner_invitation1.package_id,
        &conn,
    );
    assert!(not_found.is_err());
    if let Err(Error::Database(diesel::NotFound)) = not_found {
    } else {
        panic!()
    }
}

#[actix_rt::test]
async fn accept_collaborator_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let conn = DB_POOL.get().unwrap();

    let owner_invitation = setup_invitation(None);
    let package_collaborator = PackageCollaborator::get(
        owner_invitation.package_id,
        owner_invitation.invited_user_id,
        &DB_POOL.get().unwrap(),
    );
    assert!(package_collaborator.is_err());

    owner_invitation.accept(&conn).unwrap();
    let not_found = OwnerInvitation::find_by_token(&owner_invitation.token, &conn);
    assert!(not_found.is_err());
    if let Err(Error::Database(diesel::NotFound)) = not_found {
    } else {
        panic!()
    }

    let package_collaborator = PackageCollaborator::get(
        owner_invitation.package_id,
        owner_invitation.invited_user_id,
        &DB_POOL.get().unwrap(),
    )
    .unwrap();
    assert_eq!(package_collaborator.role, Role::Collaborator as i32);
}

#[actix_rt::test]
async fn accept_owner_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let conn = DB_POOL.get().unwrap();

    let owner_invitation = setup_invitation(Some(true));
    let _ = PackageCollaborator::new_collaborator(
        owner_invitation.package_id,
        owner_invitation.invited_user_id,
        owner_invitation.invited_user_id,
        &DB_POOL.get().unwrap(),
    );
    let invited_collaborator = PackageCollaborator::get(
        owner_invitation.package_id,
        owner_invitation.invited_user_id,
        &DB_POOL.get().unwrap(),
    )
    .unwrap();
    assert_eq!(invited_collaborator.role, Role::Collaborator as i32);

    owner_invitation.accept(&conn).unwrap();
    let not_found = OwnerInvitation::find_by_token(&owner_invitation.token, &conn);
    assert!(not_found.is_err());
    if let Err(Error::Database(diesel::NotFound)) = not_found {
    } else {
        panic!()
    }

    let package_collaborator = PackageCollaborator::get(
        owner_invitation.package_id,
        owner_invitation.invited_user_id,
        &DB_POOL.get().unwrap(),
    )
    .unwrap();
    assert_eq!(package_collaborator.role, Role::Owner as i32);
}

#[actix_rt::test]
async fn delete_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let conn = DB_POOL.get().unwrap();

    let owner_invitation = setup_invitation(None);
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

    let owner_invitation = setup_invitation(None);
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

    let owner_invitation = setup_invitation(None);
    env::set_var("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS", "-1");
    owner_invitation.is_expired();
}

#[actix_rt::test]
#[should_panic]
async fn is_expired_panics_if_expiration_days_is_not_an_integer() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let owner_invitation = setup_invitation(None);
    env::set_var("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS", "invalid-integer");
    owner_invitation.is_expired();
}

#[actix_rt::test]
async fn create_transfer_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let owner_invitation1 = setup_invitation(Some(true));
    assert_eq!(owner_invitation1.token.len(), TOKEN_LENGTH);
    assert!(owner_invitation1.is_transferring);
}

#[actix_rt::test]
async fn create_new_invitation_if_existing_one_is_expired() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let conn = DB_POOL.get().unwrap();

    let owner_invitation = setup_invitation(None);
    let token = owner_invitation.token;
    let created_at = owner_invitation.created_at;

    env::set_var("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS", "0");
    let owner_invitation = OwnerInvitation::create(
        owner_invitation.invited_user_id,
        owner_invitation.invited_by_user_id,
        owner_invitation.package_id,
        None,
        None,
        &conn,
    )
    .unwrap();
    assert_ne!(token, owner_invitation.token);
    assert_ne!(created_at, owner_invitation.created_at);
}

#[actix_rt::test]
#[should_panic]
async fn not_create_new_invitation_if_it_already_exists() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let conn = DB_POOL.get().unwrap();

    let owner_invitation = setup_invitation(None);
    env::set_var("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS", "1");
    OwnerInvitation::create(
        owner_invitation.invited_user_id,
        owner_invitation.invited_by_user_id,
        owner_invitation.package_id,
        None,
        None,
        &conn,
    )
    .unwrap();
}

#[actix_rt::test]
async fn find_by_invited_account_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let conn = DB_POOL.get().unwrap();

    let owner_invitation = setup_invitation(None);
    env::set_var("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS", "1");
    let invited_account =
        OwnerInvitation::find_by_invited_account(owner_invitation.invited_user_id, &conn).unwrap();

    assert_eq!(invited_account.len(), 1);
    assert_eq!(
        invited_account[0].invited_user_id,
        owner_invitation.invited_user_id
    );
    assert_eq!(invited_account[0].package_id, owner_invitation.package_id);
    assert_eq!(
        invited_account[0].invited_by_user_id,
        owner_invitation.invited_by_user_id
    );
}

#[actix_rt::test]
async fn find_by_invited_account_returns_err_if_invitation_expired() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let conn = DB_POOL.get().unwrap();

    let owner_invitation = setup_invitation(None);
    env::set_var("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS", "0");

    let invited_account =
        OwnerInvitation::find_by_invited_account(owner_invitation.invited_user_id, &conn).unwrap();
    assert_eq!(invited_account.len(), 0);
}

#[actix_rt::test]
async fn delete_by_id_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let db = &DB_POOL;
    let conn = db.get().unwrap();

    let res = OwnerInvitation::delete_by_id(1, 1, &conn).unwrap();
    assert_eq!(res, 0);

    let owner_invitation = setup_invitation(None);

    let res = OwnerInvitation::delete_by_id(
        owner_invitation.invited_user_id,
        owner_invitation.package_id,
        &conn,
    )
    .unwrap();
    assert_eq!(res, 1);
    let not_found = OwnerInvitation::find_by_id(
        owner_invitation.invited_user_id,
        owner_invitation.package_id,
        &conn,
    );
    assert!(not_found.is_err());
    if let Err(Error::Database(diesel::NotFound)) = not_found {
    } else {
        panic!()
    }
}
