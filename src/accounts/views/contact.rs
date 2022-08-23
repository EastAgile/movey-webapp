use jelly::actix_web::{web::Form, HttpRequest};
use jelly::prelude::*;
use jelly::Result;
use reqwest::Url;

use crate::accounts::forms::ContactForm;
use crate::accounts::jobs::{SendContactEmail, SendContactRequestEmail};

#[derive(Debug, serde::Deserialize)]
pub struct RecaptchaResponse {
    pub success: bool,
    #[serde(rename(deserialize = "error-codes"))]
    pub error_codes: Option<Vec<String>>,
}

pub async fn send_contact(request: HttpRequest, form: Form<ContactForm>) -> Result<HttpResponse> {
    let captcha_secret_key =
        std::env::var("CAPTCHA_SECRET_KEY").expect("CAPTCHA_SECRET_KEY is not set!");
    let form = form.into_inner();

    let mut url = Url::parse("https://www.google.com/recaptcha/api/siteverify").unwrap();
    url.query_pairs_mut()
        .extend_pairs(&[("secret", captcha_secret_key), ("response", form.token)]);

    let client = reqwest::blocking::Client::new();
    let response = client.get(url).header("User-Agent", "Movey").send()?;
    let response = response.json::<RecaptchaResponse>()?;

    if response.success {
        request.queue(SendContactRequestEmail {
            to: "movey@eastagile.com".to_string(),
            name: form.name.clone(),
            email: form.email.clone(),
            category: form.category.clone(),
            description: form.description.clone(),
        })?;

        request.queue(SendContactEmail { to: form.email })?;
        request.render(200, "accounts/contact_success.html", Context::new())
    } else {
        request.render(200, "policy/contact.html", {
            let categories = std::env::var("CATEGORIES").expect("CATEGORIES is not set!");
            let categories: Vec<String> = serde_json::from_str(categories.as_str()).unwrap();
            let error_messages = response.error_codes.unwrap().join(", ");
            let mut context = Context::new();
            context.insert("categories", &categories);
            context.insert("error", &error_messages);
            context
        })
    }
}
