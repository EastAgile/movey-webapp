use cucumber::{given, then, when};
use std::fs;
use thirtyfour::prelude::*;

use super::super::world::TestWorld;

#[given("The server has an invalid captcha secret key")]
async fn has_invalid_captcha_secret_key(_world: &mut TestWorld) {
    std::env::set_var("CAPTCHA_SECRET_KEY", "JUST_A_RANDOM_AND_INVALID_SECRET_KEY");
}

#[when("I click on the contact link on the footer")]
async fn click_on_contact_link(world: &mut TestWorld) {
    // Keys for testing with Google reCaptcha, should allow testing go smoothly
    // Refs: https://developers.google.com/recaptcha/docs/faq
    std::env::set_var("CAPTCHA_SECRET_KEY", "6LeIxAcTAAAAAGG-vFI1TnRWxMZNFuojJ4WifJWe");
    std::env::set_var("JELLY_CAPTCHA_SITE_KEY", "6LeIxAcTAAAAAJcZVRqyHh71UMIEGNQ_MXjiZKhI");

    let contact_link = world
        .driver
        .find_element(By::ClassName("contact"))
        .await
        .unwrap();
    contact_link.click().await.unwrap();
}

#[then("I should see the contact form page")]
async fn see_contact_form_page(world: &mut TestWorld) {
    let handles = world.driver.window_handles().await.unwrap();
    world.driver.switch_to().window(&handles[1]).await.unwrap();

    assert_eq!(
        world.driver.current_url().await.unwrap(),
        format!("{}contact", world.root_url).as_str()
    );
    let page_name = world
        .driver
        .find_element(By::ClassName("page_name"))
        .await
        .unwrap();
    assert_eq!(page_name.text().await.unwrap(), "Contact Us");
    let form_cta = world
        .driver
        .find_element(By::ClassName("form-cta"))
        .await
        .unwrap();
    assert_eq!(form_cta.text().await.unwrap(), "Submit a request");

    world
        .driver
        .find_element(By::ClassName("packages-sort"))
        .await
        .unwrap();
    world.driver.find_element(By::Id("name")).await.unwrap();
    world.driver.find_element(By::Id("email")).await.unwrap();
    world.driver.find_element(By::Id("descr")).await.unwrap();
}

#[when("I fill in form information and submit the form on contact page")]
async fn fill_in_contact_form(world: &mut TestWorld) {
    std::fs::remove_dir_all("./emails").unwrap_or_default();

    let droplist = world
        .driver
        .find_element(By::ClassName("packages-sort"))
        .await
        .unwrap();

    let options = droplist.find_elements(By::Name("category")).await.unwrap();
    options[0].click().await.unwrap();

    let user_name = world.driver.find_element(By::Name("name")).await.unwrap();
    user_name.send_keys("John Doe").await.unwrap();

    let email_field = world.driver.find_element(By::Name("email")).await.unwrap();
    email_field.send_keys("email@host.com").await.unwrap();

    let description = world.driver.find_element(By::Id("descr")).await.unwrap();
    description.click().await.unwrap();
    description
        .send_keys("Hello this is a test.")
        .await
        .unwrap();

    let captcha_btn = world
        .driver
        .find_element(By::ClassName("g-recaptcha"))
        .await
        .unwrap();
    captcha_btn.click().await.unwrap();

    // Wait for reCaptcha verification
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    let submit_btn = world
        .driver
        .find_element(By::ClassName("contact-btn"))
        .await
        .unwrap();
    submit_btn.click().await.unwrap();
}

#[then("I should receive a thank you email")]
async fn receive_thankyou_email(_world: &mut TestWorld) {
    let emails_dir = fs::read_dir("./emails").unwrap();
    assert_eq!(emails_dir.count(), 2);

    for email in fs::read_dir("./emails").unwrap() {
        let content = fs::read_to_string(email.unwrap().path()).unwrap();
        if content.contains("Subject: New Contact Request") {
            assert!(content.contains("To: movey@eastagile.com"));
            assert!(content.contains("Hello Admin,"));
            assert!(content.contains("A message was sent from John Doe"));
            assert!(content.contains("Contact Email: email@host.com"));
            assert!(content.contains("Contact for: Account"));
            assert!(content.contains("Description: Hello this is a test."));
        } else if content.contains("Subject: Thank you for contacting us") {
            assert!(content.contains("To: email@host.com"));
            assert!(content.contains("Hello there,"));
            assert!(content.contains(
                "We=E2=80=99ve received your request and will get back to you shortly."
            ));
        } else {
            panic!()
        }
    }
}

#[then(regex = r"^I should see an error '(.+)' and a message to try again$")]
async fn receive_contact_error(world: &mut TestWorld, message: String) {
    let captcha_error = world
        .driver
        .find_element(By::ClassName("captcha-error"))
        .await;
    assert!(captcha_error.is_ok());

    let captcha_error_message = captcha_error
        .unwrap()
        .find_element(By::Tag("p"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(captcha_error_message, format!("Captcha verification error: {}. Please try again.", message))
}

#[then("I should not receive a thank you email")]
async fn not_receive_thankyou_email(_world: &mut TestWorld) {
    let emails_dir = fs::read_dir("./emails");
    assert!(emails_dir.is_err());
}
