use cucumber::{then, when};
use mainlib::{
    api::services::setting::controllers::profile::LoggedInUser, 
    accounts::Account, 
    test::DB_POOL,
};
use reqwest::StatusCode;
use thirtyfour::prelude::*;
use urlencoding::decode;

use crate::features::world::TestResponse;

use super::super::world::TestWorld;

#[when("I use the api to get my profile")]
async fn get_logged_in_user(world: &mut TestWorld) {
    let client = reqwest::Client::new();
    let res = client
        .get(format!("{}api/v1/me", world.root_url))
        .header("Cookie", 
            format!("sessionid={}", world.driver.get_cookie("sessionid").await.unwrap()
                .value().to_string())
        )
        .send()
        .await.unwrap();

    world.response = Some(TestResponse {
        status_code: res.status(),
        content_type: res.headers().get(reqwest::header::CONTENT_TYPE).unwrap().to_str().unwrap().to_string(),
        body: res.text().await.unwrap(),
    })
}

#[when("my account is deleted but my browser is not signed out")]
async fn delete_account(world: &mut TestWorld) {
    let account = 
        Account::get_by_email(&world.account.email, &DB_POOL).await.unwrap();
    Account::delete(account.id, &DB_POOL).await.unwrap();
}

#[then("I should get information about my profile")]
async fn see_account_information(world: &mut TestWorld) {
    assert!(world.response.is_some());

    let response = world.response.as_ref().unwrap();
    assert_eq!(response.status_code, StatusCode::OK);
    assert_eq!(response.content_type, "application/json");

    let user = serde_json::from_str::<LoggedInUser>(&response.body).unwrap();
    assert_eq!(user.email, world.account.email);

    let account = Account::get_by_email(&user.email, &DB_POOL).await.unwrap();
    assert_eq!(account.id, user.id);
    assert_eq!(account.name, user.name);
}

#[then("I should get an empty response from the api")]
async fn empty_response_from_api(world: &mut TestWorld) {
    assert!(world.response.is_some());

    let response = world.response.as_ref().unwrap();
    assert_eq!(response.status_code, StatusCode::OK);
    assert_eq!(response.content_type, "application/json");
    assert_eq!(response.body, "{}");
}

#[then("I should see that my browser is signed out")]
async fn see_default_account_information(world: &mut TestWorld) {
    let remember_me_cookie = world.driver.get_cookie("remember_me_token").await;
    assert!(remember_me_cookie.is_err());

    // An example cookie for a guest user
    // ioiJgadEx5U86eeMZD0rMj+aJA4m4/bzh55wMOdf7DY={}

    let sessionid = world.driver.get_cookie("sessionid").await.unwrap();
    let sessionid_decoded = decode(&sessionid.value().to_string())
        .unwrap().into_owned().replace("\"", "");
    assert_eq!(sessionid_decoded.len(), 46, "{}", sessionid_decoded);
}

#[then("I should see that sign up button is displayed")]
async fn sign_up_button_is_displayed(world: &mut TestWorld) {
    let sign_up_button = world
        .driver
        .find_element(By::ClassName("sign-up-li"))
        .await.unwrap();
    assert!(!sign_up_button.class_name().await.unwrap().unwrap().contains("hide"));

    let sign_in_button = world
        .driver
        .find_element(By::ClassName("sign-in-li"))
        .await.unwrap();
    assert!(!sign_in_button.class_name().await.unwrap().unwrap().contains("hide"));
}
