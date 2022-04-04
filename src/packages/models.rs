use diesel::{Queryable, Identifiable, AsChangeset, Insertable};
use diesel::prelude::*;

use jelly::chrono::{DateTime, Utc, offset};
use jelly::error::Error;
use jelly::serde::{Deserialize, Serialize};
use jelly::DieselPgPool;

// use super::forms::{LoginForm, NewAccountForm};
use crate::schema::packages::dsl::*;
use crate::schema::packages;
use crate::schema::package_versions::dsl::*;
use crate::schema::package_versions;

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
    pub readme_content: String,
    pub license: String,
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
