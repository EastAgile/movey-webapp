//! Your Service Description here, etc.

use jelly::actix_web::dev;
#[cfg(not(feature = "test"))]
use std::env;
use std::io;
#[cfg(not(feature = "test"))]
use std::sync::Mutex;

#[macro_use]
extern crate diesel;

extern crate diesel_full_text_search;

#[macro_use]
extern crate log;

#[macro_use]
extern crate diesel_migrations;

pub mod accounts;
pub mod api;
pub mod dashboard;
pub mod github_service;
pub mod packages;
pub mod pages;
pub mod policy;
pub mod settings;
mod utils;

pub mod schema;

pub mod jobs;
pub mod request;
pub mod test;

use jelly::{DieselPgPool, Server};

#[cfg(not(feature = "test"))]
pub async fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let _lock = stdout.lock();
    dotenv::dotenv().ok();
    let sentry_url = env::var("SENTRY_URL").unwrap_or_else(|_| {
        warn!("No sentry URL set, skipped");
        "".to_string()
    });
    let sentry_environment =
        env::var("SENTRY_ALERT_ENVIRONMENT").unwrap_or_else(|_| "STAGING".to_string());
    let _guard = sentry::init((
        sentry_url,
        sentry::ClientOptions {
            environment: Some(sentry_environment.into()),
            ..Default::default()
        },
    ));
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
