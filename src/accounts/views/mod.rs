//!  Views for user auth.

use jelly::actix_session::UserSession;
use jelly::actix_web::http::header;
use jelly::prelude::*;
use jelly::Result;

pub mod login;
pub mod register;
pub mod reset_password;
pub mod utils;
pub mod verify;

pub async fn logout(request: HttpRequest) -> Result<HttpResponse> {
    request.get_session().clear();
    Ok(HttpResponse::Found()
        .header(header::SET_COOKIE, "remember_me_token=\"\"; path=/; expires=Thu, 01 Jan 1970 00:00:00 GMT")
        .header(header::LOCATION, "/accounts/login/")
        .finish()
    )
}
