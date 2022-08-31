use cucumber::{then, when};
use thirtyfour::By;

use crate::features::world::TestWorld;

#[when("I access a public profile")]
async fn access_public_profile(world: &mut TestWorld) {
    world
        .driver
        .get(format!("{}users/{}", world.root_url, world.first_account.slug))
        .await
        .unwrap()
}

#[when("I access a package details page")]
async fn access_test_package_details(world: &mut TestWorld) {
    world
        .driver
        .get(format!("{}/packages/test-package", world.root_url))
        .await
        .unwrap()
}

#[when("I click on the owner name")]
async fn click_on_owner_name(world: &mut TestWorld) {
    let owner_name = world
        .driver
        .find_element(By::ClassName("package-owners-info"))
        .await
        .unwrap();
    owner_name.click().await.unwrap()
}

#[then("I should see the public profile")]
async fn see_public_profile(world: &mut TestWorld) {
    let handles = world.driver.window_handles().await.unwrap();
    world.driver.switch_to().window(&handles[1]).await.unwrap();
    assert_eq!(
        world.driver.current_url().await.unwrap(),
        format!("{}users/{}", world.root_url, world.first_account.slug)
    );

    let owner_display = world
        .driver
        .find_element(By::ClassName("owner-display"))
        .await;
    assert!(owner_display.is_ok());

    let owner_name = world
        .driver
        .find_element(By::ClassName("owner-name"))
        .await
        .unwrap();
    let name = world.first_account.email.split('@').next().unwrap();
    assert_eq!(owner_name.text().await.unwrap(), name)
}

#[then("I should see packages that he/she owns")]
async fn see_packages_in_public_profile(world: &mut TestWorld) {
    let packages_list = world
        .driver
        .find_elements(By::ClassName("package-list-item"))
        .await;
    assert!(packages_list.is_ok());

    let packages_list = packages_list.unwrap();
    assert_eq!(packages_list.len(), 1);

    let package_name = packages_list[0]
        .find_element(By::ClassName("package-list-item-title"))
        .await
        .unwrap();
    let package_name = package_name.text().await.unwrap();
    assert!(package_name.contains("test-package"));
}
