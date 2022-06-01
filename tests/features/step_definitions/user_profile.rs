use crate::TestWorld;
use thirtyfour::prelude::*;
use cucumber::{given, when, then};

#[given("I visit the Profile page")]
pub async fn visit_profile_page(world: &mut TestWorld) {
    world.go_to_root_url().await;
    world.driver.get("http://localhost:17002/settings/profile").await.unwrap();
}

#[when("I enter my current password into 'Current password' textbox")]
async fn enter_password(world: &mut TestWorld) {
    let password_field = world.driver.find_element(By::ClassName("current-password")).await.unwrap();
    password_field.send_keys(world.account.password.clone()).await.unwrap();

    let repeat_password_field = world.driver.find_element(By::ClassName("password-confirm")).await.unwrap();
    repeat_password_field.send_keys(world.account.password.clone()).await.unwrap();
}

#[when("I enter new valid password into 'New password' textbox")]
async fn enter_new_password(world: &mut TestWorld) {
    let new_password_field = world.driver.find_element(By::ClassName("new-password")).await.unwrap();
    world.account.password = String::from("BFfMgH79?}#;*Q'");
    new_password_field.send_keys(world.account.password.clone()).await.unwrap();
}

#[when("I repeat the same new valid password into 'Repeat new password' textbox")]
async fn repeat_new_password(world: &mut TestWorld) {
    let repeat_password_field = world.driver.find_element(By::ClassName("password-confirm")).await.unwrap();
    repeat_password_field.send_keys(world.account.password.clone()).await.unwrap();
}

#[when("I click on 'Save' button")]
async fn click_on_save_button(world: &mut TestWorld) {
    let save_button = world.driver.find_element(By::ClassName("password-confirm")).await.unwrap();
    save_button.click().await.unwrap();
}

#[then("I should see the Profile page")]
async fn see_account_settings_page(world: &mut TestWorld) {
    assert_eq!(
        world.driver.current_url().await.unwrap(),
        "http://localhost:17002/settings/profile"
    );

    world.driver.find_element(By::Css("input.email.disabled")).await.unwrap();
    // assert!(!email.is_enabled().await.unwrap());
    world.driver.find_element(By::ClassName("current-password")).await.unwrap();
    world.driver.find_element(By::ClassName("new-password")).await.unwrap();
    world.driver.find_element(By::ClassName("password-confirm")).await.unwrap();
    world.driver.find_element(By::ClassName("save")).await.unwrap();
    world.driver.find_element(By::ClassName("discard")).await.unwrap();
}

#[then(expr = "I should see a popup with text {word}")]
async fn see_popup_with_text(world: &mut TestWorld, word: String) {
    let popup = world.driver.find_element(By::ClassName(".popup")).await.unwrap();
    assert_eq!(popup.text().await.unwrap(), word);
}

#[then("I am signed out of my account and redirected to Sign in page")]
async fn signed_out_and_redirected_to_sign_in_page(world: &mut TestWorld) {
    match world.driver.get_cookie("sessionid").await {
        Ok(_) => panic!(),
        Err(_) => ()
    };
    assert_eq!(
        world.driver.current_url().await.unwrap(),
        "http://localhost:17002/accounts/login/"
    );
}

#[then("I should be able to sign in again with new password")]
async fn sign_in_with_new_password(world: &mut TestWorld) {
    let email_field = world.driver.find_element(By::ClassName("email")).await.unwrap();
    email_field.send_keys(world.account.email.clone()).await.unwrap();

    let password_field = world.driver
        .find_element(By::Name("password"))
        .await.unwrap();
    password_field.send_keys("So$trongpas0word!").await.unwrap();
    let create_account_button = world.driver
        .find_element(By::ClassName("login-btn"))
        .await.unwrap();
    create_account_button.click().await.unwrap();
}
