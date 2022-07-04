use crate::accounts::Account;
use crate::schema::accounts;
use crate::schema::accounts::dsl::*;
use crate::schema::api_tokens;
use crate::schema::api_tokens::dsl::*;
use crate::utils::token::SecureToken;
use diesel::prelude::*;
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
        account: &Account,
        api_key_name: &str,
        pool: &DieselPgPool,
    ) -> Result<CreatedApiToken> {
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

    pub async fn revoke_by_id(owner_id: i32, pool: &DieselPgPool) -> Result<()> {
        let connection = pool.get()?;

        diesel::delete(api_tokens.filter(account_id.eq(owner_id)))
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
    use crate::settings::models::token::ApiToken;
    use crate::test::{DatabaseTestContext, DB_POOL};
    use diesel::result::DatabaseErrorKind;
    use diesel::result::Error::DatabaseError;
    use jelly::error::Error;
    use jelly::forms::{EmailField, PasswordField};

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

    #[actix_rt::test]
    async fn get_token_works() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();

        let account = setup_user().await;

        let new_api_token = ApiToken::insert(&account, "name1", &DB_POOL).await.unwrap();
        let token_id = ApiToken::get(&new_api_token.plaintext, &DB_POOL).await.unwrap();
        assert_eq!(token_id, 1);
    }

    #[actix_rt::test]
    async fn revoke_works() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();

        let account = setup_user().await;

        let token1 = ApiToken::insert(&account, "name1", &DB_POOL).await.unwrap();
        ApiToken::insert(&account, "name2", &DB_POOL).await.unwrap();
        assert_eq!(ApiToken::get_by_account(account.id, &DB_POOL).await.unwrap().len(), 2);
        
        ApiToken::revoke(token1.model.id, &DB_POOL).await.unwrap();
        let tokens = ApiToken::get_by_account(account.id, &DB_POOL).await.unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].name, "name2");
    }

    #[actix_rt::test]
    async fn revoke_by_id_works() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();

        let account = setup_user().await;

        ApiToken::insert(&account, "name1", &DB_POOL).await.unwrap();
        ApiToken::insert(&account, "name2", &DB_POOL).await.unwrap();
        assert_eq!(ApiToken::get_by_account(account.id, &DB_POOL).await.unwrap().len(), 2);
        
        ApiToken::revoke_by_id(account.id, &DB_POOL).await.unwrap();
        assert_eq!(ApiToken::get_by_account(account.id, &DB_POOL).await.unwrap().len(), 0);
    }
}
