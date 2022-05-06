use cucumber::{when, then};
use thirtyfour::prelude::*;

use super::super::world::TestWorld;

#[then("I should see the dark version of the header")]
async fn see_dark_header(world: &mut TestWorld) {
    let _header_element = world.driver.find_element(By::ClassName("dark")).await.unwrap();
}

#[when("I click on the search icon on the dark header")]
async fn click_on_header_search_icon(world: &mut TestWorld) {
    let header_element = world.driver.find_element(By::ClassName("dark")).await.unwrap();
    let header_search_element = header_element.find_element(By::ClassName("search_icon")).await.unwrap();
    header_search_element.click().await.unwrap();
}

#[then("I should see the header search overlay")]
async fn see_header_search_overlay(world: &mut TestWorld) {
    let search_overlay_element = world.driver.find_element(By::ClassName("header-search-overlay")).await.unwrap();
    let displayed = search_overlay_element.is_displayed().await.unwrap();
    assert_eq!(displayed, true);
}
