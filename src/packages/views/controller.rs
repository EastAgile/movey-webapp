use convert_case::{Case, Casing};
use jelly::actix_web::{web::Path, web::Query, HttpRequest};
use jelly::forms::TextField;
use jelly::prelude::*;
use jelly::request::DatabasePool;
use jelly::Result;

use crate::packages::{Package, PackageVersion, PackageVersionSort};
use crate::packages::models::{PackageSortField, PackageSortOrder};

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
pub struct PackageSearchParams {
    pub query: TextField,
    pub field: Option<PackageSortField>,
    pub order: Option<PackageSortOrder>,
    pub page: Option<i64>,
}

pub async fn show_search_results(
    request: HttpRequest,
    mut search: Query<PackageSearchParams>
) -> Result<HttpResponse> {
    let db = request.db_pool()?;
    if let None = search.field {
        search.field = Some(PackageSortField::Name);
    }
    if let None = search.order {
        search.order = Some(PackageSortOrder::Desc);
    }
    let (packages, total_count, total_pages) = Package::search(
        &search.query.value,
        &search.field.as_ref().unwrap(),
        &search.order.as_ref().unwrap(),
        search.page,
        None,
        &db).await.unwrap();

    let current_page = search.page.unwrap_or_else(|| 1);
    let field_name = match &search.field {
        Some(f) => f.to_string().to_case(Case::Snake),
        None => "".to_string()
    };

    request.render(200, "search/search_results.html", {
        let mut ctx = Context::new();
        ctx.insert("query", &search.query.value);
        ctx.insert("sort_type", &field_name);
        ctx.insert("current_page", &current_page);
        ctx.insert("packages", &packages);
        ctx.insert("total_count", &total_count);
        ctx.insert("total_pages", &total_pages);
        ctx
    })
}
