//! URL dispatcher for user account related API endpoints.

use jelly::actix_web::web::{post, resource, scope, ServiceConfig};

pub mod services;

pub fn configure(config: &mut ServiceConfig) {
    config.service(
        scope("/api/v1/")
            .service(
                resource("/post_package/").route(post().to(services::package::index::post_package)),
            )
    );
}
