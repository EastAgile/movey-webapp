use jelly::actix_session::UserSession;
use jelly::actix_web::{web, web::Form, web::Query, HttpMessage, HttpRequest};
use jelly::prelude::*;
use jelly::request::{Authentication, DatabasePool};
use jelly::Result;
use oauth2::{basic::BasicClient, CsrfToken, Scope};
use serde::Deserialize;

use crate::utils::request_utils;
use jelly::actix_web::{
    cookie::{Cookie, CookieJar, Key},
    http::header,
};
use time::Duration;

use crate::accounts::forms::LoginForm;
use crate::accounts::Account;

/// The login form.
pub async fn form(request: HttpRequest) -> Result<HttpResponse> {
    if request_utils::is_authenticated(&request)? {
        return request.redirect("/settings/profile");
    }
    request.render(200, "accounts/login.html", {
        let mut ctx = Context::new();
        ctx.insert("form", &LoginForm::default());
        let flash = request.cookie("flash");
        if let Some(message) = flash {
            ctx.insert("flash", message.value());
        }
        ctx
    })
}

/// POST-handler for logging in.
pub async fn authenticate(request: HttpRequest, form: Form<LoginForm>) -> Result<HttpResponse> {
    if request_utils::is_authenticated(&request)? {
        return request.redirect("/settings/profile");
    }

    let mut form = form.into_inner();
    if !form.is_valid() {
        return request.render(400, "accounts/login.html", {
            let mut context = Context::new();
            context.insert("error", "Invalid email or password! Try again.");
            context.insert("form", &form);
            context
        });
    }

    let db = request.db_pool()?;
    let error_message = match Account::authenticate(&form, db) {
        Ok(user) => {
            let user_id = user.id;
            Account::update_last_login(user_id, db)?;
            request.set_user(user)?;

            return if form.remember_me == "off" {
                request.redirect("/settings/profile")
            } else {
                let key = std::env::var("SECRET_KEY").expect("SECRET_KEY not set!");
                let value = user_id;
                let max_age_days = std::env::var("MAX_REMEMBER_ME_DAYS")
                    .expect("MAX_REMEMBER_ME_DAYS not set!")
                    .parse::<i64>()
                    .expect("MAX_REMEMBER_ME_DAYS must be an integer");

                let mut jar = CookieJar::new();
                jar.signed(&Key::derive_from(key.as_bytes())).add(
                    Cookie::build("remember_me_token", value.to_string())
                        .path("/")
                        .max_age(Duration::days(max_age_days))
                        .http_only(true)
                        .finish(),
                );

                Ok(HttpResponse::Found()
                    .header(
                        header::SET_COOKIE,
                        jar.get("remember_me_token")
                            .expect("Getting key from cookie jar should not fail.")
                            .encoded()
                            .to_string(),
                    )
                    .header(header::LOCATION, "/settings/profile")
                    .finish())
            };
        }
        Err(Error::Generic(e)) => e,
        Err(_) => String::from("Invalid email or password! Try again."),
    };

    request.render(400, "accounts/login.html", {
        let mut context = Context::new();
        context.insert("error", error_message.as_str());
        context.insert("form", &form);
        context
    })
}

#[derive(Deserialize)]
pub struct GithubAuthorizePrompt {
    pub prompt: bool,
}

pub async fn oauth(
    request: HttpRequest,
    auth_prompt: Option<Query<GithubAuthorizePrompt>>,
    client: web::Data<BasicClient>,
) -> Result<HttpResponse> {
    let mut oauth_request = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("user:email".to_string()));
    if auth_prompt.is_some() && auth_prompt.unwrap().prompt {
        oauth_request = oauth_request.add_extra_param("login", "");
    }
    let (authorize_url, csrf_state) = oauth_request.url();

    request.get_session().set("oauth_state", &csrf_state)?;
    Ok(HttpResponse::Found()
        .header(header::LOCATION, authorize_url.to_string())
        .finish())
}
