use jelly::actix_web::HttpRequest;
use jelly::prelude::*;
use jelly::Result;

pub async fn form(request: HttpRequest) -> Result<HttpResponse> {
    request.render(200, "settings/tokens.html", Context::new())
}
