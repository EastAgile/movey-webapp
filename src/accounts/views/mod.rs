//!  Views for user auth.

use crate::constants::Value;
use jelly::actix_session::UserSession;
use jelly::actix_web::http::header;
use jelly::prelude::*;
use jelly::Result;

pub mod contact;
pub mod login;
pub mod register;
pub mod reset_password;
pub mod utils;
pub mod verify;

pub async fn logout(request: HttpRequest) -> Result<HttpResponse> {
    request.get_session().clear();
    Ok(HttpResponse::Found()
        .header(header::SET_COOKIE, Value::RememberMeTokenInvalidate)
        .header(header::LOCATION, "/accounts/login/")
        .finish())
}
