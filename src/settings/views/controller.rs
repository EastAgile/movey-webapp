use jelly::actix_web::{web::Path, web::Query, HttpRequest};
use jelly::forms::TextField;
use jelly::prelude::*;
use jelly::request::DatabasePool;
use jelly::Result;

pub async fn profile(request: HttpRequest) -> Result<HttpResponse> {
    request.render(200, "settings/profile.html",Context::new())
}