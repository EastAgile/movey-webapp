use crate::TestWorld;
use thirtyfour::prelude::*;
use cucumber::{given, when, then};

#[given("I visit the Profile page")]
pub async fn visit_profile_page(world: &mut TestWorld) {
    world.go_to_root_url().await;
    world.driver.get("http://localhost:17002/settings/profile").await.unwrap();
}
