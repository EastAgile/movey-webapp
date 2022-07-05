//! Your Service Description here, etc.

#[cfg(not(feature = "test"))]
use std::env;
use std::io;
#[cfg(not(feature = "test"))]
use std::sync::Mutex;
use jelly::actix_web::dev;

#[macro_use]
extern crate diesel;

extern crate diesel_full_text_search;

#[macro_use]
extern crate log;

#[macro_use]
extern crate diesel_migrations;
extern crate core;

pub mod accounts;
pub mod api;
pub mod dashboard;
pub mod github_service;
pub mod packages;
pub mod pages;
pub mod settings;
pub mod policy;
mod utils;

pub mod schema;

pub mod request;
pub mod test;
pub mod jobs;

use jelly::{DieselPgPool, Server};

#[cfg(not(feature = "test"))]
pub async fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let _lock = stdout.lock();
	dotenv::dotenv().ok();
    let sentry_url = env::var("SENTRY_URL").unwrap_or_else(|_| "".to_string());
    let _guard = sentry::init((sentry_url, sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    }));
    let (server, pool) = start_server().await?;
    let is_crawling = env::var("CRAWLING").unwrap_or_else(|_| "".to_string());
    if is_crawling.to_lowercase() == "true" {
        actix_rt::spawn(async {
            let gh_crawler = jobs::GithubCrawler {
                repo_urls: vec![],
                repos_data: Mutex::new(vec![]),
                pool,
            };
            gh_crawler.run().await;
        });
    }
    server.await
}

#[cfg(feature = "test")]
pub async fn main() -> io::Result<()> {
    let (server, _) = start_server().await?;
    server.await
}

async fn start_server() -> io::Result<(dev::Server, DieselPgPool)> {
    Server::new()
        .register_service(pages::configure)
        .register_service(accounts::configure)
        .register_jobs(accounts::jobs::configure)
        .register_service(packages::configure)
        .register_service(dashboard::configure)
        .register_service(api::configure)
        .register_service(settings::configure)
        .register_service(policy::configure)
        .run()
        .await
}
