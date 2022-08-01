use jelly::guards::Auth;

pub mod paginate;
pub mod token;
pub mod request_utils;
#[cfg(test)]
pub mod tests;

pub fn new_auth() -> Auth {
    Auth {
        redirect_to: "/accounts/login/",
    }
}
