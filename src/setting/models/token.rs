use crate::accounts::Account;
use crate::schema::api_tokens;
use crate::schema::api_tokens::dsl::*;
use crate::utils::token::SecureToken;
use diesel::prelude::*;
use diesel::{Associations, ExpressionMethods, Identifiable, Queryable, RunQueryDsl};

use jelly::chrono::NaiveDateTime;
use jelly::error::Error;
use jelly::serde::Serialize;
use jelly::DieselPgPool;
use jelly::Result;
use sha2::{Digest, Sha256};

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
        account: &Account,
        api_key_name: &str,
        pool: &DieselPgPool,
    ) -> Result<CreatedApiToken> {
        ApiToken::max_token_reached(account, pool).await?;
        let connection = pool.get()?;
        let secure_token = SecureToken::generate();
        let model: ApiToken = diesel::insert_into(api_tokens::table)
            .values((
                api_tokens::account_id.eq(account.id),
                api_tokens::name.eq(api_key_name),
                api_tokens::token.eq(&secure_token.inner.sha256),
            ))
            .get_result(&connection)?;

        Ok(CreatedApiToken {
            plaintext: secure_token.plaintext,
            model,
        })
    }

    pub async fn max_token_reached(account: &Account, pool: &DieselPgPool) -> Result<()> {
        let connection = pool.get()?;
        let max_token_per_user = std::env::var("MAX_TOKEN")
            .expect("MAX_TOKEN not set!")
            .parse::<i64>()
            .unwrap();
        let count: i64 = ApiToken::belonging_to(account)
            .count()
            .get_result(&connection)?;

        if count < max_token_per_user {
            Ok(())
        } else {
            Err(Error::Generic(String::from("Too many tokens created.")))
        }
    }

    pub async fn get(api_token: &String, pool: &DieselPgPool) -> Result<i32> {
        let connection = pool.get()?;
        let sha256 = Sha256::digest(api_token.as_bytes());
        let sha256 = format!("{:x?}", sha256.as_slice());
        let result = api_tokens.filter(api_tokens::token.eq(sha256))
            .select(id).first::<i32>(&connection)?;
        Ok(result)
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
    use std::env;
    async fn setup_user() -> Account {
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
        let uid = Account::register(&form, &DB_POOL).await.unwrap();
        Account::get(uid, &DB_POOL).await.unwrap()
    }

    #[actix_rt::test]
    async fn api_token_insert_works() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();

        let account = setup_user().await;
        let new_api_token = ApiToken::insert(&account, "name1", &DB_POOL).await.unwrap();
        assert_eq!(new_api_token.plaintext.len(), 32);
        assert_eq!(new_api_token.model.account_id, account.id);
        assert_eq!(new_api_token.model.name, "name1");
    }

    #[actix_rt::test]
    async fn api_token_insert_failed_with_duplicated_name() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();

        let account = setup_user().await;
        ApiToken::insert(&account, "name1", &DB_POOL).await.unwrap();
        match ApiToken::insert(&account, "name1", &DB_POOL).await {
            Err(Error::Database(DatabaseError(DatabaseErrorKind::UniqueViolation, _))) => (),
            _ => panic!(),
        }
    }

    #[actix_rt::test]
    async fn api_token_max_token_reached_works() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();
        env::set_var("MAX_TOKEN", "2");

        let account = setup_user().await;

        ApiToken::max_token_reached(&account, &DB_POOL)
            .await
            .unwrap();
        ApiToken::insert(&account, "name1", &DB_POOL).await.unwrap();
        ApiToken::max_token_reached(&account, &DB_POOL)
            .await
            .unwrap();
        ApiToken::insert(&account, "name2", &DB_POOL).await.unwrap();

        if let Err(Error::Generic(message)) = ApiToken::insert(&account, "name3", &DB_POOL).await {
            assert_eq!(message, "Too many tokens created.")
        } else {
            panic!()
        }

        if let Err(Error::Generic(message)) = ApiToken::max_token_reached(&account, &DB_POOL).await {
            assert_eq!(message, "Too many tokens created.")
        } else {
            panic!()
        }
    }

    #[actix_rt::test]
    async fn get_token_works() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();

        let account = setup_user().await;
        let new_api_token = ApiToken::insert(&account, "name1", &DB_POOL).await.unwrap();
        let token_id = ApiToken::get(&new_api_token.plaintext, &DB_POOL).await.unwrap();
        assert_eq!(token_id, 1);
    }

}
