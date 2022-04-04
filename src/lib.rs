//! Your Service Description here, etc.

use std::io;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate log;

#[macro_use]
extern crate diesel_migrations;

pub mod accounts;
pub mod dashboard;
pub mod pages;
pub mod api;
pub mod packages;

pub mod schema;

pub mod test;

use jelly::Server;

pub async fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let _lock = stdout.lock();

    Server::new()
        .register_service(pages::configure)
        .register_service(accounts::configure)
        .register_jobs(accounts::jobs::configure)
        .register_service(dashboard::configure)
        .register_service(api::configure)
        .run()
        .await?
        .await
}
