#![allow(unused_imports)]

use crate::dev::Payload;
use crate::github_service::GithubRepoData;
use futures::future::Ready;
use jelly::actix_web::FromRequest;
use jelly::prelude::*;
use jelly::DieselPgPool;
use mockall::mock;
use mockall_double::double;

#[double]
use crate::github_service::GithubService as MockGithubService;

mock! {
    pub HttpRequest {}

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

pub struct GithubService {}

impl GithubService {
    pub fn new() -> MockGithubService {
        let mut mock_gh_service = MockGithubService::new();
        mock_gh_service.expect_fetch_repo_data().returning(|_, _| {
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
