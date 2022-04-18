use jelly::actix_session::UserSession;
use jelly::actix_web::{web, web::Form, HttpRequest};
use jelly::prelude::*;
use jelly::request::{Authentication, DatabasePool};
use jelly::Result;
use oauth2::{
    basic::BasicClient,
    CsrfToken, Scope,
};

use crate::request;
use jelly::actix_web::{
    cookie::{Cookie, CookieJar, Key},
    http::header,
};
use time::Duration;

use crate::accounts::forms::LoginForm;
use crate::accounts::Account;

/// The login form.
pub async fn form(request: HttpRequest) -> Result<HttpResponse> {
    if request::is_authenticated(&request).await? {
        return request.redirect("/dashboard/");
    }

    request.render(200, "accounts/login.html", {
        let mut ctx = Context::new();
        let google_client_id =
            std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID not set!");
        ctx.insert("form", &LoginForm::default());
        ctx.insert("client_id", &google_client_id);
        ctx
    })
}

/// POST-handler for logging in.
pub async fn authenticate(request: HttpRequest, form: Form<LoginForm>) -> Result<HttpResponse> {
    if request::is_authenticated(&request).await? {
        return request.redirect("/dashboard/");
    }

    let mut form = form.into_inner();
    if !form.is_valid() {
        return request.render(400, "accounts/login.html", {
            let mut context = Context::new();
            let google_client_id =
                std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID not set!");
            context.insert("error", "Invalid email or password.");
            context.insert("form", &form);
            context.insert("client_id", &google_client_id);
            context
        });
    }

    let db = request.db_pool()?;
    if let Ok(user) = Account::authenticate(&form, db).await {
        Account::update_last_login(user.id, db).await?;

        if form.remember_me == "off" {
            request.set_user(user)?;
            return request.redirect("/dashboard/");
        } else {
            let key = std::env::var("SECRET_KEY").expect("SECRET_KEY not set!");
            let value = user.id;
            let max_age_days = 30;

            let mut jar = CookieJar::new();
            jar.signed(&Key::derive_from(key.as_bytes())).add(
                Cookie::build("remember_me_token", value.to_string())
                    .path("/")
                    .max_age(Duration::days(max_age_days))
                    .http_only(true)
                    .finish(),
            );

            return Ok(HttpResponse::Found()
                .header(
                    header::SET_COOKIE,
                    jar.get("remember_me_token").unwrap().encoded().to_string(),
                )
                .header(header::LOCATION, "/dashboard/")
                .finish());
        }
    }

    request.render(400, "accounts/login.html", {
        let mut context = Context::new();
        let google_client_id =
            std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID not set!");
        context.insert("error", "Invalid email or password.");
        context.insert("form", &form);
        context.insert("client_id", &google_client_id);
        context
    })
}

pub async fn oauth(request: HttpRequest, client: web::Data<BasicClient>) -> Result<HttpResponse> {
    if request.is_authenticated()? {
        return request.redirect("/dashboard/");
    }

    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("public_repo".to_string()))
        .add_scope(Scope::new("user:email".to_string()))
        .url();

    request.get_session().set("oauth_state", &csrf_state)?;
    Ok(HttpResponse::Found()
        .header(header::LOCATION, authorize_url.to_string())
        .finish())
}
