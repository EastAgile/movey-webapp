use cucumber::{then, when};
use thirtyfour::prelude::*;

use super::super::world::TestWorld;
#[when("I click on the Sign up button on the home page")]
pub async fn click_on_sign_up_button(world: &mut TestWorld) {
    let signup_button = world
        .driver
        .find_element(By::ClassName("sign-up"))
        .await
        .unwrap();
    signup_button.click().await.unwrap();
}

#[when("I fill in my email and password and submit the form on the sign up page")]
pub async fn fill_in_sign_up_form(world: &mut TestWorld) {
    std::fs::remove_dir_all("./emails").unwrap_or_default();

    let email_field = world.driver.find_element(By::Name("email")).await.unwrap();
    email_field.send_keys("email@host.com").await.unwrap();

    let password_field = world
        .driver
        .find_element(By::Name("password"))
        .await
        .unwrap();
    password_field.send_keys("So$trongpas0word!").await.unwrap();

    let i_agree = world
        .driver
        .find_element(By::Name("i_agree"))
        .await
        .unwrap();
    i_agree.click().await.unwrap();

    let create_account_button = world
        .driver
        .find_element(By::ClassName("create_account_btn"))
        .await
        .unwrap();
    create_account_button.click().await.unwrap();
}

#[when(
    expr = "I fill in a valid email with value of '{word}' and an invalid password with value of {word}"
)]
async fn fill_in_invalid_password(world: &mut TestWorld, email: String, invalid_password: String) {
    let email_field = world.driver.find_element(By::Name("email")).await.unwrap();
    email_field.send_keys(email).await.unwrap();

    let password_field = world
        .driver
        .find_element(By::Name("password"))
        .await
        .unwrap();
    password_field.send_keys(invalid_password).await.unwrap();

    let i_agree = world
        .driver
        .find_element(By::Name("i_agree"))
        .await
        .unwrap();
    i_agree.click().await.unwrap();

    let create_account_button = world
        .driver
        .find_element(By::ClassName("create_account_btn"))
        .await
        .unwrap();
    create_account_button.click().await.unwrap();
}

#[when("I click on the verification link")]
async fn click_verification_link(world: &mut TestWorld) {
    let path = std::fs::read_dir("./emails").unwrap().next();
    let contents = std::fs::read_to_string(path.unwrap().unwrap().path()).unwrap();
    let contents = contents.split('\n').collect::<Vec<&str>>();
    for line in contents {
        if line.contains("/accounts/verify/") {
            let line_in_test = line
                .replace("17001", "17002")
                .replace("127.0.0.1", "localhost");
            world.driver.get(line_in_test).await.unwrap();
        }
    }
}

#[then("I should see the sign up page")]
async fn see_sign_up_page(world: &mut TestWorld) {
    assert_eq!(
        world.driver.current_url().await.unwrap(),
        "http://localhost:17002/accounts/register/"
    );
}

#[then("I should see the Verify Your Account page")]
async fn see_my_account_created(world: &mut TestWorld) {
    let heading = world.driver.find_element(By::Tag("h1")).await.unwrap();
    let heading_text = heading.text().await.unwrap();
    assert_eq!(heading_text, "Verify Account");
}

#[then(regex = r"^I should see the error '(.+)'$")]
async fn see_error_message(world: &mut TestWorld, message: String) {
    let errors_element = world
        .driver
        .find_element(By::ClassName("error"))
        .await
        .unwrap();
    let errors_message = errors_element.text().await.unwrap();
    assert!(errors_message.contains(&message));
}

#[then("I should receive a verification email")]
async fn receive_verification_email(_world: &mut TestWorld) {
    let path = std::fs::read_dir("./emails").unwrap().next();
    let contents = std::fs::read_to_string(path.unwrap().unwrap().path()).unwrap();
    assert!(contents.contains("email@host.com"));
    assert!(contents.contains("verify your account"));
}

#[then(
    "I should receive an email warning that someone is trying to create an account with my email"
)]
async fn receive_warning_email(_world: &mut TestWorld) {
    let path = std::fs::read_dir("./emails").unwrap().next();
    let contents = std::fs::read_to_string(path.unwrap().unwrap().path()).unwrap();
    assert!(contents.contains("email@host.com"));
    assert!(contents.contains("Someone just attempted to register for an account"));
}
