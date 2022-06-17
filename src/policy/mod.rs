use jelly::actix_web::web::{resource, scope, ServiceConfig};
use jelly::prelude::*;
use jelly::Result;


pub async fn term_(request: HttpRequest) -> Result<HttpResponse> {
    request.render(200, "policy/term.html", Context::new())
}

pub async fn policy_(request: HttpRequest) -> Result<HttpResponse> {
    request.render(200, "policy/policy.html", Context::new())
}

pub async fn contact(request: HttpRequest) -> Result<HttpResponse> {
    request.render(200, "policy/contact.html", Context::new())
}

pub async fn about(request: HttpRequest) -> Result<HttpResponse> {
    request.render(200, "policy/about.html", Context::new())
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
            .service(
                resource("/contact").to(contact),
            )
            .service(
                resource("/about").to(about),
            )
    );
}
