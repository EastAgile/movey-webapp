use jelly::actix_web::{web::Form, HttpRequest};
use jelly::prelude::*;
use jelly::Result;

use crate::accounts::forms::ContactForm;
use crate::accounts::jobs::SendContactEmail;
use crate::accounts::jobs::SendContactRequestEmail;

pub async fn send_contact(request: HttpRequest, form: Form<ContactForm>) -> Result<HttpResponse> {
    let form = form.into_inner();

    request.queue(SendContactRequestEmail {
        to: "movey@eastagile.com".to_string(),
        name: form.name.clone(),
        email: form.email.clone(),
        category: form.category.clone(),
        description: form.description.clone(),
    })?;

    request.queue(SendContactEmail {
        to: form.email.clone(),
    })?;

    request.render(200, "accounts/contact_success.html", Context::new())
}
