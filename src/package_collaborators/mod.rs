pub mod models;
pub mod views;
pub mod jobs;

use jelly::actix_web::web::{get, resource, scope, ServiceConfig};
pub use models::package_collaborator;

pub fn configure(config: &mut ServiceConfig) {
    config.service(scope("/collaborators").service(
        resource("/accept/{token}").route(get().to(views::invitation::accept_invite_with_token)),
    ));
}
