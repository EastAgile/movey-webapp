#[cfg(test)]
mod tests;

use crate::schema;
use crate::schema::external_invitations;
use diesel::prelude::*;
use diesel::{Identifiable, Insertable, Queryable};
use jelly::chrono::{NaiveDateTime, Utc};
use jelly::Result;
use jelly::{chrono, DieselPgConnection};
use std::env;

#[derive(Clone, Debug, PartialEq, Eq, Identifiable, Queryable)]
#[primary_key(external_user_email, package_id)]
pub struct ExternalInvitation {
    pub external_user_email: String,
    pub invited_by_user_id: i32,
    pub package_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Clone, Debug)]
#[table_name = "external_invitations"]
struct NewRecord {
    external_user_email: String,
    invited_by_user_id: i32,
    package_id: i32,
}

impl ExternalInvitation {
    pub fn create(
        external_user_email: &str,
        invited_by_user_id: i32,
        package_id: i32,
        conn: &DieselPgConnection,
    ) -> Result<Self> {
        conn.transaction(|| -> Result<()> {
            let existing = external_invitations::table
                .find((external_user_email, package_id))
                .for_update()
                .first::<ExternalInvitation>(conn);

            if let Ok(existing) = existing {
                if existing.is_expired() {
                    diesel::delete(&existing).execute(conn)?;
                }
            }
            Ok(())
        })?;

        let res: ExternalInvitation = diesel::insert_into(external_invitations::table)
            .values(&NewRecord {
                external_user_email: String::from(external_user_email),
                invited_by_user_id,
                package_id,
            })
            .on_conflict_do_nothing()
            .get_result(conn)?;

        Ok(res)
    }

    pub fn find_by_id(
        external_user_email: &str,
        package_id: i32,
        conn: &DieselPgConnection,
    ) -> Result<Self> {
        Ok(external_invitations::table
            .find((external_user_email, package_id))
            .first::<Self>(conn)?)
    }

    pub fn find_by_email(
        external_user_email: &str,
        conn: &DieselPgConnection,
    ) -> Result<Vec<Self>> {
        Ok(external_invitations::table
            .filter(external_invitations::external_user_email.eq(external_user_email))
            .get_results::<Self>(conn)?)
    }

    pub fn delete(&self, conn: &DieselPgConnection) -> Result<()> {
        diesel::delete(self).execute(conn)?;
        Ok(())
    }

    pub fn delete_by_id(
        external_user_email_: &str,
        package_id_: i32,
        conn: &DieselPgConnection,
    ) -> Result<usize> {
        use schema::external_invitations::dsl::*;
        let no_deleted_rows = diesel::delete(
            external_invitations.filter(
                external_user_email
                    .eq(external_user_email_)
                    .and(package_id.eq(package_id_)),
            ),
        )
        .execute(conn)?;
        Ok(no_deleted_rows)
    }

    pub fn find_by_package_id(package_id: i32, conn: &DieselPgConnection) -> Result<Vec<String>> {
        Ok(external_invitations::table
            .filter(external_invitations::package_id.eq(package_id))
            .select(external_invitations::external_user_email)
            .load::<String>(conn)?)
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
