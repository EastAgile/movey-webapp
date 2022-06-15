use jelly::{anyhow::anyhow, error::Error};
use reqwest::blocking::{multipart, Response};
use reqwest::header;
use serde::Deserialize;
use std::env;
use std::hash::{Hash, Hasher};

#[derive(Deserialize)]
struct GithubResponse {
    download_url: Option<String>,
}

#[derive(Deserialize)]
struct MoveToml {
    package: PackageToml,
}

#[derive(Deserialize)]
struct PackageToml {
    name: String,
    version: String,
}

#[derive(Clone, Deserialize, Eq, PartialEq)]
pub struct GithubRepoInfo {
    pub description: Option<String>,
    pub size: i32,
}

#[derive(Clone, Eq, Deserialize)]
pub struct GithubRepoData {
    pub name: String,
    pub version: String,
    pub readme_content: String,
    pub info: GithubRepoInfo,
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

pub static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

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
    // don't need to be async because it is used in rayon crate
    // and rayon is not a good fit for async
    pub fn fetch_repo_data(&self, input_url: &String) -> Result<GithubRepoData, Error> {
        let repo_url = input_url.replace("https://github.com/", "https://api.github.com/repos/");
        let response = call_github_api(&repo_url)?;
        let mut repo_info = match response.json::<GithubRepoInfo>() {
            Ok(info) => info,
            Err(error) => {
                error!(
                    "Error getting repo description and size. url: {:?}, error: {}",
                    repo_url, error
                );
                GithubRepoInfo {
                    description: None,
                    size: -1,
                }
            }
        };

        let content_url = format!("{}{}", repo_url, "/contents");
        let response = call_github_api(&content_url)?;
        let response_json: Vec<GithubResponse> = response.json()?;
        if response_json.is_empty() {
            return Err(Error::Generic(format!("Invalid repo url: {}", &content_url)));
        }

        let readme_urls = response_json
            .iter()
            .filter(|content| {
                content.download_url.is_some()
                    && content
                        .download_url
                        .as_ref()
                        .unwrap()
                        .ends_with("/README.md")
            })
            .collect::<Vec<&GithubResponse>>();
        let readme_content = if readme_urls.len() > 0 {
            let response =
                call_github_api(readme_urls.first().unwrap().download_url.as_ref().unwrap())?;
            match response.text() {
                Ok(content) => {
                    // generate description from readme if not existed
                    if repo_info.description.is_none() {
                        repo_info.description = call_deep_ai_api(content.clone())?;
                        if repo_info.description.is_none() {
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
                            repo_info.description = Some(description_from_readme);
                        }
                    }
                    content
                },
                Err(error) => {
                    error!(
                        "Error getting README.md content. url: {:?}, error: {}",
                        content_url, error
                    );
                    String::from("")
                }
            }
        } else {
            warn!("Link to README.md not found. url: {}", &content_url);
            String::from("")
        };

        let move_toml_urls = response_json
            .iter()
            .filter(|content| {
                content.download_url.is_some()
                    && content
                        .download_url
                        .as_ref()
                        .unwrap()
                        .to_lowercase()
                        .ends_with("move.toml")
            })
            .collect::<Vec<&GithubResponse>>();
        let move_toml_content = if move_toml_urls.len() > 0 {
            let response =
                call_github_api(move_toml_urls.first().unwrap().download_url.as_ref().unwrap())?;
            match response.text() {
                Ok(content) => content,
                Err(error) => {
                    error!(
                        "Error getting Move.toml content. url: {:?}, error: {}",
                        content_url, error
                    );
                    String::from("")
                }
            }
        } else {
            error!("Link to Move.toml not found. url: {}", &content_url);
            String::from("")
        };
        match toml::from_str::<MoveToml>(&move_toml_content) {
            Ok(move_toml) => Ok(GithubRepoData {
                name: move_toml.package.name,
                version: move_toml.package.version,
                readme_content,
                info: repo_info,
                url: String::from(""),
                rev: "".to_string(),
            }),
            Err(error) => {
                warn!(
                    "Invalid Move.toml url: {}, content: {}, error: {}",
                    &content_url, &move_toml_content, &error
                );
                Ok(GithubRepoData {
                    name: String::from(""),
                    version: String::from(""),
                    readme_content,
                    info: repo_info,
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
        .header(header::AUTHORIZATION, format!("Bearer {}", &access_token))
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
        output: String
    }

    match response.unwrap().json::<DeepApiResponse>() {
        Ok(response) => {
            if response.output == "" {
                return Ok(None);
            }
            Ok(Some(response.output))
        },
        Err(error) => {
            error!("Error getting response from deepai.org. error: {}", error);
            Ok(None)
        }
    }
}
