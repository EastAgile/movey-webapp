//! A custom Error type, along with a custom Result wrapper, that we use for
//! returning responses. This module handles converting several differing
//! error formats into the one we use for responding.

use actix_web::{HttpResponse, ResponseError};
use std::{error, fmt};

/// This enum represents the largest classes of errors we can expect to
/// encounter in the lifespan of our application. Feel free to add to this
/// as necessary; `Generic()` exists for anything further in the stack that
/// might not fit here by default.
#[derive(Debug)]
pub enum Error {
    ActixWeb(actix_web::error::Error),
    Anyhow(anyhow::Error),
    Database(sqlx::Error),
    Generic(String),
    Template(tera::Error),
    Json(serde_json::error::Error),
    Radix(radix::RadixErr),
    InvalidPassword,
    InvalidAccountToken,
    PasswordHasher(djangohashers::HasherError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::ActixWeb(e) => Some(e),
            Error::Anyhow(e) => Some(e.root_cause()),
            Error::Database(e) => Some(e),
            Error::Template(e) => Some(e),
            Error::Json(e) => Some(e),
            Error::Radix(e) => Some(e),

            Error::Generic(_)
            | Error::InvalidPassword
            | Error::InvalidAccountToken
            | Error::PasswordHasher(_) => None,
        }
    }
}

impl From<actix_web::error::Error> for Error {
    fn from(e: actix_web::error::Error) -> Self {
        Error::ActixWeb(e)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(e: serde_json::error::Error) -> Self {
        Error::Json(e)
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Error::Database(e)
    }
}

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        Error::Anyhow(e)
    }
}

impl From<tera::Error> for Error {
    fn from(e: tera::Error) -> Self {
        Error::Template(e)
    }
}

impl From<radix::RadixErr> for Error {
    fn from(e: radix::RadixErr) -> Self {
        Error::Radix(e)
    }
}

impl From<djangohashers::HasherError> for Error {
    fn from(e: djangohashers::HasherError) -> Self {
        Error::PasswordHasher(e)
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError()
            .content_type("text/html; charset=utf-8")
            .body(&render(self))
    }
}

/// A generic method for rendering an error to present to the browser.
/// This should only be called in non-production settings.
pub(crate) fn render<E: std::fmt::Debug>(e: E) -> String {
    format!(
        r#"<!DOCTYPE html>
        <html>
        <head>
            <meta http-equiv="Content-Type" content="text/html; charset=utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0, user-scalable=no, maximum-scale=1.0">
            <title>Jelly: An Error Occurred</title>
            <style>
                html, body {{
                    margin: 0;
                    padding: 0;
                    background: #F0DEE0;
                    color: #111;
                    font-family: -apple-system, "Helvetica Neue", Helvetica, "Segoe UI", Ubuntu, arial, sans-serif;
                }}
                
                h1 {{ margin: 0; background: #F05758; border-bottom: 1px solid #C7484A; padding: 20px; font-size: 30px; font-weight: 600; line-height: 40px; }}
                
                code {{
                    display: block;
                    font-family: "Anonymous Pro", Consolas, Menlo, Monaco, Lucida Console, Liberation Mono, DejaVu Sans Mono, Bitstream Vera Sans Mono, Courier New, monospace, serif; 
                    font-size: 16px;
                    line-height: 20px;
                    padding: 20px;
                }}
            </style>
        </head>
        <body>
            <h1>Error</h1>
            <code>{:#?}<code>
        </body>
        </html>
    "#,
        e
    )
}
