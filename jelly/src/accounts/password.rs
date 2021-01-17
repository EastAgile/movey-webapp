use djangohashers::make_password;
use rand::{thread_rng, Rng};

const PASSWORD_LEN: usize = 30;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    abcdefghijklmnopqrstuvwxyz\
    0123456789)(*&^%$#@!~";

/// Generates a random password and returns it hashed.
pub fn make_random_password() -> String {
    let mut rng = thread_rng();

    let password: String = (0..PASSWORD_LEN).map(|_| {
        let idx = rng.gen_range(0..CHARSET.len());
        CHARSET[idx] as char
    }).collect();

    make_password(&password)
}
