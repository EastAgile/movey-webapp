use jelly::actix_web::{web, HttpRequest};
use jelly::prelude::*;
use jelly::Result;
use mockall_double::double;
use serde::{Deserialize, Serialize};

use crate::packages::Package;
use crate::settings::models::token::ApiToken;

#[double]
use crate::github_service::GithubService;

#[derive(Serialize, Deserialize)]
pub struct PackageRequest {
    github_repo_url: String,
    description: String,
    rev: String,
    total_files: i32,
    total_size: i32,
    token: String,
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
    if let Err(_) = ApiToken::get(&res.token, db).await {
        return Ok(HttpResponse::BadRequest().body("Invalid Api Token"));
    }

    let account_id = ApiToken::associated_account(&res.token, &db).await?.id;

    Package::create(
        &res.github_repo_url,
        &res.description,
        &res.rev,
        res.total_files,
        res.total_size,
        Some(account_id),
        &service,
        None,
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

pub async fn increase_download_count(request: HttpRequest, form: web::Form<DownloadInfo>) -> Result<HttpResponse> {
    let db = request.db_pool()?;
    let service = GithubService::new();
    let form = form.into_inner();
    if let Ok(res) = Package::increase_download_count(
        &form.url, &form.rev, &form.subdir, &service, &db
    ).await {
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
