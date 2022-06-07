use cucumber::{given, then, when};
use std::fs;
use std::path::Path;
use jelly::forms::{EmailField, PasswordField};
use thirtyfour::prelude::*;
use mainlib::accounts::Account;
use mainlib::accounts::forms::NewAccountForm;
use mainlib::test::DB_POOL;

use super::super::world::TestWorld;

#[given("I am not logged in")]
async fn not_logged_in(world: &mut TestWorld) {
    world.go_to_root_url().await;
    world
        .driver
        .execute_script(
            "
            let xhr = new XMLHttpRequest();
            xhr.open('POST', 'logout/');
            xhr.send();
        ",
        )
        .await
        .unwrap();
    assert_eq!(world.driver.current_url().await.unwrap(), world.root_url);
}

#[when("I access the Sign In page")]
async fn visit_signin_page(world: &mut TestWorld) {
    world
        .driver
        .get(format!("{}accounts/login/", world.root_url).as_str())
        .await
        .unwrap();
}

#[when("I click on the Forgot Passwork link on sign in form")]
async fn click_forgot_password(world: &mut TestWorld) {
    let link = world
        .driver
        .find_element(By::LinkText("Forgot Password?"))
        .await
        .unwrap();
    link.click().await.unwrap();
}

#[then("I should see the Forgot Password page")]
async fn see_forgot_password_page(world: &mut TestWorld) {
    assert_eq!(
        world.driver.current_url().await.unwrap(),
        format!("{}accounts/reset/", world.root_url).as_str()
    );
}

#[given("I have registered an email")]
async fn register_email(_world: &mut TestWorld) {
    fs::remove_dir_all("./emails").unwrap_or_else(|_| ());
    let form = NewAccountForm {
        email: EmailField {
            value: "test@email.com".to_string(),
            errors: vec![],
        },
        password: PasswordField {
            value: "So$trongpas0word!".to_string(),
            errors: vec![],
            hints: vec![],
        },
    };
    Account::register(&form, &DB_POOL).await.unwrap();
}

#[when("I fill in a registered email and submit the form on Forgot Password page")]
async fn fill_in_registered_email(world: &mut TestWorld) {
    fs::remove_dir_all("./emails").unwrap_or_else(|_| ());

    let email_field = world.driver.find_element(By::Name("email")).await.unwrap();
    email_field.send_keys("test@email.com").await.unwrap();

    let submit_btn = world
        .driver
        .find_element(By::ClassName("submit-btn"))
        .await
        .unwrap();
    submit_btn.click().await.unwrap();
}

#[then("I should see the Confirm Request page")]
async fn see_confirm_request_page(world: &mut TestWorld) {
    assert_eq!(
        world.driver.current_url().await.unwrap(),
        format!("{}accounts/reset/", world.root_url).as_str()
    );
    assert_eq!(world.driver.title().await.unwrap(), "Confirm Request");

    let title = world.driver.find_element(By::Tag("h1")).await.unwrap();
    assert_eq!(title.text().await.unwrap(), "Thank You");
}

#[then("I should receive an email that contains valid password reset link")]
async fn receive_verification_email(_world: &mut TestWorld) {
    let email_dir = fs::read_dir("./emails").unwrap().next();
    let email = fs::read_to_string(email_dir.unwrap().unwrap().path()).unwrap();
    assert!(email.contains("test@email.com"));
    assert!(email.contains("Reset Your Password"));
}

#[when("I fill in an unregistered email and submit the form on Forgot Password page")]
async fn fill_in_unregistered_email(world: &mut TestWorld) {
    fs::remove_dir_all("./emails").unwrap_or_else(|_| ());

    let email_field = world.driver.find_element(By::Name("email")).await.unwrap();
    email_field
        .send_keys("unregistered@email.com")
        .await
        .unwrap();

    let submit_btn = world
        .driver
        .find_element(By::ClassName("submit-btn"))
        .await
        .unwrap();
    submit_btn.click().await.unwrap();
}

#[then("I should not receive an email")]
async fn not_receive_verification_email(_world: &mut TestWorld) {
    assert!(!Path::new("./emails").exists());
}
