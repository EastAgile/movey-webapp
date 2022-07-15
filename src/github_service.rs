use jelly::error::Error;
use reqwest::blocking::multipart;
use reqwest::header;
use serde::Deserialize;
use std::env;
use std::hash::{Hash, Hasher};
use jelly::error::Error::Generic;

#[cfg(test)]
use crate::test::mock::Client;
#[cfg(test)]
use crate::test::mock::MockResponse as Response;

#[cfg(not(test))]
use reqwest::blocking::Client;
#[cfg(not(test))]
use reqwest::blocking::Response;

#[derive(Deserialize)]
struct MoveToml {
    package: PackageToml,
}

#[derive(Deserialize)]
struct PackageToml {
    name: String,
    version: String,
}

#[derive(Clone, Debug, Eq, Deserialize)]
pub struct GithubRepoData {
    pub name: String,
    pub version: String,
    pub readme_content: String,
    pub description: String,
    pub size: i32,
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
    pub default_branch: String,
}

#[derive(Deserialize)]
pub struct GithubRepoCommit {
    pub sha: String,
}

pub static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"), );

#[cfg(test)]
use mockall::{automock, predicate::*};
use oauth2::http::StatusCode;

pub struct GithubService {}

impl GithubService {
    pub fn new() -> Self {
        GithubService {}
    }
}

#[cfg_attr(test, automock)]
impl GithubService {
    pub fn fetch_repo_data(&self, repo_url: &String, path: Option<String>, mut rev: Option<String>) -> Result<GithubRepoData, Error> {
        let mut github_info = get_repo_description_and_size(&repo_url)?;
        if github_info.default_branch.is_empty() {
            github_info.default_branch = "master".to_string();
        }

        if rev.is_none() {
            if let Ok(sha) = get_repo_latest_commit_sha(&repo_url) {
                rev = Some(sha)
            } else {
                rev = Some(github_info.default_branch);
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
            match call_github_api(&readme_url)?.text() {
                Ok(content) => {
                    // generate description from readme if not existed
                    if github_info.description.is_none() {
                        let mut description = call_deep_ai_api(content.clone())?;
                        if description.len() > 400 {
                            let deepai_summary = call_deep_ai_api(description.clone())?;
                            if deepai_summary.len() > 0 {
                                description = deepai_summary;
                            }
                        }
                        description = strip_markdown::strip_markdown(&description);

                        if description.is_empty() {
                            description = strip_markdown::strip_markdown(&content);
                        }
                        if description.len() > 400 {
                            description = description[0..400].to_string();
                        }
                        github_info.description = Some(description);
                    }
                    readme_content = content
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
            Ok(content) => {
                move_toml_content = content
            }
            Err(error) => {
                error!(
                        "Error getting Move.toml content. url: {:?}, error: {}",
                        move_url, error
                    );
            }
        };

        match toml::from_str::<MoveToml>(&move_toml_content) {
            Ok(move_toml) => Ok(GithubRepoData {
                name: move_toml.package.name,
                version: move_toml.package.version,
                readme_content,
                description: github_info.description.unwrap_or("".to_string()),
                size: github_info.size,
                url: String::from(""),
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
                    description: github_info.description.unwrap_or("".to_string()),
                    size: github_info.size,
                    url: String::from(""),
                    rev,
                })
            }
        }
    }
}

fn call_github_api(url: &str) -> Result<Response, Error> {
    let access_token = env::var("GITHUB_ACCESS_TOKEN")
        .expect("Unable to pull GITHUB_ACCESS_TOKEN");
    let client = Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;
    let res = client
        .get(url)
        .header(header::AUTHORIZATION, format!("token {}", &access_token))
        .send()?;
    Ok(res)
}

fn call_deep_ai_api(content: String) -> Result<String, Error> {
    let access_token = env::var("DEEP_AI_API_KEY")
        .expect("Unable to pull DEEP_AI_API_KEY");
    // not be able to mock both get and post func of Client at the moment,
    // use full path to avoid using MockClient
    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;
    let form = multipart::Form::new()
        .text("text", content);
    let response = client
        .post("https://api.deepai.org/api/summarization")
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
            if response.output == "" {
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
            Err(Generic(format!("Error getting repo commit. url: {:?}, error: Empty response", url)))
        }
        Err(error) => {
            error!(
                    "Error getting repo commit. url: {:?}, error: {}",
                    url, error
                );
            Err(Generic(format!("Error getting repo commit. url: {:?}, error: {}", url, error)))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::github_service::{call_github_api, get_repo_description_and_size, get_repo_latest_commit_sha, GithubService};
    use crate::test::stub::{GH_REPO_INFO_STUB, SHA_STUB};

    #[actix_rt::test]
    async fn fetch_repo_data_works() {
        crate::test::init();
        let gh_service = GithubService::new();
        let gh_repo_info = gh_service.fetch_repo_data(
            &"https://github.com/EastAgile/ea-movey".to_string(),
            Some("/random-url".to_string()),
            None
        ).unwrap();

        assert_eq!(gh_repo_info.name, "A".to_string());
        assert_eq!(gh_repo_info.version, "0.0.0".to_string());
        assert_eq!(gh_repo_info.readme_content,
                   "[package]\nname=\"A\"\nversion=\"0.0.0\"".to_string());
        assert_eq!(gh_repo_info.description, "description".to_string());
        assert_eq!(gh_repo_info.size, 10);
        assert_eq!(gh_repo_info.rev, "abcdef123456".to_string());
    }

    #[actix_rt::test]
    async fn get_repo_description_and_size_mock_successfully() {
        crate::test::init();

        let gh_repo_info = get_repo_description_and_size("/random-url").unwrap();
        let gh_repo_info_stub = GH_REPO_INFO_STUB.clone();
        assert_eq!(gh_repo_info.description, gh_repo_info_stub.description);
        assert_eq!(gh_repo_info.default_branch, gh_repo_info_stub.default_branch);
        assert_eq!(gh_repo_info.size, gh_repo_info_stub.size);
    }

    #[actix_rt::test]
    async fn get_repo_latest_commit_sha_mock_successfully() {
        crate::test::init();
        let sha = get_repo_latest_commit_sha("/random-url").unwrap();
        assert_eq!(sha, SHA_STUB.to_string())
    }

    #[actix_rt::test]
    async fn call_github_api_mock_successfully() {
        crate::test::init();
        call_github_api("/random-url").unwrap();
    }
}
