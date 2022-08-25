use crate::package_collaborators::package_collaborator::PackageCollaborator;
use crate::packages::Package;
use crate::test::{DatabaseTestContext, DB_POOL};
use jelly::prelude::*;
use crate::test::util::setup_user;

async fn setup_collaborator() -> (i32, i32) {
    let owner_id = setup_user(Some(String::from("user1@host.com")), None).await;
    let collaborator_id = setup_user(Some(String::from("user2@host.com")), None).await;
    let pid = Package::create_test_package(
        &"package1".to_string(),
        &"".to_string(),
        &"".to_string(),
        &"1.0.0".to_string(),
        &"".to_string(),
        &"".to_string(),
        -1,
        -1,
        Some(owner_id),
        &DB_POOL,
    )
        .await
        .unwrap();

    PackageCollaborator::new_collaborator(pid, collaborator_id, owner_id, &DB_POOL.get().unwrap()).unwrap();
    (pid, collaborator_id)
}
#[actix_rt::test]
async fn new_collaborator_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let (pid, uid) = setup_collaborator().await;
    let rel = PackageCollaborator::get(pid, uid, &DB_POOL.get().unwrap());
    assert!(rel.is_ok());
    assert_eq!(rel.as_ref().unwrap().package_id, pid);
    assert_eq!(rel.unwrap().account_id, uid);

    let uid2 = setup_user(Some("second@host.com".to_string()), None).await;
    PackageCollaborator::new_collaborator(pid, uid2, uid, &DB_POOL.get().unwrap()).unwrap();

    let res = PackageCollaborator::get_in_bulk_order_by_role(
        pid,
        vec![uid, uid2],
        &DB_POOL.get().unwrap()
    ).unwrap();

    assert_eq!(res.len(), 2);
}

#[actix_rt::test]
async fn get_non_existed_returns_err() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let not_found = PackageCollaborator::get(100000, 1000000, &DB_POOL.get().unwrap());
    assert!(not_found.is_err());
    if let Err(Error::Database(diesel::NotFound)) = not_found {
    } else {
        panic!()
    }
}

#[actix_rt::test]
async fn delete_by_id_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let db = &DB_POOL;
    let conn = db.get().unwrap();
    let res = PackageCollaborator::delete_by_id(1, 1, &conn).unwrap();
    assert_eq!(res, 0);
    let (pid, uid) = setup_collaborator().await;
    let res = PackageCollaborator::delete_by_id(uid, pid, &conn).unwrap();
    assert_eq!(res, 1);
    let not_found = PackageCollaborator::get(pid, uid, &DB_POOL.get().unwrap());
    assert!(not_found.is_err());
    if let Err(Error::Database(diesel::NotFound)) = not_found {
    } else {
        panic!()
    }
}

