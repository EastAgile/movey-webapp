use jelly::actix_web::{web::Path, HttpRequest};
use jelly::prelude::*;
use jelly::request::DatabasePool;
use jelly::Result;

use crate::packages::Package;

pub async fn show_package(
    request: HttpRequest,
    Path(package_id): Path<String>,
) -> Result<HttpResponse> {
    let db = request.db_pool()?;
    let uid = package_id.parse::<i32>().unwrap();
    let package = Package::get(uid, &db).await.unwrap();
    let package_latest_version = &package.get_versions(&db).await.unwrap()[0];

    return request.render(200, "packages/show.html", {
        let mut ctx = Context::new();
        ctx.insert("package", &package);
        ctx.insert("package_version", &package_latest_version);
        ctx
    });
}
