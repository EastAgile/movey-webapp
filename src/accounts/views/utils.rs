use jelly::prelude::*;
use jelly::request::DatabasePool;
use jelly::Result;

use crate::accounts::Account;
#[cfg(test)]
use crate::test::mock::MockHttpRequest as HttpRequest;
use jelly::accounts::OneTimeUseTokenGenerator;
#[cfg(not(test))]
use jelly::actix_web::HttpRequest;

/// Decodes the pieces used in verify and reset-password URL structures,
/// and validates them. If they're valid, it will return the Account in
/// question - if not, it will raise a generic error.
///
/// Flows should silence this error and display a generic message to
/// the user to avoid leaking information.
pub fn validate_token(
    request: &HttpRequest,
    uidb64: &str,
    ts: &str,
    token: &str,
) -> Result<Account> {
    if let Ok(uid_bytes) = base64_url::decode(&uidb64) {
        if let Ok(uid_str) = std::str::from_utf8(&uid_bytes) {
            if let Ok(uid) = uid_str.parse::<i32>() {
                let db = request.db_pool()?;

                if let Ok(account) = Account::get(uid, db) {
                    // Actix-web route params are iffy here, so...
                    // we rebuild the full token before passing in.
                    let token = format!("{}-{}", ts, token);

                    if account.is_token_valid(&token) {
                        return Ok(account);
                    }
                }
            }
        }
    }

    Err(Error::InvalidAccountToken)
}
