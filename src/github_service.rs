use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::sync::Arc;
use serde::Deserialize;
use jelly::error::Error;

#[derive(Deserialize, Debug)]
struct GithubResponse {
    download_url: String,
}

#[derive(Deserialize)]
struct MoveToml {
    package: PackageToml
}

#[derive(Deserialize)]
struct PackageToml {
    name: String,
    version: String
}

#[derive(Clone, Debug, Eq, Deserialize)]
pub struct GithubRepoData {
    pub name: String,
    pub version: String,
    pub readme_content: String
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


pub static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

#[cfg(test)]
use mockall::{automock, predicate::*};
use reqwest::header;


pub struct GithubService {}

impl GithubService {
    pub fn new() -> Self {
        GithubService {}
    }
}

#[cfg_attr(test, automock)]
impl GithubService {
    pub fn fetch_repo_data(&self, input_url: &String) -> Result<GithubRepoData, Error> {
        let url = format!("{}{}", input_url.replace("https://github.com/", "https://api.github.com/repos/"), "/readme");

        let client = reqwest::blocking::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build().unwrap();

        let response = client.get(url).send().unwrap();
        let response_json: GithubResponse = response.json().unwrap();
        let readme_url = response_json.download_url;

        let response = client.get(&readme_url).send().unwrap();
        let readme_content = response.text().unwrap();

        let move_toml_url = readme_url.replace("README.md", "Move.toml");
        let response = client.get(&move_toml_url).send().unwrap();
        let move_toml_content = response.text().unwrap();

        let move_toml: MoveToml = toml::from_str(&move_toml_content).unwrap();

        Ok(GithubRepoData {
            name: move_toml.package.name,
            version: move_toml.package.version,
            readme_content
        })
    }
}
