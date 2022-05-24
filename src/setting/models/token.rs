use diesel::{Identifiable, Queryable, Associations};

use jelly::serde::{Deserialize, Serialize};
use jelly::chrono::{NaiveDate, NaiveDateTime};
use jelly::DieselPgPool;
use jelly::Result;
use crate::accounts::Account;
use crate::utils::token::SecureToken;
use crate::schema::api_tokens;
use crate::schema::api_tokens::dsl::*;

// #[derive(Clone, Debug, PartialEq, Eq, Queryable, Associations, Serialize, Identifiable)]
#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations, Clone)]
#[belongs_to(Account)]
pub struct ApiToken {
    pub id: i32,
    #[serde(skip)]
    pub account_id: i32,
    // #[serde(skip)]
    // token: SecureToken,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub last_used_at: Option<NaiveDateTime>,
}

impl ApiToken {
    /// Generates a new named API token for a user
    pub fn insert(conn: &DieselPgPool, user_id: i32, api_key_name: &str) -> Result<CreatedApiToken> {
        let token = SecureToken::generate();
        // let model: ApiToken = diesel::insert_into(api_tokens::table)
        //     .values((
        //         api_tokens::user_id.eq(user_id),
        //         api_tokens::name.eq(name),
        //         api_tokens::token.eq(&*token),
        //     ))
        //     .get_result(conn)?;
        //
        // Ok(CreatedApiToken {
        //     plaintext: token.plaintext().into(),
        //     model,
        // })
        Ok(CreatedApiToken {
            model: ApiToken {
                id: 0,
                account_id: 0,
                name: "".to_string(),
                created_at: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
                last_used_at: None
            },
            plaintext: "".to_string()
        })
    }
}

pub struct CreatedApiToken {
    pub model: ApiToken,
    pub plaintext: String,
}
// #[cfg(test)]
// mod tests {
//     use diesel::{QueryDsl, RunQueryDsl};
//     use jelly::forms::{EmailField, PasswordField};
//     use crate::accounts::Account;
//     use crate::accounts::forms::NewAccountForm;
//     use crate::schema::api_tokens::dsl::api_tokens;
//     use crate::setting::models::token::ApiToken;
//     use crate::test::{DatabaseTestContext, DB_POOL};
//
//     async fn setup_user() -> i32 {
//         let form = NewAccountForm {
//             email: EmailField {
//                 value: "email@host.com".to_string(),
//                 errors: vec![],
//             },
//             password: PasswordField {
//                 value: "So$trongpas0word!".to_string(),
//                 errors: vec![],
//                 hints: vec![],
//             },
//         };
//         Account::register(&form, &DB_POOL).await.unwrap()
//     }
//
//     #[actix_rt::test]
//     async fn insert_api_token_works() {
//         crate::test::init();
//         let _ctx = DatabaseTestContext::new();
//
//         let uid = setup_user().await;
//         let new_api_token = ApiToken::insert(&DB_POOL, uid, "name1").unwrap();
//         assert_eq!(new_api_token.plaintext.len(), 16);
//         assert_eq!(new_api_token.model.account_id, uid);
//         assert_eq!(new_api_token.model.name, "name1");
//     }
//
// }
