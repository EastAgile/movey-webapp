#[cfg(test)]
mod tests;

use crate::schema::pending_invitations;
use diesel::prelude::*;
use diesel::{Identifiable, Insertable, Queryable};
use jelly::chrono::{NaiveDateTime, Utc};
use jelly::Result;
use jelly::{chrono, DieselPgConnection};
use std::env;

#[derive(Clone, Debug, PartialEq, Eq, Identifiable, Queryable)]
#[primary_key(pending_user_email, package_id)]
pub struct PendingInvitation {
    pub pending_user_email: String,
    pub invited_by_user_id: i32,
    pub package_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Clone, Debug)]
#[table_name = "pending_invitations"]
struct NewRecord {
    pending_user_email: String,
    invited_by_user_id: i32,
    package_id: i32,
}

impl PendingInvitation {
    pub fn create(
        pending_user_email: &str,
        invited_by_user_id: i32,
        package_id: i32,
        conn: &DieselPgConnection,
    ) -> Result<Self> {
        conn.transaction(|| -> Result<()> {
            let existing: Option<PendingInvitation> = pending_invitations::table
                .find((pending_user_email, package_id))
                .for_update()
                .first(conn)
                .optional()?;

            if let Some(existing) = existing {
                if existing.is_expired() {
                    diesel::delete(&existing).execute(conn)?;
                }
            }
            Ok(())
        })?;

        let res: PendingInvitation = diesel::insert_into(pending_invitations::table)
            .values(&NewRecord {
                pending_user_email: String::from(pending_user_email),
                invited_by_user_id,
                package_id,
            })
            .on_conflict_do_nothing()
            .get_result(conn)?;

        Ok(res)
    }

    pub fn find_by_id(
        pending_user_email: &str,
        package_id: i32,
        conn: &DieselPgConnection,
    ) -> Result<Self> {
        Ok(pending_invitations::table
            .find((pending_user_email, package_id))
            .first::<Self>(conn)?)
    }

    pub fn find_by_email(pending_user_email: &str, conn: &DieselPgConnection) -> Result<Vec<Self>> {
        Ok(pending_invitations::table
            .filter(pending_invitations::pending_user_email.eq(pending_user_email))
            .load::<Self>(conn)?)
    }

    pub fn delete(&self, conn: &DieselPgConnection) -> Result<()> {
        diesel::delete(self).execute(conn)?;
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
