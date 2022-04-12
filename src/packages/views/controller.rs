use jelly::actix_web::{web::Path, web::Query, HttpRequest};
use jelly::prelude::*;
use jelly::request::DatabasePool;
use jelly::Result;

use crate::packages::{Package, PackageVersion, PackageVersionSort};

pub async fn show_package(
    request: HttpRequest,
    Path(package_name): Path<String>,
) -> Result<HttpResponse> {
    let db = request.db_pool()?;
    let package = Package::get_by_name(&package_name, &db).await.unwrap();
    let package_latest_version = &PackageVersion::from_package_id(package.id, &PackageVersionSort::Latest, &db).await.unwrap()[0];

    return request.render(200, "packages/show.html", {
        let mut ctx = Context::new();
        ctx.insert("package", &package);
        ctx.insert("package_version", &package_latest_version);
        ctx.insert("package_tab", "readme");
        ctx
    });
}

#[derive(serde::Serialize, serde::Deserialize)]
struct VersionParams {
    sort_type: Option<String>
}

pub async fn show_package_versions(
    request: HttpRequest,
    Path(package_name): Path<String>,
) -> Result<HttpResponse> {
    let db = request.db_pool()?;
    let package = Package::get_by_name(&package_name, &db).await.unwrap();
    let package_latest_version = &PackageVersion::from_package_id(package.id, &PackageVersionSort::Latest, &db).await.unwrap()[0];

    let params = Query::<VersionParams>::from_query(request.query_string()).unwrap();
    let default_sort: String = String::from("latest");
    let sort_type_text: &String = params.sort_type.as_ref().unwrap_or(&default_sort);

    let sort_type = match sort_type_text.as_str() {
        "oldest" => PackageVersionSort::Oldest,
        "most_downloads" => PackageVersionSort::MostDownloads,
        _ => PackageVersionSort::Latest
    };
    let package_versions = PackageVersion::from_package_id(package.id, &sort_type, &db).await.unwrap();

    return request.render(200, "packages/versions.html", {
        let mut ctx = Context::new();
        ctx.insert("package", &package);
        ctx.insert("package_version", &package_latest_version);
        ctx.insert("versions", &package_versions);
        ctx.insert("package_tab", "versions");
        ctx.insert("sort_type", &sort_type_text);
        ctx
    });
}
