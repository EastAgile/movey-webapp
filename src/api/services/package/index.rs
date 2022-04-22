use jelly::actix_web::{web, HttpRequest};
use jelly::prelude::*;
use jelly::Result;
use mockall_double::double;
use serde::{Serialize, Deserialize};

use crate::packages::Package;

#[double] use crate::github_service::GithubService;

#[derive(Serialize, Deserialize)]
pub struct PackageRequest {
    github_repo_url: String,
    description: String,
    rev: String,
    total_files: i32,
    total_size: i32,
}

pub async fn post_package(request: HttpRequest, res: web::Json<PackageRequest>) -> Result<HttpResponse> {
    let db = request.db_pool()?;
    let service = GithubService::new();
    Package::create(&res.github_repo_url, &res.description, &res.rev, res.total_files, res.total_size,&service, &db).await?;

    Ok(HttpResponse::Ok().body(""))
}
// ,&total_files, &total_size