use crate::accounts::forms::ChangePasswordForm;
use crate::accounts::Account;
use crate::constants;
use crate::packages::Package;
use crate::settings::models::token::ApiToken;

use jelly::actix_session::UserSession;
use jelly::actix_web::http::header;
use jelly::actix_web::web::Form;
use jelly::actix_web::HttpRequest;
use jelly::prelude::*;
use jelly::request::DatabasePool;
use jelly::Result;

pub async fn profile(request: HttpRequest) -> Result<HttpResponse> {
    let user = request.user()?;
    let db = request.db_pool()?;
    let account = Account::get(user.id, db).await?;
    request.render(200, "settings/profile.html", {
        let mut context = Context::new();
        context.insert("account", &account);
        context.insert("profile_tab", "profile");
        context
    })
}

pub async fn change_password(
    request: HttpRequest,
    form: Form<ChangePasswordForm>,
) -> Result<HttpResponse> {
    let mut form = form.into_inner();
    let user = request.user()?;
    let db = request.db_pool()?;
    let account = Account::get(user.id, db).await?;
    form.name = Some(account.name.clone());
    form.email = Some(account.email.clone());

    if !form.is_valid() {
        return request.render(200, "settings/profile.html", {
            let mut context = Context::new();
            context.insert("form", &form);
            context.insert("is_ok", &false);
            context.insert("account", &account);
            context.insert("profile_tab", "profile");
            context
        });
    }
    let result = Account::change_password(
        user.id,
        form.current_password.value,
        form.new_password.value,
        db,
    )
    .await;
    let message;
    let is_ok = match result {
        Ok(_) => {
            message = "Change password successfully";
            true
        }
        Err(Error::InvalidPassword) => {
            message = "Wrong password. Try again!";
            false
        }
        Err(_) => {
            message = "Unexpected error. Try again!";
            false
        }
    };
    if is_ok {
        request.get_session().clear();
        return Ok(HttpResponse::Found()
            .header(header::SET_COOKIE, constants::REMEMBER_ME_TOKEN_INVALIDATE)
            .header(
                header::SET_COOKIE,
                format!("flash={}; path=/; Max-Age=10", message),
            )
            .header(header::LOCATION, "/accounts/login/")
            .finish());
    }
    request.render(200, "settings/profile.html", {
        let mut context = Context::new();
        context.insert("error", message);
        context.insert("account", &account);
        context.insert("connect-status", &account.email);
        context.insert("profile_tab", "profile");
        context
    })
}

pub async fn show_packages(request: HttpRequest) -> Result<HttpResponse> {
    let db = request.db_pool()?;
    if let Ok(user) = request.user() {
        let packages = Package::get_by_account(user.id, db).await?;

        request.render(200, "settings/user_packages.html", {
            let mut ctx = Context::new();
            ctx.insert("packages", &packages);
            ctx.insert("profile_tab", "packages");
            ctx
        })
    } else {
        Ok(HttpResponse::NotFound().body("Cannot find user"))
    }
}

pub async fn show_downloads(request: HttpRequest) -> Result<HttpResponse> {
    let db = request.db_pool()?;
    if let Ok(user) = request.user() {
        let download = Package::get_downloads(user.id, db).await?;

        request.render(200, "settings/downloads.html", {
            let mut ctx = Context::new();
            ctx.insert("profile_tab", "downloads");
            ctx.insert("total_downloads", &download);
            ctx
        })
    } else {
        Ok(HttpResponse::NotFound().body("Cannot find user"))
    }
}

pub async fn show_invitations(request: HttpRequest) -> Result<HttpResponse> {
    let db = request.db_pool()?;
    if let Ok(user) = request.user() {
        request.render(200, "settings/invitations.html", {
            let mut ctx = Context::new();
            ctx.insert("profile_tab", "invitations");
            ctx
        })
    } else {
        Ok(HttpResponse::NotFound().body("Cannot find user"))
    }
}

pub async fn show_tokens(request: HttpRequest) -> Result<HttpResponse> {
    let db = request.db_pool()?;
    if let Ok(user) = request.user() {
        let tokens = ApiToken::get_by_account(user.id, db).await?;

        request.render(200, "settings/tokens.html", {
            let mut ctx = Context::new();
            ctx.insert("tokens", &tokens);
            ctx.insert("profile_tab", "tokens");
            ctx
        })
    } else {
        Ok(HttpResponse::NotFound().body("Cannot find user"))
    }
}
