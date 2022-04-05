use jelly::actix_session::UserSession;
use jelly::actix_web::{web::Form, HttpRequest, web};
use jelly::actix_web::web::head;
use jelly::prelude::*;
use jelly::request::{Authentication, DatabasePool};
use jelly::Result;
use oauth2::basic::BasicClient;
use oauth2::{CsrfToken, Scope};
use oauth2::http::header;

use crate::accounts::forms::LoginForm;
use crate::accounts::Account;

/// The login form.
pub async fn form(request: HttpRequest) -> Result<HttpResponse> {
    if request.is_authenticated()? {
        return request.redirect("/dashboard/");
    }

    request.render(200, "accounts/login.html", {
        let mut ctx = Context::new();
        ctx.insert("form", &LoginForm::default());
        ctx
    })
}

/// POST-handler for logging in.
pub async fn authenticate(request: HttpRequest, form: Form<LoginForm>) -> Result<HttpResponse> {
    if request.is_authenticated()? {
        return request.redirect("/dashboard/");
    }

    let mut form = form.into_inner();
    if !form.is_valid() {
        return request.render(400, "accounts/login.html", {
            let mut context = Context::new();
            context.insert("error", "Invalid email or password.");
            context.insert("form", &form);
            context
        });
    }

    let db = request.db_pool()?;
    if let Ok(user) = Account::authenticate(&form, db).await {
        Account::update_last_login(user.id, db).await?;
        request.set_user(user)?;
        return request.redirect("/dashboard/");
    }

    request.render(400, "accounts/login.html", {
        let mut context = Context::new();
        context.insert("error", "Invalid email or password.");
        context.insert("form", &form);
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

    info!(
        "Open this URL in your browser:\n{}\n",
        authorize_url.to_string()
    );
    request.get_session().set("state", &csrf_state);
    Ok(HttpResponse::Found()
        .header(header::LOCATION, authorize_url.to_string())
        .finish()
    )
}

