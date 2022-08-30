#![allow(unused_imports)]

use crate::dev::Payload;
use crate::github_service::{GithubRepoCommit, GithubRepoData, GithubRepoInfo};
use futures::future::Ready;
use jelly::accounts::User;
use jelly::actix_session::Session;
use jelly::actix_web::FromRequest;
use jelly::jobs::Job;
use jelly::DieselPgPool;
use jelly::{prelude::*, DieselPgConnection};
use mockall::mock;
use mockall_double::double;
use reqwest::blocking::Response;
use reqwest::header::HeaderName;
use reqwest::StatusCode;
use serde::Serialize;

#[double]
use crate::github_service::GithubService as MockGithubService;

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
        fn db_connection(&self) -> Result<DieselPgConnection, Error>;
    }

    impl FromRequest for HttpRequest {
        type Error = ();
        type Future = Ready<Result<Self, Self::Error>>;
        type Config = ();

        fn from_request(req: &HttpRequest, payload: &mut Payload)
            -> Ready<Result<MockHttpRequest, ()>>;
    }
}

pub struct GithubService {}

impl GithubService {
    pub fn new() -> MockGithubService {
        let mut mock_gh_service = MockGithubService::new();
        mock_gh_service
            .expect_fetch_repo_data()
            .returning(|_, _, _| {
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
