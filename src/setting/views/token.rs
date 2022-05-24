use jelly::actix_web::{web, web::Form, HttpRequest};
use jelly::prelude::*;
use jelly::Result;

/// The login form.
pub async fn form(request: HttpRequest) -> Result<HttpResponse> {

    request.render(200, "settings/tokens.html", {
        Context::new()
    })
}