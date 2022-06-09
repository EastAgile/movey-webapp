//! URL dispatcher for user account related API endpoints.

use jelly::actix_web::web::{get, post, put, delete, resource, scope, ServiceConfig};

pub mod services;

pub fn configure(config: &mut ServiceConfig) {
    config.service(
        scope("/api/v1/")
            .service(
                resource("/post_package/").route(post().to(services::package::index::post_package)),
            )
            .service(
                resource("/search_package/")
                    .route(post().to(services::package::index::search_package)),
            ).service(
				resource("/download")
					.route(get().to(services::package::index::increment_download))
            )
            .service(
                resource("/tokens/{token_id}")
                    .route(delete().to(services::setting::controllers::token::revoke_token))
            )
            .service(
                resource("/tokens")
                    .route(put().to(services::setting::controllers::token::create_token))
            )
    );
}
