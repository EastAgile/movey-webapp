use jelly::actix_web::web::{get, post, resource, scope, ServiceConfig};

pub mod models;
pub mod views;

pub use models::Package;

pub fn configure(config: &mut ServiceConfig) {
    config.service(
        scope("/packages/")
            .service(
                resource("/{package_id}")
                    .route(get().to(views::controller::show_package)),
            )
    );
}
