use crate::settings::models::token::CreatedApiToken;
use jelly::chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct EncodableApiTokenWithToken {
    pub id: i32,
    pub name: String,
    pub token: String,
    pub created_at: NaiveDateTime,
    pub last_used_at: Option<NaiveDateTime>,
}

impl From<CreatedApiToken> for EncodableApiTokenWithToken {
    fn from(token: CreatedApiToken) -> Self {
        EncodableApiTokenWithToken {
            id: token.model.id,
            name: token.model.name,
            token: token.plaintext,
            created_at: token.model.created_at,
            last_used_at: token.model.last_used_at,
        }
    }
}
