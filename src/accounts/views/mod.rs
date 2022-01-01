//!  Views for user auth.

use jelly::actix_session::UserSession;
use jelly::prelude::*;
use jelly::Result;

pub mod login;
pub mod register;
pub mod reset_password;
pub mod utils;
pub mod verify;

pub async fn logout(request: HttpRequest) -> Result<HttpResponse> {
    request.get_session().clear();
    request.redirect("/")
}
