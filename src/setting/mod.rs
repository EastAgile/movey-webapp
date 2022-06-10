mod views;
pub mod models;

use jelly::actix_web::web::{get, resource, scope, ServiceConfig};
use jelly::guards::Auth;

pub fn configure(config: &mut ServiceConfig) {
    let guard = Auth {
        redirect_to: "/accounts/login/",
    };

    config.service(
        scope("/settings")
            .wrap(guard)
            .service(resource("/tokens").route(get().to(views::token::form)))
    );
}
