use jelly::actix_web::web::{delete, get, put, resource, scope, ServiceConfig};

pub mod controllers;
pub mod views;

pub fn configure(config: &mut ServiceConfig) {
    config.service(
        scope("/api/v1/settings")
            .service(
                resource("/tokens/{token_id}")
                    .route(delete().to(controllers::token::revoke_token)),
            )
            .service(
                resource("/tokens")
                    .route(put().to(controllers::token::create_token)),
            )
            .service(
                resource("/me")
                    .route(get().to(controllers::profile::get_logged_in_user)),
            )
    );
}
