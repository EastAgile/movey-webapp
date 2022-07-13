use std::{fs, thread, time};
use crate::TestWorld;
use cucumber::{when, then};
use thirtyfour::By;
use thirtyfour::components::select::SelectElement;

#[when("I select a request category")]
async fn select_request_category(world: &mut TestWorld) {
    let select_element = world.driver
        .find_element(By::ClassName("packages-sort"))
        .await.unwrap();
    let select = SelectElement::new(&select_element).await.unwrap();
    let _ = select.select_by_index(1).await;
}

#[when("I enter my name")]
async fn enter_name(world: &mut TestWorld) {
    let name_field = world.driver
        .find_element(By::ClassName("name"))
        .await.unwrap();
    name_field.send_keys("Name").await.unwrap();
}

#[when("I enter my email")]
async fn enter_email(world: &mut TestWorld) {
    let email_field = world.driver
        .find_element(By::ClassName("email"))
        .await.unwrap();
    email_field.send_keys("mail@host.com").await.unwrap();
}

#[when("I enter my request message")]
async fn enter_request_message(world: &mut TestWorld) {
    let description = world.driver
        .find_element(By::ClassName("descr"))
        .await.unwrap();
    description.send_keys("This is a message").await.unwrap();
}

#[when("I click on 'Submit' button")]
async fn click_on_save_button(world: &mut TestWorld) {
    let _ = fs::remove_dir_all("./emails");
    let submit_button = world.driver.find_element(By::Id("contact-btn")).await.unwrap();
    submit_button.click().await.unwrap();
}

#[then("The system should received a Contact Request email")]
async fn see_contact_us_page(_world: &mut TestWorld) {
    thread::sleep(time::Duration::from_millis(2000));
    let email_dir = fs::read_dir("./emails").unwrap().next();
    let email = fs::read_to_string(email_dir.unwrap().unwrap().path()).unwrap();
    assert!(email.contains("test@email.com"), "Received: {}", email);
    assert!(email.contains("Your Password Has Been Reset"), "Received: {}", email);
}
