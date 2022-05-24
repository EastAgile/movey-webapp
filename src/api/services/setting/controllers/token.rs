use crate::accounts::Account;
use crate::api::services::setting::views::EncodableApiTokenWithToken;
use crate::request;
use crate::setting::models::token::ApiToken;
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::DatabaseError;
use jelly::actix_web::http::header::ContentType;
use jelly::actix_web::web;
use jelly::prelude::*;
use jelly::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TokenRequest {
    pub name: String,
}

pub async fn create_token(
    request: HttpRequest,
    req: web::Json<TokenRequest>,
) -> Result<HttpResponse> {
    if !request::is_authenticated(&request).await? {
        return Ok(HttpResponse::Unauthorized().body(""));
    }
    let db = request.db_pool()?;
    let user = request.user()?;
    let account = Account::get(user.id, db).await?;

    let max_token_per_user = std::env::var("MAX_TOKEN")
        .expect("MAX_TOKEN not set!")
        .parse::<i64>()
        .unwrap();
    let count: i64 = ApiToken::belonging_to(&account)
        .count()
        .get_result(&db.get()?)?;
    if count > max_token_per_user {
        return Ok(HttpResponse::BadRequest().body("Too many tokens created."));
    }

    let api_token = ApiToken::insert(user.id, &req.name, db).await;
    let api_token = match api_token {
        Err(Error::Database(DatabaseError(DatabaseErrorKind::UniqueViolation, _))) => {
            return Ok(HttpResponse::BadRequest().body("That name has already been taken."))
        }
        Err(_) => return Ok(HttpResponse::InternalServerError().body("")),
        Ok(api_token) => api_token,
    };
    let api_token = EncodableApiTokenWithToken::from(api_token);
    Ok(HttpResponse::Ok().set(ContentType::json()).json(&api_token))
}
