use crate::accounts::Account;

use diesel::dsl::{count, now, sum};
use diesel::prelude::*;
use diesel::sql_types::{Integer, Nullable, Text, Timestamptz};
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable};

use diesel_full_text_search::{plainto_tsquery, TsVectorExtensions};

use diesel::result::Error::NotFound;
use jelly::chrono::{DateTime, NaiveDateTime, Utc};
use jelly::error::Error;
use jelly::serde::{Deserialize, Serialize};
use jelly::DieselPgPool;

use crate::github_service::GithubRepoData;
use mockall_double::double;

#[cfg(test)]
mod tests;

#[double]
use crate::github_service::GithubService;
use crate::schema::package_versions;
use crate::schema::package_versions::dsl::*;
use crate::schema::packages;
use crate::schema::packages::dsl::*;
use crate::utils::paginate::LoadPaginated;

pub const PACKAGES_PER_PAGE: i64 = 10;

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Queryable,
    Identifiable,
    AsChangeset,
    QueryableByName,
    Associations,
)]
#[table_name = "packages"]
#[belongs_to(Account)]
pub struct Package {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub repository_url: String,
    pub total_downloads_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub account_id: Option<i32>,
}

type PackageColumns = (
    packages::id,
    packages::name,
    packages::description,
    packages::repository_url,
    packages::total_downloads_count,
    packages::created_at,
    packages::updated_at,
    packages::account_id,
);

pub const PACKAGE_COLUMNS: PackageColumns = (
    packages::id,
    packages::name,
    packages::description,
    packages::repository_url,
    packages::total_downloads_count,
    packages::created_at,
    packages::updated_at,
    packages::account_id,
);

#[derive(Debug, Serialize, Deserialize, QueryableByName, Queryable)]
pub struct PackageSearchResult {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "Text"]
    pub name: String,
    #[sql_type = "Text"]
    pub description: String,
    #[sql_type = "Integer"]
    pub total_downloads_count: i32,
    #[sql_type = "Timestamptz"]
    pub created_at: NaiveDateTime,
    #[sql_type = "Timestamptz"]
    pub updated_at: NaiveDateTime,
    #[sql_type = "Text"]
    pub version: String,
}

#[derive(Insertable)]
#[table_name = "packages"]
pub struct NewPackage {
    pub name: String,
    pub description: String,
    pub repository_url: String,
    pub account_id: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub enum PackageSortField {
    #[serde(alias = "name")]
    Name,
    #[serde(alias = "description")]
    Description,
    #[serde(alias = "most_downloads")]
    MostDownloads,
    #[serde(alias = "newly_added")]
    NewlyAdded,
    #[serde(alias = "recently_updated")]
    RecentlyUpdated,
}

// Convert to a value used in view template
impl std::fmt::Display for PackageSortField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let enum_name = match self {
            PackageSortField::Name => "name",
            PackageSortField::Description => "description",
            PackageSortField::MostDownloads => "most_downloads",
            PackageSortField::NewlyAdded => "newly_added",
            PackageSortField::RecentlyUpdated => "recently_updated",
        };
        write!(f, "{}", enum_name)
    }
}

impl PackageSortField {
    // Convert to a value used in ORM
    pub fn to_column_name(&self) -> String {
        String::from(match self {
            PackageSortField::Name => "name",
            PackageSortField::Description => "description",
            PackageSortField::MostDownloads => "total_downloads_count",
            PackageSortField::NewlyAdded => "created_at",
            PackageSortField::RecentlyUpdated => "updated_at",
        })
    }
}

#[derive(Serialize, Deserialize)]
pub enum PackageSortOrder {
    #[serde(alias = "asc")]
    Asc,
    #[serde(alias = "desc")]
    Desc,
}

impl PackageSortOrder {
    fn to_order_direction(&self) -> String {
        String::from(match self {
            PackageSortOrder::Asc => "ASC",
            PackageSortOrder::Desc => "DESC",
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, AsChangeset, Clone)]
pub struct PackageVersion {
    pub id: i32,
    pub package_id: i32,
    pub version: String,
    pub readme_content: Option<String>,
    pub license: Option<String>,
    pub downloads_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub rev: Option<String>,
    pub total_files: Option<i32>,
    pub total_size: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "package_versions"]
pub struct NewPackageVersion {
    pub package_id: i32,
    pub version: String,
    pub readme_content: String,
    pub rev: String,
    pub total_files: i32,
    pub total_size: i32,
}

#[derive(Serialize, Deserialize)]
pub enum PackageVersionSort {
    Latest,
    Oldest,
    MostDownloads,
}

impl Package {
    pub async fn count(pool: &DieselPgPool) -> Result<i64, Error> {
        let connection = pool.get()?;
        let result = packages
            .select(count(packages::id))
            .first::<i64>(&connection)?;

        Ok(result)
    }

    pub async fn create(
        repo_url: &String,
        package_description: &String,
        version_rev: &String,
        version_files: i32,
        version_size: i32,
        account_id_: Option<i32>,
        service: &GithubService,
        subdir: Option<String>,
        pool: &DieselPgPool,
    ) -> Result<i32, Error> {
        let github_data = service.fetch_repo_data(&repo_url, subdir, None)?;

        Package::create_from_crawled_data(
            repo_url,
            package_description,
            version_rev,
            version_files,
            version_size,
            account_id_,
            github_data,
            pool,
        )
        .await
    }

    pub async fn create_from_crawled_data(
        repo_url: &str,
        package_description: &str,
        version_rev: &str,
        version_files: i32,
        version_size: i32,
        account_id_: Option<i32>,
        github_data: GithubRepoData,
        pool: &DieselPgPool,
    ) -> Result<i32, Error> {
        let connection = pool.get()?;
        let record = match Package::get_by_name(&github_data.name, &pool).await {
            Ok(package) => package,
            Err(_) => {
                let new_package = NewPackage {
                    name: github_data.name,
                    description: package_description.to_string(),
                    repository_url: repo_url.to_string(),
                    account_id: account_id_,
                };

                let record = diesel::insert_into(packages::table)
                    .values(new_package)
                    .returning(PACKAGE_COLUMNS)
                    .get_result::<Package>(&connection)?;

                record
            }
        };

        // Only creates new version if same user with package owner
        if record.account_id == account_id_ || record.account_id.is_none() {
            if let Err(_) = record.get_version(&github_data.version, &pool).await {
                PackageVersion::create(
                    record.id,
                    github_data.version,
                    github_data.readme_content,
                    version_rev.to_string(),
                    version_files,
                    version_size,
                    pool,
                )
                .await?;
            }
        }

        Ok(record.id)
    }

    pub async fn get(uid: i32, pool: &DieselPgPool) -> Result<Self, Error> {
        let connection = pool.get()?;
        let result = packages
            .find(uid)
            .select(PACKAGE_COLUMNS)
            .first::<Package>(&connection)?;

        Ok(result)
    }

    pub async fn get_by_name(package_name: &String, pool: &DieselPgPool) -> Result<Self, Error> {
        let connection = pool.get()?;

        let result = packages
            .filter(name.eq(package_name))
            .select(PACKAGE_COLUMNS)
            .first::<Package>(&connection)?;

        Ok(result)
    }

    pub async fn get_by_account(
        owner_id: i32,
        pool: &DieselPgPool,
    ) -> Result<Vec<PackageSearchResult>, Error> {
        let connection = pool.get()?;

        let result = packages
            .inner_join(package_versions::table)
            .select((packages::id, packages::name, packages::description, packages::total_downloads_count, packages::created_at, packages::updated_at, diesel::dsl::sql::<diesel::sql_types::Text>("max(version) as version")))
            .filter(account_id.eq(owner_id))
            .filter(diesel::dsl::sql("TRUE GROUP BY packages.id, name, description, total_downloads_count, packages.created_at, packages.updated_at")) // workaround since diesel 1.x doesn't support GROUP_BY dsl yet
            .load::<PackageSearchResult>(&connection)?;

        Ok(result)
    }

    pub async fn get_downloads(owner_id: i32, pool: &DieselPgPool) -> Result<i64, Error> {
        let connection = pool.get()?;
        let result = packages
            .select(sum(total_downloads_count))
            .filter(account_id.eq(owner_id))
            .first::<Option<i64>>(&connection)?;

        match result {
            Some(result) => Ok(result),
            None => Ok(0),
        }
    }

    pub async fn get_version(
        &self,
        version_name: &String,
        pool: &DieselPgPool,
    ) -> Result<PackageVersion, Error> {
        let connection = pool.get()?;
        let result = package_versions
            .filter(package_id.eq(self.id).and(version.eq(version_name)))
            .first::<PackageVersion>(&connection)?;

        Ok(result)
    }

    pub async fn increase_download_count(
        url: &String,
        rev_: &String,
        subdir: &String,
        service: &GithubService,
        pool: &DieselPgPool,
    ) -> Result<usize, Error> {
        let connection = pool.get()?;

        let mut https_url = url.to_owned();
        if url.starts_with("git@github.com") {
            https_url = url
                .replace(":", "/")
                .replace("git@", "https://")
                .replace(".git", "")
                .to_owned();
        }
        if https_url.ends_with(".git") {
            https_url = https_url[0..https_url.len() - 4].to_string();
        }

        let package_id_ = packages
            .filter(repository_url.eq(&https_url))
            .select(packages::id)
            .first(&connection);

        let package_id_ = match package_id_ {
            Ok(package_id_) => {
                let package_version_id = package_versions
                    .filter(package_id.eq(package_id_).and(rev.eq(rev_)))
                    .select(package_versions::id)
                    .first::<i32>(&connection);

                match package_version_id {
                    Ok(_) => (),
                    Err(NotFound) => {
                        // Package is found but version is not, creating shadow version
                        let github_data = if subdir.is_empty() {
                            service.fetch_repo_data(&https_url, None, Some(rev_.clone()))?
                        } else {
                            let subdir_with_toml = format!("{}/Move.toml", subdir);
                            service.fetch_repo_data(
                                &https_url,
                                Some(subdir_with_toml),
                                Some(rev_.clone()),
                            )?
                        };

                        PackageVersion::create(
                            package_id_,
                            github_data.version,
                            github_data.readme_content,
                            rev_.clone(),
                            -1,
                            github_data.size,
                            pool,
                        )
                        .await?;
                    }
                    Err(e) => {
                        return Err(Error::Database(e));
                    }
                };

                package_id_
            }
            Err(NotFound) => {
                // Package is not found, creating shadow package and package version
                let github_data = if subdir.is_empty() {
                    service.fetch_repo_data(&https_url, None, None)?
                } else {
                    let subdir_with_toml = format!("{}/Move.toml", subdir);
                    service.fetch_repo_data(&https_url, Some(subdir_with_toml), None)?
                };
                if !subdir.is_empty() {
                    https_url = format!("{}/blob/{}/{}", https_url, github_data.rev, subdir);
                }
                Package::create_from_crawled_data(
                    &https_url,
                    &github_data.description.clone(),
                    &rev_,
                    -1,
                    github_data.size,
                    None,
                    github_data,
                    &pool,
                )
                .await?
            }
            Err(e) => {
                return Err(Error::Database(e));
            }
        };

        let mut changed_rows = diesel::update(package_versions)
            .filter(package_id.eq(package_id_).and(rev.eq(rev_)))
            .set(downloads_count.eq(downloads_count + 1))
            .execute(&connection)?;

        changed_rows += diesel::update(packages)
            .filter(packages::id.eq(package_id_))
            .set(total_downloads_count.eq(total_downloads_count + 1))
            .execute(&connection)?;

        Ok(changed_rows)
    }

    pub async fn auto_complete_search(
        search_query: &str,
        pool: &DieselPgPool,
    ) -> Result<Vec<(String, String, String)>, Error> {
        let connection = pool.get()?;
        let result: Vec<(String, String, String)> = packages::table
            .inner_join(package_versions::table)
            .filter(name.ilike(format!("%{}%", search_query)))
            .filter(diesel::dsl::sql("TRUE GROUP BY packages.id, name, description, total_downloads_count, packages.created_at, packages.updated_at"))
            .select((packages::name, packages::description, diesel::dsl::sql::<diesel::sql_types::Text>("max(version) as version")))
            .load::<(String, String, String)>(&connection)?;

        Ok(result)
    }

    pub async fn search(
        search_query: &str,
        sort_field: &PackageSortField,
        sort_order: &PackageSortOrder,
        page: Option<i64>,
        per_page: Option<i64>,
        pool: &DieselPgPool,
    ) -> Result<(Vec<PackageSearchResult>, i64, i64), Error> {
        let connection = pool.get()?;
        let field = sort_field.to_column_name();
        let order = sort_order.to_order_direction();
        let order_query = format!("packages.{} {}", field, order);
        let search_query: &str = &search_query.split(" ").collect::<Vec<&str>>().join(" & ");

        let page = page.unwrap_or_else(|| 1);
        let per_page = per_page.unwrap_or_else(|| PACKAGES_PER_PAGE);
        if page < 1 || per_page < 1 {
            return Err(Error::Generic(String::from("Invalid page number.")));
        }

        let result: (Vec<PackageSearchResult>, i64, i64) = packages::table
            .inner_join(package_versions::table)
            .select((packages::id, packages::name, packages::description, packages::total_downloads_count, packages::created_at, packages::updated_at, diesel::dsl::sql::<diesel::sql_types::Text>("max(version) as version")))
            .filter(name.ilike(format!("%{}%", search_query))
                .or(tsv.matches(plainto_tsquery(search_query))))
            .filter(diesel::dsl::sql("TRUE GROUP BY packages.id, name, description, total_downloads_count, packages.created_at, packages.updated_at")) // workaround since diesel 1.x doesn't support GROUP_BY dsl yet
            .order(diesel::dsl::sql::<diesel::sql_types::Text>(&order_query))
            .load_with_pagination(&connection, Some(page), Some(per_page))?;

        return Ok(result);
    }

    pub async fn all_packages(
        sort_field: &PackageSortField,
        sort_order: &PackageSortOrder,
        page: Option<i64>,
        per_page: Option<i64>,
        pool: &DieselPgPool,
    ) -> Result<(Vec<PackageSearchResult>, i64, i64), Error> {
        let connection = pool.get()?;
        let field = sort_field.to_column_name();
        let order = sort_order.to_order_direction();
        let order_query = format!("packages.{} {}", field, order);

        let page = page.unwrap_or_else(|| 1);
        let per_page = per_page.unwrap_or_else(|| PACKAGES_PER_PAGE);
        if page < 1 || per_page < 1 {
            return Err(Error::Generic(String::from("Invalid page number.")));
        }

        let result: (Vec<PackageSearchResult>, i64, i64) = packages::table
            .inner_join(package_versions::table)
            .select((packages::id, packages::name, packages::description, packages::total_downloads_count, packages::created_at, packages::updated_at, diesel::dsl::sql::<diesel::sql_types::Text>("max(version) as version")))
            .filter(diesel::dsl::sql("TRUE GROUP BY packages.id, name, description, total_downloads_count, packages.created_at, packages.updated_at")) // workaround since diesel 1.x doesn't support GROUP_BY dsl yet
            .order(diesel::dsl::sql::<diesel::sql_types::Text>(&order_query))
            .load_with_pagination(&connection, Some(page), Some(per_page))?;

        return Ok(result);
    }
}

impl PackageVersion {
    pub async fn count(pool: &DieselPgPool) -> Result<i64, Error> {
        let connection = pool.get()?;
        let result = package_versions
            .select(count(package_versions::id))
            .first::<i64>(&connection)?;

        Ok(result)
    }

    pub async fn create(
        version_package_id: i32,
        version_name: String,
        version_readme_content: String,
        version_rev: String,
        version_files: i32,
        version_size: i32,
        pool: &DieselPgPool,
    ) -> Result<PackageVersion, Error> {
        let connection = pool.get()?;

        let new_package_version = NewPackageVersion {
            package_id: version_package_id,
            version: version_name,
            readme_content: version_readme_content,
            rev: version_rev,
            total_files: version_files,
            total_size: version_size,
        };

        let record = diesel::insert_into(package_versions::table)
            .values(new_package_version)
            .get_result::<PackageVersion>(&connection)?;

        diesel::update(packages)
            .filter(packages::id.eq(version_package_id))
            .set(packages::updated_at.eq(now))
            .execute(&connection)?;

        Ok(record)
    }

    pub async fn from_package_id(
        uid: i32,
        sort_type: &PackageVersionSort,
        pool: &DieselPgPool,
    ) -> Result<Vec<PackageVersion>, Error> {
        let connection = pool.get()?;
        let versions = package_versions.filter(package_id.eq(uid));

        let records = match sort_type {
            PackageVersionSort::Latest => versions
                .order_by(package_versions::dsl::id.desc())
                .load::<PackageVersion>(&connection)?,
            PackageVersionSort::Oldest => versions
                .order_by(package_versions::dsl::id.asc())
                .load::<PackageVersion>(&connection)?,
            PackageVersionSort::MostDownloads => versions
                .order_by(package_versions::dsl::downloads_count.desc())
                .load::<PackageVersion>(&connection)?,
        };

        Ok(records)
    }
}

// Helpers for integration tests only. Wondering why cfg(test) below doesn't work... (commented out for now)
#[cfg(any(test, feature = "test"))]
#[derive(Insertable)]
#[table_name = "packages"]
pub struct NewTestPackage {
    pub name: String,
    pub description: String,
    pub repository_url: String,
    pub total_downloads_count: i32,
}

#[cfg(any(test, feature = "test"))]
impl Package {
    pub async fn create_test_package(
        package_name: &String,
        repo_url: &String,
        package_description: &String,
        package_version: &String,
        package_readme_content: &String,
        version_rev: &String,
        version_files: i32,
        version_size: i32,
        account_id_: Option<i32>,
        pool: &DieselPgPool,
    ) -> Result<i32, Error> {
        let connection = pool.get()?;

        let new_package = NewPackage {
            name: package_name.to_string(),
            description: package_description.to_string(),
            repository_url: repo_url.to_string(),
            account_id: account_id_,
        };

        let record = diesel::insert_into(packages::table)
            .values(new_package)
            .returning(PACKAGE_COLUMNS)
            .get_result::<Package>(&connection)?;

        PackageVersion::create(
            record.id,
            package_version.to_string(),
            package_readme_content.to_string(),
            version_rev.to_string(),
            version_files,
            version_size,
            pool,
        )
        .await
        .unwrap();
        Ok(record.id)
    }

    pub async fn create_test_package_with_downloads(
        package_name: &String,
        repo_url: &String,
        package_description: &String,
        package_downloads_count: i32,
        pool: &DieselPgPool,
    ) -> Result<i32, Error> {
        let connection = pool.get()?;

        let new_package = NewTestPackage {
            name: package_name.to_string(),
            description: package_description.to_string(),
            repository_url: repo_url.to_string(),
            total_downloads_count: package_downloads_count,
        };

        let record = diesel::insert_into(packages::table)
            .values(new_package)
            .returning(PACKAGE_COLUMNS)
            .get_result::<Package>(&connection)?;

        PackageVersion::create(
            record.id,
            String::from("0.0.1"),
            String::from("readme"),
            String::from("rev"),
            5,
            500,
            pool,
        )
        .await
        .unwrap();

        Ok(record.id)
    }
}
