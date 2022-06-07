use crate::accounts::forms::ChangePasswordForm;
use crate::accounts::Account;
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
    let account = Account::get(user.id, db).await.unwrap();
    request.render(200, "settings/profile.html", {
        let mut context = Context::new();
        context.insert("email", &account.email);
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
            context.insert("email", &account.email);
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
            .header(
                header::SET_COOKIE,
                "remember_me_token=\"\"; path=/; expires=Thu, 01 Jan 1970 00:00:00 GMT",
            )
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
        context.insert("email", &account.email);
        context.insert("connect-status", &account.email);
        context
    })
}
