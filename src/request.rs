use jelly::accounts::User;
use jelly::actix_session::UserSession;
use jelly::actix_web::{HttpMessage, HttpRequest};
use jelly::prelude::*;
use jelly::Result;

use crate::accounts::Account;

pub async fn is_authenticated(request: &HttpRequest) -> Result<bool> {
    if request.is_authenticated()? {
        return renew_token(request).await;
    }
    Ok(false)
}

pub async fn renew_token(request: &HttpRequest) -> Result<bool> {
    if let None = request.get_session().get::<User>("sku")? {
        let remember_me_cookie = request.cookie("remember_me_token");
        if let Some(cookie) = remember_me_cookie {
            let cookie = cookie.value();
            let index = cookie.find("=").unwrap();
            let uid = &cookie[index + 1..].parse::<i32>().unwrap();

            let acc = Account::get(*uid, request.db_pool()?).await?;
            let user = User {
                id: acc.id,
                name: acc.name,
                is_admin: acc.is_admin,
                is_anonymous: false,
            };
            request.set_user(user)?;
        }
    }
    Ok(true)
}
