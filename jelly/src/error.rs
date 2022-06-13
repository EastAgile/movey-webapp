//! A custom Error type, along with a custom Result wrapper, that we use for
//! returning responses. This module handles converting several differing
//! error formats into the one we use for responding.

use actix_web::{HttpResponse, ResponseError};
use std::{error, fmt};
use diesel::{
    r2d2::PoolError,
    result::{Error as DBError},
};

/// This enum represents the largest classes of errors we can expect to
/// encounter in the lifespan of our application. Feel free to add to this
/// as necessary; `Generic()` exists for anything further in the stack that
/// might not fit here by default.
#[derive(Debug)]
pub enum Error {
    ActixWeb(actix_web::error::Error),
    Anyhow(anyhow::Error),
    Pool(PoolError),
    Database(DBError),
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
            Error::Pool(e) => Some(e),
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

impl From<DBError> for Error {
    fn from(e: DBError) -> Self {
        Error::Database(e)
    }
}

impl From<PoolError> for Error {
    fn from(e: PoolError) -> Self {
        Error::Pool(e)
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
        HttpResponse::NotFound()
            .content_type("text/html; charset=utf-8")
            .body(&render(self))
    }
}

/// A generic method for rendering an error to present to the browser.
/// This should only be called in non-production settings.
pub(crate) fn render<E: std::fmt::Debug>(e: E) -> String {
    r#"<!DOCTYPE html>
        <!--[if IE 8]><html class="lt-ie9"><![endif]-->
        <!--[if gt IE 8]><!--><html><!--<![endif]-->
        <head>
            <meta http-equiv="Content-Type" content="text/html; charset=utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0, user-scalable=no, maximum-scale=1.0">
            <title>Not Found</title>
            <link rel="icon" type="image/x-icon" href="/static/resources/Favicon_Light.png">
            <link href="https://fonts.googleapis.com/css2?family=Mulish:wght@400;500;700&family=Syncopate:wght@400;700&display=swap" rel="stylesheet"/>
            <link href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css" rel="stylesheet"/>
            <link href="http://fonts.cdnfonts.com/css/inter" rel="stylesheet"/>
            <link href="https://cdnjs.cloudflare.com/ajax/libs/foundation/6.7.4/css/foundation.min.css" rel="stylesheet" />
            <script src="https://cdnjs.cloudflare.com/ajax/libs/foundation/6.7.4/js/foundation.min.js"></script>
            <script src="https://code.jquery.com/jquery-3.6.0.min.js"></script>

            <link href="/static/css/layout.css" rel="stylesheet"/>
            <link href="/static/css/header_dark.css" rel="stylesheet"/>
            <link href="/static/css/footer.css" rel="stylesheet"/>
            <link href="/static/css/404/404.css" rel="stylesheet"/>
            <script src="/static/js/search/search_bar.js"></script>
            <script src="/static/js/header.js"></script>
            <!--[if lte IE 8]>
            (function(i,e){for(;i<10;i++)document.createElement(e[i]);})(0,['section','article','aside','header','footer','nav','figure','figcaption','time','mark']);
            <![endif]-->
        </head>
        <body>
            <header>
              <div class="header-container">
                <a href="/">
                  <div class="header-logo"></div>
                </a>
                <nav>
                  <ul class="nav-links">
                  </ul>
                  <ul id="right-wrapper">
                    <li>
                      <button class="search-btn" data-active="false">
                        <i id="search-btn-icon" class="fa fa-search"></i>
                      </button>
                    </li>
                    <li class="sign-in-li">
                      <a class="sign-in" href="/accounts/login/">
                        SIGN IN
                      </a>
                    </li>
                    <li class="sign-up-li">
                      <a class="sign-up" href="/accounts/register/">
                        SIGN UP
                      </a>
                    </li>
                    <li id="account-dropdown" class="hide">
                      <a class="profile-link" href="/settings/profile">
                        <button id="account-icon">
                          A
                        </button>
                      </a>
                      <img id="account-dropdown-toggle" src="/static/resources/chevron_down_icon_white.svg">
                      <ul id="account-dropdown-list">
                        <li>
                          <a href="/settings/profile"> Account Settings</a>
                        </li>
                        <li>
                          <form class="logout-form" method="post" action="/accounts/logout/">
                            <a>Sign Out</a>
                          </form>
                        </li>
                      </ul>
                    </li>
                  </ul>
                </nav>
              </div>
            </header>
            <div id="search-bar">
              <form action="/packages/search">
                <button type="submit">
                  <i class="fa fa-search"></i>
                </button>
                <input id="search-field"
                  type="text"
                  placeholder="Search packages..."
                  name="query"
                  autofocus
                >
                <i class="fa fa-times-circle"></i>
              </form>
            </div>
            <script>
              new Header();
              new SearchBar();
            </script>

            <div class="not-found-wrapper">
                <p>Uh oh! We couldn't find that page.</p>
            </div>

            <footer>
              <div class="footer-container">
                <div class="footer-about">
                  <a href="/">
                    <img
                            src="/static/resources/logo_blue.svg"
                            height="30"
                            width="235" />
                  </a>
                  <div class="about-us">
                    We're East Agile, the company behind Movey, the Move package manager, the Movey Registry.
                  </div>
                </div>
                <div class="footer-center">
                  <div class="link-container">
                    <a class="footer-link terms" href="/terms-of-use">TERMS & CONDITIONS</a>
                    <a class="footer-link policy" href="/policy">PRIVACY POLICY</a>
                  </div>
                  <ul class="social-icons">
                    <li>
                      <a class="" href="https://github.com/ea-open-source" target="_blank">
                        <img class="icon" src="/static/resources/github_small.svg"/>
                      </a>
                    </li>
                    <li>
                      <a class="" href="https://twitter.com/MoveyEastAgile" target="_blank">
                        <img class="" src="/static/resources/twitter_small.svg"/>
                      </a>
                    </li>
                  </ul>
                </div>
                <div class="footer-copyright">
                  <div class="copyright">
                  © 2022 East Agile. All rights reserved.
                  </div>
                  <a href="https://www.eastagile.com" target="_blank">
                  <img src="/static/resources/EA_logo_primary.svg"
                       height="40"
                       width="160" />
                      </a>
                </div>
              </div>
            </footer>
        </body>
        </html>
    "#.to_string()
}
