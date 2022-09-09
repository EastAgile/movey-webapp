use jelly::actix_web::web::{get, post, resource, scope, ServiceConfig};

pub mod controller;
#[cfg(test)]
mod tests;
pub mod view;

pub fn configure(config: &mut ServiceConfig) {
    config.service(
        scope("/api/v1/packages")
            .service(
                resource("/upload")
                    .route(post().to(controller::register_package)),
            )
            .service(
                resource("/count")
                    .route(post().to(controller::increase_download_count)),
            )
            .service(
                resource("{package_name}/badge")
                    .route(get().to(controller::package_badge_info)),
            )
    );
    config.service(
        resource("/search_package")
            .route(post().to(controller::search_package)),
    );
}
