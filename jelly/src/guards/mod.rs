//! Guards that can be (and commonly are) used for requests.

use actix_web::guard::{Guard, Header};

pub mod auth;
pub use auth::{Auth, AuthMiddleware};

pub fn accepts_json() -> impl Guard {
    Header("content-type", "application/json")
}

pub fn accepts_form() -> impl Guard {
    Header("content-type", "application/x-www-form-urlencoded")
}
