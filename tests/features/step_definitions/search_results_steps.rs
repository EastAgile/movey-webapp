use cucumber::{given, then, when};
use mainlib::packages::Package;
use mainlib::test::DB_POOL;
use thirtyfour::prelude::*;

use super::super::world::TestWorld;

#[given("There are packages in the database")]
async fn package_in_db(_world: &mut TestWorld) {
    Package::create_test_package_with_downloads(
        &"rand".to_string(),
        &"repo_url".to_string(),
        &"Random number generators and other randomness functionality.".to_string(),
        1000,
        &DB_POOL,
    )
    .await
    .unwrap();
    Package::create_test_package_with_downloads(
        &"random_derive".to_string(),
        &"repo_url".to_string(),
        &"Procedurally defined macro for automatically deriving rand::Rand for structs and enums"
            .to_string(),
        5000,
        &DB_POOL,
    )
    .await
    .unwrap();
    Package::create_test_package_with_downloads(
        &"faker_rand".to_string(),
        &"repo_url".to_string(),
        &"Fake data generators for lorem ipsum, names, emails, and more".to_string(),
        2500,
        &DB_POOL,
    )
    .await
    .unwrap();
    Package::create_test_package_with_downloads(
        &"rand_derive2".to_string(),
        &"repo_url".to_string(),
        &"Generate customizable random types with the rand crate".to_string(),
        300,
        &DB_POOL,
    )
    .await
    .unwrap();
}

#[when("I access the Homepage")]
async fn visit_homepage(world: &mut TestWorld) {
    world.go_to_root_url().await;
}

#[when("I input a string on search bar")]
async fn input_search(world: &mut TestWorld) {
    let search_bar = world
        .driver
        .find_element(By::Id("search-bar"))
        .await
        .unwrap();
    search_bar.send_keys("rand").await.unwrap();
}

#[when("I submit the search form")]
async fn submit_search_form(world: &mut TestWorld) {
    let search_bar = world
        .driver
        .find_element(By::Id("search-bar"))
        .await
        .unwrap();
    search_bar.send_keys(Keys::Enter).await.unwrap()
}

#[then("I should see the Search Results page")]
async fn see_search_results(world: &mut TestWorld) {
    assert_eq!(
        world.driver.title().await.unwrap().as_str(),
        "Search results for rand | Movey"
    );
    let package_count = world
        .driver
        .find_element(By::Css(".setting-bar p"))
        .await
        .unwrap();
    assert_eq!(package_count.text().await.unwrap(), "4 results for 'rand'");
    let package_count = world
        .driver
        .find_elements(By::ClassName("package-list-item"))
        .await
        .unwrap()
        .len();
    assert_eq!(package_count, 4);
}

#[given("I have searched for packages with a string")]
async fn search_by_text(world: &mut TestWorld) {
    world
        .driver
        .get("http://localhost:17002/packages/search?query=rand")
        .await
        .unwrap();
    assert_eq!(
        world.driver.title().await.unwrap().as_str(),
        "Search results for rand | Movey"
    );
    let package_count = world
        .driver
        .find_element(By::Css(".setting-bar p"))
        .await
        .unwrap();
    assert_eq!(package_count.text().await.unwrap(), "4 results for 'rand'");
    let package_count = world
        .driver
        .find_elements(By::ClassName("package-list-item"))
        .await
        .unwrap()
        .len();
    assert_eq!(package_count, 4);
}

#[when(regex = r"^I select sort by '(.*?)'$")]
async fn select_sort_option(world: &mut TestWorld, option: String) {
    let select_element = world
        .driver
        .find_element(By::ClassName("select2-container"))
        .await
        .unwrap();
    select_element.click().await.unwrap();

    let dropdown_element = world
        .driver
        .find_element(By::ClassName("select2-dropdown"))
        .await
        .unwrap();
    let option_elements = dropdown_element
        .find_elements(By::ClassName("select2-results__option"))
        .await
        .unwrap();
    for option_element in option_elements.into_iter() {
        let option_text = option_element.text().await.unwrap();
        if option_text == option {
            option_element.click().await.unwrap();
            return;
        }
    }
    panic!("Sort field not available!");
}

#[then(expr = "I should see the results sorted by {word}")]
async fn see_sorted_items(world: &mut TestWorld, field: String) {
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
