use jelly::actix_web::web::{resource, scope, ServiceConfig};
use jelly::prelude::*;
use jelly::Result;


pub async fn term_(request: HttpRequest) -> Result<HttpResponse> {
    request.render(200, "policy/term.html", {
        let mut context = Context::new();
        context
    })
}

pub async fn policy_(request: HttpRequest) -> Result<HttpResponse> {
    request.render(200, "policy/policy.html", {
        let mut context = Context::new();
        context
    })
}

pub fn configure(config: &mut ServiceConfig) {
    config.service(
        scope("/")
            .service(
                resource("/terms-of-use").to(term_),
            )
            .service(
                resource("/policy").to(policy_),
            )
    );
}
