// Implements a basic Account model, with support for creating/updating/deleting
// users, along with welcome email and verification.

use diesel::prelude::*;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
#[allow(unused_imports)]
use diesel::result::{Error as DBError};

use jelly::accounts::{OneTimeUseTokenGenerator, User};
use jelly::chrono::{offset, DateTime, Utc};
use jelly::djangohashers::{check_password, make_password};
use jelly::error::Error;
use jelly::serde::{Deserialize, Serialize};
use jelly::DieselPgPool;

use super::forms::{LoginForm, NewAccountForm};
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
            name: form.name.value.clone(),
            email: form.email.value.clone(),
            password: "".to_string(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::{DatabaseTestContext, DB_POOL};
    use jelly::forms::{EmailField, PasswordField};

    async fn setup_user() -> i32 {
        let form = NewAccountForm {
            name: Default::default(),
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

        let login_form = LoginForm {
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
        };
        let user = Account::authenticate(&login_form, &DB_POOL).await.unwrap();
        assert_eq!(user.id, uid);
    }

    #[actix_rt::test]
    async fn authenticate_with_wrong_email_return_err() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();
        let _uid = setup_user().await;

        let login_form = LoginForm {
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
        match Account::authenticate(&login_form, &DB_POOL).await {
            Err(Error::Database(DBError::NotFound)) => (),
            _ => panic!(),
        }
    }
    #[actix_rt::test]
    async fn authenticate_with_wrong_password_return_err() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();
        let _uid = setup_user().await;

        let login_form = LoginForm {
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
        match Account::authenticate(&login_form, &DB_POOL).await {
            Err(Error::InvalidPassword) => (),
            _ => panic!(),
        }
    }
}
