pub mod jobs;
pub mod views;

use crate::accounts::Account;
use jelly::actix_web::web::{self, get, post, resource, scope, ServiceConfig};
use jelly::Result;
use jelly::{actix_web, prelude::*};
use std::env;

pub async fn term_(request: HttpRequest) -> Result<HttpResponse> {
    request.render(200, "policy/term.html", Context::new())
}

pub async fn policy_(request: HttpRequest) -> Result<HttpResponse> {
    request.render(200, "policy/policy.html", Context::new())
}

pub async fn about_us(request: HttpRequest) -> Result<HttpResponse> {
    request.render(200, "policy/about.html", Context::new())
}

pub async fn contact(request: HttpRequest) -> Result<HttpResponse> {
    let categories_s = env::var("CATEGORIES").unwrap();
    let categories: Vec<String> = serde_json::from_str(categories_s.as_str()).unwrap();
    let mut ctx = Context::new();
    ctx.insert("categories", &categories);
    let user = request.user();

    let auto_generated_host =
        env::var("NO_REPLY_EMAIL_DOMAIN").expect("NO_REPLY_EMAIL_DOMAIN is not set!");

    if let Ok(user_) = user {
        if user_.id != 0 {
            let account = Account::get(user_.id, request.db_pool()?)?;
            ctx.insert("name", &account.name);
            ctx.insert(
                "email",
                if account.email.contains(&auto_generated_host) {
                    ""
                } else {
                    &account.email
                },
            );
            ctx.insert("categories", &categories);
        }
    }
    request.render(200, "policy/contact.html", ctx)
}

pub fn configure(config: &mut ServiceConfig) {
    config.service(
        scope("/")
            .service(resource("/terms-of-use").to(term_))
            .service(resource("/policy").to(policy_))
            .service(resource("/about").to(about_us))
            .service(
                resource("/contact")
                    .app_data(web::FormConfig::default().error_handler(|err, req| {
                        actix_web::error::InternalError::from_response(
                            err,
                            req.render(400, "400.html", Context::new()).unwrap(),
                        )
                        .into()
                    }))
                    .route(get().to(contact))
                    .route(post().to(views::send_contact)),
            ),
    );
}
