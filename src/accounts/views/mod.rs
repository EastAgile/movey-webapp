//!  Views for user auth.

use jelly::actix_session::UserSession;
use jelly::actix_web::{
    HttpMessage,
    http::header,
};
use jelly::prelude::*;
use jelly::Result;

pub mod login;
pub mod register;
pub mod reset_password;
pub mod utils;
pub mod verify;

pub async fn logout(request: HttpRequest) -> Result<HttpResponse> {
    request.get_session().clear();
    if let Some(_) = request.cookie("remember_me_token") {
        return Ok(HttpResponse::Found()
            .header(header::SET_COOKIE, "remember_me_token=\"\"; path=/; expires=Thu, 01 Jan 1970 00:00:00 GMT")
            .header(header::LOCATION, "/accounts/login/")
            .finish()
        );
    }
    Ok(HttpResponse::Found()
        .header(header::SET_COOKIE, "sign_out=true; Path=/; Max-Age=10")
        .header(header::LOCATION, "/accounts/login/")
        .finish()
    )
}
