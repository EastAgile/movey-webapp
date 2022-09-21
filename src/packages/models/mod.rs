use crate::sql::lower;

use diesel::dsl::{count, now, sum};
use diesel::prelude::*;
use diesel::sql_types::{Integer, Text, Timestamptz};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable};

use diesel_full_text_search::{plainto_tsquery, TsVectorExtensions};

use diesel::result::Error::NotFound;
use diesel::result::{DatabaseErrorKind, Error as DBError};
use jelly::chrono::{DateTime, NaiveDateTime, Utc};
use jelly::error::Error;
use jelly::serde::{Deserialize, Serialize};
use jelly::{DieselPgConnection, DieselPgPool};

use crate::github_service::GithubRepoData;
use crate::package_collaborators::package_collaborator::{PackageCollaborator, Role};
use jelly::Result;
use mockall_double::double;
use rayon::prelude::*;

#[cfg(test)]
mod tests;

#[double]
use crate::github_service::GithubService;
use crate::schema::package_collaborators;
use crate::schema::package_versions;
use crate::schema::package_versions::dsl::*;
use crate::schema::packages;
use crate::schema::packages::dsl::*;
use crate::utils::paginate::LoadPaginated;
use crate::utils::token::generate_secure_alphanumeric_string;

use super::views::serializer::slugify_package_name;

pub const PACKAGES_PER_PAGE: i64 = 10;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, AsChangeset, QueryableByName)]
#[table_name = "packages"]
pub struct Package {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub repository_url: String,
    pub total_downloads_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub slug: Option<String>,
}

type PackageColumns = (
    packages::id,
    packages::name,
    packages::description,
    packages::repository_url,
    packages::total_downloads_count,
    packages::created_at,
    packages::updated_at,
    packages::slug,
);

pub const PACKAGE_COLUMNS: PackageColumns = (
    packages::id,
    packages::name,
    packages::description,
    packages::repository_url,
    packages::total_downloads_count,
    packages::created_at,
    packages::updated_at,
    packages::slug,
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
    pub slug: Option<String>,
    #[sql_type = "Text"]
    pub version: String,
}

#[derive(Insertable)]
#[table_name = "packages"]
#[derive(Clone)]
pub struct NewPackage {
    pub name: String,
    pub description: String,
    pub repository_url: String,
    pub slug: String,
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
    pub downloads_count: i32,
}

#[derive(Serialize, Deserialize)]
pub enum PackageVersionSort {
    Latest,
    Oldest,
    MostDownloads,
}

impl Package {
    pub fn hmm(pool: &DieselPgPool) -> Result<()> {

        // conn.transaction(|| -> Result<()> {
        let conn = pool.get()?;
            let results: Vec<Package> = packages
                .select(PACKAGE_COLUMNS)
                .load(&conn)?;
            results.into_par_iter().for_each(|p| {
                let conn = pool.get().unwrap();
                let original_slug = slug::slugify(p.name);
                let mut new_slug = original_slug.clone();
                loop {
                    let is_existed = packages
                        .filter(slug.eq(new_slug.clone()))
                        .count()
                        .get_result(&conn);
                    if let Ok(0) = is_existed {
                        diesel::update(packages.filter(packages::id.eq(p.id)))
                            .set(packages::slug.eq(new_slug.clone()))
                            .execute(&conn).unwrap();
                        break;
                    } else {
                        new_slug = format!("{}-{}", &original_slug, generate_secure_alphanumeric_string(4));
                    }
                }
            });
            // for p in results {
            //     let original_slug = slug::slugify(p.name);
            //     let mut new_slug = original_slug.clone();
            //     loop {
            //         let is_existed = packages
            //             .filter(slug.eq(new_slug.clone()))
            //             .count()
            //             .get_result(&conn);
            //         if let Ok(0) = is_existed {
            //             diesel::update(packages.filter(packages::id.eq(p.id)))
            //                 .set(packages::slug.eq(new_slug.clone()))
            //                 .execute(&conn)?;
            //             break;
            //         } else {
            //             new_slug = format!("{}-{}", &original_slug, generate_secure_alphanumeric_string(4));
            //         }
            //     }
            // };
            Ok(())
        // })
    }

    pub fn count(pool: &DieselPgPool) -> Result<i64> {
        let connection = pool.get()?;
        let result = packages
            .select(count(packages::id))
            .first::<i64>(&connection)?;

        Ok(result)
    }

    pub fn create(
        repo_url: &str,
        package_description: &str,
        version_rev: &str,
        version_files: i32,
        version_size: i32,
        account_id_: Option<i32>,
        service: &GithubService,
        subdir: Option<String>,
        pool: &DieselPgPool,
    ) -> Result<Package> {
        let github_data = service.fetch_repo_data(repo_url, subdir, None)?;

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
    }

    pub fn create_from_crawled_data(
        repo_url: &str,
        package_description: &str,
        version_rev: &str,
        version_files: i32,
        version_size: i32,
        account_id_: Option<i32>,
        github_data: GithubRepoData,
        pool: &DieselPgPool,
    ) -> Result<Package> {
        let conn = pool.get()?;
        conn.transaction(|| -> Result<Package> {
            let (record, package_owner_id) =
                match Package::get_by_name_and_repo_url(&github_data.name, repo_url, &conn) {
                    Ok(package) => {
                        let collaborators =
                            PackageCollaborator::get_by_package_id(package.id, &conn)?;
                        let owner_id = if collaborators.len() > 0 {
                            Some(collaborators[0])
                        } else {
                            None
                        };
                        (package, owner_id)
                    }
                    Err(_) => {
                        let mut new_package = NewPackage {
                            name: github_data.name.clone(),
                            description: package_description.to_string(),
                            repository_url: repo_url.to_string(),
                            slug: slugify_package_name(&github_data.name),
                        };
                        let maximum_allowed_collisions = std::env::var("MAX_COLLISIONS_ALLOWED")
                            .unwrap_or_else(|_| "3".to_string())
                            .parse::<usize>()
                            .unwrap();
                        let mut insert_result = diesel::insert_into(packages::table)
                            .values(new_package.clone())
                            .on_conflict(packages::slug)
                            .do_nothing()
                            .returning(PACKAGE_COLUMNS)
                            .get_result::<Package>(&conn);
                        if insert_result.is_err() {
                            for i in 0..maximum_allowed_collisions {
                                new_package.slug = format!(
                                    "{}-{}",
                                    &new_package.slug,
                                    generate_secure_alphanumeric_string(4)
                                );
                                insert_result = diesel::insert_into(packages::table)
                                    .values(new_package.clone())
                                    .on_conflict(packages::slug)
                                    .do_nothing()
                                    .returning(PACKAGE_COLUMNS)
                                    .get_result::<Package>(&conn);
                                if insert_result.is_ok() {
                                    break;
                                };
                                if i == maximum_allowed_collisions - 1 {
                                    return Err(Error::Generic(String::from("asdz")));
                                }
                            }
                        }
                        let inserted_record = insert_result.unwrap();
                        if account_id_.is_some() {
                            PackageCollaborator::new_owner(
                                inserted_record.id,
                                account_id_.unwrap(),
                                account_id_.unwrap(),
                                &conn,
                            )?;
                        }
                        (inserted_record, account_id_)
                    }
                };

            // Only creates new version if same user with package owner
            if package_owner_id == account_id_ {
                let package_version_not_exist = record.get_version(&github_data.version, &conn);
                if package_version_not_exist.is_err() {
                    let e = package_version_not_exist.unwrap_err();
                    if let Error::Database(NotFound) = e {
                        PackageVersion::create(
                            record.id,
                            github_data.version,
                            github_data.readme_content,
                            version_rev.to_string(),
                            version_files,
                            version_size,
                            None,
                            &conn,
                        )?;
                    } else {
                        return Err(e);
                    }
                } else {
                    // return package version already exists error
                    return Err(Error::Database(DBError::DatabaseError(
                        DatabaseErrorKind::UniqueViolation,
                        Box::new(record.slug),
                    )));
                }
            } else {
                return Err(Error::Database(DBError::DatabaseError(
                    DatabaseErrorKind::ForeignKeyViolation,
                    Box::new(record.slug),
                )));
            }

            Ok(record)
        })
    }

    pub fn get(uid: i32, pool: &DieselPgPool) -> Result<Self> {
        let connection = pool.get()?;
        let result = packages
            .find(uid)
            .select(PACKAGE_COLUMNS)
            .first::<Package>(&connection)?;

        Ok(result)
    }

    pub fn get_by_name(package_name: &str, pool: &DieselPgPool) -> Result<Self> {
        let connection = pool.get()?;

        let result = packages
            .filter(name.eq(package_name))
            .select(PACKAGE_COLUMNS)
            .first::<Package>(&connection)?;

        Ok(result)
    }

    pub fn get_by_slug(package_slug: &str, conn: &DieselPgConnection) -> Result<Self> {
        let result = packages
            .filter(slug.eq(package_slug))
            .select(PACKAGE_COLUMNS)
            .first::<Package>(conn)?;

        Ok(result)
    }

    pub fn get_by_name_and_repo_url(
        package_name: &str,
        repo_url: &str,
        conn: &DieselPgConnection,
    ) -> Result<Self> {
        let result = packages
            .filter(name.eq(package_name).and(repository_url.eq(repo_url)))
            .select(PACKAGE_COLUMNS)
            .first::<Package>(conn)?;

        Ok(result)
    }

    pub fn get_by_name_case_insensitive(
        package_name: &str,
        pool: &DieselPgPool,
    ) -> Result<Vec<Self>> {
        let connection = pool.get()?;

        Ok(packages
            .filter(lower(name).eq(package_name.to_lowercase()))
            .select(PACKAGE_COLUMNS)
            .load::<Package>(&connection)?)
    }

    pub fn get_badge_info(
        package_name: &str,
        pool: &DieselPgPool,
    ) -> Result<Vec<(String, i32, String, i32)>> {
        let connection = pool.get()?;

        let result: Vec<(String, i32, String, i32)> = packages::table
            .inner_join(package_versions::table)
            .filter(lower(packages::name).eq(package_name.to_lowercase()))
            .filter(diesel::dsl::sql(
                "TRUE GROUP BY packages.name, packages.total_downloads_count, package_versions.version, package_versions.downloads_count",
            ))
            .select((
                packages::name,
                packages::total_downloads_count,
                package_versions::version,
                package_versions::downloads_count,
            ))
            .load::<(String, i32, String, i32)>(&connection)?;

        Ok(result)
    }

    pub fn get_by_account(owner_id: i32, pool: &DieselPgPool) -> Result<Vec<PackageSearchResult>> {
        let connection = pool.get()?;

        let result = packages
            .inner_join(package_collaborators::table)
            .filter(package_collaborators::account_id.eq(owner_id))
            .inner_join(package_versions::table)
            .select((packages::id, packages::name, packages::description, packages::total_downloads_count, packages::created_at, packages::updated_at, packages::slug, diesel::dsl::sql::<diesel::sql_types::Text>("max(version) as version")))
            .filter(diesel::dsl::sql("TRUE GROUP BY packages.id, name, description, total_downloads_count, packages.created_at, packages.updated_at, packages.slug")) // workaround since diesel 1.x doesn't support GROUP_BY dsl yet
            .load::<PackageSearchResult>(&connection)?;

        Ok(result)
    }

    pub fn get_by_account_paginated(
        owner_id: i32,
        sort_field: &PackageSortField,
        sort_order: &PackageSortOrder,
        page: Option<i64>,
        per_page: Option<i64>,
        pool: &DieselPgPool,
    ) -> Result<(Vec<PackageSearchResult>, i64, i64)> {
        let connection = pool.get()?;
        let field = sort_field.to_column_name();
        let order = sort_order.to_order_direction();
        let order_query = format!("packages.{} {}", field, order);

        let page = page.unwrap_or(1);
        let per_page = per_page.unwrap_or(PACKAGES_PER_PAGE);
        if page < 1 || per_page < 1 {
            return Err(Error::Generic(String::from("Invalid page number.")));
        }

        let result: (Vec<PackageSearchResult>, i64, i64) = packages::table
            .inner_join(package_collaborators::table)
            .filter(package_collaborators::account_id.eq(owner_id).and(package_collaborators::role.eq(Role::Owner as i32)))
            .inner_join(package_versions::table)
            .select((packages::id, packages::name, packages::description, packages::total_downloads_count, packages::created_at, packages::updated_at, packages::slug, diesel::dsl::sql::<diesel::sql_types::Text>("max(version) as version")))
            .filter(diesel::dsl::sql("TRUE GROUP BY packages.id, name, description, total_downloads_count, packages.created_at, packages.updated_at, packages.slug")) // workaround since diesel 1.x doesn't support GROUP_BY dsl yet
            .order(diesel::dsl::sql::<diesel::sql_types::Text>(&order_query))
            .load_with_pagination(&connection, Some(page), Some(per_page))?;

        Ok(result)
    }

    pub fn get_downloads(owner_id: i32, pool: &DieselPgPool) -> Result<i64> {
        let connection = pool.get()?;
        let result = packages
            .inner_join(package_collaborators::table)
            .filter(
                package_collaborators::account_id
                    .eq(owner_id)
                    .and(package_collaborators::role.eq(Role::Owner as i32)),
            )
            .select(sum(total_downloads_count))
            .first::<Option<i64>>(&connection)?;

        match result {
            Some(result) => Ok(result),
            None => Ok(0),
        }
    }

    pub fn get_version(
        &self,
        version_name: &String,
        conn: &DieselPgConnection,
    ) -> Result<PackageVersion> {
        let result = package_versions
            .filter(package_id.eq(self.id).and(version.eq(version_name)))
            .first::<PackageVersion>(conn)?;

        Ok(result)
    }

    pub fn change_owner(
        package_id_: i32,
        new_owner_id: i32,
        conn: &DieselPgConnection,
    ) -> Result<()> {
        diesel::update(package_collaborators::table)
            .filter(
                package_collaborators::package_id
                    .eq(package_id_)
                    .and(package_collaborators::role.eq(Role::Owner as i32)),
            )
            .set(package_collaborators::account_id.eq(new_owner_id))
            .execute(conn)?;
        Ok(())
    }

    pub fn increase_download_count(
        url: &String,
        rev_: &String,
        subdir: &String,
        service: &GithubService,
        pool: &DieselPgPool,
    ) -> Result<usize> {
        let connection = pool.get()?;

        let mut https_url = url.to_owned();
        if url.starts_with("git@github.com") {
            https_url = url
                .replace(':', "/")
                .replace("git@", "https://")
                .replace(".git", "");
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
                            None,
                            &connection,
                        )?;
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
                    service.fetch_repo_data(
                        &https_url,
                        Some(subdir_with_toml),
                        Some(rev_.clone()),
                    )?
                };
                if !subdir.is_empty() {
                    https_url = format!("{}/blob/{}/{}", https_url, github_data.rev, subdir);
                }
                Package::create_from_crawled_data(
                    &https_url,
                    &github_data.description.clone(),
                    rev_,
                    -1,
                    github_data.size,
                    None,
                    github_data,
                    pool,
                )?
                .id
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

    pub fn auto_complete_search(
        search_query: &str,
        pool: &DieselPgPool,
    ) -> Result<Vec<(String, String, String, String)>> {
        let connection = pool.get()?;
        let result: Vec<(String, String, String, String)> = packages::table
            .inner_join(package_versions::table)
            .filter(name.ilike(format!("%{}%", search_query)))
            .filter(diesel::dsl::sql("TRUE GROUP BY packages.id, name, description, total_downloads_count, packages.created_at, packages.updated_at, slug"))
            .select((packages::name, packages::description, diesel::dsl::sql::<diesel::sql_types::Text>("max(version) as version"), packages::slug))
            .load::<(String, String, String, String)>(&connection)?;

        Ok(result)
    }

    pub fn search(
        search_query: &str,
        sort_field: &PackageSortField,
        sort_order: &PackageSortOrder,
        page: Option<i64>,
        per_page: Option<i64>,
        pool: &DieselPgPool,
    ) -> Result<(Vec<PackageSearchResult>, i64, i64)> {
        let connection = pool.get()?;
        let field = sort_field.to_column_name();
        let order = sort_order.to_order_direction();
        let order_query = format!("packages.{} {}", field, order);
        let search_query: &str = &search_query.split(' ').collect::<Vec<&str>>().join(" & ");

        let page = page.unwrap_or(1);
        let per_page = per_page.unwrap_or(PACKAGES_PER_PAGE);
        if page < 1 || per_page < 1 {
            return Err(Error::Generic(String::from("Invalid page number.")));
        }

        let result: (Vec<PackageSearchResult>, i64, i64) = packages::table
            .inner_join(package_versions::table)
            .select((packages::id, packages::name, packages::description, packages::total_downloads_count, packages::created_at, packages::updated_at, packages::slug, diesel::dsl::sql::<diesel::sql_types::Text>("max(version) as version")))
            .filter(name.ilike(format!("%{}%", search_query))
                .or(tsv.matches(plainto_tsquery(search_query))))
            .filter(diesel::dsl::sql("TRUE GROUP BY packages.id, name, description, total_downloads_count, packages.created_at, packages.updated_at, packages.slug")) // workaround since diesel 1.x doesn't support GROUP_BY dsl yet
            .order(diesel::dsl::sql::<diesel::sql_types::Text>(&order_query))
            .load_with_pagination(&connection, Some(page), Some(per_page))?;

        Ok(result)
    }

    pub fn all_packages(
        sort_field: &PackageSortField,
        sort_order: &PackageSortOrder,
        page: Option<i64>,
        per_page: Option<i64>,
        pool: &DieselPgPool,
    ) -> Result<(Vec<PackageSearchResult>, i64, i64)> {
        let connection = pool.get()?;
        let field = sort_field.to_column_name();
        let order = sort_order.to_order_direction();
        let order_query = format!("packages.{} {}", field, order);

        let page = page.unwrap_or(1);
        let per_page = per_page.unwrap_or(PACKAGES_PER_PAGE);
        if page < 1 || per_page < 1 {
            return Err(Error::Generic(String::from("Invalid page number.")));
        }

        let result: (Vec<PackageSearchResult>, i64, i64) = packages::table
            .inner_join(package_versions::table)
            .select((packages::id, packages::name, packages::description, packages::total_downloads_count, packages::created_at, packages::updated_at, packages::slug, diesel::dsl::sql::<diesel::sql_types::Text>("max(version) as version")))
            .filter(diesel::dsl::sql("TRUE GROUP BY packages.id, name, description, total_downloads_count, packages.created_at, packages.updated_at, packages.slug")) // workaround since diesel 1.x doesn't support GROUP_BY dsl yet
            .order(diesel::dsl::sql::<diesel::sql_types::Text>(&order_query))
            .load_with_pagination(&connection, Some(page), Some(per_page))?;

        Ok(result)
    }
}

impl PackageVersion {
    pub fn count(pool: &DieselPgPool) -> Result<i64> {
        let connection = pool.get()?;
        let result = package_versions
            .select(count(package_versions::id))
            .first::<i64>(&connection)?;

        Ok(result)
    }

    pub fn delete_by_package_id(package_id_: i32, pool: &DieselPgPool) -> Result<usize> {
        let connection = pool.get()?;
        let result = diesel::delete(package_versions.filter(package_id.eq(package_id_)))
            .execute(&connection)?;

        Ok(result)
    }

    pub fn create(
        version_package_id: i32,
        version_name: String,
        version_readme_content: String,
        version_rev: String,
        version_files: i32,
        version_size: i32,
        version_download: Option<i32>,
        conn: &DieselPgConnection,
    ) -> Result<PackageVersion> {
        let new_package_version = NewPackageVersion {
            package_id: version_package_id,
            version: version_name,
            readme_content: version_readme_content,
            rev: version_rev,
            total_files: version_files,
            total_size: version_size,
            downloads_count: version_download.unwrap_or(0),
        };

        let record = diesel::insert_into(package_versions::table)
            .values(new_package_version)
            .get_result::<PackageVersion>(conn)?;

        diesel::update(packages)
            .filter(packages::id.eq(version_package_id))
            .set(packages::updated_at.eq(now))
            .execute(conn)?;

        Ok(record)
    }

    pub fn from_package_id(
        uid: i32,
        sort_type: &PackageVersionSort,
        pool: &DieselPgPool,
    ) -> Result<Vec<PackageVersion>> {
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
    pub slug: String,
}

#[cfg(any(test, feature = "test"))]
impl Package {
    pub fn create_test_package(
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
    ) -> Result<i32> {
        let connection = pool.get()?;

        let new_package = NewPackage {
            name: package_name.to_string(),
            description: package_description.to_string(),
            repository_url: repo_url.to_string(),
            slug: slugify_package_name(package_name),
        };

        let record = diesel::insert_into(packages::table)
            .values(new_package)
            .returning(PACKAGE_COLUMNS)
            .get_result::<Package>(&connection)?;

        if account_id_.is_some() {
            PackageCollaborator::new_owner(
                record.id,
                account_id_.unwrap(),
                account_id_.unwrap(),
                &connection,
            )?;
        }

        PackageVersion::create(
            record.id,
            package_version.to_string(),
            package_readme_content.to_string(),
            version_rev.to_string(),
            version_files,
            version_size,
            None,
            &connection,
        )
        .unwrap();
        Ok(record.id)
    }

    pub fn create_test_package_with_downloads(
        package_name: &String,
        repo_url: &String,
        package_description: &String,
        package_downloads_count: i32,
        pool: &DieselPgPool,
    ) -> Result<i32> {
        let connection = pool.get()?;

        let new_package = NewTestPackage {
            name: package_name.to_string(),
            description: package_description.to_string(),
            repository_url: repo_url.to_string(),
            total_downloads_count: package_downloads_count,
            slug: slugify_package_name(package_name),
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
            None,
            &connection,
        )
        .unwrap();

        Ok(record.id)
    }

    pub fn create_test_package_with_multiple_versions(
        package_name: &String,
        repo_url: &String,
        package_description: &String,
        package_downloads_count: i32,
        pool: &DieselPgPool,
    ) -> Result<i32> {
        let conn = pool.get().unwrap();

        let new_package = NewTestPackage {
            name: package_name.to_string(),
            description: package_description.to_string(),
            repository_url: repo_url.to_string(),
            total_downloads_count: package_downloads_count,
            slug: package_name.to_string(),
        };
        let record = diesel::insert_into(packages::table)
            .values(new_package)
            .returning(PACKAGE_COLUMNS)
            .get_result::<Package>(&conn)?;

        PackageVersion::create(
            record.id,
            String::from("0.0.1"),
            String::from("readme"),
            String::from("rev"),
            5,
            500,
            Some(500),
            &conn,
        )
        .unwrap();

        PackageVersion::create(
            record.id,
            String::from("0.0.2"),
            String::from("readme"),
            String::from("rev"),
            5,
            1000,
            Some(1000),
            &conn,
        )
        .unwrap();
        Ok(record.id)
    }
}
