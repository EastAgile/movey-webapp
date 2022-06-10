use jelly::guards::Auth;

pub mod paginate;
pub mod token;

pub fn new_auth() -> Auth {
    Auth {
        redirect_to: "/accounts/login/",
    }
}
