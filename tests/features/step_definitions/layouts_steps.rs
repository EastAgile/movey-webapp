use cucumber::{when, then};
use thirtyfour::prelude::*;

use super::super::world::TestWorld;

#[when("I click on the search icon on the dark header")]
async fn click_on_header_search_icon(world: &mut TestWorld) {
    let header_search_element = world.driver.find_element(By::ClassName("search-btn")).await.unwrap();
    header_search_element.click().await.unwrap();
}

#[then("I should see the header search overlay")]
async fn see_header_search_overlay(world: &mut TestWorld) {
    let search_overlay_element = world.driver.find_element(By::Id("search-bar")).await.unwrap();
    let displayed = search_overlay_element.is_displayed().await.unwrap();
    assert_eq!(displayed, true);
}
