use jelly::error::Error;
use reqwest::blocking::{multipart, Response};
use reqwest::header;
use serde::Deserialize;
use std::env;
use std::hash::{Hash, Hasher};

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

#[derive(Default, Deserialize)]
pub struct GithubRepoInfo {
    pub description: Option<String>,
    pub size: i32,
    pub default_branch: String,
}

pub static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"), );

#[cfg(test)]
use mockall::{automock, predicate::*};

pub struct GithubService {}

impl GithubService {
    pub fn new() -> Self {
        GithubService {}
    }
}

#[cfg_attr(test, automock)]
impl GithubService {
    pub fn fetch_repo_data(&self, repo_url: &String, path: Option<String>) -> Result<GithubRepoData, Error> {
        let mut github_info = get_repo_description_and_size(repo_url)?;

        if github_info.default_branch.is_empty() {
            github_info.default_branch = "master".to_string();
        }
        let readme_url = format!(
            "{}/{}/README.md",
            repo_url.replace("https://github.com", "https://raw.githubusercontent.com"),
            github_info.default_branch
        );
        let mut readme_content = "".to_string();
        match call_github_api(&readme_url)?.text() {
            Ok(content) => {
                // generate description from readme if not existed
                if github_info.description.is_none() {
                    github_info.description = call_deep_ai_api(content.clone())?;
                    if github_info.description.is_none() {
                        let content_stripped = &content
                            .replace("\n", " ")
                            .replace("\r", "")
                            .replace("#", "");
                        let mut description_from_readme =
                            String::from("[Generated from README]\n") +
                                content_stripped;
                        if description_from_readme.len() > 100 {
                            description_from_readme = description_from_readme[0..100].to_string();
                            description_from_readme += "...";
                        }
                        github_info.description = Some(description_from_readme);
                    }
                }
                readme_content = content
            }
            Err(error) => {
                error!(
                        "Error getting README.md content. url: {:?}, error: {}",
                        readme_url, error
                    );
            }
        }

        let move_url = match path {
            Some(path) => {
                format!("{}/{}", readme_url.replace("/README.md", ""), path)
            }
            None => {
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
                rev: "".to_string(),
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
                    rev: "".to_string(),
                })
            }
        }
    }
}

fn call_github_api(url: &str) -> Result<Response, Error> {
    let access_token = env::var("GITHUB_ACCESS_TOKEN")
        .expect("Unable to pull GITHUB_ACCESS_TOKEN");
    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;
    let res = client
        .get(url)
        .header(header::AUTHORIZATION, format!("token {}", &access_token))
        .send()?;

    Ok(res)
}

fn call_deep_ai_api(content: String) -> Result<Option<String>, Error> {
    let access_token = env::var("DEEP_AI_API_KEY")
        .expect("Unable to pull DEEP_AI_API_KEY");
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
        return Ok(None);
    };

    #[derive(Deserialize)]
    struct DeepApiResponse {
        output: String,
    }

    match response.unwrap().json::<DeepApiResponse>() {
        Ok(response) => {
            if response.output == "" {
                return Ok(None);
            }
            Ok(Some(response.output))
        }
        Err(error) => {
            error!("Error getting response from deepai.org. error: {}", error);
            Ok(None)
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
