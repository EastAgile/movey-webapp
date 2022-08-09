//!  Views for user auth.

use crate::constants;
use jelly::actix_session::UserSession;
use jelly::actix_web::http::header;
use jelly::prelude::*;
use jelly::Result;

pub mod avatar;
pub mod login;
pub mod public_profile;
pub mod register;
pub mod reset_password;
pub mod utils;
pub mod verify;

pub async fn logout(request: HttpRequest) -> Result<HttpResponse> {
    request.get_session().clear();
    Ok(HttpResponse::Found()
        .header(header::SET_COOKIE, constants::REMEMBER_ME_TOKEN_INVALIDATE)
        .header(header::LOCATION, "/accounts/login/")
        .finish())
}
