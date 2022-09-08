pub mod views;

use jelly::actix_web::web::{get, resource, scope, ServiceConfig};

pub fn configure(config: &mut ServiceConfig) {
    config.service(
        scope("/users")
            .service(resource("/{account_slug}").route(get().to(views::get_public_profile))),
    );
}
