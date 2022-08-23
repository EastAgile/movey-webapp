//! URL dispatcher for user account related API endpoints.

use jelly::actix_web::web::{delete, get, post, put, resource, scope, ServiceConfig};

pub mod services;

pub fn configure(config: &mut ServiceConfig) {
    config.service(
        scope("/api/v1")
            .service(
                resource("/packages/upload")
                    .route(post().to(services::package::controller::register_package))
            )
            .service(
                resource("/packages/count")
                    .route(post().to(services::package::controller::increase_download_count))
            )
            .service(
                resource("/search_package")
                    .route(post().to(services::package::controller::search_package))
            )
            .service(
                resource("/tokens/{token_id}")
                    .route(delete().to(services::setting::controllers::token::revoke_token))
            )
            .service(
                resource("/tokens")
                    .route(put().to(services::setting::controllers::token::create_token))
            )
            .service(
                resource("/me")
                    .route(get().to(services::setting::controllers::profile::get_logged_in_user))
            )
            .service(
                scope("/packages/{package_name}")
                    .service(
                        resource("/badge")
                            .route(get().to(services::package::controller::package_badge_info))
                    )
                    .service(
                        resource("/collaborators/create")
							.route(post().to(services::collaborators::controllers::add_collaborators))
                    )
                    .service(
                        resource("/collaborators/remove")
							.route(delete().to(services::collaborators::controllers::remove_collaborator))
                    )
                    .service(resource("/transfer")
						.route(post().to(services::collaborators::controllers::transfer_ownership))
					),
            )
            .service(resource("/owner_invitations/handle")
				.route(post().to(services::collaborators::controllers::handle_invite))
            )
    );
}
