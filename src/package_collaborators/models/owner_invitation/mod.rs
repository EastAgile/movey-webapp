#[cfg(test)]
mod tests;

use crate::package_collaborators::package_collaborator::{PackageCollaborator, Role};
use crate::schema::{owner_invitations, package_collaborators, packages, accounts};
use crate::utils::token::SecureToken;
use diesel::prelude::*;
use diesel::{Identifiable, Insertable, Queryable};
use jelly::chrono::{NaiveDateTime, Utc};
use jelly::Result;
use jelly::{chrono, DieselPgConnection};
use serde::Serialize;
use std::env;

#[derive(Clone, Debug, Eq, Identifiable, Queryable)]
#[primary_key(invited_user_id, package_id)]
pub struct OwnerInvitation {
    pub invited_user_id: i32,
    pub invited_by_user_id: i32,
    pub package_id: i32,
    pub token: String,
    pub is_transferring: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Clone, Debug, Eq, Queryable, Serialize)]
pub struct OwnerInvitationQuery {
    pub invited_user_id: i32,
    pub invited_by_user_id: i32,
    pub invited_by_user_email: String,
    pub package_id: i32,
    pub package_name: String,
    pub is_transferring: bool,
}

impl PartialEq for OwnerInvitation {
    fn eq(&self, other: &OwnerInvitation) -> bool {
        self.invited_user_id == other.invited_user_id
            && self.invited_by_user_id == other.invited_by_user_id
            && self.package_id == other.package_id
            && SecureToken::hash(&self.token) == other.token
            && self.created_at == other.created_at
    }
}

impl PartialEq for OwnerInvitationQuery {
    fn eq(&self, other: &OwnerInvitationQuery) -> bool {
        self.invited_user_id == other.invited_user_id
            && self.invited_by_user_id == other.invited_by_user_id
            && self.package_id == other.package_id
            && self.is_transferring == other.is_transferring
    }
} 

#[derive(Insertable, Clone, Debug)]
#[table_name = "owner_invitations"]
struct NewRecord {
    invited_user_id: i32,
    invited_by_user_id: i32,
    package_id: i32,
    token: String,
    is_transferring: bool,
    created_at: Option<NaiveDateTime>,
}

impl OwnerInvitation {
    pub fn create(
        invited_user_id: i32,
        invited_by_user_id: i32,
        package_id: i32,
        is_transferring: Option<bool>,
        created_at: Option<NaiveDateTime>,
        conn: &DieselPgConnection,
    ) -> Result<Self> {
        // Before actually creating the invite, check if an expired invitation already exists
        // and delete it from the database. This allows obtaining a new invite if the old one
        // expired, instead of returning "already exists".
        conn.transaction(|| -> Result<()> {
            // This does a SELECT FOR UPDATE + DELETE instead of a DELETE with a WHERE clause to
            // use the model's `is_expired` method, centralizing our expiration checking logic.
            let existing: Option<OwnerInvitation> = owner_invitations::table
                .find((invited_user_id, package_id))
                .for_update()
                .first(conn)
                .optional()?;

            if let Some(existing_) = existing {
                if existing_.is_expired() {
                    diesel::delete(&existing_).execute(conn)?;
                }
            }
            Ok(())
        })?;

        let secure_token = SecureToken::generate();
        let mut res: OwnerInvitation = diesel::insert_into(owner_invitations::table)
            .values(&NewRecord {
                invited_user_id,
                invited_by_user_id,
                package_id,
                token: secure_token.inner.sha256,
                is_transferring: is_transferring.unwrap_or(false),
                created_at,
            })
            // The ON CONFLICT DO NOTHING clause results in not creating the invite if another one
            // already exists. This does not cause problems with expired invitation as those are
            // deleted before doing this INSERT.
            .on_conflict_do_nothing()
            .get_result(conn)?;

        res.token = secure_token.plaintext;
        Ok(res)
    }

    pub fn find_by_token(token: &str, conn: &DieselPgConnection) -> Result<Self> {
        let hashed_token = SecureToken::hash(token);
        Ok(owner_invitations::table
            .filter(owner_invitations::token.eq(hashed_token))
            .first::<Self>(conn)?)
    }

    pub fn find_by_id(
        invited_user_id: i32,
        package_id: i32,
        conn: &DieselPgConnection,
    ) -> Result<Self> {
        Ok(owner_invitations::table
            .find((invited_user_id, package_id))
            .first::<Self>(conn)?)
    }

    pub fn find_by_invited_account(
        invited_user_id: i32,
        conn: &DieselPgConnection,
    ) -> Result<Vec<OwnerInvitationQuery>>{
        Ok(owner_invitations::table
            .filter(owner_invitations::invited_user_id.eq(invited_user_id))
            .inner_join(packages::table.on(owner_invitations::package_id.eq(packages::id)))
            .inner_join(accounts::table.on(owner_invitations::invited_by_user_id.eq(accounts::id)))
            .select((owner_invitations::invited_user_id, owner_invitations::invited_by_user_id, accounts::email, owner_invitations::package_id, packages::name, owner_invitations::is_transferring))
            .load::<OwnerInvitationQuery>(conn)?)
    }

    pub fn delete(&self, conn: &DieselPgConnection) -> Result<()> {
        diesel::delete(self).execute(conn)?;
        Ok(())
    }

    pub fn accept(&self, conn: &DieselPgConnection) -> Result<()> {
        if self.is_transferring {
            conn.transaction(|| -> Result<()> {
                diesel::update(package_collaborators::table)
                    .set(package_collaborators::role.eq(Role::Collaborator as i32))
                    .filter(package_collaborators::account_id.eq(self.invited_by_user_id))
                    .execute(conn)?;
                diesel::update(package_collaborators::table)
                    .set(package_collaborators::role.eq(Role::Owner as i32))
                    .filter(package_collaborators::account_id.eq(self.invited_user_id))
                    .execute(conn)?;
                self.delete(conn)
            })?
        } else {
            conn.transaction(|| -> Result<()> {
                PackageCollaborator::new_collaborator(
                    self.package_id,
                    self.invited_user_id,
                    self.invited_by_user_id,
                    conn,
                )?;
                self.delete(conn)
            })?
        }
        Ok(())
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at() <= Utc::now().naive_utc()
    }

    fn expires_at(&self) -> NaiveDateTime {
        let expiration_days = env::var("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS")
            .expect("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS not set!");
        let no_days = expiration_days
            .parse::<i64>()
            .expect("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS is not an integer!");
        if no_days < 0 {
            panic!("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS cannot be less than 0")
        }
        let days = chrono::Duration::days(no_days);
        self.created_at + days
    }
}
