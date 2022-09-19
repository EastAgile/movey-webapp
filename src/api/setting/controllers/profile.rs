use crate::accounts::Account;
use crate::constants;
use crate::utils::request_utils;
use jelly::actix_session::UserSession;
use jelly::actix_web::http::header;
use jelly::actix_web::http::header::ContentType;
use jelly::prelude::*;
use jelly::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoggedInUser {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub avatar: Option<String>,
}

pub async fn get_logged_in_user(request: HttpRequest) -> Result<HttpResponse> {
    if !request_utils::is_authenticated(&request)? {
        request.get_session().clear();
        return Ok(HttpResponse::Ok()
            .set(ContentType::json())
            .header(header::SET_COOKIE, constants::REMEMBER_ME_TOKEN_INVALIDATE)
            .body("{}"));
    }
    let user = request.user()?;
    let db = request.db_pool()?;
    let account = Account::get(user.id, db);
    if let Ok(account) = account {
        Ok(HttpResponse::Ok()
            .set(ContentType::json())
            .json(&LoggedInUser {
                id: account.id,
                name: account.name,
                email: account.email,
                avatar: account.avatar,
            }))
    } else {
        request.get_session().clear();
        Ok(HttpResponse::Ok()
            .set(ContentType::json())
            .header(header::SET_COOKIE, constants::REMEMBER_ME_TOKEN_INVALIDATE)
            .body("{}"))
    }
}
