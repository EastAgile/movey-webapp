//! Admin dashboard.

use crate::utils::new_auth;
use jelly::actix_web::web::{resource, scope, ServiceConfig};

mod views;

pub fn configure(config: &mut ServiceConfig) {
    let guard = new_auth();

    config.service(
        scope("/dashboard")
            .wrap(guard)
            // Index
            .service(resource("").to(views::dashboard)),
    );
}
