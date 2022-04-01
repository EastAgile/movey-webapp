use cucumber::{given, when, then};
use thirtyfour::prelude::*;

use super::super::world::TestWorld;

#[given("I am a guest / unregistered user")]
async fn a_guest_user(world: &mut TestWorld) {
}

#[when("I access the Movey website")]
async fn visit_home_page(world: &mut TestWorld) {
    world.go_to_root_url().await;
}

#[then("I should see the Movey home page")]
async fn see_home_page(world: &mut TestWorld) {
  assert_eq!(world.driver.current_url().await.unwrap(), "http://localhost:17002/");

  let subtitle = world.driver.find_element(By::ClassName("subtitle")).await.unwrap();
  let subtitle_text = subtitle.text().await.unwrap();
  assert_eq!(subtitle_text, "Reproducible builds and deployments.");

  world.driver.find_element(By::ClassName("search-container")).await.unwrap();
  world.driver.find_element(By::ClassName("stat-container")).await.unwrap();
}
