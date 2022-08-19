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
use jelly::{DieselPgConnection, DieselPgPool};

use super::forms::{LoginForm, NewAccountForm};
use super::views::verify::GithubOauthUser;
use crate::schema::accounts;
use crate::schema::accounts::dsl::*;
use crate::schema::api_tokens::dsl::{
    account_id as api_tokens_account_id, api_tokens, name as api_tokens_name,
};
use crate::schema::packages::dsl::{account_id as packages_account_id, packages};

#[cfg(test)]
mod tests;

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
    pub github_login: Option<String>,
    pub github_id: Option<i64>,
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

    pub async fn get_by_email_or_gh_login(term: &str, pool: &DieselPgPool) -> Result<Self, Error> {
        let connection = pool.get()?;
        let result = accounts
            .filter(email.eq(term).or(github_login.eq(term)))
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
        new_record.password = hashword;

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
        let hashword = make_password(account_password);

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

    pub async fn register_from_github(
        oauth_user: &GithubOauthUser,
        pool: &DieselPgPool,
    ) -> Result<User, Error> {
        let connection = pool.get()?;

        let account = if let Ok(record) = Account::get_by_email(&oauth_user.email, pool).await {
            // if there already is an account with this email, update it with git info then return
            diesel::update(accounts.filter(id.eq(record.id)))
                .set((
                    github_login.eq(oauth_user.login.clone()),
                    github_id.eq(oauth_user.id),
                    has_verified_email.eq(true),
                ))
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

    pub async fn get_by_github_id(gid: i64, pool: &DieselPgPool) -> Result<Self, Error> {
        let connection = pool.get()?;
        let result = accounts
            .filter(github_id.eq(gid))
            .first::<Account>(&connection)?;

        Ok(result)
    }

    pub async fn merge_github_account_and_movey_account(
        gh_account_id: i32,
        movey_account_id: i32,
        gh_id: i64,
        gh_login: String,
        pool: &DieselPgPool,
    ) -> Result<(), Error> {
        let conn = pool.get()?;

        conn.build_transaction().run::<_, _, _>(|| {
            diesel::update(packages.filter(packages_account_id.eq(gh_account_id)))
                .set(packages_account_id.eq(movey_account_id))
                .execute(&conn)?;

            diesel::update(api_tokens.filter(api_tokens_account_id.eq(movey_account_id)))
                .set(api_tokens_name.eq(api_tokens_name.concat("__movey")))
                .execute(&conn)?;

            diesel::update(api_tokens.filter(api_tokens_account_id.eq(gh_account_id)))
                .set((
                    api_tokens_account_id.eq(movey_account_id),
                    api_tokens_name.eq(api_tokens_name.concat("__github")),
                ))
                .execute(&conn)?;

            diesel::delete(accounts.filter(github_id.eq(gh_id))).execute(&conn)?;

            diesel::update(accounts.filter(id.eq(movey_account_id)))
                .set((github_id.eq(gh_id), github_login.eq(gh_login)))
                .execute(&conn)?;

            Ok(())
        })
    }

    pub async fn update_movey_account_with_github_info(
        movey_id: i32,
        gh_id: i64,
        gh_login: String,
        pool: &DieselPgPool,
    ) -> Result<(), Error> {
        let conn = pool.get()?;
        diesel::update(accounts.filter(id.eq(movey_id)))
            .set((github_id.eq(gh_id), github_login.eq(gh_login)))
            .execute(&conn)?;

        Ok(())
    }

    pub fn is_generated_email(&self) -> bool {
        let no_reply_email_domain =
            std::env::var("NO_REPLY_EMAIL_DOMAIN").expect("NO_REPLY_EMAIL_DOMAIN is not set!");
        self.email.ends_with(&no_reply_email_domain)
    }

    pub fn get_accounts(
        account_ids: Vec<i32>,
        conn: &DieselPgConnection,
    ) -> Result<Vec<Self>, Error> {
        Ok(accounts::table
            .filter(accounts::id.eq_any(account_ids))
            .load::<Self>(conn)?)
    }
}

#[cfg(any(test, feature = "test"))]
impl Account {
    pub async fn delete(account_id: i32) -> Result<(), Error> {
        let pool = &crate::test::DB_POOL;
        let conn = pool.get()?;
        diesel::delete(accounts.filter(id.eq(account_id))).execute(&conn)?;

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
        let email_ = form.email.value.clone();
        let name_from_email = email_.split('@').next().unwrap();
        NewAccount {
            name: String::from(name_from_email),
            email: form.email.value.clone(),
            password: "".to_string(),
        }
    }
}

#[derive(Insertable)]
#[table_name = "accounts"]
pub struct NewGithubAccount {
    pub name: String,
    pub email: String,
    pub github_login: String,
    pub password: String,
    pub has_verified_email: bool,
    pub github_id: i64,
}

impl NewGithubAccount {
    fn from_oauth_user(oauth_user: &GithubOauthUser) -> Self {
        NewGithubAccount {
            name: "".to_string(),
            email: oauth_user.email.clone(),
            github_login: oauth_user.login.clone(),
            password: {
                // Give it a dummy password because postgres complains
                let plaintext = crate::utils::token::generate_secure_alphanumeric_string(32);
                make_password(&plaintext)
            },
            has_verified_email: true,
            github_id: oauth_user.id,
        }
    }
}
