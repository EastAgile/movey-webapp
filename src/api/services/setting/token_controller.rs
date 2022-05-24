use jelly::actix_web::{web, HttpRequest};
use jelly::prelude::*;
use jelly::Result;
use serde::{Deserialize, Serialize};
use crate::accounts::Account;
use crate::request;
use crate::setting::models::token::ApiToken;
use crate::diesel::query_dsl::BelongingToDsl;
use crate::diesel::QueryDsl;
use diesel::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct PackageRequest {
    pub name: String
}

pub async fn create_token(
    request: HttpRequest,
    res: web::Json<PackageRequest>,
) -> Result<HttpResponse> {
    if request::is_authenticated(&request).await? {
        return Ok(HttpResponse::Unauthorized().body(""));
    }
    let db = request.db_pool()?;
    let user = request.user()?;
    let account = Account::get(user.id, db).await?;

    let max_token_per_user = std::env::var("MAX_TOKEN")
        .expect("MAX_TOKEN not set!").parse::<i64>().unwrap();
    let count: i64 = ApiToken::belonging_to(&account).count().get_result(&db.get()?)?;
    // if count >= max_token_per_user {
    //     return Err(bad_request(&format!(
    //         "maximum tokens per user is: {}",
    //         max_token_per_user
    //     )));
    // }

    let api_token = ApiToken::insert(db, user.id, &res.name)?;

    // let db = request.db_pool()?;
    //
    //
    // let service = GithubService::new();
    // Package::create(
    //     &res.github_repo_url,
    //     &res.description,
    //     &res.rev,
    //     res.total_files,
    //     res.total_size,
    //     &service,
    //     &db,
    // )
    //     .await?;

    Ok(HttpResponse::Ok().body(""))
}
