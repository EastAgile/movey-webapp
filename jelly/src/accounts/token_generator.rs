use std::env;

use chrono::{TimeZone, Utc};
use constant_time_eq::constant_time_eq;
use hmac::{Hmac, Mac, NewMac};
use radix::RadixNum;
use sha2::Sha256;

use crate::error::Error;

type HmacSha256 = Hmac<Sha256>;

const KEY_SALT: &str = "com.jelly.accounts.token_generator";

/// Returns the number of seconds since 2001. Used for comparisons.
fn num_seconds() -> i64 {
    let now = Utc::now();
    let y2k = Utc.ymd(2001, 1, 1).and_hms(0, 0, 0);
    now.signed_duration_since(y2k).num_seconds()
}

/// Hashes our value, using a combination of our SECRET_KEY and
/// KEY_SALT values.
fn hash(value: &str, timestamp: u64) -> Result<String, Error> {
    let value = format!("{}{}", value, timestamp);

    let mut ts: RadixNum = timestamp.into();
    ts = ts.with_radix(36)?;

    // This is enforced at server startup, so it's safe to do here...
    // but we'll .expect() to provide some clarity to be safe.
    let secret_key =
        env::var("SECRET_KEY").expect("Unable to pull SECRET_KEY for account token generation");

    let key = format!("{}{}", KEY_SALT, secret_key);
    let mut hasher = HmacSha256::new_varkey(key.as_bytes())
        .map_err(|e| Error::Generic(format!("Error generating HMACSHA256: {:?}", e)))?;

    hasher.update(value.as_bytes());

    let result = hasher.finalize();

    // A "straightforward" way to adapt Python's [::2] syntax.
    let hash = format!("{:x}", result.into_bytes())
        .split("")
        .enumerate()
        .filter(|&(idx, _)| idx == 0 || (idx - 1) % 2 == 0)
        .map(|(_, c)| c)
        .collect::<Vec<&str>>()
        .join("");

    Ok(format!("{}-{}", ts.as_str().to_lowercase(), hash))
}

/// An entry point for models to implement to enable reset password
/// and verification logic.
pub trait OneTimeUseTokenGenerator {
    /// The value that should be used as the token hash. Ideally, you want
    /// this to be filled with information that is guaranteed to be different
    /// after this flow (e.g, a last login field, a password field).
    ///
    /// For example, Django uses:
    ///
    /// {user.pk}{user.password}{login_timestamp}{timestamp}{email}
    fn hash_value(&self) -> String;

    /// Returns a verification token that can be used in a URL.
    /// Expires after env var PASSWORD_RESET_TIMEOUT (or 259200
    /// if not configured).
    fn create_reset_token(&self) -> Result<String, Error> {
        let value = self.hash_value();
        let since = num_seconds();
        hash(&value, since as u64)
    }

    /// Validates that the token we received is still acceptable;
    /// internally this does both constant time comparison checks
    /// as well as timestamp validation.
    fn is_token_valid(&self, token: &str) -> bool {
        // Try to split the token, barf if a bad format is found.
        let split = token.split("-").collect::<Vec<&str>>();
        if split.len() != 2 {
            return false;
        }

        // We intentionally ignore a class of errors and will simply report
        // to the user that the token is invalid or expired.
        if let Ok(timestamp) = RadixNum::from_str(split[0], 36) {
            if let Ok(ts) = timestamp.as_decimal() {
                let value = self.hash_value();

                let cmp_token = hash(&value, ts as u64);
                if cmp_token.is_err() {
                    return false;
                }

                // This is important - must be constant time or it's vulnerable to a
                // timing attack.
                if !constant_time_eq(cmp_token.unwrap().as_bytes(), token.as_bytes()) {
                    return false;
                }

                // A bit kludgy, but it works fine.
                let timeout = match env::var("PASSWORD_RESET_TIMEOUT") {
                    Ok(v) => {
                        if let Ok(t) = v.parse::<usize>() {
                            t
                        } else {
                            259200
                        }
                    }

                    Err(_) => 259200,
                };

                let since = num_seconds() as usize;
                if (since - ts) > timeout {
                    return false;
                }

                return true;
            }
        }

        false
    }
}
