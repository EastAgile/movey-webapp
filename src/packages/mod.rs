use jelly::actix_web::web::{get, resource, scope, ServiceConfig};

pub mod models;
pub mod views;

pub use models::{NewPackage, NewPackageVersion, Package, PackageVersion, PackageVersionSort};

pub fn configure(config: &mut ServiceConfig) {
    config.service(
        scope("/packages")
            .service(
                //TODO: check for package name having white space
                resource("/{package_slug}/versions")
                    .route(get().to(views::controller::show_package_versions)),
            )
            .service(
                resource("/{package_slug}/collaborators")
                    .route(get().to(views::controller::show_package_settings)),
            )
            .service(resource("/search").route(get().to(views::controller::show_search_results)))
            .service(resource("/owned").route(get().to(views::controller::show_owned_packages)))
            .service(resource("/{package_slug}").route(get().to(views::controller::show_package)))
            .service(resource("").route(get().to(views::controller::packages_index))),
    );
}
