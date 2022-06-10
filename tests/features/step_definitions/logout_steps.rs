use cucumber::{then, when};
use thirtyfour::prelude::*;

use super::super::world::TestWorld;

#[when("I click on the Log out button")]
async fn click_log_out(world: &mut TestWorld) {
    let account_dropdown = world.driver
        .find_element(By::Id("account-dropdown-toggle"))
        .await.unwrap();
    account_dropdown.click().await.unwrap();

    let logout_link = world.driver
        .find_element(By::ClassName("logout-form"))
        .await.unwrap();
        logout_link.click().await.unwrap();
}

#[then("I should see the home page")]
async fn see_home_page(world: &mut TestWorld) {
    assert_eq!(
        world.driver.current_url().await.unwrap(),
        "http://localhost:17002/"
    );
}
