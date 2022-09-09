//! URL dispatcher for user account related API endpoints.

use jelly::actix_web::web::ServiceConfig;

pub mod collaborators;
pub mod package;
pub mod setting;

pub fn configure(config: &mut ServiceConfig) {
    collaborators::configure(config);
    package::configure(config);
    setting::configure(config);
}
