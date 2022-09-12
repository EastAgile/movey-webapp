use super::super::world::TestWorld;
use cucumber::{given, then, when};
use std::thread;
use std::time::Duration;
use thirtyfour::common::keys::Keys;
use thirtyfour::prelude::*;

async fn type_into_search_bar(world: &mut TestWorld) {
    let search_bar = world
        .driver
        .find_element(By::Id("search-bar"))
        .await
        .unwrap();
    search_bar.send_keys("ran").await.unwrap();
}

#[given("I am a guest / unregistered user")]
async fn a_guest_user(_world: &mut TestWorld) {}

#[when("I access the Movey website")]
async fn visit_home_page(world: &mut TestWorld) {
    world.go_to_root_url().await;
}

#[then("I should see the Movey home page")]
async fn see_home_page(world: &mut TestWorld) {
    assert_eq!(
        world.driver.current_url().await.unwrap(),
        "http://localhost:17002/"
    );

    let subtitle = world
        .driver
        .find_element(By::ClassName("subtitle"))
        .await
        .unwrap();
    let subtitle_text = subtitle.text().await.unwrap();
    assert_eq!(subtitle_text, "Reproducible builds and deployments.");

    world
        .driver
        .find_element(By::ClassName("search-container"))
        .await
        .unwrap();
    world
        .driver
        .find_element(By::ClassName("stat-container"))
        .await
        .unwrap();
}

#[when("I search for package on the search bar")]
async fn search_for_packages(world: &mut TestWorld) {
    type_into_search_bar(world).await;
    thread::sleep(Duration::from_millis(2000));
}

#[then("I should see the dropdown show matching packages")]
async fn dropdown_confirm(world: &mut TestWorld) {
    let suggestion = world
        .driver
        .find_element(By::Id("suggestions"))
        .await
        .unwrap();
    let count = suggestion.find_elements(By::Tag("div")).await.unwrap();
    assert_ne!(1, count.len());
    assert_ne!(0, count.len());
}
#[when("I click on an item in the dropdown")]
async fn click_on_dropdown(world: &mut TestWorld) {
    let suggestion = world
        .driver
        .find_element(By::Id("suggestions"))
        .await
        .unwrap();
    let suggest = suggestion
        .find_element(By::Css("#suggestion1 .package-name"))
        .await
        .unwrap();

    assert_eq!(suggest.text().await.unwrap(), "rand");
    suggest.click().await.unwrap()
}

#[then("I should be redirected to that package detail page")]
async fn confirm_redirect(world: &mut TestWorld) {
    let current_url = world.driver.current_url().await.unwrap();
    assert_eq!(current_url, "http://localhost:17002/packages/rand");
}

#[when("I press enter on the search bar or click on search icon")]
async fn to_search_result_page(world: &mut TestWorld) {
    visit_home_page(world).await;
    assert_eq!(
        world.driver.current_url().await.unwrap(),
        "http://localhost:17002/"
    );
    type_into_search_bar(world).await;
    thread::sleep(Duration::from_millis(2000));
    let search_bar = world
        .driver
        .find_element(By::Id("search-bar"))
        .await
        .unwrap();
    search_bar.send_keys(Keys::Enter).await.unwrap();
    let current_url = world.driver.current_url().await.unwrap();
    assert_eq!(
        current_url,
        "http://localhost:17002/packages/search?query=ran"
    );
}
#[then("I should be redirected to the search results page")]
async fn show_search_results_page(_world: &mut TestWorld) {}
