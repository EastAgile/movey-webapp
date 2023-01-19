use crate::github_service::{GithubRepoData, APP_USER_AGENT};
use crate::utils::presenter::validate_version;
use core::time::Duration;
use jelly::actix_web::http::header;
use jelly::actix_web::rt::time::delay_for;
use jelly::DieselPgPool;
use mockall_double::double;
use rayon::prelude::*;
use serde::Deserialize;
use std::collections::HashSet;
use std::env;
use std::iter::FromIterator;
use std::sync::Mutex;

#[double]
use crate::github_service::GithubService;
use crate::packages::Package;

#[derive(Deserialize)]
pub struct GithubSearchRepository {
    pub html_url: String,
}

#[derive(Deserialize)]
pub struct GithubSearchItem {
    pub url: String,
    pub path: String,
    pub repository: GithubSearchRepository,
    pub html_url: String,
}

#[derive(Deserialize)]
pub struct GithubSearchResult {
    pub items: Vec<GithubSearchItem>,
}

pub struct GithubCrawler {
    pub repo_urls: Vec<GithubSearchItem>,
    pub repos_data: Mutex<Vec<GithubRepoData>>,
    pub pool: DieselPgPool,
}

impl GithubCrawler {
    pub async fn run(mut self) {
        let mut order = "desc";
        // Github API response is unstable, need to query it multiple times to get all packages
        for page in 1..120 {
            if page == 61 {
                order = "asc";
            }
            // Github API only allows us to query first ten pages
            let url = format!(
                "https://api.github.com/search/code?q=\
                package%20in:file%20extension:toml%20filename:Move%20language:TOML&per_page=100&page={}&order={}&sort=indexed",
                (page % 10) + 1,
                order
            );
            self.find_new_repos(&url);
            let gh_service = GithubService::new();
            self.scrape(&gh_service);
            self.save_to_db();
            if page != 50 {
                delay_for(Duration::from_secs(60)).await;
            }
        }
    }

    fn find_new_repos(&mut self, url: &str) {
        let access_token = env::var("GITHUB_ACCESS_TOKEN")
            .expect("Unable to pull GITHUB_ACCESS_TOKEN for account token generation");
        let client = match reqwest::blocking::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()
        {
            Ok(client) => client,
            Err(e) => {
                error!("Error building request: {:?}", e);
                return;
            }
        };
        let response = match client
            .get(url)
            .header(header::AUTHORIZATION, format!("Bearer {}", &access_token))
            .send()
        {
            Ok(response) => response,
            Err(e) => {
                error!("Error sending request: {:?}", e);
                return;
            }
        };
        let response_json: GithubSearchResult = match response.json() {
            Ok(response_json) => response_json,
            Err(_) => GithubSearchResult { items: vec![] },
        };
        self.repo_urls = response_json.items;
        info!("Found {} packages", self.repo_urls.len());
    }

    fn scrape(&mut self, github_service: &GithubService) {
        self.repos_data = Mutex::new(vec![]);
        (0..self.repo_urls.len()).into_par_iter().for_each(|index| {
            if let Some(item) = self.repo_urls.get(index) {
                let mut no_of_trial = 3;
                loop {
                    match github_service.fetch_repo_data(
                        &item.repository.html_url,
                        Some(item.path.clone()),
                        None,
                    ) {
                        Ok(mut repo_data) => {
                            let mut guard = match self.repos_data.lock() {
                                Ok(guard) => guard,
                                Err(e) => {
                                    error!("Error acquiring guard for repo scraping: {:?}", e);
                                    return;
                                }
                            };
                            let default_branch = if repo_data.url.is_empty() {
                                "master".to_string()
                            } else {
                                repo_data.url
                            };
                            repo_data.url = if item.path == "Move.toml" {
                                // Move.toml is in top directory, get
                                item.repository.html_url.clone()
                            } else {
                                // Move.toml in subdir, so get the url to subdir, e.g:
                                // "path": "aptos-move/framework/move-stdlib/Move.toml"
                                item.html_url.replace("/Move.toml", "")
                            };
                            // Beautify the url
                            // e.g: https://github.com/alinush/aptos-core/blob/d594ba96bc97438be82755c15845410fd5c2a5e0
                            // 		/aptos-move/framework/move-stdlib/Move.toml
                            if repo_data.url.contains("/blob/") {
                                let mut repo_url_tokens =
                                    repo_data.url.split('/').collect::<Vec<&str>>();
                                if repo_url_tokens[5] == "blob" {
                                    repo_url_tokens[5] = "tree";
                                    repo_url_tokens[6] = &default_branch;
                                    repo_data.url = repo_url_tokens.join("/");
                                }
                            }
                            // e.g: "url": "https://api.github.com/repositories/467805361/contents/aptos-move/framework/move-stdlib
                            // 				/Move.toml?ref=d594ba96bc97438be82755c15845410fd5c2a5e0"
                            repo_data.rev = item.url.split("ref=").last().unwrap_or("").to_string();
                            guard.push(repo_data);
                            break;
                        }
                        Err(e) => {
                            no_of_trial -= 1;
                            if no_of_trial == 0 {
                                error!(
                                    "Cannot get package info. url: {}, error: {}",
                                    &item.repository.html_url, e
                                );
                            };
                        }
                    }
                }
            };
        });
    }

    fn save_to_db(&self) {
        let guard = match self.repos_data.lock() {
            Ok(guard) => guard,
            Err(e) => {
                error!("Error acquiring guard to save repo: {:?}", e);
                return;
            }
        };
        let repos_data: HashSet<GithubRepoData> = HashSet::from_iter(guard.iter().cloned());
        repos_data.par_iter().for_each(|repo_data| {
            let invalid_name_or_version =
                validate_version(&repo_data.version);
            if invalid_name_or_version.is_empty() {
                let _ = Package::create_from_crawled_data(
                    &repo_data.url,
                    &repo_data.description,
                    &repo_data.rev,
                    -1,
                    repo_data.size,
                    None,
                    repo_data.clone(),
                    &self.pool,
                );
            } else {
                error!(
                    "Crawler: either {} is not a valid name or {} is not a valid version. url: {}",
                    repo_data.name, repo_data.version, repo_data.url
                );
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packages::Package;
    use crate::test::{DatabaseTestContext, DB_POOL};
    use httpmock::prelude::GET;
    use httpmock::MockServer;
    use jelly::database;
    use serde_json::json;

    #[actix_rt::test]
    async fn save_to_db() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();
        let pool = database::init_database();

        let crawler = GithubCrawler {
            repo_urls: vec![],
            repos_data: Mutex::new(vec![
                GithubRepoData {
                    name: "name1".to_string(),
                    version: "0.0.0".to_string(),
                    readme_content: "readme1".to_string(),
                    license: "license1".to_string(),
                    description: "".to_string(),
                    size: 0,
                    stars_count: 0,
                    forks_count: 0,
                    url: "".to_string(),
                    rev: "".to_string(),
                },
                GithubRepoData {
                    name: "name2".to_string(),
                    version: "0.0.0".to_string(),
                    readme_content: "readme1".to_string(),
                    license: "license1".to_string(),
                    description: "".to_string(),
                    size: 0,
                    stars_count: 0,
                    forks_count: 0,
                    url: "".to_string(),
                    rev: "".to_string(),
                },
            ]),
            pool: pool.clone(),
        };
        crawler.save_to_db();
        assert_eq!(Package::count(&DB_POOL).unwrap(), 2);

        let crawler = GithubCrawler {
            repo_urls: vec![],
            repos_data: Mutex::new(vec![
                GithubRepoData {
                    name: "name3".to_string(),
                    version: "0.0.0".to_string(),
                    readme_content: "readme1".to_string(),
                    license: "license1".to_string(),
                    description: "".to_string(),
                    size: 0,
                    stars_count: 0,
                    forks_count: 0,
                    url: "".to_string(),
                    rev: "".to_string(),
                },
                GithubRepoData {
                    name: "name3".to_string(),
                    version: "0.0.0".to_string(),
                    readme_content: "readme1".to_string(),
                    license: "license1".to_string(),
                    description: "".to_string(),
                    size: 0,
                    stars_count: 0,
                    forks_count: 0,
                    url: "".to_string(),
                    rev: "".to_string(),
                },
            ]),
            pool: pool.clone(),
        };
        crawler.save_to_db();
        assert_eq!(Package::count(&DB_POOL).unwrap(), 3);

        let crawler = GithubCrawler {
            repo_urls: vec![],
            repos_data: Mutex::new(vec![
                GithubRepoData {
                    name: "valid-package-name".to_string(),
                    version: "invalid_version".to_string(),
                    readme_content: "readme1".to_string(),
                    license: "license1".to_string(),
                    description: "".to_string(),
                    size: 0,
                    stars_count: 0,
                    forks_count: 0,
                    url: "".to_string(),
                    rev: "".to_string(),
                },
            ]),
            pool,
        };
        crawler.save_to_db();
        assert_eq!(Package::count(&DB_POOL).unwrap(), 3);
    }

    #[actix_rt::test]
    async fn scrape_works() {
        dotenv::dotenv().ok();
        let _ctx = DatabaseTestContext::new();
        let pool = database::init_database();
        let mut crawler = GithubCrawler {
            repo_urls: vec![GithubSearchItem {
                repository: GithubSearchRepository {
                    html_url: "repo_url1".to_string(),
                },
                url: "".to_string(),
                path: "".to_string(),
                html_url: "".to_string(),
            }],
            repos_data: Mutex::new(vec![]),
            pool,
        };
        let mut mock_github_service = GithubService::new();
        mock_github_service
            .expect_fetch_repo_data()
            .returning(|_, _, _| {
                Ok(GithubRepoData {
                    name: "name1".to_string(),
                    version: "version1".to_string(),
                    readme_content: "readme_content1".to_string(),
                    license: "license1".to_string(),
                    description: "".to_string(),
                    size: 0,
                    stars_count: 0,
                    forks_count: 0,
                    url: "".to_string(),
                    rev: "".to_string(),
                })
            });
        crawler.scrape(&mock_github_service);
        let repo_data = crawler.repos_data.lock().unwrap();
        assert_eq!(repo_data.len(), 1);

        let stub1 = GithubRepoData {
            name: "name1".to_string(),
            version: "version1".to_string(),
            readme_content: "readme_content1".to_string(),
            license: "license1".to_string(),
            description: "".to_string(),
            size: 0,
            stars_count: 0,
            forks_count: 0,
            url: "".to_string(),
            rev: "".to_string(),
        };
        assert_eq!(repo_data[0], stub1);
    }

    #[actix_rt::test]
    async fn scrape_beautify_git_url() {
        dotenv::dotenv().ok();
        let _ctx = DatabaseTestContext::new();
        let pool = database::init_database();
        let mut crawler = GithubCrawler {
            repo_urls: vec![GithubSearchItem {
                repository: GithubSearchRepository {
                    html_url: "https://github.com/alinush/aptos-core".to_string(),
                },
                url: "".to_string(),
                path: "aptos-move/framework/move-stdlib/Move.toml".to_string(),
                html_url: "https://github.com/alinush/aptos-core/blob/xoxoxoxo/aptos-move/framework/move-stdlib/Move.toml".to_string(),
            }],
            repos_data: Mutex::new(vec![]),
            pool,
        };
        let mut mock_github_service = GithubService::new();
        mock_github_service
            .expect_fetch_repo_data()
            .returning(|_, _, _| {
                Ok(GithubRepoData {
                    name: "MoveStdlib".to_string(),
                    version: "1.5.0".to_string(),
                    readme_content: "readme_content1".to_string(),
                    license: "license1".to_string(),
                    description: "".to_string(),
                    size: 0,
                    url: "master".to_string(),
                    rev: "".to_string(),
                    stars_count: 0,
                    forks_count: 0,
                })
            });
        crawler.scrape(&mock_github_service);
        let repo_data = crawler.repos_data.lock().unwrap();
        assert_eq!(repo_data.len(), 1);

        let stub1 = GithubRepoData {
            name: "MoveStdlib".to_string(),
            version: "1.5.0".to_string(),
            readme_content: "readme_content1".to_string(),
            license: "license1".to_string(),
            description: "".to_string(),
            size: 0,
            stars_count: 0,
            forks_count: 0,
            url:
            "https://github.com/alinush/aptos-core/tree/master/aptos-move/framework/move-stdlib"
                .to_string(),
            rev: "".to_string(),
        };
        assert_eq!(repo_data[0], stub1);
        assert_eq!(repo_data[0].readme_content, stub1.readme_content);
        assert_eq!(repo_data[0].description, stub1.description);
        assert_eq!(repo_data[0].size, stub1.size);
        assert_eq!(repo_data[0].url, stub1.url);
        assert_eq!(repo_data[0].rev, stub1.rev);
    }

    #[actix_rt::test]
    async fn find_new_repos_works() {
        crate::test::init();

        let access_token = env::var("GITHUB_ACCESS_TOKEN").unwrap();
        let server = MockServer::start();
        let server_mock = server.mock(|when, then| {
            when.method(GET)
                .path_contains("/search/code")
                .header("User-Agent", APP_USER_AGENT)
                .header("authorization", format!("Bearer {}", &access_token));
            then.status(200).json_body(json!(
                {
                    "items": [
                        {
                            "url": "test url",
                            "path": "test path",
                            "repository": {
                                "html_url": "test inner html url"
                            },
                            "html_url": "test outer html url"
                        },
                    ]
                }
            ));
        });
        let mut gh_crawler = GithubCrawler {
            repo_urls: vec![],
            repos_data: Mutex::new(vec![]),
            pool: (*DB_POOL).clone(),
        };
        assert_eq!(gh_crawler.repo_urls.len(), 0);
        gh_crawler.find_new_repos(&format!("{}/search/code", server.base_url()));
        server_mock.assert();
        assert_eq!(gh_crawler.repo_urls.len(), 1);
        let repo_url = gh_crawler.repo_urls.get(0).unwrap();
        assert_eq!(&repo_url.url, "test url");
        assert_eq!(&repo_url.path, "test path");
        assert_eq!(&repo_url.repository.html_url, "test inner html url");
        assert_eq!(&repo_url.html_url, "test outer html url");
    }

    #[actix_rt::test]
    async fn find_new_repos_get_empty_repo_urls_if_response_body_mismatches() {
        crate::test::init();

        let access_token = env::var("GITHUB_ACCESS_TOKEN").unwrap();
        let server = MockServer::start();
        let server_mock = server.mock(|when, then| {
            when.method(GET)
                .path_contains("/search/code")
                .header("User-Agent", APP_USER_AGENT)
                .header("authorization", format!("Bearer {}", &access_token));
            then.status(200).json_body(json!({}));
        });
        let mut gh_crawler = GithubCrawler {
            repo_urls: vec![],
            repos_data: Mutex::new(vec![]),
            pool: (*DB_POOL).clone(),
        };
        gh_crawler.find_new_repos(&format!("{}/search/code", server.base_url()));
        server_mock.assert();
        assert_eq!(gh_crawler.repo_urls.len(), 0);
    }
}