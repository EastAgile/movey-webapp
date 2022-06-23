use jelly::actix_web::{web::Form, HttpRequest};
use jelly::prelude::*;
use jelly::request::{DatabasePool};
use jelly::Result;

use crate::accounts::forms::NewAccountForm;
use crate::accounts::jobs::{SendAccountOddRegisterAttemptEmail, SendVerifyAccountEmail};
use crate::accounts::Account;
use crate::request;

pub async fn form(request: HttpRequest) -> Result<HttpResponse> {
    if request::is_authenticated(&request).await? {
        return request.redirect("/settings/profile");
    }

    request.render(200, "accounts/register.html", {
        let mut ctx = Context::new();
        ctx.insert("form", &NewAccountForm::default());
        ctx
    })
}

pub async fn create_account(
    request: HttpRequest,
    form: Form<NewAccountForm>,
) -> Result<HttpResponse> {
    if request::is_authenticated(&request).await? {
        return request.redirect("/settings/profile");
    }

    let mut form = form.into_inner();
    if !form.is_valid() {
        return request.render(400, "accounts/register.html", {
            let mut ctx = Context::new();
            ctx.insert("form", &form);
            ctx
        });
    }

    // Catch this error
    // if duplicate:
    //  - send email to existing user asking if they were trying to sign in
    //  - pass requesting user through normal "fake" flow to avoid leaking if
    //      an account exists?
    let db = request.db_pool()?;
    match Account::register(&form, db).await {
        Ok(uid) => {
            request.queue(SendVerifyAccountEmail { to: uid })?;
        }

        Err(e) => {
            error!("Error with registering: {:?}", e);
            request.queue(SendAccountOddRegisterAttemptEmail {
                to: form.email.value.clone(),
            })?;
        }
    }

    // No matter what, just appear as if it worked.
    request.redirect("/accounts/verify/")
}
