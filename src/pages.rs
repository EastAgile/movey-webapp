use crate::packages::{Package, PackageVersion};
use jelly::actix_web::web::{resource, ServiceConfig};
use jelly::prelude::*;
use jelly::Result;
use std::env;

pub async fn homepage(request: HttpRequest) -> Result<HttpResponse> {
    dotenv::dotenv().ok();
    let redirect_host = env::var("REDIRECT_HOST").unwrap_or_else(|_| "www.movey.org".to_string());
    if request.connection_info().host() == redirect_host {
        return request.redirect("https://www.movey.net");
    }

    request.render(200, "home.html", {
        let db = request.db_pool()?;
        let package_count = Package::count(db)?;
        let package_version_count = PackageVersion::count(db)?;

        let mut context = Context::new();
        context.insert("package_count", &package_count);
        context.insert("package_version_count", &package_version_count);
        context
    })
}

pub fn configure(config: &mut ServiceConfig) {
    config.service(resource("/").to(homepage));
}
