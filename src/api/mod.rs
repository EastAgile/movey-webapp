//! URL dispatcher for user account related API endpoints.

use jelly::actix_web::web::{delete, get, post, put, resource, scope, ServiceConfig};

pub mod services;

pub fn configure(config: &mut ServiceConfig) {
    config.service(
        scope("/api/v1")
            .service(
                scope("/packages")
                    .service(
                        resource("/register")
                            .route(post().to(services::package::controller::register_package))
                    )
                    .service(
                        resource("/count")
                            .route(post().to(services::package::controller::increase_download_count)),
                    )
                    .service(
                        resource("/{pkg_name}/badge")
                            .route(get().to(services::package::controller::package_badge_info)),
                    )
            )
            .service(
                resource("/search_package")
                    .route(post().to(services::package::controller::search_package)),
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
