use diesel::{Queryable, Identifiable, AsChangeset, Insertable};
use diesel::prelude::*;

use jelly::chrono::{DateTime, Utc};
use jelly::error::Error;
use jelly::serde::{Deserialize, Serialize};
use jelly::DieselPgPool;

use mockall_double::double;

// use super::forms::{LoginForm, NewAccountForm};
use crate::schema::packages::dsl::*;
use crate::schema::packages;
use crate::schema::package_versions::dsl::*;
use crate::schema::package_versions;

#[double]
use crate::github_service::GithubService;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, AsChangeset)]
pub struct Package {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub repository_url: String,
    pub total_downloads_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name="packages"]
pub struct NewPackage {
    pub name: String,
    pub description: String,
    pub repository_url: String
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
}

#[derive(Insertable)]
#[table_name="package_versions"]
pub struct NewPackageVersion {
    pub package_id: i32,
    pub version: String,
    pub readme_content: String
}

#[derive(Serialize, Deserialize)]
pub enum PackageVersionSort {
    Latest,
    Oldest,
    MostDownloads
}

impl Package {
    pub async fn create(repo_url: &String, package_description: &String, service: &GithubService, pool: &DieselPgPool) -> Result<i32, Error> {
        let connection = pool.get()?;

        let github_data = service.fetch_repo_data(&repo_url).unwrap();

        let record = match Package::get_by_name(&github_data.name, &pool).await {
            Ok(package) => {
                package
            }
            Err(_) => {
                let new_package = NewPackage {
                    name: github_data.name,
                    description: package_description.to_string(),
                    repository_url: repo_url.to_string()
                };

                let record = diesel::insert_into(packages::table)
                    .values(new_package)
                    .get_result::<Package>(&connection)?;

                record
            }
        };

        match record.get_version(&github_data.version, &pool).await {
            Ok(_) => {}
            Err(_) => {
                PackageVersion::create(record.id, github_data.version, github_data.readme_content, pool).await.unwrap();
            }
        };

        Ok(record.id)
    }

    pub async fn get(uid: i32, pool: &DieselPgPool) -> Result<Self, Error> {
        let connection = pool.get()?;
        let result = packages
            .find(uid)
            .first::<Package>(&connection)?;

        Ok(result)
    }

    pub async fn get_by_name(package_name: &String, pool: &DieselPgPool) -> Result<Self, Error> {
        let connection = pool.get()?;
        let result = packages
            .filter(name.eq(package_name))
            .first::<Package>(&connection)?;

        Ok(result)
    }

    pub async fn get_version(&self, version_name: &String, pool: &DieselPgPool) -> Result<PackageVersion, Error> {
        let connection = pool.get()?;
        let result = package_versions
            .filter(package_id.eq(self.id).and(version.eq(version_name)))
            .first::<PackageVersion>(&connection)?;

        Ok(result)
    }
}

impl PackageVersion {
    pub async fn create(version_package_id: i32, version_name: String, version_readme_content: String, pool: &DieselPgPool) -> Result<PackageVersion, Error> {
        let connection = pool.get()?;

        let new_package_version = NewPackageVersion {
            package_id: version_package_id,
            version: version_name,
            readme_content: version_readme_content
        };

        let record = diesel::insert_into(package_versions::table)
            .values(new_package_version)
            .get_result::<PackageVersion>(&connection)?;

        Ok(record)
    }

    pub async fn from_package_id(uid: i32, sort_type: &PackageVersionSort, pool: &DieselPgPool) -> Result<Vec<PackageVersion>, Error> {
        let connection = pool.get()?;
        let versions = package_versions.filter(package_id.eq(uid));

        let records = match sort_type {
            PackageVersionSort::Latest => {
                versions.order_by(package_versions::dsl::id.desc()).load::<PackageVersion>(&connection)?
            }
            PackageVersionSort::Oldest => {
                versions.order_by(package_versions::dsl::id.asc()).load::<PackageVersion>(&connection)?
            }
            PackageVersionSort::MostDownloads => {
                versions.order_by(package_versions::dsl::downloads_count.desc()).load::<PackageVersion>(&connection)?
            }
        };

        Ok(records)
    }
}

#[cfg(test)]
mod tests {
    use mockall::{predicate::*};
    use crate::test::{DB_POOL, DatabaseTestContext};
    use super::*;

    use crate::github_service::GithubRepoData;

    #[actix_rt::test]
    async fn create_package_works() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();

        let mut mock_github_service = GithubService::new();
        mock_github_service.expect_fetch_repo_data()
            .with(eq("repo_url".to_string()))
            .returning(|_| Ok(GithubRepoData {
                name: "name".to_string(),
                version: "version".to_string(),
                readme_content: "readme_content".to_string(),
            }));

        let uid = Package::create(&"repo_url".to_string(), &"package_description".to_string(), &mock_github_service, &DB_POOL).await.unwrap();

        let package = Package::get(uid, &DB_POOL).await.unwrap();
        assert_eq!(package.name, "name");
        assert_eq!(package.description, "package_description");

        let package_version = &PackageVersion::from_package_id(uid, &PackageVersionSort::Latest, &DB_POOL).await.unwrap()[0];
        assert_eq!(package_version.version, "version");
        match &package_version.readme_content {
            Some(content) => {
                assert_eq!(content, "readme_content");
            },
            None => { panic!("readme content is wrong") }
        }
    }

    #[actix_rt::test]
    async fn get_versions_by_latest() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();

        let mut mock_github_service = GithubService::new();
        mock_github_service.expect_fetch_repo_data()
            .returning(|_| Ok(GithubRepoData {
                name: "name".to_string(),
                version: "first_version".to_string(),
                readme_content: "first_readme_content".to_string(),
            }));

        let uid = Package::create(&"repo_url".to_string(), &"package_description".to_string(), &mock_github_service, &DB_POOL).await.unwrap();

        PackageVersion::create(uid, "second_version".to_string(), "second_readme_content".to_string(), &DB_POOL).await.unwrap();

        let versions = PackageVersion::from_package_id(uid, &PackageVersionSort::Latest, &DB_POOL).await.unwrap();

        assert_eq!(versions.len(), 2);
        assert_eq!(versions[0].version, "second_version");
        assert_eq!(versions[1].version, "first_version");
    }

    #[actix_rt::test]
    async fn get_versions_by_oldest() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();
        let mut mock_github_service = GithubService::new();
        mock_github_service.expect_fetch_repo_data()
            .returning(|_| Ok(GithubRepoData {
                name: "name".to_string(),
                version: "first_version".to_string(),
                readme_content: "first_readme_content".to_string(),
            }));

        let uid = Package::create(&"repo_url".to_string(), &"package_description".to_string(), &mock_github_service, &DB_POOL).await.unwrap();

        PackageVersion::create(uid, "second_version".to_string(), "second_readme_content".to_string(), &DB_POOL).await.unwrap();

        let versions = PackageVersion::from_package_id(uid, &PackageVersionSort::Oldest, &DB_POOL).await.unwrap();

        assert_eq!(versions.len(), 2);
        assert_eq!(versions[0].version, "first_version");
        assert_eq!(versions[1].version, "second_version");
    }

    #[actix_rt::test]
    async fn get_versions_by_most_downloads() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();
        let mut mock_github_service = GithubService::new();
        mock_github_service.expect_fetch_repo_data()
            .returning(|_| Ok(GithubRepoData {
                name: "name".to_string(),
                version: "first_version".to_string(),
                readme_content: "first_readme_content".to_string(),
            }));

        let uid = Package::create(&"repo_url".to_string(), &"package_description".to_string(), &mock_github_service, &DB_POOL).await.unwrap();

        let mut version_2 = PackageVersion::create(uid, "second_version".to_string(), "second_readme_content".to_string(), &DB_POOL).await.unwrap();
        version_2.downloads_count = 5;
        _ = &version_2.save_changes::<PackageVersion>(&*(DB_POOL.get().unwrap())).unwrap();

        let versions = PackageVersion::from_package_id(uid, &PackageVersionSort::MostDownloads, &DB_POOL).await.unwrap();

        assert_eq!(versions.len(), 2);
        assert_eq!(versions[0].version, "second_version");
        assert_eq!(versions[1].version, "first_version");
    }
}

// Helpers for integration tests only. Wondering why cfg(test) below doesn't work... (commented out for now)
#[cfg(any(test, feature = "test"))]
impl Package {
    pub async fn create_test_package(package_name: &String, repo_url: &String, package_description: &String, package_version: &String, package_readme_content: &String, pool: &DieselPgPool) -> Result<i32, Error> {
        let connection = pool.get()?;

        let new_package = NewPackage {
            name: package_name.to_string(),
            description: package_description.to_string(),
            repository_url: repo_url.to_string()
        };

        let record = diesel::insert_into(packages::table)
            .values(new_package)
            .get_result::<Package>(&connection)?;

        PackageVersion::create(record.id, package_version.to_string(), package_readme_content.to_string(), pool).await.unwrap();

        Ok(record.id)
    }
}
