use cucumber::{then, when};
use std::fs;
use thirtyfour::prelude::*;

use super::super::world::TestWorld;

#[when("I click on the contact link on the footer")]
async fn click_on_contact_link(world: &mut TestWorld) {
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
        "http://localhost:17002/contact"
    );
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
