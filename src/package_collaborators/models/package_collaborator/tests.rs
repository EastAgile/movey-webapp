use crate::package_collaborators::package_collaborator::PackageCollaborator;
use crate::packages::Package;
use crate::test::{DatabaseTestContext, DB_POOL};
use crate::utils::tests::setup_user;

#[actix_rt::test]
async fn new_collaborator_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

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

    let rel = PackageCollaborator::get(pid, uid, &DB_POOL.get().unwrap());
    assert!(rel.is_ok());
    assert_eq!(rel.as_ref().unwrap().package_id, pid);
    assert_eq!(rel.unwrap().account_id, uid);
}
