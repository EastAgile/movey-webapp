use cucumber::{given, when, then};
use thirtyfour::prelude::*;

use super::super::world::TestWorld;

#[given("I visit the root page")]
pub async fn visit_root_page(world: &mut TestWorld) {
    world.go_to_root_url().await;
    world.driver.get("http://localhost:17002/home").await.unwrap();
}

#[when("I click on the Register link")]
async fn click_on_register_link(world: &mut TestWorld) {
    let register_link = world.driver.find_element(By::LinkText("Register")).await.unwrap();
    register_link.click().await.unwrap();
}

#[when("I fill in the registration form")]
async fn fill_in_registration_form(world: &mut TestWorld) {
    let name_field = world.driver.find_element(By::Name("name")).await.unwrap();
    name_field.send_keys("test name").await.unwrap();

    let email_field = world.driver.find_element(By::Name("email")).await.unwrap();
    email_field.send_keys("test@email.com").await.unwrap();

    let password_field = world.driver.find_element(By::Name("password")).await.unwrap();
    password_field.send_keys("x,W-4,jfn").await.unwrap();
}

#[when("I click on Create account button")]
async fn click_on_registration_create_account_button(world: &mut TestWorld) {
    let create_account_button = world.driver.find_element(By::ClassName("create_account_btn")).await.unwrap();
    create_account_button.click().await.unwrap();
}

#[then("I should see my account created")]
async fn see_my_account_created(world: &mut TestWorld) {
    let heading = world.driver.find_element(By::Tag("h1")).await.unwrap();
    let heading_text = heading.text().await.unwrap();
    assert_eq!(heading_text, "Verify Account");
}
