use actix_session::UserSession;
use actix_web::{
    cookie::{CookieJar, Key, Cookie},
    HttpRequest, HttpMessage,
};

use crate::accounts::User;
use crate::error::Error;


/// `Authentication` is kind of a request guard - it returns a Future which will resolve
/// with either the current authenticated user, or "error" out if the user has no session data
/// that'd tie them to a user profile, or if the session cache can't be read, or if the database
/// has issues, or... pick your poison I guess.
///
pub trait Authentication {
    /// Returns whether a user session exists and is valid.
    fn is_authenticated(&self) -> Result<bool, Error>;

    /// Sets a serializable user instance.
    fn set_user(&self, account: User) -> Result<(), Error>;

    /// Returns a User, if it can be extracted properly.
    fn user(&self) -> Result<User, Error>;
}

impl Authentication for HttpRequest {
    #[inline(always)]
    fn is_authenticated(&self) -> Result<bool, Error> {
        if self
            .get_session()
            .get::<serde_json::Value>("sku")?
            .is_some() {
            Ok(true)
        }
        else {
            let my_cookie = match self.cookie("remember_me_token") {
                Some(val) => val.value().to_owned(),
                None => return Ok(false)
            };
            let index = match my_cookie.find("=") {
                Some(i) => i,
                None => return Ok(false)
            };
            let user_id = &my_cookie[index + 1..];

            let key = std::env::var("SECRET_KEY").expect("SECRET_KEY not set!");
            let mut jar = CookieJar::new();
            jar.signed(&Key::derive_from(key.as_bytes()))
                .add(Cookie::new("re_signed", user_id.to_owned()));

            let is_my_cookie = my_cookie.contains(jar.get("re_signed").unwrap().value());
            
            Ok(is_my_cookie)
        }
    }

    fn set_user(&self, account: User) -> Result<(), Error> {
        self.get_session().set("sku", account)?;
        Ok(())
    }

    fn user(&self) -> Result<User, Error> {
        match self.get_session().get("sku")? {
            Some(user) => Ok(user),
            None => Ok(User::default()),
        }
    }
}
