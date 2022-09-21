use jelly::actix_web::web::{delete, post, resource, scope, ServiceConfig};

pub mod controllers;
pub mod views;

pub fn configure(config: &mut ServiceConfig) {
    config.service(
        scope("/api/v1/collaborators")
            .service(
                scope("/packages/{package_name}")
                    .service(
                        resource("/create").route(
                            post().to(controllers::add_collaborators),
                        ),
                    )
                    .service(resource("/remove").route(
                        delete().to(controllers::remove_collaborator),
                    ))
                    .service(resource("/transfer").route(
                        post().to(controllers::transfer_ownership),
                    )),
            )
            .service(resource("/handle").route(
                post().to(controllers::handle_invite),
            )),
    );
}
