use crate::TestWorld;
use cucumber::{given, when, then};
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
    let page_name = world.driver
        .find_element(By::ClassName("page_name"))
        .await.unwrap();
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
    let page_name = world.driver
        .find_element(By::ClassName("page_name"))
        .await.unwrap();
    assert_eq!(page_name.text().await.unwrap(), "Privacy Policy");
}

#[when("I access the Contact us page")]
async fn visit_contact_us_page(world: &mut TestWorld) {
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
    let page_name = world.driver
        .find_element(By::ClassName("page_name"))
        .await.unwrap();
    assert_eq!(page_name.text().await.unwrap(), "Contact Us");
    let form_cta = world.driver
        .find_element(By::ClassName("form-cta"))
        .await.unwrap();
    assert_eq!(form_cta.text().await.unwrap(), "Submit a request");

    world.driver
        .find_element(By::ClassName("packages-sort"))
        .await.unwrap();
    world.driver
        .find_element(By::Id("name"))
        .await.unwrap();
    world.driver
        .find_element(By::Id("email"))
        .await.unwrap();
    world.driver
        .find_element(By::Id("descr"))
        .await.unwrap();
}
