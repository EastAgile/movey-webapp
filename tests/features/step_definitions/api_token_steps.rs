use cucumber::{given, then, when};
use jelly::forms::{EmailField, PasswordField};
use mainlib::accounts::forms::NewAccountForm;
use mainlib::accounts::Account;
use mainlib::test::DB_POOL;
use thirtyfour::prelude::*;

use super::super::world::TestWorld;
use super::signup_steps::*;

#[when("I access the API Tokens page")]
async fn access_api_token_page(world: &mut TestWorld) {
    world.driver.get("http://localhost:17002/settings/tokens").await.unwrap();
}

#[when("I click on the New Token button")]
async fn click_on_new_token(world: &mut TestWorld) {
    let new_token_button = world.driver.find_element(By::ClassName("new-token-button")).await.unwrap();
    new_token_button.click().await.unwrap();
}

#[when("I enter the new token name")]
async fn enter_new_token_name(world: &mut TestWorld) {
    let new_token_text_box = world.driver.find_element(By::ClassName("new-token")).await.unwrap();
    new_token_text_box.send_keys("token name").await.unwrap();
}

#[when("I click on the Create button")]
async fn click_create_button(world: &mut TestWorld) {
    let create_button = world.driver.find_element(By::ClassName("create-button")).await.unwrap();
    create_button.click().await.unwrap();
}
#[then("I should see the API Tokens page")]
async fn see_api_token_page(world: &mut TestWorld) {
    assert_eq!(
        world.driver.current_url().await.unwrap(),
        "http://localhost:17002/settings/tokens"
    );
    world.driver.find_element(By::ClassName("api-tokens")).await.unwrap();
    world.driver.find_element(By::ClassName("new-token-button")).await.unwrap();
}

#[then("I should see the New Token Name text box")]
async fn see_api_token_page1(world: &mut TestWorld) {
    world.driver.find_element(By::ClassName("new-token")).await.unwrap();
}