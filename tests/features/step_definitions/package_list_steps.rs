use cucumber::{given, then};
use thirtyfour::prelude::*;

use super::super::world::TestWorld;

#[given("I am on the package list page")]
async fn go_to_package_list_page(world: &mut TestWorld) {
    world
        .driver
        .get("http://localhost:17002/packages/")
        .await
        .unwrap();
    assert_eq!(
        world.driver.title().await.unwrap().as_str(),
        "All Movey Packages | Movey"
    );
    let package_count = world
        .driver
        .find_elements(By::ClassName("package-list-item"))
        .await
        .unwrap()
        .len();
    assert_eq!(package_count, 4);

    let pagination_message = world
        .driver
        .find_element(By::ClassName("pagination-info-message"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(pagination_message, "Displaying 1 - 4 of 4 total results")
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
        "name" => vec!["faker_rand", "rand", "rand_derive2", "random_derive"],
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
