use jelly::actix_web::{web, HttpRequest};
use jelly::prelude::*;
use jelly::Result;
use serde::{Serialize, Deserialize};

use crate::packages::Package;

#[derive(Serialize, Deserialize)]
pub struct PackageRequest {
    github_repo_url: String,
    description: String
}

pub async fn post_package(request: HttpRequest, res: web::Json<PackageRequest>) -> Result<HttpResponse> {
    let db = request.db_pool()?;
    Package::create(&res.github_repo_url, &res.description, &db).await?;

    Ok(HttpResponse::Ok().body(""))
}
