use jelly::accounts::User;
use jelly::actix_web::{
    web::{Form, Path},
    HttpRequest,
};
use jelly::prelude::*;
use jelly::Result;

use crate::accounts::forms::{ EmailForm, ContactForm};
use crate::accounts::jobs::{ SendResetPasswordEmail, SendContactRequestEmail,SendPasswordWasResetEmail};

use crate::accounts::Account;

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
    censored_email.push_str(&email[email.find('@').ok_or(Error::Generic("Invalid email".to_string()))?..]);

    request.render(200, "accounts/reset_password/requested.html", {
        let mut context = Context::new();
        context.insert("censored_email", &censored_email);
        context.insert("sent", &true);
        context
    })
}


pub async fn send_contact(request: HttpRequest, form: Form<ContactForm>) -> Result<HttpResponse> {
    let mut form = form.into_inner();

    request.queue(SendContactRequestEmail{
        to: "khoa.tran@stanyangroup.com".to_string(),
        name: form.name.clone(),
        email: form.email.clone(),
        category: form.category.clone(),
        description: form.description.clone()
    })?;
    
    
    request.render(200, "accounts/contactSuccess.html", {
        let mut context = Context::new();
        context.insert("category",&form.category);
        context.insert("email", &form.email);
        context.insert("name", &form.name);
        context.insert("description", &form.description);
        context
    })
}
