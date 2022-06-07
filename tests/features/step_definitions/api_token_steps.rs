use cucumber::{given, then, when};
use mainlib::setting::models::token::ApiToken;
use mainlib::test::DB_POOL;
use std::thread;
use std::time::Duration;
use thirtyfour::prelude::*;
use mainlib::accounts::Account;

use super::super::world::TestWorld;

#[given("I have created the maximum number of allowed tokens")]
async fn maximum_tokens(_world: &mut TestWorld) {
    let max_token = std::env::var("MAX_TOKEN")
        .expect("MAX_TOKEN not set!")
        .parse::<i32>()
        .expect("MAX_TOKEN must be an integer");
    let account = Account::get(1, &DB_POOL).await.unwrap();
    for n in 0..max_token {
        ApiToken::insert(&account, &n.to_string(), &DB_POOL).await.unwrap();
    }
}
#[when("I access the API Tokens page")]
async fn access_api_token_page(world: &mut TestWorld) {
    world
        .driver
        .get("http://localhost:17002/settings/tokens")
        .await
        .unwrap();
}

#[when("I click on the New Token button")]
async fn click_on_new_token(world: &mut TestWorld) {
    let new_token_button = world
        .driver
        .find_element(By::ClassName("new-token-button"))
        .await
        .unwrap();
    new_token_button.click().await.unwrap();
}

#[when("I enter a new token name")]
async fn enter_new_token_name(world: &mut TestWorld) {
    let new_token_text_box = world
        .driver
        .find_element(By::ClassName("new-token"))
        .await
        .unwrap();
    new_token_text_box.send_keys("token name").await.unwrap();
}

#[when("I click on the Create button")]
async fn click_create_button(world: &mut TestWorld) {
    let create_button = world
        .driver
        .find_element(By::ClassName("create-button"))
        .await
        .unwrap();
    create_button.click().await.unwrap();
    thread::sleep(Duration::from_millis(1000));
}

#[when("I enter a token name that is already existed")]
async fn enter_duplicated_name(world: &mut TestWorld) {
    let account = Account::get(1, &DB_POOL).await.unwrap();
    ApiToken::insert(&account, "token name", &DB_POOL).await.unwrap();
    let new_token_text_box = world
        .driver
        .find_element(By::ClassName("new-token"))
        .await
        .unwrap();
    new_token_text_box.send_keys("token name").await.unwrap();
}

#[then("I should see the API Tokens page")]
async fn see_api_token_page(world: &mut TestWorld) {
    assert_eq!(
        world.driver.current_url().await.unwrap(),
        "http://localhost:17002/settings/tokens"
    );
    world
        .driver
        .find_element(By::ClassName("api-tokens"))
        .await
        .unwrap();
    world
        .driver
        .find_element(By::ClassName("new-token-button"))
        .await
        .unwrap();
}

#[then("I should see the New Token Name text box")]
async fn see_token_text_box(world: &mut TestWorld) {
    world
        .driver
        .find_element(By::ClassName("new-token"))
        .await
        .unwrap();
}

#[then("I should see the new token")]
async fn see_new_token(world: &mut TestWorld) {
    let new_token = world
        .driver
        .find_element(By::ClassName("created-token"))
        .await
        .unwrap();
    let new_token = new_token.text().await.unwrap();
    assert_eq!(new_token.len(), 32)
}
