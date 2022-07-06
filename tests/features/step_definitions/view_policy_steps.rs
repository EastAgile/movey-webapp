use crate::TestWorld;
use cucumber::{given, when, then};

#[when("I access the Terms of use page")]
async fn visit_terms_of_use_page(world: &mut TestWorld) {
    world
        .driver
        .get(format!("{}accounts/terms-of-use", world.root_url).as_str())
        .await
        .unwrap();
}

#[then("I should see the Terms of use page")]
async fn see_terms_of_use_page(world: &mut TestWorld) {
    assert_eq!(
        world.driver.current_url().await.unwrap(),
        format!("{}accounts/terms-of-use", world.root_url).as_str()
    );
}

#[when("I access the Policy page")]
async fn visit_policy_page(world: &mut TestWorld) {
    world
        .driver
        .get(format!("{}accounts/policy", world.root_url).as_str())
        .await
        .unwrap();
}

#[then("I should see the Policy page")]
async fn see_policy_page(world: &mut TestWorld) {
    assert_eq!(
        world.driver.current_url().await.unwrap(),
        format!("{}accounts/policy", world.root_url).as_str()
    );
}

#[when("I access the Contact us page")]
async fn visit_contact_us_page(world: &mut TestWorld) {
    world
        .driver
        .get(format!("{}accounts/contact", world.root_url).as_str())
        .await
        .unwrap();
}

#[then("I should see the Contact us page")]
async fn see_contact_us_page(world: &mut TestWorld) {
    assert_eq!(
        world.driver.current_url().await.unwrap(),
        format!("{}accounts/contact", world.root_url).as_str()
    );
}

#[when("I access the About us page")]
async fn visit_about_us_page(world: &mut TestWorld) {
    world
        .driver
        .get(format!("{}accounts/about", world.root_url).as_str())
        .await
        .unwrap();
}

#[then("I should see the About us page")]
async fn see_about_us_page(world: &mut TestWorld) {
    assert_eq!(
        world.driver.current_url().await.unwrap(),
        format!("{}accounts/about", world.root_url).as_str()
    );
}
