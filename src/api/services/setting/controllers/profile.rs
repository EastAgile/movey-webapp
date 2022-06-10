use crate::accounts::Account;
use crate::request;
use jelly::actix_web::http::header::ContentType;
use jelly::prelude::*;
use jelly::Result;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct LoggedInUser {
    pub id: i32,
    pub name: String,
    pub email: String
}

pub async fn get_logged_in_user(request: HttpRequest) -> Result<HttpResponse> {
    if !request::is_authenticated(&request).await? {
        return Ok(HttpResponse::Ok().set(ContentType::json()).body("{}"))
    }
    let user = request.user()?;
    let db = request.db_pool()?;
    let account = Account::get(user.id, db).await?;
    let logged_in_user = LoggedInUser {
        id: account.id,
        name: account.name,
        email: account.email
    };
    Ok(HttpResponse::Ok().set(ContentType::json()).json(&logged_in_user))
}
