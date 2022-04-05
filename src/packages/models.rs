use diesel::{Queryable, Identifiable, AsChangeset, Insertable};
use diesel::prelude::*;

use jelly::chrono::{DateTime, Utc};
use jelly::error::Error;
use jelly::serde::{Deserialize, Serialize};
use jelly::DieselPgPool;

// use super::forms::{LoginForm, NewAccountForm};
use crate::schema::packages::dsl::*;
use crate::schema::packages;
use crate::schema::package_versions::dsl::*;
use crate::schema::package_versions;

use crate::github_service;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, AsChangeset)]
pub struct Package {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub repository_url: String,
    pub total_downloads_count: i32,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name="packages"]
pub struct NewPackage {
    pub name: String,
    pub description: String,
    pub repository_url: String
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, AsChangeset)]
pub struct PackageVersion {
    pub id: i32,
    pub package_id: i32,
    pub version: String,
    pub readme_content: Option<String>,
    pub license: Option<String>,
    pub downloads_count: i32,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name="package_versions"]
pub struct NewPackageVersion {
    pub package_id: i32,
    pub version: String,
    pub readme_content: String
}

impl Package {
    pub async fn create(repo_url: &String, package_description: &String, pool: &DieselPgPool) -> Result<i32, Error> {
        let connection = pool.get()?;

        let github_data = github_service::fetch_repo_data(&repo_url).unwrap();

        // TODO: check if package exists, check if version exists

        let new_package = NewPackage {
            name: github_data.name,
            description: package_description.to_string(),
            repository_url: repo_url.to_string()
        };

        let record = diesel::insert_into(packages::table)
            .values(new_package)
            .get_result::<Package>(&connection)?;

        let new_package_version = NewPackageVersion {
            package_id: record.id,
            version: github_data.version,
            readme_content: github_data.readme_content
        };

        diesel::insert_into(package_versions::table)
            .values(new_package_version)
            .get_result::<PackageVersion>(&connection)?;

        Ok(record.id)
    }
}
