use jelly::actix_web::web::{get, resource, scope, ServiceConfig};

pub mod models;
pub mod views;

pub use models::{Package, PackageSort, PackageVersion, PackageVersionSort, NewPackage, NewPackageVersion};

pub fn configure(config: &mut ServiceConfig) {
    config.service(
        scope("/packages/")
            .service(
                resource("/{package_name}/versions")
                    .route(get().to(views::controller::show_package_versions)),
            )
            .service(
                resource("/{package_name}")
                    .route(get().to(views::controller::show_package)),
            )
            .service(
                resource("/search/")
                    .route(get().to(views::controller::show_search_results)),
            )
    );
}
