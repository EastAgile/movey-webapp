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
    pub async fn create(repo_url: &String, package_description: &String, service: &GithubService, pool: &DieselPgPool) -> Result<i32, Error> {
        let connection = pool.get()?;

        let github_data = service.fetch_repo_data(&repo_url).unwrap();

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

    pub async fn get(uid: i32, pool: &DieselPgPool) -> Result<Self, Error> {
        let connection = pool.get()?;
        let result = packages
            .find(uid)
            .first::<Package>(&connection)?;

        Ok(result)
    }

    pub async fn get_versions(&self, pool: &DieselPgPool) -> Result<Vec<PackageVersion>, Error> {
        let connection = pool.get()?;
        let result = package_versions
            .filter(package_id.eq(self.id))
            .load::<PackageVersion>(&connection)?;

        Ok(result)
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

        let package_version = &package.get_versions(&DB_POOL).await.unwrap()[0];
        assert_eq!(package_version.version, "version");
        match &package_version.readme_content {
            Some(content) => {
                assert_eq!(content, "readme_content");
            },
            None => { panic!("readme content is wrong") }
        }
    }
}
