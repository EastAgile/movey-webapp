use diesel::result::Error as DBError;
use jelly::accounts::User;
use jelly::actix_session::UserSession;
use jelly::actix_web::http::header;
use jelly::actix_web::{HttpMessage, HttpRequest};
use jelly::anyhow::anyhow;
use jelly::error::Error;
use jelly::prelude::*;
use jelly::Result;

use crate::accounts::Account;
use crate::constants;

pub async fn is_authenticated(request: &HttpRequest) -> Result<bool> {
    Ok(request.is_authenticated()? && renew_token(request).await?)
}

pub fn clear_cookie(request: &HttpRequest) -> HttpResponse {
    request.get_session().clear();
    HttpResponse::Unauthorized()
        .header(header::SET_COOKIE, constants::REMEMBER_ME_TOKEN_INVALIDATE)
        .body("")
}

pub async fn renew_token(request: &HttpRequest) -> Result<bool> {
    if request.get_session().get::<User>("sku")?.is_none() {
        if let Some(cookie) = request.cookie("remember_me_token") {
            let cookie = cookie.value();
            let index = cookie
                .find('=')
                .ok_or_else(|| anyhow!("Error parsing cookie: Should be key=value."))?;
            let uid = &cookie[index + 1..]
                .parse::<i32>()
                .map_err(|e| anyhow!("Error parsing user id from cookie: {:?}", e))?;

            let account = match Account::get(*uid, request.db_pool()?).await {
                Ok(account) => account,
                Err(Error::Database(DBError::NotFound)) => {
                    error!(
                        "Account from remember me token doesn't exist or have been removed. uid={}",
                        uid
                    );
                    return Ok(false);
                }
                Err(e) => {
                    error!(
                        "Error getting account from remember me token: {:?}. uid={}",
                        e, uid
                    );
                    return Err(Error::Anyhow(anyhow!(
                        "Error getting account from remember me token"
                    )));
                }
            };

            let user = User {
                id: account.id,
                name: account.name,
                is_admin: account.is_admin,
                is_anonymous: false,
            };
            request.set_user(user)?;
        }
    }
    Ok(true)
}
