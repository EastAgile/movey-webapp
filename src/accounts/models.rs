// Implements a basic Account model, with support for creating/updating/deleting
// users, along with welcome email and verification.

use diesel::{Queryable, Identifiable, AsChangeset, Insertable};
use diesel::prelude::*;

use jelly::accounts::{OneTimeUseTokenGenerator, User};
use jelly::chrono::{DateTime, Utc, offset};
use jelly::djangohashers::{check_password, make_password};
use jelly::error::Error;
use jelly::serde::{Deserialize, Serialize};
use jelly::DieselPgPool;

use super::forms::{LoginForm, NewAccountForm};
use crate::schema::accounts::dsl::*;
use crate::schema::accounts;

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
        let result = accounts
            .find(uid)
            .first::<Account>(&connection)?;

        return Ok(result);
    }

    pub async fn get_by_email(account_email: &str, pool: &DieselPgPool) -> Result<Self, Error> {
        let connection = pool.get()?;
        let result = accounts
            .filter(email.eq(account_email))
            .first::<Account>(&connection)?;

        return Ok(result);
    }

    pub async fn authenticate(form: &LoginForm, pool: &DieselPgPool) -> Result<User, Error> {
        let connection = pool.get()?;
        let user = accounts
            .filter(email.eq(&form.email.value))
            .first::<Account>(&connection)?;

        if !check_password(&form.password, &user.password)? {
            return Err(Error::InvalidPassword);
        }

        return Ok(User {
            id: user.id,
            name: user.name,
            is_admin: user.is_admin,
            is_anonymous: false,
        })
    }

    pub async fn fetch_email(uid: i32, pool: &DieselPgPool) -> Result<(String, String), Error> {
        // let data = sqlx::query!(
        //     "
        //     SELECT
        //         name, email
        //     FROM accounts WHERE id = $1
        // ",
        //     id
        // )
        // .fetch_one(pool)
        // .await?;

        // Ok((data.name, data.email))
        Err(Error::Generic("Not implemented!".to_string()))
    }

    pub async fn fetch_name_from_email(account_email: &str, pool: &DieselPgPool) -> Result<String, Error> {
        // let data = sqlx::query!(
        //     "
        //     SELECT name FROM accounts WHERE email = $1
        // ",
        //     email
        // )
        // .fetch_one(pool)
        // .await?;

        // Ok(data.name)
        Err(Error::Generic("Not implemented!".to_string()))
    }

    pub async fn register(form: &NewAccountForm, pool: &DieselPgPool) -> Result<(), Error> {
        let connection = pool.get()?;
        let hashword = make_password(&form.password);

        let mut new_record = NewAccount::from_form(form);
        new_record.password = hashword.to_string();

        diesel::insert_into(accounts::table)
            .values(new_record)
            .get_result::<Account>(&connection)?;

        return Ok(())
    }

    pub async fn mark_verified(uid: i32, pool: &DieselPgPool) -> Result<(), Error> {
        let connection = pool.get()?;

        diesel::update(accounts.filter(id.eq(uid)))
            .set((has_verified_email.eq(true), last_login.eq(offset::Utc::now())))
            .execute(&connection)?;

        return Ok(())
    }

    pub async fn update_last_login(uid: i32, pool: &DieselPgPool) -> Result<(), Error> {
        let connection = pool.get()?;

        diesel::update(accounts.filter(id.eq(uid)))
            .set(last_login.eq(offset::Utc::now()))
            .execute(&connection)?;

        return Ok(())
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

        return Ok(())
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
#[table_name="accounts"]
pub struct NewAccount {
    pub name: String,
    pub email: String,
    pub password: String
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
