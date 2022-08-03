pub const DEFAULT_AVATAR_STYLE: &str = "monsterid";
pub const RATING: &str = "pg";
pub const DEFAULT_AVATAR_SIZE: u16 = 200;

#[derive(Clone)]
pub struct Gravatar {
    email: String,
    size: Option<u16>,
}
impl Gravatar {
    pub fn new(email: &str, size: Option<u16>) -> Gravatar {
        Gravatar {
            email: email.to_string(),
            size,
        }
    }
    /// Returns the image URL of the user's Gravatar with all specified parameters.
    pub fn image_url(self: &Self) -> String {
        // Generate MD5 hash of email
        let digest = md5::compute(&self.email.trim().to_ascii_lowercase().as_bytes());
        let hash = format!("{:x}", digest);
        // Create base URL using the hash
        let mut url = format!("https://secure.gravatar.com/avatar/{}", hash);

        if let Some(s) = self.size {
            url = format!("{}?s={}", url, s);
        } else {
            url = format!("{}?s={}", url, DEFAULT_AVATAR_SIZE);
        }

        url = format!("{}&d={}&r={}", url, DEFAULT_AVATAR_STYLE, RATING);
        url
    }
}

#[test]
fn test_base_url() {
    let url = Gravatar::new("email@example.com", None).image_url();
    assert_eq!(
        url.as_str(),
        "https://secure.gravatar.com/avatar/5658ffccee7f0ebfda2b226238b1eb6e?s=200&d=monsterid&r=pg"
    );
}

#[test]
fn test_hash_procedure() {
    let url = Gravatar::new("  EMaiL@exAMplE.cOm ", None).image_url();
    assert_eq!(
        url.as_str(),
        "https://secure.gravatar.com/avatar/5658ffccee7f0ebfda2b226238b1eb6e?s=200&d=monsterid&r=pg"
    );
}

#[test]
fn test_size() {
    let url = Gravatar::new("email@example.com", Some(50)).image_url();
    assert_eq!(
        url.as_str(),
        "https://secure.gravatar.com/avatar/5658ffccee7f0ebfda2b226238b1eb6e?s=50&d=monsterid&r=pg"
    );
}