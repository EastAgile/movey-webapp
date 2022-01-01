//! Admin dashboard.

use jelly::actix_web::web::{resource, scope, ServiceConfig};
use jelly::guards::Auth;

mod views;

pub fn configure(config: &mut ServiceConfig) {
    let guard = Auth {
        redirect_to: "/accounts/login/",
    };

    config.service(
        scope("/dashboard/")
            .wrap(guard)
            // Index
            .service(resource("").to(views::dashboard)),
    );
}
