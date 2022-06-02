use jelly::actix_web::web::{get, post, resource, scope, ServiceConfig};
pub mod views;

use crate::utils::new_auth;

pub fn configure(config: &mut ServiceConfig) {
    let guard = new_auth();

    config.service(
        scope("/settings")
            .wrap(guard)
            .service(
                resource("/profile")
                    .route(get().to(views::controller::profile))
                    .route(post().to(views::controller::change_password)),
            )
    );
}
