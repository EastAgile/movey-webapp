use jelly::actix_web::web::{get, post, resource, scope, ServiceConfig};
pub mod views;
pub mod models;

use jelly::prelude::*;
use jelly::Result;

use crate::utils::new_auth;

pub async fn show_packages(request: HttpRequest) -> Result<HttpResponse> {
    request.render(200, "settings/user_packages.html", {
        let mut context = Context::new();
        context
    })
}


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
            .service(
                resource("/packages")
                .route(get().to(views::controller::show_packages)),
            )
            .service(
                resource("/downloads")
                .route(get().to(views::controller::show_downloads)),
            )
            .service(
                resource("/tokens")
                .route(get().to(views::controller::show_tokens)),
            )
            
    );
}