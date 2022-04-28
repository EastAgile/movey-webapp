use cucumber::{then, when};
use thirtyfour::prelude::*;

use super::super::world::TestWorld;

#[when("I click on the Log out button")]
async fn click_log_out(world: &mut TestWorld) {
    let log_out = world.driver
        .find_element(By::XPath("/html/body/form/button"))
        .await.unwrap();
    log_out.click().await.unwrap();
}

#[then("I should see the home page")]
async fn see_home_page(world: &mut TestWorld) {
    assert_eq!(
        world.driver.current_url().await.unwrap(),
        "http://localhost:17002/"
    );
}
