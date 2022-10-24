use crate::TestWorld;
use cucumber::{given, then};
use mainlib::packages::Package;
use mainlib::test::DB_POOL;
use thirtyfour::prelude::*;

#[given("I visit the My packages page")]
pub async fn visit_my_package_page(world: &mut TestWorld) {
    world.go_to_root_url().await;
    world
        .driver
        .get("http://localhost:17002/settings/packages")
        .await
        .unwrap();
}

#[given("I upload some packages")]
pub async fn upload_packages(world: &mut TestWorld) {
    Package::create_test_package(
        &"test-package1".to_string(),
        &"https://github.com/Elements-Studio/starswap-core".to_string(),
        &"package_description".to_string(),
        &"first_version".to_string(),
        &"first_readme_content".to_string(),
        &"license".to_string(),
        &"rev".to_string(),
        2,
        100,
        0,
        0,
        Some(world.first_account.id),
        &DB_POOL,
    )
    .unwrap();
    Package::create_test_package(
        &"test-package2".to_string(),
        &"https://github.com/Elements-Studio/rand".to_string(),
        &"package_description".to_string(),
        &"first_version".to_string(),
        &"first_readme_content".to_string(),
        &"license".to_string(),
        &"rev".to_string(),
        2,
        100,
        0,
        0,
        Some(world.first_account.id),
        &DB_POOL,
    )
    .unwrap();
}

#[then("I should see the My packages page")]
async fn see_my_package_page(world: &mut TestWorld) {
    assert_eq!(
        world.driver.current_url().await.unwrap(),
        "http://localhost:17002/settings/packages"
    );
}

#[then("I should see the list of packages that I uploaded")]
async fn see_my_packages_list(world: &mut TestWorld) {
    let total_packages = world
        .driver
        .find_element(By::ClassName("total_package"))
        .await
        .unwrap();
    assert_eq!(total_packages.text().await.unwrap(), "2 Packages");

    let package_names: Vec<WebElement> = world
        .driver
        .find_elements(By::Css(".package_content .package_name"))
        .await
        .unwrap();
    assert_eq!(package_names.len(), 2);
    let p: &WebElement = &package_names[0];
    assert!(p.text().await.unwrap().starts_with("test-package"));
    let p: &WebElement = &package_names[1];
    assert!(p.text().await.unwrap().starts_with("test-package"));
}
