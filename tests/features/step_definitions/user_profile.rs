use crate::TestWorld;
use cucumber::{given, then, when};
use thirtyfour::prelude::*;

#[given("I visit the Profile page")]
pub async fn visit_profile_page(world: &mut TestWorld) {
    world.go_to_root_url().await;
    world
        .driver
        .get("http://localhost:17002/settings/profile")
        .await
        .unwrap();
}

#[when("I enter my current password into 'Current password' textbox")]
async fn enter_password(world: &mut TestWorld) {
    let password_field = world
        .driver
        .find_element(By::Id("current-password"))
        .await
        .unwrap();
    password_field
        .send_keys(world.account.password.clone())
        .await
        .unwrap();
}

#[when("I enter new valid password into 'New password' textbox")]
async fn enter_new_password(world: &mut TestWorld) {
    let new_password_field = world
        .driver
        .find_element(By::Id("new-password"))
        .await
        .unwrap();
    world.account.password = String::from("BFfMgH79?}#;*Q'");
    new_password_field
        .send_keys(world.account.password.clone())
        .await
        .unwrap();
}

#[when("I repeat the same new valid password into 'Repeat new password' textbox")]
async fn repeat_new_password(world: &mut TestWorld) {
    let repeat_password_field = world
        .driver
        .find_element(By::Id("password-confirm"))
        .await
        .unwrap();
    repeat_password_field
        .send_keys(world.account.password.clone())
        .await
        .unwrap();
}

#[when("I enter an short password into 'New password' textbox")]
async fn enter_short_password(world: &mut TestWorld) {
    let new_password_field = world
        .driver
        .find_element(By::Id("new-password"))
        .await
        .unwrap();
    world.account.password = String::from("BFfM");
    new_password_field
        .send_keys(world.account.password.clone())
        .await
        .unwrap();
}

#[when("I repeat the same short password into 'Repeat new password' textbox")]
async fn short_password_confirmation(world: &mut TestWorld) {
    let repeat_password_field = world
        .driver
        .find_element(By::Id("password-confirm"))
        .await
        .unwrap();
    repeat_password_field
        .send_keys(world.account.password.clone())
        .await
        .unwrap();
}

#[when("I enter different password into 'Repeat new password' textbox")]
async fn different_password(world: &mut TestWorld) {
    let repeat_password_field = world
        .driver
        .find_element(By::Id("password-confirm"))
        .await
        .unwrap();
    repeat_password_field
        .send_keys(format!("{}a", world.account.password.clone()))
        .await
        .unwrap();
}

#[when("I click on 'Save' button")]
async fn click_on_save_button(world: &mut TestWorld) {
    let save_button = world.driver.find_element(By::Id("save-btn")).await.unwrap();
    save_button.click().await.unwrap();
}

#[when("I enter random texts into whatever textboxes")]
async fn enter_text_whatever_textbox(world: &mut TestWorld) {
    let text = String::from("randomhmm");
    let password_field = world
        .driver
        .find_element(By::Id("current-password"))
        .await
        .unwrap();
    password_field.send_keys(text.clone()).await.unwrap();

    let new_password_field = world
        .driver
        .find_element(By::Id("new-password"))
        .await
        .unwrap();
    new_password_field.send_keys(text.clone()).await.unwrap();

    let repeat_password_field = world
        .driver
        .find_element(By::Id("password-confirm"))
        .await
        .unwrap();
    repeat_password_field.send_keys(text.clone()).await.unwrap();
}

#[when("I click on 'Discard' button")]
async fn click_on_discard_button(world: &mut TestWorld) {
    let discard_button = world
        .driver
        .find_element(By::Id("discard-btn"))
        .await
        .unwrap();
    discard_button.click().await.unwrap();
}

#[then("I should see all textboxs return to blank")]
async fn all_textbox_blank(world: &mut TestWorld) {
    let password_field = world
        .driver
        .find_element(By::Id("current-password"))
        .await
        .unwrap();
    assert!(password_field.text().await.unwrap().is_empty());

    let new_password_field = world
        .driver
        .find_element(By::Id("new-password"))
        .await
        .unwrap();
    assert!(new_password_field.text().await.unwrap().is_empty());

    let repeat_password_field = world
        .driver
        .find_element(By::Id("password-confirm"))
        .await
        .unwrap();
    assert!(repeat_password_field.text().await.unwrap().is_empty());
}

#[then("I should see the Profile page")]
async fn see_account_settings_page(world: &mut TestWorld) {
    assert_eq!(
        world.driver.current_url().await.unwrap(),
        "http://localhost:17002/settings/profile"
    );

    world
        .driver
        .find_element(By::Css("input.email.disabled"))
        .await
        .unwrap();
    world
        .driver
        .find_element(By::Id("current-password"))
        .await
        .unwrap();
    world
        .driver
        .find_element(By::Id("new-password"))
        .await
        .unwrap();
    world
        .driver
        .find_element(By::Id("password-confirm"))
        .await
        .unwrap();
    world.driver.find_element(By::Id("save-btn")).await.unwrap();
    world
        .driver
        .find_element(By::Id("discard-btn"))
        .await
        .unwrap();
}

#[then(regex = r"^I should see a message with text '(.+)'$")]
async fn see_popup_with_text(world: &mut TestWorld, flash_message: String) {
    let message = world
        .driver
        .find_element(By::ClassName("flash_message"))
        .await
        .unwrap();
    assert_eq!(message.text().await.unwrap(), flash_message);
}

#[then("I am signed out of my account and redirected to Sign in page")]
async fn signed_out_and_redirected_to_sign_in_page(world: &mut TestWorld) {
    assert_eq!(
        world.driver.current_url().await.unwrap(),
        "http://localhost:17002/accounts/login/"
    );
}

#[then("I should see the 'Save' button is disabled")]
async fn save_button_disabled(world: &mut TestWorld) {
    assert!(!world
        .driver
        .find_element(By::Id("save-btn"))
        .await
        .unwrap()
        .is_enabled()
        .await
        .unwrap());
}
#[then("I should be able to sign in again with new password")]
async fn sign_in_with_new_password(world: &mut TestWorld) {
    let email_field = world.driver.find_element(By::Name("email")).await.unwrap();
    email_field
        .send_keys(world.account.email.clone())
        .await
        .unwrap();

    let password_field = world
        .driver
        .find_element(By::Name("password"))
        .await
        .unwrap();
    password_field
        .send_keys(world.account.password.clone())
        .await
        .unwrap();
    let create_account_button = world
        .driver
        .find_element(By::ClassName("login-btn"))
        .await
        .unwrap();
    create_account_button.click().await.unwrap();
    assert_eq!(
        world.driver.current_url().await.unwrap(),
        "http://localhost:17002/settings/profile"
    );
}
