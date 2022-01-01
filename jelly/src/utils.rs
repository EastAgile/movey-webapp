//! Implements some framework-level pieces, primarily useful in debugging scenarios.

use actix_web::web::ServiceConfig;
use actix_web::Result;
use actix_web::{HttpRequest, HttpResponse};
use tera::Context;

use crate::error::Error;
use crate::request::Render;

/// Shorthand method for throwing a big ol' 404.
#[inline(always)]
pub async fn not_found(request: HttpRequest) -> Result<HttpResponse, Error> {
    request.render(404, "404.html", Context::new())
}

/// Enables serving static files.
#[cfg(feature = "static")]
pub fn static_handler(config: &mut ServiceConfig) {
    let static_path =
        std::env::var("STATIC_ROOT").expect("Running in debug without STATIC_ROOT set!");

    let fs = actix_files::Files::new("/static", &static_path);
    config.service(fs);
}

/// A noop static handler for production usage.
#[cfg(not(feature = "static"))]
pub fn static_handler(_config: &mut ServiceConfig) {}
