use serde::{Deserialize};
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

pub struct GithubRepoData {
    pub name: String,
    pub version: String,
    pub readme_content: String
}

static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

pub fn fetch_repo_data(input_url: &String) -> Result<GithubRepoData, Error> {
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
        readme_content: readme_content
    })
}
