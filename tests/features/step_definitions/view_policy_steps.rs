use crate::TestWorld;
use cucumber::{then, when};
use thirtyfour::By;

#[when("I access the Terms of use page")]
async fn visit_terms_of_use_page(world: &mut TestWorld) {
    world
        .driver
        .get(format!("{}/terms-of-use", world.root_url).as_str())
        .await
        .unwrap();
}

#[then("I should see the Terms of use page")]
async fn see_terms_of_use_page(world: &mut TestWorld) {
    assert_eq!(
        world.driver.current_url().await.unwrap(),
        format!("{}/terms-of-use", world.root_url).as_str()
    );
    let page_name = world
        .driver
        .find_element(By::ClassName("page_name"))
        .await
        .unwrap();
    assert_eq!(page_name.text().await.unwrap(), "Terms of Use");
}

#[when("I access the Policy page")]
async fn visit_policy_page(world: &mut TestWorld) {
    world
        .driver
        .get(format!("{}/policy", world.root_url).as_str())
        .await
        .unwrap();
}

#[then("I should see the Policy page")]
async fn see_policy_page(world: &mut TestWorld) {
    assert_eq!(
        world.driver.current_url().await.unwrap(),
        format!("{}/policy", world.root_url).as_str()
    );
    let page_name = world
        .driver
        .find_element(By::ClassName("page_name"))
        .await
        .unwrap();
    assert_eq!(page_name.text().await.unwrap(), "Privacy Policy");
}

#[when("I access the About us page")]
async fn visit_about_us_page(world: &mut TestWorld) {
    world
        .driver
        .get(format!("{}/about", world.root_url).as_str())
        .await
        .unwrap();
}

#[then("I should see the About us page")]
async fn see_about_us_page(world: &mut TestWorld) {
    assert_eq!(
        world.driver.current_url().await.unwrap(),
        format!("{}/about", world.root_url).as_str()
    );
    let page_name = world
        .driver
        .find_element(By::ClassName("page_name"))
        .await
        .unwrap();
    assert_eq!(page_name.text().await.unwrap(), "About Movey");
}

#[then("I should see my name and email filled in textbox")]
async fn name_email_filled_in(world: &mut TestWorld) {
    let name_field = world.driver.find_element(By::Id("name")).await.unwrap();
    assert_eq!(
        name_field.get_attribute("value").await.unwrap(),
        Some("email".to_string())
    );

    let email_field = world.driver.find_element(By::Id("email")).await.unwrap();
    assert_eq!(
        email_field.get_attribute("value").await.unwrap(),
        Some(world.first_account.email.clone())
    );
}

#[then("I should see name field and email field are disabled")]
async fn name_email_disabled(world: &mut TestWorld) {
    let name_field = world.driver.find_element(By::Id("name")).await.unwrap();
    assert_eq!(
        name_field.get_attribute("readOnly").await.unwrap(),
        Some("true".to_string())
    );
    let email_field = world.driver.find_element(By::Id("email")).await.unwrap();
    assert_eq!(
        email_field.get_attribute("readOnly").await.unwrap(),
        Some("true".to_string())
    );
}

#[when("I access the Contact us page")]
async fn visit_contact_us_page(world: &mut TestWorld) {
    // Default key provided by Google
    // Refs: https://developers.google.com/recaptcha/docs/faq
    std::env::set_var(
        "JELLY_CAPTCHA_SITE_KEY",
        "6LeIxAcTAAAAAJcZVRqyHh71UMIEGNQ_MXjiZKhI",
    );

    world
        .driver
        .get(format!("{}/contact", world.root_url).as_str())
        .await
        .unwrap();
}

#[then("I should see the Contact us page")]
async fn see_contact_us_page(world: &mut TestWorld) {
    assert_eq!(
        world.driver.current_url().await.unwrap(),
        format!("{}/contact", world.root_url).as_str()
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
