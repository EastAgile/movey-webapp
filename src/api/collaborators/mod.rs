use jelly::actix_web::web::{delete, post, resource, scope, ServiceConfig};

pub mod controllers;
pub mod views;

pub fn configure(config: &mut ServiceConfig) {
    config.service(
        scope("/api/v1")
            .service(
                scope("/packages/{package_name}")
                    .service(
                        resource("/collaborators/create").route(
                            post().to(controllers::add_collaborators),
                        ),
                    )
                    .service(resource("/collaborators/remove").route(
                        delete().to(controllers::remove_collaborator),
                    ))
                    .service(resource("/transfer").route(
                        post().to(controllers::transfer_ownership),
                    )),
            )
            .service(
                resource("/owner_invitations/handle")
                    .route(post().to(controllers::handle_invite)),
            ),
    );
}
