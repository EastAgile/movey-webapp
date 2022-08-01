use jelly::guards::Auth;

pub mod paginate;
pub mod presenter;
pub mod token;
pub mod request_utils;
#[cfg(test)]
pub mod tests;
pub mod token;

pub fn new_auth() -> Auth {
    Auth {
        redirect_to: "/accounts/login/",
    }
}
