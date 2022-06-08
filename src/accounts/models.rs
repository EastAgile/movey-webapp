// Implements a basic Account model, with support for creating/updating/deleting
// users, along with welcome email and verification.

use diesel::prelude::*;
#[allow(unused_imports)]
use diesel::result::Error as DBError;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable};

use jelly::accounts::{OneTimeUseTokenGenerator, User};
use jelly::chrono::{offset, DateTime, Utc};
use jelly::djangohashers::{check_password, make_password};
use jelly::error::Error;
use jelly::error::Error::Generic;
use jelly::serde::{Deserialize, Serialize};
use jelly::DieselPgPool;

use super::forms::{LoginForm, NewAccountForm};
use super::views::verify::GithubOauthUser;
use crate::schema::accounts;
use crate::schema::accounts::dsl::*;

/// A user Account.
#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, AsChangeset)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub is_active: bool,
    pub is_admin: bool,
    pub has_verified_email: bool,
    pub last_login: Option<DateTime<Utc>>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub github_login: Option<String>
}

impl Account {
    pub async fn get(uid: i32, pool: &DieselPgPool) -> Result<Self, Error> {
        let connection = pool.get()?;
        let result = accounts.find(uid).first::<Account>(&connection)?;

        Ok(result)
    }

    pub async fn get_by_email(account_email: &str, pool: &DieselPgPool) -> Result<Self, Error> {
        let connection = pool.get()?;
        let result = accounts
            .filter(email.eq(account_email))
            .first::<Account>(&connection)?;

        Ok(result)
    }

    pub async fn authenticate(form: &LoginForm, pool: &DieselPgPool) -> Result<User, Error> {
        let connection = pool.get()?;
        let user = accounts
            .filter(email.eq(&form.email.value))
            .first::<Account>(&connection)?;
        if !user.has_verified_email {
            return Err(Generic(String::from(
                "Your account has not been activated.",
            )));
        }
        if !check_password(&form.password, &user.password)? {
            return Err(Error::InvalidPassword);
        }

        Ok(User {
            id: user.id,
            name: user.name,
            is_admin: user.is_admin,
            is_anonymous: false,
        })
    }

    pub async fn fetch_email(uid: i32, pool: &DieselPgPool) -> Result<(String, String), Error> {
        let connection = pool.get()?;
        let result = accounts
            .find(uid)
            .select((name, email))
            .first::<(String, String)>(&connection)?;

        Ok(result)
    }

    pub async fn fetch_name_from_email(
        account_email: &str,
        pool: &DieselPgPool,
    ) -> Result<String, Error> {
        let connection = pool.get()?;
        let result = accounts
            .filter(email.eq(account_email))
            .select(name)
            .first::<String>(&connection)?;

        Ok(result)
    }

    pub async fn register(form: &NewAccountForm, pool: &DieselPgPool) -> Result<i32, Error> {
        let connection = pool.get()?;
        let hashword = make_password(&form.password);

        let mut new_record = NewAccount::from_form(form);
        new_record.password = hashword.to_string();

        let record = diesel::insert_into(accounts::table)
            .values(new_record)
            .get_result::<Account>(&connection)?;

        Ok(record.id)
    }

    pub async fn mark_verified(uid: i32, pool: &DieselPgPool) -> Result<(), Error> {
        let connection = pool.get()?;

        diesel::update(accounts.filter(id.eq(uid)))
            .set((
                has_verified_email.eq(true),
                last_login.eq(offset::Utc::now()),
            ))
            .execute(&connection)?;

        Ok(())
    }

    pub async fn update_last_login(uid: i32, pool: &DieselPgPool) -> Result<(), Error> {
        let connection = pool.get()?;

        diesel::update(accounts.filter(id.eq(uid)))
            .set(last_login.eq(offset::Utc::now()))
            .execute(&connection)?;

        Ok(())
    }

    pub async fn update_password_and_last_login(
        uid: i32,
        account_password: &str,
        pool: &DieselPgPool,
    ) -> Result<(), Error> {
        let connection = pool.get()?;
        let hashword = make_password(&account_password);

        diesel::update(accounts.filter(id.eq(uid)))
            .set((password.eq(hashword), last_login.eq(offset::Utc::now())))
            .execute(&connection)?;

        Ok(())
    }

    pub async fn change_password(
        uid: i32,
        current_password: String,
        new_password: String,
        pool: &DieselPgPool,
    ) -> Result<(), Error> {
        let connection = pool.get()?;

        let account = Self::get(uid, pool).await?;
        if !check_password(&current_password, &account.password)? {
            return Err(Error::InvalidPassword);
        }
        let hashword = make_password(&new_password);
        diesel::update(accounts.filter(id.eq(uid)))
            .set(password.eq(hashword))
            .execute(&connection)?;

        Ok(())
    }

    pub async fn register_from_github(oauth_user: &GithubOauthUser, pool: &DieselPgPool) -> Result<User, Error> {
        let connection = pool.get()?;

        let account = if let Ok(record) = Account::get_by_email(&oauth_user.email, pool).await {
            // if there already is an account with this email, update it with git info then return
            diesel::update(accounts.filter(id.eq(record.id)))
            .set((name.eq(oauth_user.name.clone()), github_login.eq(oauth_user.login.clone())))
            .execute(&connection)?;

            record
        } else {
            // create a new account via github
            let new_record = NewGithubAccount::from_oauth_user(oauth_user);

            diesel::insert_into(accounts::table)
            .values(new_record)
            .get_result::<Account>(&connection)?
        };

        Ok(User {
            id: account.id,
            name: account.name,
            is_admin: account.is_admin,
            is_anonymous: false,
        })
    }
}

impl OneTimeUseTokenGenerator for Account {
    fn hash_value(&self) -> String {
        format!(
            "{}{}{}{}",
            self.id,
            self.password,
            match self.last_login {
                Some(ts) => format!("{}", ts.timestamp()),
                None => "Unverified".to_string(),
            },
            self.email
        )
    }
}

#[derive(Insertable)]
#[table_name = "accounts"]
pub struct NewAccount {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl NewAccount {
    fn from_form(form: &NewAccountForm) -> Self {
        return NewAccount {
            name: "".to_string(),
            email: form.email.value.clone(),
            password: "".to_string(),
        };
    }
}

#[derive(Insertable)]
#[table_name = "accounts"]
pub struct NewGithubAccount {
    pub name: String,
    pub email: String,
    pub github_login: String,
    pub password: String
}

impl NewGithubAccount {
    fn from_oauth_user(oauth_user: &GithubOauthUser) -> Self {
        return NewGithubAccount {
            name: oauth_user.name.clone(),
            email: oauth_user.email.clone(),
            github_login: oauth_user.login.clone(),
            password: {
                // Give it a dummy password because postgres complains
                let plaintext = crate::utils::token::generate_secure_alphanumeric_string(32);
                make_password(&plaintext)
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::{DatabaseTestContext, DB_POOL};
    use diesel::result::DatabaseErrorKind;
    use diesel::result::Error::DatabaseError;
    use jelly::forms::{EmailField, PasswordField};

    fn login_form() -> LoginForm {
        LoginForm {
            email: EmailField {
                value: "email@host.com".to_string(),
                errors: vec![],
            },
            password: PasswordField {
                value: "So$trongpas0word!".to_string(),
                errors: vec![],
                hints: vec![],
            },
            remember_me: "off".to_string(),
            redirect: "".to_string(),
        }
    }

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
    async fn authenticate_works() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();
        let uid = setup_user().await;
        Account::mark_verified(uid, &DB_POOL).await.unwrap();

        let user = Account::authenticate(&login_form(), &DB_POOL)
            .await
            .unwrap();
        assert_eq!(user.id, uid);
    }

    #[actix_rt::test]
    async fn authenticate_with_wrong_email_return_err() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();
        let uid = setup_user().await;
        Account::mark_verified(uid, &DB_POOL).await.unwrap();

        let invalid_login_form = LoginForm {
            email: EmailField {
                value: "wrong@host.com".to_string(),
                errors: vec![],
            },
            password: PasswordField {
                value: "So$trongpas0word!".to_string(),
                errors: vec![],
                hints: vec![],
            },
            remember_me: "off".to_string(),
            redirect: "".to_string(),
        };
        match Account::authenticate(&invalid_login_form, &DB_POOL).await {
            Err(Error::Database(DBError::NotFound)) => (),
            _ => panic!(),
        }
    }
    #[actix_rt::test]
    async fn authenticate_with_wrong_password_return_err() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();
        let uid = setup_user().await;
        Account::mark_verified(uid, &DB_POOL).await.unwrap();

        let invalid_login_form = LoginForm {
            email: EmailField {
                value: "email@host.com".to_string(),
                errors: vec![],
            },
            password: PasswordField {
                value: "wrongpassword".to_string(),
                errors: vec![],
                hints: vec![],
            },
            remember_me: "off".to_string(),
            redirect: "".to_string(),
        };
        match Account::authenticate(&invalid_login_form, &DB_POOL).await {
            Err(Error::InvalidPassword) => (),
            _ => panic!(),
        }
    }

    #[actix_rt::test]
    async fn authenticate_with_unverified_account_return_err() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();
        setup_user().await;

        match Account::authenticate(&login_form(), &DB_POOL).await {
            Err(Error::Generic(e)) => {
                assert_eq!(e, String::from("Your account has not been activated."))
            }
            _ => panic!(),
        }
    }

    #[actix_rt::test]
    async fn register_works() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();
        let uid = setup_user().await;
        let account = Account::get(uid, &DB_POOL).await.unwrap();
        assert_eq!(account.email, "email@host.com");
    }
    #[actix_rt::test]
    async fn register_with_empty_email_throws_exception() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();
        let form = NewAccountForm {
            email: EmailField {
                value: "".to_string(),
                errors: vec![],
            },
            password: PasswordField {
                value: "xxyyzz12".to_string(),
                errors: vec![],
                hints: vec![],
            },
        };
        let result = Account::register(&form, &DB_POOL).await;
        assert!(result.is_err());
        match result {
            Err(Error::Database(DatabaseError(DatabaseErrorKind::__Unknown, _))) => (),

            _ => panic!(),
        }
    }
    #[actix_rt::test]
    async fn register_with_duplicate_email_throws_exception() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();
        let form = NewAccountForm {
            email: EmailField {
                value: "email@host.com".to_string(),
                errors: vec![],
            },
            password: PasswordField {
                value: "xxyyzz123".to_string(),
                errors: vec![],
                hints: vec![],
            },
        };
        let _ = Account::register(&form, &DB_POOL).await.unwrap();
        let result = Account::register(&form, &DB_POOL).await;
        assert!(result.is_err());
        match result {
            Err(Error::Database(DatabaseError(DatabaseErrorKind::UniqueViolation, _))) => (),
            _ => panic!(),
        }
    }

    #[actix_rt::test]
    async fn change_password_works() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();
        let uid = setup_user().await;
        Account::mark_verified(uid, &DB_POOL).await.unwrap();

        let new_password = String::from("nEw$trongpas0word!");
        Account::change_password(
            uid,
            String::from("So$trongpas0word!"),
            new_password.clone(),
            &DB_POOL,
        )
        .await
        .unwrap();
        let account = Account::get(uid, &DB_POOL).await.unwrap();
        let mut login_form = login_form();
        login_form.password.value = new_password.clone();
        match Account::authenticate(&login_form, &DB_POOL).await {
            Ok(user) => assert_eq!(user.id, uid),
            _ => panic!(),
        }
    }

    #[actix_rt::test]
    async fn register_with_github_new_account_works() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();
        let oauth_user = GithubOauthUser {
            email: "a@b.com".to_string(),
            login: "git".to_string(),
            name: "git_username".to_string()
        };
        Account::register_from_github(&oauth_user, &DB_POOL).await.unwrap();

        let account = Account::get_by_email(&oauth_user.email, &DB_POOL).await.unwrap();
        assert_eq!(account.name, "git_username");
        assert_eq!(account.email, "a@b.com");
        assert_eq!(account.github_login.unwrap(), "git");
    }

    #[actix_rt::test]
    async fn register_with_github_existing_account_works() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();
        setup_user().await;
        let oauth_user = GithubOauthUser {
            email: "email@host.com".to_string(),
            login: "git".to_string(),
            name: "git_username".to_string()
        };
        Account::register_from_github(&oauth_user, &DB_POOL).await.unwrap();

        let account = Account::get_by_email(&oauth_user.email, &DB_POOL).await.unwrap();
        assert_eq!(account.name, "git_username");
        assert_eq!(account.email, "email@host.com");
        assert_eq!(account.github_login.unwrap(), "git");
    }
}
