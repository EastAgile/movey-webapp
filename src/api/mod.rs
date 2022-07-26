//! URL dispatcher for user account related API endpoints.

use jelly::actix_web::web::{delete, get, post, put, resource, scope, ServiceConfig};

pub mod services;

pub fn configure(config: &mut ServiceConfig) {
    config.service(
        scope("/api/v1")
            .service(
                resource("/post_package").route(post().to(services::package::index::post_package)),
            )
            .service(
                resource("/search_package")
                    .route(post().to(services::package::index::search_package)),
            )
            .service(
                resource("/download")
                    .route(post().to(services::package::index::increase_download_count)),
            )
            .service(
                resource("/tokens/{token_id}")
                    .route(delete().to(services::setting::controllers::token::revoke_token)),
            )
            .service(
                resource("/tokens")
                    .route(put().to(services::setting::controllers::token::create_token)),
            )
            .service(
                resource("/me")
                    .route(get().to(services::setting::controllers::profile::get_logged_in_user)),
            ),
    );
}
