use diesel::prelude::*;
use diesel::{Associations, Insertable, Queryable};
use jelly::chrono::{DateTime, Utc};
use jelly::error::Error;
use jelly::serde::{Deserialize, Serialize};
use jelly::DieselPgConnection;

use crate::accounts::Account;
use crate::packages::Package;
use crate::schema::package_collaborators;

#[cfg(test)]
mod tests;

#[derive(Serialize, Deserialize, Queryable, Insertable, Associations)]
#[table_name = "package_collaborators"]
#[belongs_to(Account)]
#[belongs_to(Package)]
pub struct PackageCollaborator {
    pub package_id: i32,
    pub account_id: i32,
    pub role: i32,
    pub created_by: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Role {
    Owner = 0,
    Collaborator = 1,
}

#[derive(Insertable)]
#[table_name = "package_collaborators"]
pub struct NewCollaborator {
    pub package_id: i32,
    pub account_id: i32,
    pub role: i32,
    pub created_by: i32,
}

impl PackageCollaborator {
    pub async fn new_collaborator(
        package_id_: i32,
        account_id_: i32,
        created_by_: i32,
        conn: &DieselPgConnection,
    ) -> Result<(), Error> {
        diesel::insert_into(package_collaborators::table)
            .values(NewCollaborator {
                package_id: package_id_,
                account_id: account_id_,
                role: Role::Collaborator as i32,
                created_by: created_by_,
            })
            .get_result::<PackageCollaborator>(conn)?;

        Ok(())
    }

    pub async fn get(package_id_: i32, account_id_: i32, conn: &DieselPgConnection) -> Result<Self, Error> {
        Ok(package_collaborators::table
            .find((package_id_, account_id_))
            .first::<Self>(conn)?)
    }
}
