use crate::packages::models::Package;
use crate::settings::models::token::ApiToken;
use jelly::actix_web::web;
use jelly::prelude::*;
use jelly::Result;
use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::test::mock::MockHttpRequest as HttpRequest;
#[cfg(not(test))]
use jelly::actix_web::HttpRequest;

#[cfg(not(test))]
use crate::github_service::GithubService;
#[cfg(test)]
use crate::test::mock::GithubService;

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
    if ApiToken::get(&req.token, db).await.is_err() {
        return Ok(HttpResponse::BadRequest().body("Invalid Api Token"));
    }

    let account_id = ApiToken::associated_account(&req.token, db).await?.id;
    let service = GithubService::new();
    if req.subdir.ends_with('\n') {
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
        Some(account_id),
        github_data,
        db,
    )
    .await?;

    Ok(HttpResponse::Ok().body(""))
}

#[derive(Clone, Deserialize)]
pub struct DownloadInfo {
    url: String,
    rev: String,
    subdir: String,
}

impl Validation for DownloadInfo {
    fn is_valid(&mut self) -> bool {
        !self.url.is_empty() && !self.rev.is_empty()
    }
}

pub async fn increase_download_count(
    request: HttpRequest,
    form: web::Form<DownloadInfo>,
) -> Result<HttpResponse> {
    let mut form = form.into_inner();
    if !form.is_valid() {
        return Ok(HttpResponse::BadRequest().body("invalid git info"));
    }
    let db = request.db_pool()?;
    let service = GithubService::new();
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
    let packages = Package::auto_complete_search(&res.search_query, db).await?;
    Ok(HttpResponse::Ok().json(packages))
}

#[cfg(test)]
mod tests;
