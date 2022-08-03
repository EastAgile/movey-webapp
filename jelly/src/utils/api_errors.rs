use serde_json::json;
use super::error_constants::*;
use std::error::Error;
use crate::utils::HttpResponse;

pub fn bad_request(msg: &str, e: &Box<dyn Error>) -> HttpResponse {
    warn!("error: {:?}", e);
    HttpResponse::BadRequest()
        .json(json!({
                "ok": false,
                "msg": msg
            })
        )
}

pub fn server_error(e: &Box<dyn Error>) -> HttpResponse {
    error!("error: {:?}", e);
    let msg = MSG_UNEXPECTED_ERROR;
    HttpResponse::InternalServerError()
        .json(json!({
                "ok": false,
                "msg": msg,
            })
        )
}

pub fn unauthorized(msg: &str, e: &Box<dyn Error>) -> HttpResponse {
    warn!("error: {:?}", e);
    HttpResponse::Unauthorized()
        .json(json!({
                "ok": false,
                "msg": msg,
            })
        )
}

pub fn forbidden(msg: &str, e: &Box<dyn Error>) -> HttpResponse {
    warn!("error: {:?}", e);
    HttpResponse::Forbidden()
        .json(json!({
                "ok": false,
                "msg": msg,
            })
        )
}

pub fn not_found(msg: &str, e: &Box<dyn Error>) -> HttpResponse {
    warn!("error: {:?}", e);
    HttpResponse::InternalServerError()
        .json(json!({
                "ok": false,
                "msg": msg,
            })
        )
}
