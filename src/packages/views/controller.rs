use jelly::actix_web::{web::Path, web::Query, HttpRequest};
use jelly::anyhow::anyhow;
use jelly::forms::TextField;
use jelly::prelude::*;
use jelly::request::DatabasePool;
use jelly::Result;

use crate::accounts::Account;
use crate::packages::models::{PackageSortField, PackageSortOrder, PACKAGES_PER_PAGE};
use crate::packages::{Package, PackageVersion, PackageVersionSort};

#[derive(serde::Serialize, serde::Deserialize)]
struct PackageShowParams {
    version: Option<String>,
}

pub async fn show_package(
    request: HttpRequest,
    Path(package_name): Path<String>,
) -> Result<HttpResponse> {
    let db = request.db_pool()?;
    let package = Package::get_by_name(&package_name, db).await?;

    let default_version: String = String::from("");
    let params = Query::<PackageShowParams>::from_query(request.query_string())
        .map_err(|e| Error::Generic(format!("Error getting query params: {:?}", e)))?;
    let version: &String = params.version.as_ref().unwrap_or(&default_version);

    let package_version: PackageVersion;

    if version.is_empty() {
        let versions =
            PackageVersion::from_package_id(package.id, &PackageVersionSort::Latest, db).await?;
        package_version = versions[0].clone();
    } else {
        package_version = package.get_version(version, db).await?
    }

    let account_name = if let Some(uid) = package.account_id {
        let account = Account::get(uid, db).await?;
        if account.name.is_empty() {
            account.email
        } else {
            account.name
        }
    } else {
        "".to_string()
    };

    request.render(200, "packages/show.html", {
        let mut ctx = Context::new();
        ctx.insert("package", &package);
        ctx.insert("package_version", &package_version);
        ctx.insert("account_name", &account_name);
        ctx.insert("package_tab", "readme");
        ctx
    })
}

#[derive(serde::Serialize, serde::Deserialize)]
struct VersionParams {
    sort_type: Option<String>,
}

pub async fn show_package_versions(
    request: HttpRequest,
    Path(package_name): Path<String>,
) -> Result<HttpResponse> {
    let db = request.db_pool()?;
    let package = Package::get_by_name(&package_name, db).await?;
    let package_latest_version =
        &PackageVersion::from_package_id(package.id, &PackageVersionSort::Latest, db).await?[0];

    let params = Query::<VersionParams>::from_query(request.query_string()).map_err(|e| {
        error!("Error parsing params: {:?}", e);
        anyhow!("Error parsing params: {:?}", e)
    })?;
    let default_sort: String = String::from("latest");
    let sort_type_text: &str = params.sort_type.as_ref().unwrap_or(&default_sort);

    let sort_type = match sort_type_text {
        "oldest" => PackageVersionSort::Oldest,
        "most_downloads" => PackageVersionSort::MostDownloads,
        _ => PackageVersionSort::Latest,
    };
    let package_versions = PackageVersion::from_package_id(package.id, &sort_type, db).await?;

    request.render(200, "packages/versions.html", {
        let mut ctx = Context::new();
        ctx.insert("package", &package);
        ctx.insert("package_version", &package_latest_version);
        ctx.insert("versions", &package_versions);
        ctx.insert("package_tab", "versions");
        ctx.insert("sort_type", &sort_type_text);
        ctx
    })
}

pub async fn show_package_settings(
    request: HttpRequest,
    Path(package_name): Path<String>,
) -> Result<HttpResponse> {
    let db = request.db_pool()?;
    let package = Package::get_by_name(&package_name, db).await?;
    let package_latest_version =
        &PackageVersion::from_package_id(package.id, &PackageVersionSort::Latest, db).await?[0];

    
    request.render(200, "packages/owner_settings.html", {
        let mut ctx = Context::new();
        ctx.insert("package", &package);
        ctx.insert("package_tab", "settings");
        ctx.insert("package_version", &package_latest_version);
        ctx
    })
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
    mut search: Query<PackageSearchParams>,
) -> Result<HttpResponse> {
    let db = request.db_pool()?;
    if search.field.is_none() {
        search.field = Some(PackageSortField::Name);
    }
    if search.order.is_none() {
        search.order = if let Some(PackageSortField::Name) = search.field {
            Some(PackageSortOrder::Asc)
        } else {
            Some(PackageSortOrder::Desc)
        }
    }
    let (packages, total_count, total_pages) = Package::search(
        &search.query.value,
        search.field.as_ref().unwrap(),
        search.order.as_ref().unwrap(),
        search.page,
        None,
        db,
    )
    .await?;

    let current_page = search.page.unwrap_or(1);
    if current_page < 1 {
        return Err(Error::Generic(String::from("Invalid page number.")));
    }
    let field_name = match &search.field {
        Some(f) => f.to_string(),
        None => "".to_string(),
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

pub async fn show_owned_packages(request: HttpRequest) -> Result<HttpResponse> {
    let db = request.db_pool()?;
    if let Ok(user) = request.user() {
        let packages = Package::get_by_account(user.id, db).await?;

        request.render(200, "search/search_results.html", {
            let mut ctx = Context::new();
            ctx.insert("packages", &packages);
            ctx
        })
    } else {
        Ok(HttpResponse::NotFound().body("Cannot find user"))
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PackageIndexParams {
    pub field: Option<PackageSortField>,
    pub order: Option<PackageSortOrder>,
    pub page: Option<i64>,
}

pub async fn packages_index(
    request: HttpRequest,
    mut params: Query<PackageIndexParams>,
) -> Result<HttpResponse> {
    let db = request.db_pool()?;
    if params.field.is_none() {
        params.field = Some(PackageSortField::Name);
    }
    if params.order.is_none() {
        params.order = if let Some(PackageSortField::Name) = params.field {
            Some(PackageSortOrder::Asc)
        } else {
            Some(PackageSortOrder::Desc)
        }
    }
    let (packages, total_count, total_pages) = Package::all_packages(
        params.field.as_ref().unwrap(),
        params.order.as_ref().unwrap(),
        params.page,
        None,
        db,
    )
    .await?;

    let current_page = params.page.unwrap_or(1);
    if current_page < 1 {
        return Err(Error::Generic(String::from("Invalid page number.")));
    }
    let field_name = match &params.field {
        Some(f) => f.to_string(),
        None => "".to_string(),
    };
    let display_pagination_start = (current_page - 1) * PACKAGES_PER_PAGE + 1;
    let display_pagination_end: usize = (display_pagination_start as usize) + packages.len() - 1;

    request.render(200, "packages/index.html", {
        let mut ctx = Context::new();
        ctx.insert("sort_type", &field_name);
        ctx.insert("current_page", &current_page);
        ctx.insert("packages", &packages);
        ctx.insert("display_pagination_start", &display_pagination_start);
        ctx.insert("display_pagination_end", &display_pagination_end);
        ctx.insert("total_count", &total_count);
        ctx.insert("total_pages", &total_pages);
        ctx
    })
}
