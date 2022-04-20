use jelly::actix_web::{web::Path, web::Query, HttpRequest};
use jelly::prelude::*;
use jelly::request::DatabasePool;
use jelly::Result;

use crate::packages::{Package, PackageSort, PackageVersion, PackageVersionSort};

#[derive(serde::Serialize, serde::Deserialize)]
struct PackageShowParams {
    version: Option<String>
}

pub async fn show_package(
    request: HttpRequest,
    Path(package_name): Path<String>,
) -> Result<HttpResponse> {
    let db = request.db_pool()?;
    let package = Package::get_by_name(&package_name, &db).await.unwrap();

    let default_version: String = String::from("");
    let params = Query::<PackageShowParams>::from_query(request.query_string()).unwrap();
    let version: &String = params.version.as_ref().unwrap_or(&default_version);

    let package_version: PackageVersion;

    if version == "" {
        let versions = PackageVersion::from_package_id(package.id, &PackageVersionSort::Latest, &db).await.unwrap();
        package_version = versions[0].clone();
    } else {
        package_version = package.get_version(version, &db).await.unwrap().clone()
    }

    return request.render(200, "packages/show.html", {
        let mut ctx = Context::new();
        ctx.insert("package", &package);
        ctx.insert("package_version", &package_version);
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
    let sort_type_text: &str = params.sort_type.as_ref().unwrap_or(&default_sort);

    let sort_type = match sort_type_text {
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

#[derive(serde::Serialize, serde::Deserialize)]
struct PackageSearchParams {
    sort_type: Option<String>,
    query: Option<String>,
}

pub async fn show_search_results(
    request: HttpRequest,
) -> Result<HttpResponse> {
    let db = request.db_pool()?;
    let params = Query::<PackageSearchParams>::from_query(request.query_string()).unwrap();
    let default_sort = String::from("name");
    let mut sort_type_text = params.sort_type.as_ref().unwrap_or(&default_sort);
    let sort_type = match sort_type_text.as_str() {
        // "description" => PackageSort::Description,
        // "summary" => PackageSort::Summary,
        "most_downloads" => PackageSort::MostDownloads,
        "newly_added" => PackageSort::NewlyAdded,
        _ => {
            sort_type_text = &default_sort;
            PackageSort::Name
        }
    };
    let default_query = String::from("untitled");
    let query_text = params.query.as_ref().unwrap_or(&default_query);
    let package_list = Package::search_by_name(query_text, &sort_type, &db).await.unwrap();

    request.render(200, "search/search_results.html", {
        let mut ctx = Context::new();
        ctx.insert("query", &query_text);
        ctx.insert("sort_type", &sort_type_text);
        ctx.insert("packages", &package_list);
        ctx.insert("package_count", &package_list.len());
        ctx
    })
}
