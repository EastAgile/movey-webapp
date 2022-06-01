use jelly::guards::Auth;

pub mod paginate;

pub fn new_auth() -> Auth {
    Auth {
        redirect_to: "/accounts/login/",
    }
}