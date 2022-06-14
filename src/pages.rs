use crate::packages::{Package, PackageVersion};
use jelly::actix_web::web::{resource, ServiceConfig};
use jelly::prelude::*;
use jelly::Result;

pub async fn homepage(request: HttpRequest) -> Result<HttpResponse> {
    request.render(200, "home.html", {
        let db = request.db_pool()?;
        let package_count = Package::count(db).await?;
        let package_version_count = PackageVersion::count(db).await?;

        let mut context = Context::new();
        context.insert("package_count", &package_count);
        context.insert("package_version_count", &package_version_count);
        context
    })
}

pub fn configure(config: &mut ServiceConfig) {
    config.service(resource("/").to(homepage));
}
