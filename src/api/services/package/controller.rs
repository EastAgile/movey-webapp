#[cfg(test)]
use crate::test::mock::MockHttpRequest as HttpRequest;
#[cfg(not(test))]
use jelly::actix_web::HttpRequest;

use diesel::result::DatabaseErrorKind;
use diesel::result::Error as DBError;
use jelly::actix_web::web;
use jelly::prelude::*;
use jelly::Result;
use serde::{Deserialize, Serialize};

#[cfg(not(test))]
use crate::github_service::GithubService;
#[cfg(test)]
use crate::test::mock::GithubService;

use crate::api::services::package::view::PackageBadgeRespond;
use crate::packages::Package;
use crate::settings::models::token::ApiToken;
use crate::utils::presenter::validate_name_and_version;

#[derive(Serialize, Deserialize)]
pub struct PackageRequest {
    pub github_repo_url: String,
    pub total_files: i32,
    pub token: String,
    pub subdir: String,
}

#[derive(Serialize, Deserialize)]
pub struct PackageSearch {
    search_query: String,
}

pub async fn register_package(
    request: HttpRequest,
    mut req: web::Json<PackageRequest>,
) -> Result<HttpResponse> {
    let db = match request.db_pool() {
        Ok(pool) => pool,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError()
                .body("Something went wrong, please try again later."))
        }
    };
    if ApiToken::get(&req.token, db).await.is_err() {
        return Ok(HttpResponse::BadRequest().body("Invalid API token."));
    }

    let token_account_id = match ApiToken::associated_account(&req.token, db).await {
        Ok(account) => account.id,
        Err(_) => return Ok(HttpResponse::BadRequest().body("Invalid API token.")),
    };
    let service = GithubService::new();
    if req.subdir.ends_with('\n') {
        req.subdir.pop();
    };
    let subdir = if req.subdir.is_empty() {
        None
    } else {
        let mut subdir = req.subdir.clone();
        subdir.push_str("Move.toml");
        Some(subdir)
    };
    let github_data = match service.fetch_repo_data(&req.github_repo_url, subdir, None) {
        Ok(data) => data,
        Err(_) => return Ok(HttpResponse::NotFound().body("Cannot get package info from Github.")),
    };
    if !req.subdir.is_empty() {
        req.github_repo_url = format!(
            "{}/blob/{}/{}",
            req.github_repo_url, github_data.rev, req.subdir
        );
    }
    let hints = validate_name_and_version(&github_data.name, &github_data.version);
    if !hints.is_empty() {
        return Ok(HttpResponse::BadRequest().body(format!(
            "Cannot upload package.\nHints: {}.",
            hints.join("; ")
        )));
    }

    let package_name = github_data.name.clone();
    let result = Package::create_from_crawled_data(
        &req.github_repo_url,
        &github_data.description.clone(),
        &github_data.rev.clone(),
        req.total_files,
        github_data.size,
        Some(token_account_id),
        github_data,
        db,
    )
    .await;
    if let Err(Error::Database(DBError::DatabaseError(kind, _))) = result {
        let domain = std::env::var("JELLY_DOMAIN").expect("JELLY_DOMAIN is not set");
        let error_message = format!(
            "Cannot upload package.\n{}",
            match kind {
                DatabaseErrorKind::UniqueViolation => format!(
                    "Version already exists for package at {domain}/packages/{package_name}. \
                    Please commit your changes to Github and try again."
                ),
                _ => "Something went wrong, please try again later.".to_string(),
            }
        );
        return Ok(HttpResponse::BadRequest().body(error_message));
    }

    Ok(HttpResponse::Ok().body(package_name))
}

#[derive(Clone, Deserialize)]
pub struct DownloadInfo {
    pub url: String,
    pub rev: String,
    pub subdir: String,
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
        return Ok(HttpResponse::BadRequest().body("Invalid git info."));
    }

    let db = match request.db_pool() {
        Ok(pool) => pool,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError()
                .body("Something went wrong, please try again later."))
        }
    };
    let service = GithubService::new();
    if let Ok(res) =
        Package::increase_download_count(&form.url, &form.rev, &form.subdir, &service, db).await
    {
        Ok(HttpResponse::Ok().body(res.to_string()))
    } else {
        Ok(HttpResponse::NotFound().body("Cannot find url or rev."))
    }
}

pub async fn search_package(
    request: HttpRequest,
    res: web::Json<PackageSearch>,
) -> Result<HttpResponse> {
    let db = request.db_pool()?;
    let packages_result = Package::auto_complete_search(&res.search_query, db).await?;
    Ok(HttpResponse::Ok().json(packages_result))
}

#[derive(Deserialize)]
pub struct BadgeRequest {
    pkg_name: String,
}

pub async fn package_badge_info(
    request: HttpRequest,
    info: web::Query<BadgeRequest>,
) -> Result<HttpResponse> {
    let info = info.into_inner();
    let db = request.db_pool()?;
    let result = Package::get_badge_info(&info.pkg_name, db).await?;
    if !result.is_empty() {
        let respond = PackageBadgeRespond::from(result);
        return Ok(HttpResponse::Ok().json(respond));
    }
    Ok(HttpResponse::NotFound().finish())
}
