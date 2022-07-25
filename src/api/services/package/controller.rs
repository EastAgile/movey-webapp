use diesel::prelude::*;
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, RunQueryDsl};
use jelly::actix_web::{web, web::Path, HttpRequest};
use jelly::prelude::*;
use jelly::DieselPgPool;
use jelly::Result;
use mockall_double::double;
use serde::{Deserialize, Serialize};

use crate::api::services::package::model::PackageBadgeRespond;
#[double]
use crate::github_service::GithubService;
use crate::packages::models::PACKAGE_COLUMNS;
use crate::packages::{Package, PackageVersion};
use crate::schema::package_versions;
use crate::schema::package_versions::dsl::*;
use crate::schema::packages;
use crate::schema::packages::dsl::*;
use crate::settings::models::token::ApiToken;

#[derive(Serialize, Deserialize)]
pub struct PackageRequest {
    github_repo_url: String,
    total_files: i32,
    token: String,
    subdir: String,
}

#[derive(Serialize, Deserialize)]
pub struct PackageSearch {
    search_query: String,
}

pub async fn post_package(
    request: HttpRequest,
    mut req: web::Json<PackageRequest>,
) -> Result<HttpResponse> {
    let db = request.db_pool()?;
    if let Err(_) = ApiToken::get(&req.token, db).await {
        return Ok(HttpResponse::BadRequest().body("Invalid Api Token"));
    }

    let token_account_id = ApiToken::associated_account(&req.token, &db).await?.id;
    let service = GithubService::new();
    if req.subdir.ends_with("\n") {
        req.subdir.pop();
    };
    let subdir = if req.subdir.is_empty() {
        None
    } else {
        let mut subdir = req.subdir.clone();
        subdir.push_str("/Move.toml");
        Some(subdir)
    };
    let github_data = service.fetch_repo_data(&req.github_repo_url, subdir, None)?;
    if !req.subdir.is_empty() {
        req.github_repo_url = format!(
            "{}/blob/{}/{}",
            req.github_repo_url, github_data.rev, req.subdir
        );
    }
    Package::create_from_crawled_data(
        &req.github_repo_url,
        &github_data.description.clone(),
        &github_data.rev.clone(),
        req.total_files,
        github_data.size,
        Some(token_account_id),
        github_data,
        &db,
    )
    .await?;

    Ok(HttpResponse::Ok().body(""))
}

#[derive(Deserialize)]
pub struct DownloadInfo {
    url: String,
    rev: String,
    subdir: String,
}

pub async fn increase_download_count(
    request: HttpRequest,
    form: web::Form<DownloadInfo>,
) -> Result<HttpResponse> {
    let db = request.db_pool()?;
    let service = GithubService::new();
    let form = form.into_inner();
    if let Ok(res) =
        Package::increase_download_count(&form.url, &form.rev, &form.subdir, &service, &db).await
    {
        Ok(HttpResponse::Ok().body(res.to_string()))
    } else {
        Ok(HttpResponse::NotFound().body("Cannot find url or rev"))
    }
}

pub async fn search_package(
    request: HttpRequest,
    res: web::Json<PackageSearch>,
) -> Result<HttpResponse> {
    let db = request.db_pool()?;
    let packages_result = Package::auto_complete_search(&res.search_query, &db).await?;
    Ok(HttpResponse::Ok().json(packages_result))
}

pub async fn package_badge_info(
    request: HttpRequest,
    Path(pkg_name): Path<String>,
) -> Result<HttpResponse> {
    let connection = request.db_pool()?.get()?;
    let result: Vec<(String, i32, String, i32)> = packages::table
        .inner_join(package_versions::table)
        .filter(packages::name.eq(pkg_name))
        .filter(diesel::dsl::sql(
            "TRUE GROUP BY packages.name, packages.total_downloads_count, package_versions.version, package_versions.downloads_count",
        ))
        .select((
            packages::name,
            packages::total_downloads_count,
            package_versions::version,
            package_versions::downloads_count,
        ))
        .load::<(String, i32, String, i32)>(&connection)?;
    if result.len() > 0 {
        let respond = PackageBadgeRespond::from(result);
        return Ok(HttpResponse::Ok().json(respond));
    }
    Ok(HttpResponse::NotFound().finish())
}
