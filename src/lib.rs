//! Your Service Description here, etc.

use std::env;
use std::io;
use jelly::actix_web::dev;

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
pub mod pages;
pub mod github_service;
pub mod packages;
pub mod policy;
mod utils;

pub mod schema;

pub mod request;
pub mod test;
pub mod setting;

use jelly::Server;

pub async fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let _lock = stdout.lock();

    dotenv::dotenv().ok();
    let sentry_url = env::var("SENTRY_URL").unwrap_or_else(|_| "".to_string());
    let _guard = sentry::init((sentry_url, sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    }));

    Server::new()
        .register_service(pages::configure)
        .register_service(accounts::configure)
        .register_jobs(accounts::jobs::configure)
        .register_service(packages::configure)
        .register_service(dashboard::configure)
        .register_service(api::configure)
        .register_service(setting::configure)
        .register_service(policy::configure)
        .run()
        .await?
        .await
}

#[cfg(feature = "test")]
pub async fn test_main() -> dev::Server {
    Server::new()
        .register_service(pages::configure)
        .register_service(accounts::configure)
        .register_jobs(accounts::jobs::configure)
        .register_service(packages::configure)
        .register_service(dashboard::configure)
        .register_service(api::configure)
        .run()
        .await
        .unwrap()
}
