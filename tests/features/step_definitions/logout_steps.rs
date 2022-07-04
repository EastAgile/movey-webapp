use cucumber::when;
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
