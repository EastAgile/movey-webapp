use crate::TestWorld;
use mainlib::accounts::Account;
use mainlib::setting::models::token::ApiToken;
use mainlib::test::DB_POOL;
use std::thread;
use std::time::Duration;
use thirtyfour::prelude::*;
use cucumber::{given, when, then};

#[when("I click on the tokens tab")]
pub async fn click_on_profile_tokens_tab(world: &mut TestWorld) {
    let tokens_tab_element = world
        .driver
        .find_element(By::ClassName("tab-versions"))
        .await
        .unwrap();
    tokens_tab_element.click().await.unwrap();
}

#[then("I should see the profile tokens page")]
pub async fn see_profile_tokens_page(world: &mut TestWorld) {
    let tokens_title = world
        .driver
        .find_element(By::ClassName("token-list-title"))
        .await
        .unwrap();
    assert_eq!(tokens_title.text().await.unwrap(), "Access Tokens")
}

#[given("I have an existing api token")]
pub async fn have_existing_api_token(world: &mut TestWorld) {
    let account = Account::get(1, &DB_POOL).await.unwrap();
    ApiToken::insert(&account, "token 1", &DB_POOL).await.unwrap();
}

#[given("I visit the profile tokens page")]
pub async fn visit_profile_tokens_page(world: &mut TestWorld) {
    world.go_to_root_url().await;
    world.driver.get("http://localhost:17002/settings/tokens").await.unwrap();
}

#[then("I should see my existing api token")]
pub async fn see_existing_api_token(world: &mut TestWorld) {
    let token_item = &world
        .driver
        .find_elements(By::ClassName("token-item"))
        .await
        .unwrap()[0];
    let token_name_element = token_item
        .find_element(By::ClassName("token-name"))
        .await
        .unwrap();
    assert_eq!(token_name_element.text().await.unwrap(), "token 1");
}

#[when("I click on create a new token")]
pub async fn click_on_create_new_token(world: &mut TestWorld) {
    let create_token_element = world
        .driver
        .find_element(By::ClassName("create-new-token-btn"))
        .await
        .unwrap();
    create_token_element.click().await.unwrap();
}

#[when("I fill in the token name and submit")]
pub async fn fill_in_token_name_and_submit(world: &mut TestWorld) {
    let name_field = world
        .driver
        .find_element(By::ClassName("new-token-input"))
        .await
        .unwrap();
    name_field.send_keys("token 2").await.unwrap();

    let create_token_element = world
        .driver
        .find_element(By::ClassName("new-token-submit-btn"))
        .await
        .unwrap();
    create_token_element.click().await.unwrap();
    thread::sleep(Duration::from_millis(1000));
}

#[then("I should see a new token created")]
pub async fn see_new_api_token_created(world: &mut TestWorld) {
    let token_item = &world
        .driver
        .find_elements(By::ClassName("token-item"))
        .await
        .unwrap()[1];
    let token_name_element = token_item
        .find_element(By::ClassName("token-name"))
        .await
        .unwrap();
    assert_eq!(token_name_element.text().await.unwrap(), "token 2");
}

#[when("I revoke the existing token")]
pub async fn revoke_existing_token(world: &mut TestWorld) {
    let token_item = &world
        .driver
        .find_elements(By::ClassName("token-item"))
        .await
    .unwrap()[0];

    let token_revoke_button = token_item
        .find_element(By::ClassName("revoke-token-btn"))
        .await
        .unwrap();
    token_revoke_button.click().await.unwrap();

    let _ = &world.driver.switch_to().alert().accept().await.unwrap();
    thread::sleep(Duration::from_millis(1000));
}

#[then("I should not see my existing token anymore")]
pub async fn should_not_see_existing_api_token(world: &mut TestWorld) {
    let token_item = &world
        .driver
        .find_elements(By::ClassName("token-item"))
        .await
        .unwrap()[0];
    let token_name_element = token_item
        .find_element(By::ClassName("token-name"))
        .await
        .unwrap();
    assert_eq!(token_name_element.text().await.unwrap(), "token 2");
}

#[when("I revoke the new token")]
pub async fn revoke_new_token(world: &mut TestWorld) {
    let token_item = &world
        .driver
        .find_elements(By::ClassName("token-item"))
        .await
    .unwrap()[0];

    let token_revoke_button = token_item
        .find_element(By::ClassName("revoke-token-btn"))
        .await
        .unwrap();
    token_revoke_button.click().await.unwrap();

    let _ = &world.driver.switch_to().alert().accept().await.unwrap();
    thread::sleep(Duration::from_millis(1000));
}

#[then("I should not see my new token anymore")]
pub async fn should_not_see_new_api_token(world: &mut TestWorld) {
    let token_items = &world
        .driver
        .find_elements(By::Css(".tokens-list .token-item"))
        .await
        .unwrap();
    assert_eq!(token_items.len(), 0);
}

#[when("I fill in the existing token name and submit")]
pub async fn fill_in_existing_token_name_and_submit(world: &mut TestWorld) {
    let name_field = world
        .driver
        .find_element(By::ClassName("new-token-input"))
        .await
        .unwrap();
    name_field.send_keys("token 1").await.unwrap();

    let create_token_element = world
        .driver
        .find_element(By::ClassName("new-token-submit-btn"))
        .await
        .unwrap();
    create_token_element.click().await.unwrap();
    thread::sleep(Duration::from_millis(1000));
}

#[then("I should see the token error that name has already been taken")]
pub async fn should_see_token_error_name_taken(world: &mut TestWorld) {
    let error_element = &world
        .driver
        .find_element(By::Css(".tokens-error"))
        .await
        .unwrap();
    assert_eq!(error_element.text().await.unwrap(), "That name has already been taken.");
}

#[then("I should see the token error that maximum token is reached")]
pub async fn should_see_token_error_maximum_reached(world: &mut TestWorld) {
    let error_element = &world
        .driver
        .find_element(By::Css(".tokens-error"))
        .await
        .unwrap();
    assert_eq!(error_element.text().await.unwrap(), "Too many tokens created.");
}
