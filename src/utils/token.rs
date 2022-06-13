use rand::{distributions::Uniform, rngs::OsRng, Rng};
use sha2::{Digest, Sha256};

const TOKEN_LENGTH: usize = 32;

pub struct SecureToken {
    pub sha256: String,
}

impl SecureToken {
    pub fn generate() -> NewSecureToken {
        let plaintext = generate_secure_alphanumeric_string(TOKEN_LENGTH);
        let sha256 = Self::hash(&plaintext);
        NewSecureToken {
            plaintext,
            inner: Self { sha256 },
        }
    }

    pub fn hash(plaintext: &str) -> String {
        let sha256 = Sha256::digest(plaintext.as_bytes());
        format!("{:x?}", sha256.as_slice())
    }
}

pub struct NewSecureToken {
    pub plaintext: String,
    pub inner: SecureToken,
}

pub fn generate_secure_alphanumeric_string(len: usize) -> String {
    const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

    OsRng
        .sample_iter(Uniform::from(0..CHARS.len()))
        .map(|idx| CHARS[idx] as char)
        .take(len)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::utils::token::{generate_secure_alphanumeric_string, SecureToken, TOKEN_LENGTH};

    #[actix_rt::test]
    async fn generate_secure_alphanumeric_string_works() {
        let token = generate_secure_alphanumeric_string(TOKEN_LENGTH);
        assert_eq!(token.len(), TOKEN_LENGTH);
    }

    #[actix_rt::test]
    async fn generate_secure_alphanumeric_string_works_with_zero_len() {
        let plain_token = generate_secure_alphanumeric_string(0);
        assert_eq!(plain_token.len(), 0);
        assert_eq!(plain_token, String::from(""));
    }

    #[actix_rt::test]
    async fn secure_token_generate_works() {
        let token = SecureToken::generate();
        assert_eq!(token.plaintext.len(), TOKEN_LENGTH);
    }

    #[actix_rt::test]
    async fn secure_token_hash_works() {
        let formatted_sha256 = SecureToken::hash(&"qwerty123456".to_string());
        assert_eq!(formatted_sha256, "[3a, 57, 45, a0, 5f, 87, dd, ee, 1d, b6, 8b, 21, 7d, c0, 43, bf, a2, 6, d1, c7, aa, a1, dd, a, 7d, d7, 6b, 85, 2a, 73, 35, 97]");
    }
}
