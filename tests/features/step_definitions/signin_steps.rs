use cucumber::{given, then, when};
use jelly::forms::{EmailField, PasswordField};
use mainlib::accounts::forms::NewAccountForm;
use mainlib::accounts::Account;
use mainlib::test::DB_POOL;
use thirtyfour::prelude::*;

use super::super::world::TestWorld;
use super::signup_steps::*;

#[given("I am an user on Movey")]
async fn an_user(_world: &mut TestWorld) {
    let form = NewAccountForm {
        email: EmailField {
            value: "email@host.com".to_string(),
            errors: vec![],
        },
        password: PasswordField {
            value: "So$trongpas0word!".to_string(),
            errors: vec![],
            hints: vec![],
        },
    };
    let uid = Account::register(&form, &DB_POOL).await.unwrap();
    Account::mark_verified(uid, &DB_POOL).await.unwrap();
}

#[given("I am not signed in")]
async fn non_signed_in_user(world: &mut TestWorld) {
    world.driver.delete_cookie("sessionid").await.unwrap_or_default();
}

#[given("I am signed in")]
async fn signed_in_user(world: &mut TestWorld) {
    visit_sign_in_page(world).await;
    fill_in_sign_in_form(world).await;
}

#[given("I am signed in with remember me option")]
async fn signed_in_with_remember_me(world: &mut TestWorld) {
    visit_sign_in_page(world).await;

    let email_field = world.driver.find_element(By::Name("email")).await.unwrap();
    email_field.send_keys("email@host.com").await.unwrap();
    
    let password_field = world.driver
        .find_element(By::Name("password"))
        .await.unwrap();
    password_field.send_keys("So$trongpas0word!").await.unwrap();
    let remember_me = world.driver
        .find_element(By::Name("remember_me"))
        .await.unwrap();
    remember_me.click().await.unwrap();
    let create_account_button = world.driver
        .find_element(By::ClassName("login-btn"))
        .await.unwrap();
    create_account_button.click().await.unwrap();
}
#[given("I registered an account and have not activated it")]
async fn register_an_account(world: &mut TestWorld) {
    world.go_to_root_url().await;
    click_on_sign_up_button(world).await;
    fill_in_sign_up_form(world).await;
}
#[when("I click on the Sign in button on the home page")]
async fn click_on_sign_in_button(world: &mut TestWorld) {
    let sign_in_button = world.driver
        .find_element(By::ClassName("sign-in"))
        .await.unwrap();
    sign_in_button.click().await.unwrap();
}

#[when("I fill in my email and password and submit the form on the sign in page")]
async fn fill_in_sign_in_form(world: &mut TestWorld) {
    let email_field = world.driver.find_element(By::Name("email")).await.unwrap();
    email_field.send_keys("email@host.com").await.unwrap();

    let password_field = world.driver
        .find_element(By::Name("password"))
        .await.unwrap();
    password_field.send_keys("So$trongpas0word!").await.unwrap();
    let create_account_button = world.driver
        .find_element(By::ClassName("login-btn"))
        .await.unwrap();
    create_account_button.click().await.unwrap();
}

#[when("I fill in wrong email and submit the form on the sign in page")]
async fn fill_in_wrong_email(world: &mut TestWorld) {
    let email_field = world.driver.find_element(By::Name("email")).await.unwrap();
    email_field.send_keys("wrong@host.com").await.unwrap();

    let password_field = world.driver
        .find_element(By::Name("password"))
        .await.unwrap();
    password_field.send_keys("So$trongpas0word!").await.unwrap();
    let create_account_button = world.driver
        .find_element(By::ClassName("login-btn"))
        .await.unwrap();
    create_account_button.click().await.unwrap();
}

#[when("I fill in blank email and submit the form on the sign in page")]
async fn fill_in_blank_email(world: &mut TestWorld) {
    let email_field = world.driver.find_element(By::Name("email")).await.unwrap();
    email_field.send_keys("").await.unwrap();

    let password_field = world.driver
        .find_element(By::Name("password"))
        .await.unwrap();
    password_field.send_keys("So$trongpas0word!").await.unwrap();
    let create_account_button = world.driver
        .find_element(By::ClassName("login-btn"))
        .await.unwrap();
    create_account_button.click().await.unwrap();
}

#[when("I fill in wrong password and submit the form on the sign in page")]
async fn fill_in_wrong_password(world: &mut TestWorld) {
    let email_field = world.driver.find_element(By::Name("email")).await.unwrap();
    email_field.send_keys("email@host.com").await.unwrap();

    let password_field = world.driver
        .find_element(By::Name("password"))
        .await.unwrap();
    password_field.send_keys("wrongpassword").await.unwrap();
    let create_account_button = world.driver
        .find_element(By::ClassName("login-btn"))
        .await.unwrap();
    create_account_button.click().await.unwrap();
}

#[when("I fill in blank password and submit the form on the sign in page")]
async fn fill_in_blank_password(world: &mut TestWorld) {
    let email_field = world.driver.find_element(By::Name("email")).await.unwrap();
    email_field.send_keys("email@host.com").await.unwrap();

    let password_field = world.driver
        .find_element(By::Name("password"))
        .await.unwrap();
    password_field.send_keys("").await.unwrap();
    let create_account_button = world.driver
        .find_element(By::ClassName("login-btn"))
        .await.unwrap();
    create_account_button.click().await.unwrap();
}

#[when("I access the Sign in page")]
async fn visit_sign_in_page(world: &mut TestWorld) {
    world.driver
        .get("http://localhost:17002/accounts/login/")
        .await.unwrap()
}

#[when("I access the Dashboard page")]
async fn visit_dashboard_page(world: &mut TestWorld) {
    world.driver
        .get("http://localhost:17002/dashboard/")
        .await.unwrap()
}

#[when("I close all browser tabs and reopen my browser")]
async fn clear_default_session(world: &mut TestWorld) {
    world.driver
        .delete_cookie("sessionid")
        .await.unwrap()
}

#[when("my permanent session is expired")]
async fn clear_permanent_session(world: &mut TestWorld) {
    world.driver
        .delete_cookie("sessionid")
        .await.unwrap();
    world.driver
        .delete_cookie("remember_me_token")
        .await.unwrap()
}

#[when("I sign in into my account")]
async fn sign_in(world: &mut TestWorld) {
    world.go_to_root_url().await;
    click_on_sign_in_button(world).await;
    fill_in_sign_in_form(world).await;
}

#[then("I should see the sign in page")]
async fn see_sign_up_page(world: &mut TestWorld) {
    assert_eq!(
        world.driver.current_url().await.unwrap(),
        "http://localhost:17002/accounts/login/"
    );

    let heading = world.driver.find_element(By::Tag("h1")).await.unwrap();
    let heading_text = heading.text().await.unwrap();
    assert_eq!(heading_text, "Login");

    world.driver
        .find_element(By::ClassName("login-form"))
        .await.unwrap();
}

#[then("I should see that Im logged in")]
async fn signed_in(world: &mut TestWorld) {
    assert_eq!(
        world.driver.current_url().await.unwrap(),
        "http://localhost:17002/dashboard/"
    );

    let welcome = world.driver
        .find_element(By::XPath("/html/body/div/p"))
        .await.unwrap();
    let welcome_text = welcome.text().await.unwrap();
    assert!(welcome_text.contains("Welcome back"));

    world.driver
        .find_element(By::XPath("/html/body/form"))
        .await.unwrap();
}

#[then("I should be on the Dashboard page")]
async fn see_dashboard_page(world: &mut TestWorld) {
    assert_eq!(
        world.driver.current_url().await.unwrap(),
        "http://localhost:17002/dashboard/"
    );
}
