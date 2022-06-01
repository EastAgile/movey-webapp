use jelly::actix_web::web::{get, resource, scope, ServiceConfig};

pub mod models;
pub mod views;

pub use models::{Package, PackageVersion, PackageVersionSort, NewPackage, NewPackageVersion};
use crate::utils::new_auth;

pub fn configure(config: &mut ServiceConfig) {
    let guard = new_auth();

    config.service(
        scope("/settings")
            .wrap(guard)
            .service(
                resource("/profile")
                    .route(get().to(views::controller::profile)),
            )
    );
}
