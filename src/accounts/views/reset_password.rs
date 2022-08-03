use jelly::accounts::User;
use jelly::actix_web::{
    web::{Form, Path},
};
use jelly::prelude::*;
use jelly::Result;

use crate::accounts::forms::{ChangePasswordViaEmailForm, EmailForm};
use crate::accounts::jobs::{SendPasswordWasResetEmail, SendResetPasswordEmail};
use crate::accounts::views::utils::validate_token;
use crate::accounts::Account;
#[cfg(test)]
use crate::test::mock::MockHttpRequest as HttpRequest;
#[cfg(not(test))]
use jelly::actix_web::HttpRequest;

/// Just renders a standard "Enter Your Email" password reset page.
pub async fn form(request: HttpRequest) -> Result<HttpResponse> {
    request.render(200, "accounts/reset_password/index.html", {
        let mut context = Context::new();
        context.insert("form", &EmailForm::default());
        context.insert("sent", &false);
        context
    })
}

/// Processes the reset password request, which ultimately just passes
/// it to a background worker to execute - we do this to avoid any timing
/// attacks re: leaking user existence.
pub async fn request_reset(request: HttpRequest, form: Form<EmailForm>) -> Result<HttpResponse> {
    let mut form = form.into_inner();
    if !form.is_valid() {
        return request.render(400, "accounts/reset_password/index.html", {
            let mut context = Context::new();
            context.insert("form", &form);
            context.insert("sent", &false);
            context
        });
    }

    request.queue(SendResetPasswordEmail {
        to: form.email.value.clone(),
    })?;

    let email = form.email.value;
    let mut censored_email = String::new();
    censored_email.push_str(&email[0..1]);
    censored_email.push_str("***");
    censored_email.push_str(
        &email[email
            .find('@')
            .ok_or_else(|| Error::Generic("Invalid email".to_string()))?..],
    );

    request.render(200, "accounts/reset_password/requested.html", {
        let mut context = Context::new();
        context.insert("censored_email", &censored_email);
        context.insert("sent", &true);
        context
    })
}

/// Given a link (of form {uidb64}-{ts}-{token}), verifies the
/// token and user, and presents them a change password form.
///
/// In general, we do not want to leak information, so any errors here
/// should simply report as "invalid or expired". It's a bit verbose, but
/// such is Rust for this type of thing. Write it once and move on. ;P
pub async fn with_token(
    request: HttpRequest,
    Path((uidb64, ts, token)): Path<(String, String, String)>,
) -> Result<HttpResponse> {
    if let Ok(_account) = validate_token(&request, &uidb64, &ts, &token).await {
        return request.render(200, "accounts/reset_password/change_password.html", {
            let mut context = Context::new();
            context.insert("form", &ChangePasswordViaEmailForm::default());
            context.insert("uidb64", &uidb64);
            context.insert("ts", &ts);
            context.insert("token", &token);
            context
        });
    }

    request.render(200, "accounts/invalid_token.html", Context::new())
}

/// Verifies the password is fine, and if so, signs the user in and redirects
/// them to the dashboard with a flash message.
pub async fn reset(
    request: HttpRequest,
    Path((uidb64, ts, token)): Path<(String, String, String)>,
    form: Form<ChangePasswordViaEmailForm>,
) -> Result<HttpResponse> {
    let mut form = form.into_inner();

    if let Ok(account) = validate_token(&request, &uidb64, &ts, &token).await {
        // Note! This is a case where we need to fetch the user ahead of form validation.
        // While it would be nice to avoid the DB hit, validating that their password is secure
        // requires pulling some account values...
        form.name = Some(account.name.clone());
        form.email = Some(account.email.clone());

        if !form.is_valid() {
            return request.render(200, "accounts/reset_password/change_password.html", {
                let mut context = Context::new();
                context.insert("form", &form);
                context.insert("uidb64", &uidb64);
                context.insert("ts", &ts);
                context.insert("token", &token);
                context
            });
        }

        let pool = request.db_pool()?;
        Account::update_password_and_last_login(account.id, &form.password, pool).await?;
        // If they has come this far, assume they have verified their email (or else they won't be able to get to this page at all)
        Account::mark_verified(account.id, pool).await?;

        request.queue(SendPasswordWasResetEmail {
            to: account.email.clone(),
        })?;

        request.set_user(User {
            id: account.id,
            name: account.name,
            is_admin: account.is_admin,
            is_anonymous: false,
        })?;

        return request.render(200, "accounts/reset_password/success.html", Context::new());
    }

    request.render(200, "accounts/invalid_token.html", Context::new())
}

#[cfg(test)]
mod tests {
    use jelly::actix_web::HttpResponse;
    use jelly::actix_web::web::Form;
    use jelly::email::Context;
    use jelly::forms::EmailField;
    use crate::accounts::forms::EmailForm;
    use crate::accounts::views::reset_password::request_reset;
    use crate::test::{DB_POOL, mock};

    #[actix_rt::test]
    async fn request_reset_returns_error_with_invalid_email_form() {
        let mut mock_http_request = mock::MockHttpRequest::new();
        mock_http_request
            .expect_db_pool()
            .returning(|| Ok(&DB_POOL));
        mock_http_request
            .expect_render()
            .withf(|code: &usize, template: &str, _context: &Context| {
                code == &400 && template == "accounts/reset_password/index.html"
            })
            .returning(|_, _, _| Ok(HttpResponse::Ok().finish()));
        let form = Form(EmailForm {
            email: EmailField {
                value: "invalid".to_string(),
                errors: vec![]
            }
        });
        request_reset(mock_http_request, form).await.unwrap();
    }
}

