use crate::package_collaborators::models::owner_invitation::OwnerInvitation;
use crate::packages::Package;
use crate::test::{DatabaseTestContext, DB_POOL};
use crate::utils::tests::setup_user;
use jelly::prelude::*;

async fn setup_user_package() -> (i32, i32, i32) {
    let uid1 = setup_user(None, None).await;
    let uid2 = setup_user(Some("email1@mail.com".to_string()), None).await;
    let pid = Package::create_test_package(
        &"package1".to_string(),
        &"".to_string(),
        &"".to_string(),
        &"1.0.0".to_string(),
        &"".to_string(),
        &"".to_string(),
        -1,
        -1,
        Some(uid1),
        &DB_POOL,
    )
    .await
    .unwrap();
    (uid1, uid2, pid)
}
#[actix_rt::test]
async fn find_by_token_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let (uid1, uid2, pid) = setup_user_package().await;
    let db = &DB_POOL;
    let conn = db.get().unwrap();
    let owner_invitation1 = OwnerInvitation::create(uid1, uid2, pid, &conn).unwrap();

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

    let (uid1, uid2, pid) = setup_user_package().await;
    let db = &DB_POOL;
    let conn = db.get().unwrap();
    let owner_invitation1 = OwnerInvitation::create(uid1, uid2, pid, &conn).unwrap();

    let owner_invitation2 = OwnerInvitation::find_by_id(uid1, pid, &conn).unwrap();
    assert_eq!(owner_invitation1, owner_invitation2);
    let not_found = OwnerInvitation::find_by_id(uid2, pid, &conn);
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

    let (uid1, uid2, pid) = setup_user_package().await;
    let db = &DB_POOL;
    let conn = db.get().unwrap();
    let owner_invitation = OwnerInvitation::create(uid1, uid2, pid, &conn).unwrap();

    owner_invitation.delete(&conn).unwrap();
    let not_found = OwnerInvitation::find_by_token(&owner_invitation.token, &conn);
    assert!(not_found.is_err());
    if let Err(Error::Database(diesel::NotFound)) = not_found {
    } else {
        panic!()
    }
}
