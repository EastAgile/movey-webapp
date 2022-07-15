#![allow(unused_imports)]

use crate::dev::Payload;
use crate::github_service::{GithubRepoCommit, GithubRepoData, GithubRepoInfo};
use futures::future::Ready;
use jelly::actix_web::FromRequest;
use jelly::prelude::*;
use jelly::DieselPgPool;
use mockall::mock;
use mockall_double::double;
use jelly::jobs::Job;
use serde::Serialize;
use jelly::accounts::User;
use jelly::actix_session::Session;
use reqwest::header::HeaderName;
use reqwest::blocking::Response;
use reqwest::StatusCode;

#[double]
use crate::github_service::GithubService as MockGithubService;
use crate::test::stub::{GH_REPO_INFO_STUB, SHA_STUB};

mock! {
    pub HttpRequest {
        pub fn set_user(&self, account: User) -> Result<(), Error>;
        pub fn get_session(&self) -> Session;
        pub fn user(&self) -> Result<User, Error>;
        pub fn render(&self, code: usize, template: &str, context: Context) -> Result<HttpResponse, Error>;
        pub fn redirect(&self, location: &str) -> Result<HttpResponse, Error>;
        pub fn queue<J: Job + 'static>(&self, job: J) -> Result<(), Error>;
    }

    impl DatabasePool for HttpRequest {
        fn db_pool(&self) -> jelly::Result<&'static DieselPgPool>;
    }

    impl FromRequest for HttpRequest {
        type Error = ();
        type Future = Ready<Result<Self, Self::Error>>;
        type Config = ();

        fn from_request(req: &HttpRequest, payload: &mut Payload)
            -> Ready<Result<MockHttpRequest, ()>>;
    }
}

mock! {
    pub Client {
        pub fn user_agent(self, value: &str) -> Self;
        pub fn build(self) -> jelly::Result<Self>;
        pub fn get(&self, url: &str) -> Self;
        pub fn post(&self, url: &str) -> Self;
        pub fn header(self, key: HeaderName, value: String) -> Self;
        pub fn send(self) -> jelly::Result<MockResponse>;
    }
}

mock! {
    pub Response {
        pub fn json<T: 'static>(self) -> jelly::Result<T>;
        pub fn status(&self) -> StatusCode;
        pub fn text(self) -> jelly::Result<String>;
    }
}

pub struct Client {}

impl Client {
    pub fn builder() -> MockClient {
        let mut mock_client1 = MockClient::new();
        mock_client1.expect_user_agent().returning(|_| {
            let mut mock_client2 = MockClient::new();
            mock_client2.expect_build().returning(|| {
                let mut mock_client3 = MockClient::new();
                mock_client3.expect_get().returning(|_| {
                    let mut mock_client4 = MockClient::new();
                    mock_client4.expect_header().returning(|_, _| {
                        let mut mock_client5 = MockClient::new();
                        mock_client5.expect_send().returning(|| {
                            let mut mock_response = MockResponse::new();
                            mock_response.expect_json::<GithubRepoInfo>().returning(|| {
                                Ok(GH_REPO_INFO_STUB.clone())
                            });
                            mock_response.expect_json::<Vec<GithubRepoCommit>>().returning(|| {
                                Ok(vec![
                                    GithubRepoCommit {
                                        sha: SHA_STUB.to_string()
                                    }
                                ])
                            });
                            mock_response.expect_status().returning(|| StatusCode::OK);
                            mock_response.expect_text().returning(|| {
                                let toml = "[package]\nname=\"A\"\nversion=\"0.0.0\"";
                                Ok(String::from(toml))
                            });
                            Ok(mock_response)
                        });
                        mock_client5
                    });
                    mock_client4
                });
                Ok(mock_client3)
            });
            mock_client2
        });
        mock_client1
    }
}

pub struct GithubService {}

impl GithubService {
    pub fn new() -> MockGithubService {
        let mut mock_gh_service = MockGithubService::new();
        mock_gh_service.expect_fetch_repo_data().returning(|_, _, _| {
            Ok(GithubRepoData {
                name: "name1".to_string(),
                version: "version1".to_string(),
                readme_content: "readme_content1".to_string(),
                description: "".to_string(),
                size: 0,
                url: "".to_string(),
                rev: "".to_string(),
            })
        });
        mock_gh_service
    }
}
