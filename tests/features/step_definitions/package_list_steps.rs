use cucumber::{given, then, when};
use mainlib::packages::Package;
use mainlib::test::DB_POOL;
use thirtyfour::prelude::*;

use super::super::world::TestWorld;

#[given("I am on the package list page")]
async fn go_to_package_list_page(world: &mut TestWorld) {
    world
        .driver
        .get("http://localhost:17002/packages/index")
        .await
        .unwrap();
    assert_eq!(
        world.driver.title().await.unwrap().as_str(),
        "All Movey Packages"
    );
    let package_count = world
        .driver
        .find_elements(By::ClassName("package-list-item"))
        .await
        .unwrap()
        .len();
    assert_eq!(package_count, 4);
}

#[then(expr = "I should see the packages sorted by {word}")]
async fn see_sorted_packages(world: &mut TestWorld, field: String) {
    let package_items = world
        .driver
        .find_elements(By::Css(".package-list-item-title h1 span:first-child"))
        .await
        .unwrap();
    assert_ne!(package_items.len(), 0);
    let expected_names = match field.as_str() {
        "name" => vec!["random_derive", "rand_derive2", "rand", "faker_rand"],
        "description" => vec!["rand", "random_derive", "rand_derive2", "faker_rand"],
        "most_downloads" => vec!["random_derive", "faker_rand", "rand", "rand_derive2"],
        "newly_added" => vec!["rand_derive2", "faker_rand", "random_derive", "rand"],
        _ => vec![],
    };
    let mut real: String;
    for i in 0..package_items.len() {
        real = package_items[i].text().await.unwrap();
        assert_eq!(real, expected_names[i]);
    }
}
