use crate::accounts::Account;
use crate::api::setting::views::EncodableApiTokenWithToken;
use crate::settings::models::token::ApiToken;
use crate::utils::request_utils;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::DatabaseError;
use jelly::actix_web::http::header::ContentType;
use jelly::actix_web::{web, web::Path};
use jelly::anyhow::anyhow;
use jelly::forms::TextField;
use jelly::prelude::*;
use jelly::Result;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TokenRequest {
    pub name: TextField,
}

pub async fn create_token(
    request: HttpRequest,
    mut req: web::Json<TokenRequest>,
) -> Result<HttpResponse> {
    if !req.name.is_valid() {
        return Ok(HttpResponse::BadRequest().body(&req.name.errors[0]));
    }
    if !request_utils::is_authenticated(&request)? {
        return Ok(request_utils::clear_cookie(&request));
    }
    let user = request.user()?;
    let db = request.db_pool()?;
    let account = Account::get(user.id, db)?;
    let api_token = ApiToken::insert(&account, &req.name.value, db);

    let api_token = match api_token {
        Err(Error::Database(DatabaseError(DatabaseErrorKind::UniqueViolation, _))) => {
            return Ok(HttpResponse::BadRequest().body("That name has already been taken."))
        }
        Err(Error::Generic(error)) => return Ok(HttpResponse::BadRequest().body(error)),
        Err(_) => return Ok(HttpResponse::NotFound().body("")),
        Ok(api_token) => api_token,
    };
    let api_token = EncodableApiTokenWithToken::from(api_token);
    Ok(HttpResponse::Ok().set(ContentType::json()).json(&api_token))
}

pub async fn revoke_token(
    request: HttpRequest,
    Path(token_id): Path<String>,
) -> Result<HttpResponse> {
    if !request_utils::is_authenticated(&request)? {
        return Ok(request_utils::clear_cookie(&request));
    }
    let user = request.user()?;
    let db = request.db_pool()?;
    let token = ApiToken::get_by_id(
        token_id
            .parse::<i32>()
            .map_err(|e| anyhow!("Error parsing token id: {:?}", e))?,
        db,
    )?;

    // checks if token belongs to account
    if token.account_id == user.id {
        ApiToken::revoke(token.id, db)?;
    }

    Ok(HttpResponse::Ok().body(""))
}
