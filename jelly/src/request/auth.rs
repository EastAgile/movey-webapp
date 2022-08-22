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
            let remember_me_token = match self.cookie("remember_me_token") {
                Some(cookie) => cookie.value().to_owned(),
                None => return Ok(false)
            };
            const CRYPTOGRAPHIC_KEY_LEN: usize = 43;
            let user_id = &remember_me_token[CRYPTOGRAPHIC_KEY_LEN + 1..];

            let key = std::env::var("SECRET_KEY").expect("SECRET_KEY not set!");
            let mut jar = CookieJar::new();
            jar.signed(&Key::derive_from(key.as_bytes()))
                .add(Cookie::new("re_signed_token", user_id.to_owned()));
            Ok(remember_me_token == jar.get("re_signed_token").unwrap().value())
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
