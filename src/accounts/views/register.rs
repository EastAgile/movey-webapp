use jelly::actix_web::{web::Form, HttpRequest};
use jelly::prelude::*;
use jelly::request::{Authentication, DatabasePool};
use jelly::Result;

use crate::accounts::forms::NewAccountForm;
use crate::accounts::jobs::{SendAccountOddRegisterAttemptEmail, SendVerifyAccountEmail};
use crate::accounts::Account;

pub async fn form(request: HttpRequest) -> Result<HttpResponse> {
    if request.is_authenticated()? {
        return request.redirect("/dashboard/");
    }

    request.render(200, "accounts/register.html", {
        let mut ctx = Context::new();
        let google_client_id =
            std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID not set!");
        ctx.insert("form", &NewAccountForm::default());
        ctx.insert("client_id", &google_client_id);
        ctx
    })
}

pub async fn create_account(
    request: HttpRequest,
    form: Form<NewAccountForm>,
) -> Result<HttpResponse> {
    if request.is_authenticated()? {
        return request.redirect("/dashboard/");
    }

    let mut form = form.into_inner();
    if !form.is_valid() {
        return request.render(400, "accounts/register.html", {
            let mut ctx = Context::new();
            let google_client_id =
                std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID not set!");
            ctx.insert("form", &form);
            ctx.insert("client_id", &google_client_id);
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
