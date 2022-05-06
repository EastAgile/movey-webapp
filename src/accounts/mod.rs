//! URL dispatcher for user account related API endpoints.

use jelly::actix_web::web::{get, post, resource, scope, ServiceConfig};
use oauth2::{
    basic::BasicClient,
    AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl,
};
use std::env::var;
use jelly::actix_web::web;

pub mod forms;
pub mod jobs;
pub mod models;
mod tests;
pub mod views;

pub use models::Account;

fn oauth_client() -> BasicClient {
    let github_client_id = ClientId::new(
        var("GITHUB_CLIENT_ID").expect("Missing the GITHUB_CLIENT_ID environment variable."),
    );
    let github_client_secret = ClientSecret::new(
        var("GITHUB_CLIENT_SECRET")
            .expect("Missing the GITHUB_CLIENT_SECRET environment variable."),
    );
    let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
        .expect("Invalid token endpoint URL");
    let client = BasicClient::new(
        github_client_id,
        Some(github_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(
        RedirectUrl::new(
            var("GITHUB_REDIRECT_URL")
                .expect("Missing the GITHUB_REDIRECT_URL environment variable."),
        )
        .expect("Invalid redirect URL"),
    );
    client
}

pub fn configure(config: &mut ServiceConfig) {
    std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID not set!");

    let client = web::Data::new(oauth_client());
    config.service(
        scope("/accounts/")
            .app_data(client.clone())
            .service(
                resource("/register/")
                    .route(get().to(views::register::form))
                    .route(post().to(views::register::create_account)),
            )
            .service(
                resource("/reset/{uidb64}-{ts}-{token}/")
                    .route(get().to(views::reset_password::with_token))
                    .route(post().to(views::reset_password::reset)),
            )
            .service(
                resource("/reset/")
                    .route(get().to(views::reset_password::form))
                    .route(post().to(views::reset_password::request_reset)),
            )
            .service(
                resource("/login/")
                    .route(get().to(views::login::form))
                    .route(post().to(views::login::authenticate)),
            )
            .service(
                resource("/verify/{uidb64}-{ts}-{token}/")
                    .route(get().to(views::verify::with_token)),
            )
            .service(resource("/verify/").route(get().to(views::verify::verify)))
            .service(resource("/github/callback").route(get().to(views::verify::callback_github)))
            .service(resource("/google/callback").route(get().to(views::verify::callback_google)))
            .service(resource("/logout/").route(post().to(views::logout)))
            .service(resource("/oauth").route(get().to(views::login::oauth))),
    );
}
