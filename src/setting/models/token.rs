use crate::accounts::Account;
use crate::schema::api_tokens;
use crate::utils::token::SecureToken;
use diesel::{Associations, ExpressionMethods, Identifiable, Queryable, RunQueryDsl};
use jelly::chrono::NaiveDateTime;
use jelly::serde::Serialize;
use jelly::DieselPgPool;
use jelly::Result;

#[derive(Clone, Debug, PartialEq, Eq, Identifiable, Queryable, Associations, Serialize)]
#[belongs_to(Account)]
pub struct ApiToken {
    pub id: i32,
    pub account_id: i32,
    pub token: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub last_used_at: Option<NaiveDateTime>,
}

impl ApiToken {
    /// Generates a new named API token for a user
    pub async fn insert(
        uid: i32,
        api_key_name: &str,
        pool: &DieselPgPool,
    ) -> Result<CreatedApiToken> {
        let connection = pool.get()?;
        let secure_token = SecureToken::generate();
        let model: ApiToken = diesel::insert_into(api_tokens::table)
            .values((
                api_tokens::account_id.eq(uid),
                api_tokens::name.eq(api_key_name),
                api_tokens::token.eq(&secure_token.inner.sha256),
            ))
            .get_result(&connection)?;

        Ok(CreatedApiToken {
            plaintext: secure_token.plaintext,
            model,
        })
    }
}

pub struct CreatedApiToken {
    pub model: ApiToken,
    pub plaintext: String,
}

#[cfg(test)]
mod tests {
    use crate::accounts::forms::NewAccountForm;
    use crate::accounts::Account;
    use crate::setting::models::token::ApiToken;
    use crate::test::{DatabaseTestContext, DB_POOL};
    use diesel::result::DatabaseErrorKind;
    use diesel::result::Error::DatabaseError;
    use jelly::error::Error;
    use jelly::forms::{EmailField, PasswordField};

    async fn setup_user() -> i32 {
        let form = NewAccountForm {
            email: EmailField {
                value: "email@host.com".to_string(),
                errors: vec![],
            },
            password: PasswordField {
                value: "So$trongpas0word!".to_string(),
                errors: vec![],
                hints: vec![],
            },
        };
        Account::register(&form, &DB_POOL).await.unwrap()
    }

    #[actix_rt::test]
    async fn api_token_insert_works() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();

        let uid = setup_user().await;
        let new_api_token = ApiToken::insert(uid, "name1", &DB_POOL).await.unwrap();
        assert_eq!(new_api_token.plaintext.len(), 32);
        assert_eq!(new_api_token.model.account_id, uid);
        assert_eq!(new_api_token.model.name, "name1");
    }

    #[actix_rt::test]
    async fn api_token_insert_failed_with_duplicated_name() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();

        let uid = setup_user().await;
        ApiToken::insert(uid, "name1", &DB_POOL).await.unwrap();
        match ApiToken::insert(uid, "name1", &DB_POOL).await {
            Err(Error::Database(DatabaseError(DatabaseErrorKind::UniqueViolation, _))) => (),
            _ => panic!(),
        }
    }
}
