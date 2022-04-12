use jelly::actix_web::{web::Form, HttpRequest};
use jelly::prelude::*;
use jelly::request::{Authentication, DatabasePool};
use jelly::Result;

use crate::accounts::forms::LoginForm;
use crate::accounts::Account;

/// The login form.
pub async fn form(request: HttpRequest) -> Result<HttpResponse> {
    if request.is_authenticated()? {
        return request.redirect("/dashboard/");
    }

    request.render(200, "accounts/login.html", {
        let mut ctx = Context::new();
        let google_client_id = std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID not set!");
        ctx.insert("form", &LoginForm::default());
        ctx.insert("client_id", &google_client_id);
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
            let google_client_id = std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID not set!");
            context.insert("error", "Invalid email or password.");
            context.insert("form", &form);
            context.insert("client_id", &google_client_id);
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
        let google_client_id = std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID not set!");
        context.insert("error", "Invalid email or password.");
        context.insert("form", &form);
        context.insert("client_id", &google_client_id);
        context
    })
}
