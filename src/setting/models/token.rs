use crate::accounts::Account;
use crate::schema::accounts;
use crate::schema::accounts::dsl::*;
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

    /// Check and return an account with given plaintext api token
    pub async fn associated_account(
        plaintext_token: &str,
        pool: &DieselPgPool,
    ) -> Result<Account> {
        let connection = pool.get()?;
        let formatted_sha256 = SecureToken::hash(&plaintext_token.to_string());

        let matched_token = api_tokens
            .filter(api_tokens::token.eq(formatted_sha256))
            .first::<ApiToken>(&connection)?;

        let account = accounts
            .filter(accounts::id.eq(matched_token.account_id))
            .first::<Account>(&connection)?;

        Ok(account)
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
        let sha256 = SecureToken::hash(api_token);
        let result = api_tokens.filter(api_tokens::token.eq(sha256))
            .select(api_tokens::id).first::<i32>(&connection)?;
        Ok(result)
    }

    pub async fn get_by_account(owner_id: i32, pool: &DieselPgPool) -> Result<Vec<Self>> {
        let connection = pool.get()?;

        let result = api_tokens
            .filter(account_id.eq(owner_id))
            .order_by(api_tokens::dsl::id.desc())
            .load::<Self>(&connection)?;

        Ok(result)
    }

    pub async fn get_by_id(token_id: i32, pool: &DieselPgPool) -> Result<Self> {
        let connection = pool.get()?;

        let result = api_tokens
            .filter(api_tokens::dsl::id.eq(token_id))
            .first::<Self>(&connection)?;

        Ok(result)
    }

    pub async fn revoke(token_id: i32, pool: &DieselPgPool) -> Result<()> {
        let connection = pool.get()?;

        diesel::delete(api_tokens.filter(api_tokens::dsl::id.eq(token_id)))
            .execute(&connection)?;

        Ok(())
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
    async fn api_token_associated_account_works() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();

        let account = setup_user().await;

        let result = ApiToken::insert(&account, "name1", &DB_POOL).await.unwrap();

        if let Ok(associated_account) = ApiToken::associated_account(&result.plaintext, &DB_POOL).await {
            assert_eq!(associated_account.id, account.id)
        } else {
            panic!("Associated account not found!")
        }
    }

    #[actix_rt::test]
    async fn api_token_get_by_account_works() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();

        let account = setup_user().await;

        ApiToken::insert(&account, "name1", &DB_POOL).await.unwrap();
        ApiToken::insert(&account, "name2", &DB_POOL).await.unwrap();

        let results = ApiToken::get_by_account(account.id, &DB_POOL).await.unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].name, "name2");
        assert_eq!(results[1].name, "name1");
    }
}
