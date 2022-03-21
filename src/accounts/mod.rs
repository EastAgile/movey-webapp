//! URL dispatcher for user account related API endpoints.

use jelly::actix_web::web::{get, post, resource, scope, ServiceConfig};

pub mod forms;
pub mod jobs;
pub mod models;
pub mod views;
mod tests;

pub use models::Account;

pub fn configure(config: &mut ServiceConfig) {
    config.service(
        scope("/accounts/")
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
            .service(resource("/logout/").route(post().to(views::logout))),
    );
}
