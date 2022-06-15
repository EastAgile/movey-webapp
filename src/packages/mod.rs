use jelly::actix_web::web::{get, resource, scope, ServiceConfig};

pub mod models;
pub mod views;

pub use models::{Package, PackageVersion, PackageVersionSort, NewPackage, NewPackageVersion};

pub fn configure(config: &mut ServiceConfig) {
    config.service(
        scope("/packages/")
            .service(
                resource("/{package_name}/versions")
                    .route(get().to(views::controller::show_package_versions)),
            )
            .service(
                resource("/search")
                    .route(get().to(views::controller::show_search_results)),
            )
            .service(
                resource("/owned")
                .route(get().to(views::controller::show_owned_packages)),
            )
            .service(
                resource("/{package_name}")
                .route(get().to(views::controller::show_package)),
            )
            .service(
                resource("")
                    .route(get().to(views::controller::packages_index)),
            )
    );
}
