use super::super::world::TestWorld;
use cucumber::{given, then, when};
use jelly::forms::{EmailField, PasswordField};
use mainlib::accounts::forms::NewAccountForm;
use mainlib::accounts::Account;
use mainlib::test::DB_POOL;
use regex::Regex;
use std::fs;
use std::{thread, time};
use thirtyfour::prelude::*;

#[given("I have successfully requested a password reset link")]
async fn request_reset_link(world: &mut TestWorld) {
    fs::remove_dir_all("./emails").unwrap_or(());
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
    Account::register(&form, &DB_POOL).unwrap();

    world
        .driver
        .execute_script(
            format!(
                "
        let xhr = new XMLHttpRequest();
        xhr.open('POST', '{}accounts/reset/');
        xhr.setRequestHeader('Content-type', 'application/x-www-form-urlencoded');
        xhr.send('email=test@email.com');
    ",
                world.root_url
            )
            .as_str(),
        )
        .await
        .unwrap();
}

#[given("I have received the email that contains password reset link")]
async fn receive_reset_link(world: &mut TestWorld) {
    thread::sleep(time::Duration::from_millis(5000));
    let email_dir = fs::read_dir("./emails").unwrap().next();
    let email = fs::read_to_string(email_dir.unwrap().unwrap().path()).unwrap();

    assert!(email.contains("test@email.com"));
    assert!(email.contains("Reset Your Password"));

    let re = Regex::new(r"/accounts/reset/([^ \n]+)".to_string().as_str()).unwrap();
    let caps = re.captures(email.as_str()).unwrap();
    let reset_token = caps.get(1).map(|m| m.as_str()).unwrap();

    world.reset_token = reset_token.to_string();
    fs::remove_dir_all("./emails").unwrap_or(());
}

#[when("I access the reset password link")]
async fn access_reset_link(world: &mut TestWorld) {
    world
        .driver
        .get(format!(
            "{}accounts/reset/{}",
            world.root_url, world.reset_token
        ))
        .await
        .unwrap();
}

#[then("I should see the Reset Password page")]
async fn see_reset_password_page(world: &mut TestWorld) {
    assert_eq!(
        world.driver.title().await.unwrap(),
        "Reset Password | Movey"
    );
}

#[when("I fill in a valid password and repeat the password correctly")]
async fn fill_in_valid_password(world: &mut TestWorld) {
    let password = world.driver.find_element(By::Id("password")).await.unwrap();
    let confirm_password = world
        .driver
        .find_element(By::Id("password-confirm"))
        .await
        .unwrap();

    password.send_keys("VeryStr0ngP@ssword").await.unwrap();
    confirm_password
        .send_keys("VeryStr0ngP@ssword")
        .await
        .unwrap();
}

#[when("I submit the form on Reset Password page")]
async fn submit_reset_form(world: &mut TestWorld) {
    let submit_btn = world
        .driver
        .find_element(By::ClassName("submit-btn"))
        .await
        .unwrap();
    submit_btn.click().await.unwrap();
}

#[then("I should see the Password Changed page")]
async fn see_password_changed(world: &mut TestWorld) {
    assert_eq!(
        world.driver.title().await.unwrap(),
        "Password Changed | Movey"
    );
}

#[then("I should receive an email that confirms password has changed")]
async fn receive_confirm_email(_world: &mut TestWorld) {
    thread::sleep(time::Duration::from_millis(5000));
    let email_dir = fs::read_dir("./emails").unwrap().next();
    let email = fs::read_to_string(email_dir.unwrap().unwrap().path()).unwrap();
    assert!(email.contains("test@email.com"));
    assert!(email.contains("Your Password Has Been Reset"));
}

#[when("I fill in a valid password and repeat the password incorrectly")]
async fn fill_in_mismatch_passwords(world: &mut TestWorld) {
    let password = world.driver.find_element(By::Id("password")).await.unwrap();
    let confirm_password = world
        .driver
        .find_element(By::Id("password-confirm"))
        .await
        .unwrap();

    password.send_keys("VeryStr0ngP@ssword").await.unwrap();
    confirm_password
        .send_keys("VeryStr0ngP@ssword111")
        .await
        .unwrap();
}

#[when(expr = "I fill in an invalid password {word} and repeat the password correctly")]
async fn fill_in_invalid_password(world: &mut TestWorld, invalid_password: String) {
    let password = world.driver.find_element(By::Id("password")).await.unwrap();
    let confirm_password = world
        .driver
        .find_element(By::Id("password-confirm"))
        .await
        .unwrap();

    password.send_keys(invalid_password.clone()).await.unwrap();
    confirm_password.send_keys(invalid_password).await.unwrap();
}

#[when("I access an invalid reset password link")]
async fn access_invalid_reset_link(world: &mut TestWorld) {
    let mut invalid_link = world.reset_token.clone();
    invalid_link.remove(0);
    world
        .driver
        .get(format!("{}accounts/reset/{}", world.root_url, invalid_link))
        .await
        .unwrap();
}

#[then("I should see the Invalid or Expired page")]
async fn see_invalid_or_expired(world: &mut TestWorld) {
    assert_eq!(
        world.driver.title().await.unwrap(),
        "Invalid or Expired | Movey"
    );
}
