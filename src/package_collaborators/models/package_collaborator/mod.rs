use diesel::prelude::*;
use diesel::{Associations, Insertable, Queryable};
use jelly::chrono::{DateTime, Utc};
use jelly::serde::{Deserialize, Serialize};
use jelly::DieselPgConnection;
use jelly::Result;

use crate::accounts::Account;
use crate::packages::Package;
use crate::schema;
use crate::schema::package_collaborators;

#[cfg(test)]
mod tests;

#[derive(Serialize, Deserialize, Queryable, Insertable, Associations, Identifiable)]
#[table_name = "package_collaborators"]
#[belongs_to(Account)]
#[belongs_to(Package)]
#[primary_key(account_id, package_id)]
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
    pub fn new_collaborator(
        package_id_: i32,
        account_id_: i32,
        created_by_: i32,
        conn: &DieselPgConnection,
    ) -> Result<()> {
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

    pub fn new_owner(
        package_id_: i32,
        account_id_: i32,
        created_by_: i32,
        conn: &DieselPgConnection,
    ) -> Result<()> {
        diesel::insert_into(package_collaborators::table)
            .values(NewCollaborator {
                package_id: package_id_,
                account_id: account_id_,
                role: Role::Owner as i32,
                created_by: created_by_,
            })
            .get_result::<PackageCollaborator>(conn)?;

        Ok(())
    }

    pub fn get(package_id: i32, account_id: i32, conn: &DieselPgConnection) -> Result<Self> {
        Ok(package_collaborators::table
            .find((package_id, account_id))
            .first::<Self>(conn)?)
    }

    pub fn get_by_package_id(package_id: i32, conn: &DieselPgConnection) -> Result<Vec<i32>> {
        Ok(package_collaborators::table
            .filter(package_collaborators::package_id.eq(package_id))
            .select(package_collaborators::account_id)
            // First element is the owner of package
            .order(package_collaborators::role.asc())
            .load::<i32>(conn)?)
    }

    pub fn get_in_bulk_order_by_role(
        package_id: i32,
        account_ids: Vec<i32>,
        conn: &DieselPgConnection,
    ) -> Result<Vec<Self>> {
        Ok(package_collaborators::table
            .filter(
                package_collaborators::package_id
                    .eq(package_id)
                    .and(package_collaborators::account_id.eq_any(account_ids)),
            )
            .order(package_collaborators::role.asc()) // owner first
            .load::<Self>(conn)?)
    }

    pub fn delete_collaborator_by_id(
        account_id_: i32,
        package_id_: i32,
        conn: &DieselPgConnection,
    ) -> Result<usize> {
        use schema::package_collaborators::dsl::*;
        let no_deleted_rows = diesel::delete(
            package_collaborators.filter(
                account_id
                    .eq(account_id_)
                    .and(package_id.eq(package_id_))
                    .and(role.eq(Role::Collaborator as i32)),
            ),
        )
        .execute(conn)?;
        Ok(no_deleted_rows)
    }
}
