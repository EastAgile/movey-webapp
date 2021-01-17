use jelly::prelude::*;
use jelly::actix_web::{HttpRequest, web::Form};
use jelly::request::{Authentication, DatabasePool};
use jelly::Result;

use crate::accounts::Account;
use crate::accounts::forms::LoginForm;

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
pub async fn authenticate(
    request: HttpRequest,
    form: Form<LoginForm>
) -> Result<HttpResponse> {
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
        if user.is_admin {
            Account::update_last_login(user.id, db).await?;
            request.set_user(user)?;
            return request.redirect("/dashboard/");
        }
    }

    request.render(400, "accounts/login.html", {
        let mut context = Context::new();
        context.insert("error", "Invalid email or password.");
        context.insert("form", &form);
        context
    })
}
