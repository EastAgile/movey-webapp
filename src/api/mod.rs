//! URL dispatcher for user account related API endpoints.

use jelly::actix_web::web::{get, post, resource, scope, ServiceConfig};

pub mod services;

pub fn configure(config: &mut ServiceConfig) {
    config.service(
        scope("/api/v1/")
            .service(resource("/get_object/").route(get().to(services::test::index::get_object)))
            .service(resource("/get_json/").route(get().to(services::test::index::get_json)))
            .service(resource("/post_json/").route(post().to(services::test::index::post_json)))
    );
}
