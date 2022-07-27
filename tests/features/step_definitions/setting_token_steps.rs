use cucumber::{then, when};
use reqwest::StatusCode;
use serde_json::json;

use crate::features::world::TestResponse;

use super::super::world::TestWorld;

#[when("I try to use api to create a token")]
async fn put_request_to_create_api(world: &mut TestWorld) {
    let sessionid = world
        .driver
        .get_cookie("sessionid")
        .await
        .unwrap_or_else(|_| thirtyfour::Cookie::new("sessionid", json!({})));

    let client = reqwest::Client::new();
    let res = client
        .put(format!("{}api/v1/tokens", world.root_url))
        .header("Cookie", format!("sessionid={}", sessionid.value()))
        .json(&json!({ "name": "a_new_token" }))
        .send()
        .await
        .unwrap();

    world.response = Some(TestResponse {
        status_code: res.status(),
        content_type: "".to_string(),
        body: res.text().await.unwrap(),
    })
}

#[when("I try to use api to delete a token")]
async fn delete_request(world: &mut TestWorld) {
    let sessionid = world
        .driver
        .get_cookie("sessionid")
        .await
        .unwrap_or_else(|_| thirtyfour::Cookie::new("sessionid", json!({})));

    let client = reqwest::Client::new();
    let res = client
        .delete(format!("{}api/v1/tokens/1", world.root_url))
        .header("Cookie", format!("sessionid={}", sessionid.value()))
        .send()
        .await
        .unwrap();

    world.response = Some(TestResponse {
        status_code: res.status(),
        content_type: "".to_string(),
        body: res.text().await.unwrap(),
    })
}

#[then("I should see that I am not authorized to do so")]
async fn see_account_information(world: &mut TestWorld) {
    assert!(world.response.is_some());

    let response = world.response.as_ref().unwrap();
    assert_eq!(response.status_code, StatusCode::UNAUTHORIZED);
    assert_eq!(response.body, "");
}
