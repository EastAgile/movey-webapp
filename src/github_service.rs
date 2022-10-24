use jelly::error::Error;
use jelly::error::Error::Generic;
use reqwest::blocking::{multipart, Response};
use reqwest::header;
use serde::{Deserialize, Serialize};
use std::env;
use std::hash::{Hash, Hasher};

#[derive(Deserialize, Serialize)]
struct MoveToml {
    package: PackageToml,
}

#[derive(Deserialize, Serialize)]
struct PackageToml {
    name: String,
    version: String,
}

#[derive(Clone, Debug, Eq, Deserialize)]
pub struct GithubRepoData {
    pub name: String,
    pub version: String,
    pub readme_content: String,
    pub license: String,
    pub description: String,
    pub size: i32,
    pub stars_count: i32,
    pub forks_count: i32,
    pub url: String,
    pub rev: String,
}

impl PartialEq for GithubRepoData {
    fn eq(&self, other: &GithubRepoData) -> bool {
        self.name == other.name && self.version == other.version
    }
}

impl Hash for GithubRepoData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.version.hash(state);
    }
}

#[derive(Clone, Default, Deserialize)]
pub struct GithubRepoInfo {
    pub description: Option<String>,
    pub size: i32,
    pub stargazers_count: i32,
    pub forks_count: i32,
    pub default_branch: String,
    pub license: Option<GithubLicenseInfo>,
}

#[derive(Clone, Default, Deserialize)]
pub struct GithubLicenseInfo {
    pub key: String,
    pub name: String,
    pub url: String,
}

#[derive(Deserialize)]
pub struct GithubRepoCommit {
    pub sha: String,
}

pub static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

use crate::constants::DEEP_AI_URL;
#[cfg(test)]
use mockall::{automock, predicate::*};
use oauth2::http::StatusCode;

pub struct GithubService {}

impl GithubService {
    pub fn new() -> Self {
        GithubService {}
    }
}

impl Default for GithubService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(test, automock)]
impl GithubService {
    pub fn fetch_repo_data(
        &self,
        repo_url: &str,
        path: Option<String>,
        mut rev: Option<String>,
    ) -> Result<GithubRepoData, Error> {
        let mut github_info = get_repo_description_and_size(repo_url)?;
        if github_info.default_branch.is_empty() {
            github_info.default_branch = "master".to_string();
        }

        if rev.is_none() {
            if let Ok(sha) = get_repo_latest_commit_sha(repo_url) {
                rev = Some(sha)
            } else {
                rev = Some(github_info.default_branch.clone());
            }
        }
        let rev = rev.unwrap();

        // example readme url:
        // https://raw.githubusercontent.com/move-language/move/main/README.md
        let readme_url = format!(
            "{}/{}/README.md",
            repo_url.replace("https://github.com", "https://raw.githubusercontent.com"),
            rev
        );
        let mut readme_content = "".to_string();
        let response = call_github_api(&readme_url)?;
        if response.status() != StatusCode::NOT_FOUND {
            match response.text() {
                Ok(content) => {
                    // generate description from readme if not existed
                    if github_info.description.is_none() {
                        let mut description = call_deep_ai_api(content.clone(), None)?;
                        if description.chars().count() > 100 {
                            let deepai_summary = call_deep_ai_api(description.clone(), None)?;
                            if !deepai_summary.is_empty() {
                                description = deepai_summary;
                            }
                        }
                        description = strip_markdown::strip_markdown(&description);

                        if description.is_empty() {
                            description = strip_markdown::strip_markdown(&content);
                        }
                        description = description.chars().take(100).collect();
                        github_info.description = Some(description);
                    }

                    let formatted_readme_src_http =
                        format!("src=\"{}", readme_url.replace("README.md", ""),);
                    let formatted_readme_src_markdown =
                        format!("]({}", readme_url.replace("README.md", ""),);

                    // Add prefix url for linkable content in readme file such as: images, link...
                    let formatted_readme_content = content
                        .replace("src=\"", &formatted_readme_src_http)
                        .replace(&format!("{}http", &formatted_readme_src_http), "src=\"http")
                        .replace("](", &formatted_readme_src_markdown)
                        .replace(&format!("{}http", &formatted_readme_src_markdown), "](http");

                    readme_content = formatted_readme_content
                }
                _ => {
                    warn!("Error getting README.md content. url: {}", readme_url);
                }
            }
        }

        let move_url = match path {
            // example Move.toml url with subdir:
            // https://raw.githubusercontent.com/move-language/move/main/language/evm/hardhat-examples/contracts/ABIStruct/Move.toml
            Some(path) => {
                format!("{}/{}", readme_url.replace("/README.md", ""), path)
            }
            None => {
                // Move.toml in top directory:
                // https://raw.githubusercontent.com/taoheorg/taohe/master/Move.toml
                readme_url.replace("README.md", "Move.toml")
            }
        };

        let mut move_toml_content = "".to_string();
        match call_github_api(&move_url)?.text() {
            Ok(content) => move_toml_content = content,
            Err(error) => {
                error!(
                    "Error getting Move.toml content. url: {:?}, error: {}",
                    move_url, error
                );
            }
        };

        let license = match github_info.license {
            Some(license) => license.name,
            None => "".to_string(),
        };

        match toml::from_str::<MoveToml>(&move_toml_content) {
            Ok(move_toml) => Ok(GithubRepoData {
                name: move_toml.package.name,
                version: move_toml.package.version,
                readme_content,
                license,
                description: github_info.description.unwrap_or_else(|| "".to_string()),
                size: github_info.size,
                stars_count: github_info.stargazers_count,
                forks_count: github_info.forks_count,
                // this field is overwritten in the crawler, modified this to save default branch
                url: github_info.default_branch,
                rev,
            }),
            Err(error) => {
                warn!(
                    "Invalid Move.toml url: {}, content: {}, error: {}",
                    &move_url, &move_toml_content, &error
                );
                Ok(GithubRepoData {
                    name: String::from(""),
                    version: String::from(""),
                    readme_content,
                    license,
                    description: github_info.description.unwrap_or_else(|| "".to_string()),
                    size: github_info.size,
                    stars_count: github_info.stargazers_count,
                    forks_count: github_info.forks_count,
                    // this field is overwritten in the crawler, modified this to save default branch
                    url: github_info.default_branch,
                    rev,
                })
            }
        }
    }
}

fn call_github_api(url: &str) -> Result<Response, Error> {
    let access_token = env::var("GITHUB_ACCESS_TOKEN").expect("Unable to pull GITHUB_ACCESS_TOKEN");
    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;
    let res = client
        .get(url)
        .header(header::AUTHORIZATION, format!("token {}", &access_token))
        .send()?;
    Ok(res)
}

// url param is only used in testing
fn call_deep_ai_api(content: String, url: Option<&str>) -> Result<String, Error> {
    let access_token = env::var("DEEP_AI_API_KEY").expect("Unable to pull DEEP_AI_API_KEY");
    // not be able to mock both get and post func of Client at the moment,
    // use full path to avoid using MockClient
    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;
    let form = multipart::Form::new().text("text", content);
    let url = url.unwrap_or(DEEP_AI_URL);
    let response = client
        .post(url)
        .header("api-key", access_token)
        .multipart(form)
        .send()
        .ok();
    if response.is_none() {
        return Ok(String::new());
    };

    #[derive(Deserialize)]
    struct DeepApiResponse {
        output: String,
    }

    match response.unwrap().json::<DeepApiResponse>() {
        Ok(response) => {
            if response.output.is_empty() {
                return Ok(String::new());
            }
            Ok(response.output)
        }
        Err(error) => {
            error!("Error getting response from deepai.org. error: {}", error);
            Ok(String::new())
        }
    }
}

fn get_repo_description_and_size(repo_url: &str) -> Result<GithubRepoInfo, Error> {
    let url = repo_url.replace("https://github.com/", "https://api.github.com/repos/");
    let response = call_github_api(&url)?;
    match response.json::<GithubRepoInfo>() {
        Ok(info) => Ok(info),
        Err(error) => {
            error!(
                "Error getting repo description and size. url: {:?}, error: {}",
                url, error
            );
            Ok(Default::default())
        }
    }
}

fn get_repo_latest_commit_sha(repo_url: &str) -> Result<String, Error> {
    let mut url = repo_url.replace("https://github.com/", "https://api.github.com/repos/");
    url.push_str("/commits");
    let response = call_github_api(&url)?;
    match response.json::<Vec<GithubRepoCommit>>() {
        Ok(info) if !info.is_empty() => Ok(info.get(0).unwrap().sha.clone()),
        Ok(_) => {
            error!(
                "Error getting repo commit. url: {:?}, error: Empty response",
                url
            );
            Err(Generic(format!(
                "Error getting repo commit. url: {:?}, error: Empty response",
                url
            )))
        }
        Err(error) => {
            error!(
                "Error getting repo commit. url: {:?}, error: {}",
                url, error
            );
            Err(Generic(format!(
                "Error getting repo commit. url: {:?}, error: {}",
                url, error
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use httpmock::prelude::*;
    use httpmock::MockServer;
    use serde_json::json;
    use std::env;

    use super::*;

    #[test]
    fn get_repo_description_and_size_works() {
        crate::test::init();

        let access_token = env::var("GITHUB_ACCESS_TOKEN").unwrap();
        let server = MockServer::start();
        let server_mock = server.mock(|when, then| {
            when.method(GET)
                .header("User-Agent", APP_USER_AGENT)
                .header("authorization", format!("token {}", &access_token));
            then.status(200).json_body(json!({
                "description": "test description",
                "size": 1,
                "stargazers_count": 2,
                "forks_count": 3,
                "default_branch": "test branch",
            }));
        });

        let result = get_repo_description_and_size(&server.base_url()).unwrap();
        server_mock.assert();
        assert_eq!(result.description, Some("test description".to_string()));
        assert_eq!(result.size, 1);
        assert_eq!(result.stargazers_count, 2);
        assert_eq!(result.forks_count, 3);
        assert_eq!(result.default_branch, "test branch");
    }

    #[test]
    fn get_repo_description_and_size_returns_default_value_if_response_body_is_empty() {
        crate::test::init();

        let access_token = env::var("GITHUB_ACCESS_TOKEN").unwrap();
        let server = MockServer::start();
        let server_mock = server.mock(|when, then| {
            when.method(GET)
                .header("User-Agent", APP_USER_AGENT)
                .header("authorization", format!("token {}", &access_token));
            then.status(200);
        });

        let result = get_repo_description_and_size(&server.base_url()).unwrap();
        server_mock.assert();

        assert_eq!(result.description, None);
        assert_eq!(result.size, 0);
        assert_eq!(result.default_branch, "");
    }

    #[test]
    fn get_repo_latest_commit_sha_works() {
        crate::test::init();

        let access_token = env::var("GITHUB_ACCESS_TOKEN").unwrap();
        let server = MockServer::start();
        let server_mock = server.mock(|when, then| {
            when.method(GET)
                .path("/commits")
                .header("User-Agent", APP_USER_AGENT)
                .header("authorization", format!("token {}", &access_token));
            then.status(200).json_body(json!([
                { "sha": "test sha" }
            ]));
        });

        let result = get_repo_latest_commit_sha(&server.base_url());
        server_mock.assert();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test sha");
    }

    #[test]
    fn get_repo_latest_commit_sha_returns_err_if_response_body_is_empty() {
        crate::test::init();

        let access_token = env::var("GITHUB_ACCESS_TOKEN").unwrap();
        let server = MockServer::start();
        let server_mock = server.mock(|when, then| {
            when.method(GET)
                .path("/commits")
                .header("User-Agent", APP_USER_AGENT)
                .header("authorization", format!("token {}", &access_token));
            then.status(200);
        });

        let result = get_repo_latest_commit_sha(&server.base_url());
        server_mock.assert();
        assert!(result.is_err());
    }

    #[test]
    fn call_deep_ai_api_works() {
        crate::test::init();

        let access_token = env::var("DEEP_AI_API_KEY").expect("Unable to pull DEEP_AI_API_KEY");
        let server = MockServer::start();
        // TODO: check if the request content type is multipart
        // https://github.com/alexliesenfeld/httpmock/tree/master/tests/examples
        let server_mock = server.mock(|when, then| {
            when.method(POST)
                .path("/api/summarization")
                .header("User-Agent", APP_USER_AGENT)
                .header("api-key", access_token);
            then.status(200)
                .json_body(json!({ "output":"summarized text" }));
        });

        let result = call_deep_ai_api(
            "original text".to_string(),
            Some(&format!("{}/api/summarization", &server.base_url())),
        );
        server_mock.assert();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "summarized text");
    }

    #[test]
    fn call_deep_ai_api_returns_empty_string_if_deep_ai_response_body_is_empty() {
        crate::test::init();

        let access_token = env::var("DEEP_AI_API_KEY").expect("Unable to pull DEEP_AI_API_KEY");
        let server = MockServer::start();
        // TODO: check if the request content type is multipart
        let server_mock = server.mock(|when, then| {
            when.method(POST)
                .path("/api/summarization")
                .header("User-Agent", APP_USER_AGENT)
                .header("api-key", access_token);
            then.status(200).json_body(json!({ "output":"" }));
        });

        let result = call_deep_ai_api(
            "original text".to_string(),
            Some(&format!("{}/api/summarization", &server.base_url())),
        );
        server_mock.assert();
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn call_deep_ai_api_returns_empty_string_if_out_of_free_credit() {
        crate::test::init();

        let access_token = env::var("DEEP_AI_API_KEY").expect("Unable to pull DEEP_AI_API_KEY");
        let server = MockServer::start();
        // TODO: check if the request content type is multipart
        let server_mock = server.mock(|when, then| {
            when.method(POST)
                .path("/api/summarization")
                .header("User-Agent", APP_USER_AGENT)
                .header("api-key", access_token);
            then.status(401).json_body(json!({
                "status":"Out of free credits \
                    - please enter payment info in your dashboard: https://deepai.org/dashboard"
            }));
        });

        let result = call_deep_ai_api(
            "original text".to_string(),
            Some(&format!("{}/api/summarization", &server.base_url())),
        );
        server_mock.assert();
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn fetch_repo_data_works() {
        crate::test::init();

        let access_token = env::var("GITHUB_ACCESS_TOKEN").unwrap();
        let server = MockServer::start();
        let description_and_size_mock = server.mock(|when, then| {
            when.method(GET)
                .path("/EastAgile/ea-movey")
                .header("User-Agent", APP_USER_AGENT)
                .header("authorization", format!("token {}", &access_token));
            then.status(200).json_body(json!({
                "description": "test description",
                "size": 10,
                "stargazers_count": 20,
                "forks_count": 30,
                "default_branch": "test-default-branch",
            }));
        });
        let readme_mock = server.mock(|when, then| {
            when.method(GET)
                .path("/EastAgile/ea-movey/rev/README.md")
                .header("User-Agent", APP_USER_AGENT)
                .header("authorization", format!("token {}", &access_token));
            then.status(200).body("test readme content - <img src=\"one\" /> - <img src=\"http://two\" /> - [three](three) - [four](http://four)");
        });

        let move_toml = MoveToml {
            package: PackageToml {
                name: "test package name".to_string(),
                version: "0.0.0".to_string(),
            },
        };
        let move_toml_mock = server.mock(|when, then| {
            when.method(GET)
                .path("/EastAgile/ea-movey/rev/Move.toml")
                .header("User-Agent", APP_USER_AGENT)
                .header("authorization", format!("token {}", &access_token));
            then.status(200).body(toml::to_string(&move_toml).unwrap());
        });

        let gh_service = GithubService::new();
        let repo_url = format!("{}/EastAgile/ea-movey", server.base_url());
        let gh_repo_data = gh_service
            .fetch_repo_data(&repo_url, None, Some("rev".to_string()))
            .unwrap();

        let expected_readme_content = format!("test readme content - <img src=\"{}/rev/one\" /> - <img src=\"http://two\" /> - [three]({}/rev/three) - [four](http://four)", &repo_url, &repo_url);

        description_and_size_mock.assert();
        readme_mock.assert();
        move_toml_mock.assert();
        assert_eq!(gh_repo_data.name, "test package name");
        assert_eq!(gh_repo_data.version, "0.0.0");
        assert_eq!(gh_repo_data.readme_content, expected_readme_content);
        assert_eq!(gh_repo_data.description, "test description");
        assert_eq!(gh_repo_data.size, 10);
        assert_eq!(gh_repo_data.stars_count, 20);
        assert_eq!(gh_repo_data.forks_count, 30);
        assert_eq!(gh_repo_data.url, "test-default-branch");
        assert_eq!(gh_repo_data.rev, "rev");
    }
}
