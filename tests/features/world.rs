use std::convert::Infallible;

use async_trait::async_trait;
use cucumber::{World, WorldInit};
use thirtyfour::prelude::*;

#[derive(Debug, Default)]
pub struct AccountInformation {
    pub email: String,
    pub password: String,
    pub owned_package_name: Option<String>,
}
#[derive(Debug)]
pub struct TestResponse {
    pub status_code: reqwest::StatusCode,
    pub content_type: String,
    pub body: String,
}
// `World` is your shared, likely mutable state.
#[derive(Debug, WorldInit)]
pub struct TestWorld {
    pub driver: WebDriver,
    pub root_url: String,
    pub suggestion: String,
    pub reset_token: String,
    // default user used for own package
    pub first_account: AccountInformation,
    // second user used for transfer ownership
    pub second_account: AccountInformation,
    pub response: Option<TestResponse>,
}

// `World` needs to be implemented, so Cucumber knows how to construct it
// for each scenario.
#[async_trait(?Send)]
impl World for TestWorld {
    // We do require some error type.
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            driver: {
                let mut caps = DesiredCapabilities::chrome();
                caps.add_chrome_arg("--no-sandbox").unwrap();
                caps.add_chrome_arg("--headless").unwrap();
                caps.add_chrome_arg("--window-size=1920,1080").unwrap();
                WebDriver::new("http://localhost:4444", &caps)
                    .await
                    .unwrap()
            },
            root_url: "http://localhost:17002/".to_string(),
            suggestion: String::from(""),
            reset_token: String::from(""),
            first_account: Default::default(),
            second_account: Default::default(),
            response: None,
        })
    }
}

impl TestWorld {
    pub async fn go_to_root_url(&self) {
        self.driver.get(&self.root_url).await.unwrap()
    }

    pub async fn go_to_url(&self, relative_url: &str) {
        self.driver
            .get(format!("{}{}", &self.root_url, relative_url))
            .await
            .unwrap()
    }

    pub async fn close_browser(&self) {
        self.driver
            .handle
            .cmd(thirtyfour::common::command::Command::DeleteSession)
            .await
            .unwrap();
    }
}
