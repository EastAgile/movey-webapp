use jelly::actix_web::{web, HttpRequest};
use jelly::prelude::*;
use jelly::Result;
use mockall_double::double;
use serde::{Deserialize, Serialize};

use crate::packages::Package;

#[double]
use crate::github_service::GithubService;

#[derive(Serialize, Deserialize)]
pub struct PackageRequest {
    github_repo_url: String,
    description: String,
    rev: String,
    total_files: i32,
    total_size: i32,
}

#[derive(Serialize, Deserialize)]
pub struct PackageSearch {
    search_query: String,
}

pub async fn post_package(
    request: HttpRequest,
    res: web::Json<PackageRequest>,
) -> Result<HttpResponse> {
    let db = request.db_pool()?;
    let service = GithubService::new();
    Package::create(
        &res.github_repo_url,
        &res.description,
        &res.rev,
        res.total_files,
        res.total_size,
        &service,
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

pub async fn increment_download(request: HttpRequest, query: web::Query<DownloadInfo>) -> Result<HttpResponse> {
    let db = request.db_pool()?;
    let service = GithubService::new();
    let query = query.into_inner();
    let url = query.url;
    let rev_ = query.rev;
    let _subdir = query.subdir;

    if let Ok(res) = Package::increment_download(&url, &rev_, &service, &db).await {
        Ok(HttpResponse::Ok().body(res.to_string()))
    }
    else {
        Ok(HttpResponse::NotFound().body("Cannot find url or rev"))
    }
}

pub async fn search_package(
    request: HttpRequest,
    res: web::Json<PackageSearch>,
) -> Result<HttpResponse> {
    let db = request.db_pool()?;
    let packages = Package::auto_complete_search(&res.search_query, &db).await?;
    Ok(HttpResponse::Ok().json(packages))
}

