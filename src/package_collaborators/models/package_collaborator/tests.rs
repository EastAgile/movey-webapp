use crate::package_collaborators::package_collaborator::PackageCollaborator;
use crate::packages::Package;
use crate::test::{DatabaseTestContext, DB_POOL};
use crate::utils::tests::setup_user;
use jelly::prelude::*;

async fn setup_collaborator() -> (i32, i32) {
    let uid = setup_user(None, None).await;
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

    PackageCollaborator::new_collaborator(pid, uid, uid, &DB_POOL.get().unwrap()).unwrap();
    (pid, uid)
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

