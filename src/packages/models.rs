use diesel::prelude::*;
use diesel::sql_types::{Integer, Text, Timestamptz};
use diesel::{sql_query, AsChangeset, Identifiable, Insertable, Queryable};

use diesel_full_text_search::{plainto_tsquery, TsVectorExtensions, TsVector};

use jelly::chrono::{DateTime, NaiveDateTime, Utc};
use jelly::error::Error;
use jelly::serde::{Deserialize, Serialize};
use jelly::DieselPgPool;
use diesel::result::Error::NotFound;

use mockall_double::double;

// use super::forms::{LoginForm, NewAccountForm};
#[double]
use crate::github_service::GithubService;
use crate::utils::paginate::{LoadPaginated, Paginate};
use crate::schema::package_versions;
use crate::schema::package_versions::dsl::*;
use crate::schema::packages;
use crate::schema::packages::dsl::*;

const PACKAGES_PER_PAGE: i64 = 10;

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
}

type PackageColumns = (
    packages::id,
    packages::name,
    packages::description,
    packages::repository_url,
    packages::total_downloads_count,
    packages::created_at,
    packages::updated_at
);

const PACKAGE_COLUMNS: PackageColumns = (
    packages::id,
    packages::name,
    packages::description,
    packages::repository_url,
    packages::total_downloads_count,
    packages::created_at,
    packages::updated_at
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
}

#[derive(Serialize, Deserialize)]
pub enum PackageSortField {
    name,
    description,
    most_downloads,
    newly_added,
}

#[derive(Serialize, Deserialize)]
pub enum PackageSortOrder {
    asc,
    desc,
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
    pub async fn create(
        repo_url: &String,
        package_description: &String,
        version_rev: &String,
        version_files: i32,
        version_size: i32,
        service: &GithubService,
        pool: &DieselPgPool,
    ) -> Result<i32, Error> {
        let connection = pool.get()?;

        let github_data = service.fetch_repo_data(&repo_url).unwrap();

        let record = match Package::get_by_name(&github_data.name, &pool).await {
            Ok(package) => package,
            Err(_) => {
                let new_package = NewPackage {
                    name: github_data.name,
                    description: package_description.to_string(),
                    repository_url: repo_url.to_string(),
                };

                let record = diesel::insert_into(packages::table)
                    .values(new_package)
                    .returning(PACKAGE_COLUMNS)
                    .get_result::<Package>(&connection)?;

                record
            }
        };

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
            .await
            .unwrap();
        }

        Ok(record.id)
    }

    pub async fn get(uid: i32, pool: &DieselPgPool) -> Result<Self, Error> {
        let connection = pool.get()?;
        let result = packages.find(uid).select(PACKAGE_COLUMNS).first::<Package>(&connection)?;

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
        service: &GithubService,
        pool: &DieselPgPool
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
                        let github_data = service.fetch_repo_data(&https_url)?;
                        PackageVersion::create(
                            package_id_,
                            github_data.name,
                            github_data.readme_content,
                            rev_.clone(),
                            -1,
                            -1,
                            pool
                        ).await?;
                    },
                    Err(e) => { return Err(Error::Database(e)); }
                };

                package_id_
            },
            Err(NotFound) => {
                // Package is not found, creating shadow package and package version
                Package::create(
                    &https_url, &String::from(""), &rev_,
                    -1,
                    -1,
                    service,
                    &pool)
                .await?
            },
            Err(e) => { return Err(Error::Database(e)); }
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
    ) -> Result<Vec<String>, Error> {
        let connection = pool.get()?;
        return Ok(packages
            .filter(name.ilike(format!("{}{}{}", "%", search_query, "%")))
            .select(packages::name)
            .load::<String>(&connection)?);
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
        let field = match sort_field {
            PackageSortField::name => "name",
            PackageSortField::description => "description",
            PackageSortField::most_downloads => "total_downloads_count",
            PackageSortField::newly_added => "updated_at",
        };
        let order = match sort_order {
            PackageSortOrder::asc => "ASC",
            PackageSortOrder::desc => "DESC",
        };
        let order_query = format!("{} {}", field, order);
        let search_query: &str = &search_query.split(" ").collect::<Vec<&str>>().join(" & ");

        let page = page.unwrap_or_else(|| 1);
        let per_page = per_page.unwrap_or_else(|| PACKAGES_PER_PAGE);

        let result: (Vec<PackageSearchResult>, i64, i64) = packages::table
            .inner_join(package_versions::table)
            .select((packages::id, packages::name, packages::description, packages::total_downloads_count, packages::created_at, packages::updated_at, diesel::dsl::sql::<diesel::sql_types::Text>("max(version) as version")))
            .filter(tsv.matches(plainto_tsquery(search_query)))
            .filter(diesel::dsl::sql("TRUE GROUP BY packages.id, name, description, total_downloads_count, packages.created_at, packages.updated_at")) // workaround since diesel 1.x doesn't support GROUP_BY dsl yet
            .order(diesel::dsl::sql::<diesel::sql_types::Text>(&order_query))
            .load_with_pagination(&connection, Some(page), Some(per_page))
            .unwrap();

        return Ok(result);
    }
}

impl PackageVersion {
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
            PackageVersionSort::MostDownloads => {
                versions
                    .order_by(package_versions::dsl::downloads_count.desc())
                    .load::<PackageVersion>(&connection)?
                //versions.load::<PackageVersion>(&connection)?
            }
        };

        Ok(records)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::{DatabaseTestContext, DB_POOL};
    use mockall::predicate::*;

    use crate::github_service::GithubRepoData;

    async fn setup() -> Result<(), Error> {
        let pool = &DB_POOL;
        let connection = pool.get()?;
        Package::create_test_package(
            &"The first package".to_string(),
            &"".to_string(),
            &"description 1".to_string(),
            &"1.0.0".to_string(),
            &"".to_string(),
            &"".to_string(),
            0,
            0,
            &pool,
        )
        .await;
        Package::create_test_package(
            &"The first Diva".to_string(),
            &"".to_string(),
            &"randomly picked, and changes some".to_string(),
            &"1.0.0".to_string(),
            &"".to_string(),
            &"".to_string(),
            0,
            0,
            &pool,
        )
        .await;
        Package::create_test_package(
            &"Charles Diya".to_string(),
            &"".to_string(),
            &"randomly picked, and changes some".to_string(),
            &"1.0.0".to_string(),
            &"".to_string(),
            &"".to_string(),
            0,
            0,
            &pool,
        )
        .await;
        Ok(())
    }

    #[actix_rt::test]
    async fn search_by_single_word_works() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();
        setup().await;
        let pool = &DB_POOL;
        let search_query = "package";
        let (search_result, total_count, total_pages) = Package::search(
            search_query,
            &PackageSortField::name,
            &PackageSortOrder::desc,
            Some(1),
            None,
            pool,
        )
        .await
        .unwrap();
        assert_eq!(total_count, 1);
        assert_eq!(total_pages, 1);
        assert_eq!(search_result[0].name, "The first package");
    }

    #[actix_rt::test]
    async fn search_by_multiple_words_works() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();
        setup().await;
        let pool = &DB_POOL;
        let search_query = "the package";
        let (search_result, total_count, total_pages) = Package::search(
            search_query,
            &PackageSortField::name,
            &PackageSortOrder::desc,
            Some(1),
            None,
            pool,
        )
        .await
        .unwrap();
        assert_eq!(total_count, 1);
        assert_eq!(total_pages, 1);
        assert_eq!(search_result[0].name, "The first package");
    }

    #[actix_rt::test]
    async fn search_return_multiple_result() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();
        setup().await;
        let pool = &DB_POOL;
        let search_query = "first";
        let (search_result, total_count, total_pages) = Package::search(
            search_query,
            &PackageSortField::name,
            &PackageSortOrder::desc,
            Some(1),
            Some(1),
            pool,
        )
        .await
        .unwrap();
        assert_eq!(total_count, 2);
        assert_eq!(total_pages, 2);

        assert_eq!(search_result.len(), 1);
        assert_eq!(search_result[0].name, "The first package");

        let (search_result, _total_count, _total_pages) = Package::search(
            search_query,
            &PackageSortField::name,
            &PackageSortOrder::desc,
            Some(2),
            Some(1),
            pool,
        )
        .await
        .unwrap();
        assert_eq!(search_result.len(), 1);
        assert_eq!(search_result[0].name, "The first Diva");
    }

    #[actix_rt::test]
    async fn create_package_works() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();

        let mut mock_github_service = GithubService::new();
        mock_github_service
            .expect_fetch_repo_data()
            .with(eq("repo_url".to_string()))
            .returning(|_| {
                Ok(GithubRepoData {
                    name: "name".to_string(),
                    version: "version".to_string(),
                    readme_content: "readme_content".to_string(),
                })
            });

        let uid = Package::create(
            &"repo_url".to_string(),
            &"package_description".to_string(),
            &"1".to_string(),
            2,
            100,
            &mock_github_service,
            &DB_POOL,
        )
        .await
        .unwrap();

        let package = Package::get(uid, &DB_POOL).await.unwrap();
        assert_eq!(package.name, "name");
        assert_eq!(package.description, "package_description");

        let package_version =
            &PackageVersion::from_package_id(uid, &PackageVersionSort::Latest, &DB_POOL)
                .await
                .unwrap()[0];
        assert_eq!(package_version.version, "version");
        match &package_version.readme_content {
            Some(content) => {
                assert_eq!(content, "readme_content");
            }
            None => {
                panic!("readme content is wrong")
            }
        }
    }

    #[actix_rt::test]
    async fn get_versions_by_latest() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();

        let mut mock_github_service = GithubService::new();
        mock_github_service.expect_fetch_repo_data().returning(|_| {
            Ok(GithubRepoData {
                name: "name".to_string(),
                version: "first_version".to_string(),
                readme_content: "first_readme_content".to_string(),
            })
        });

        let uid = Package::create(
            &"repo_url".to_string(),
            &"package_description".to_string(),
            &"1".to_string(),
            2,
            100,
            &mock_github_service,
            &DB_POOL,
        )
        .await
        .unwrap();

        PackageVersion::create(
            uid,
            "second_version".to_string(),
            "second_readme_content".to_string(),
            "1".to_string(),
            2,
            100,
            &DB_POOL,
        )
        .await
        .unwrap();

        let versions = PackageVersion::from_package_id(uid, &PackageVersionSort::Latest, &DB_POOL)
            .await
            .unwrap();

        assert_eq!(versions.len(), 2);
        assert_eq!(versions[0].version, "second_version");
        assert_eq!(versions[1].version, "first_version");
    }

    #[actix_rt::test]
    async fn get_versions_by_oldest() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();
        let mut mock_github_service = GithubService::new();
        mock_github_service.expect_fetch_repo_data().returning(|_| {
            Ok(GithubRepoData {
                name: "name".to_string(),
                version: "first_version".to_string(),
                readme_content: "first_readme_content".to_string(),
            })
        });

        let uid = Package::create(
            &"repo_url".to_string(),
            &"package_description".to_string(),
            &"1".to_string(),
            2,
            3,
            &mock_github_service,
            &DB_POOL,
        )
        .await
        .unwrap();

        PackageVersion::create(
            uid,
            "second_version".to_string(),
            "second_readme_content".to_string(),
            "1".to_string(),
            2,
            3,
            &DB_POOL,
        )
        .await
        .unwrap();

        let versions = PackageVersion::from_package_id(uid, &PackageVersionSort::Oldest, &DB_POOL)
            .await
            .unwrap();

        assert_eq!(versions.len(), 2);
        assert_eq!(versions[0].version, "first_version");
        assert_eq!(versions[1].version, "second_version");
    }

    #[actix_rt::test]
    async fn get_versions_by_most_downloads() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();
        let mut mock_github_service = GithubService::new();
        mock_github_service.expect_fetch_repo_data().returning(|_| {
            Ok(GithubRepoData {
                name: "name".to_string(),
                version: "first_version".to_string(),
                readme_content: "first_readme_content".to_string(),
            })
        });

        let uid = Package::create(
            &"repo_url".to_string(),
            &"package_description".to_string(),
            &"1".to_string(),
            2,
            3,
            &mock_github_service,
            &DB_POOL,
        )
        .await
        .unwrap();

        let mut version_2 = PackageVersion::create(
            uid,
            "second_version".to_string(),
            "second_readme_content".to_string(),
            "5".to_string(),
            2,
            3,
            &DB_POOL,
        )
        .await
        .unwrap();
        version_2.downloads_count = 5;
        _ = &version_2
            .save_changes::<PackageVersion>(&*(DB_POOL.get().unwrap()))
            .unwrap();

        let versions =
            PackageVersion::from_package_id(uid, &PackageVersionSort::MostDownloads, &DB_POOL)
                .await
                .unwrap();

        assert_eq!(versions.len(), 2);
        assert_eq!(versions[0].version, "second_version");
        assert_eq!(versions[1].version, "first_version");
    }

    #[actix_rt::test]
    async fn increase_download_count_works() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();

        let url = &"https://github.com/eadungn/taohe".to_string();
        let rev_ = &"30d4792b29330cf701af04b493a38a82102ed4fd".to_string();
        let package_id_ = Package::create_test_package(
            &"Test package".to_string(),
            url,
            &"".to_string(),
            &"".to_string(),
            &"".to_string(),
            rev_,
            20,
            100,
            &DB_POOL
        ).await.unwrap();

        let package_versions_before = PackageVersion
        ::from_package_id(package_id_, &PackageVersionSort::Latest, &DB_POOL)
            .await.unwrap();
        let package_version_before = package_versions_before.first().unwrap();
        assert_eq!(package_version_before.downloads_count, 0);

        let mut mock_github_service = GithubService::new();

        mock_github_service.expect_fetch_repo_data()
            .returning(|_| Ok(GithubRepoData {
                name: "name".to_string(),
                version: "first_version".to_string(),
                readme_content: "first_readme_content".to_string(),
            }));

        Package::increase_download_count(url, rev_, &mock_github_service, &DB_POOL).await.unwrap();
        Package::increase_download_count(url, rev_, &mock_github_service, &DB_POOL).await.unwrap();
        let package_versions_after = PackageVersion
            ::from_package_id(package_id_, &PackageVersionSort::Latest, &DB_POOL)
            .await.unwrap();
        let package_version_after = package_versions_after.first().unwrap();
        assert_eq!(package_version_after.downloads_count, 2);

        let _ = Package::increase_download_count(
            &"git@github.com:eadungn/taohe.git".to_string(),
            rev_,
            &mock_github_service,
            &DB_POOL).await;
        let package_versions_after = PackageVersion
            ::from_package_id(package_id_, &PackageVersionSort::Latest, &DB_POOL)
            .await.unwrap();
        let package_version_after = package_versions_after.first().unwrap();
        assert_eq!(package_version_after.downloads_count, 3);
    }

    #[actix_rt::test]
    async fn increase_download_count_for_nonexistent_package() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();

        let url = &"https://github.com/eadungn/taohe".to_string();
        let rev_ = &"30d4792b29330cf701af04b493a38a82102ed4fd".to_string();

        let mut mock_github_service = GithubService::new();
        mock_github_service.expect_fetch_repo_data()
            .returning(|_| Ok(GithubRepoData {
                name: "name".to_string(),
                version: "first_version".to_string(),
                readme_content: "first_readme_content".to_string(),
            }));

        let rev_not_existed = package_versions
            .filter(rev.eq(rev_))
            .count()
            .get_result::<i64>(&DB_POOL.get().unwrap())
            .unwrap();
        assert_eq!(rev_not_existed, 0);

        let package_before = packages
            .select(diesel::dsl::count(packages::id))
            .first::<i64>(&DB_POOL.get().unwrap())
            .unwrap();
        let package_version_before = package_versions
            .select(diesel::dsl::count(package_versions::id))
            .first::<i64>(&DB_POOL.get().unwrap())
            .unwrap();
        assert_eq!(package_before, 0);
        assert_eq!(package_version_before, 0);

        Package::increase_download_count(url, rev_, &mock_github_service, &DB_POOL).await.unwrap();
        Package::increase_download_count(url, rev_, &mock_github_service, &DB_POOL).await.unwrap();

        let package_after = packages
            .select(diesel::dsl::count(packages::id))
            .first::<i64>(&DB_POOL.get().unwrap())
            .unwrap();
        let package_version_after = package_versions
            .select(diesel::dsl::count(package_versions::id))
            .first::<i64>(&DB_POOL.get().unwrap())
            .unwrap();
        assert_eq!(package_after, 1);
        assert_eq!(package_version_after, 1);

        let rev_existed = package_versions
            .filter(rev.eq(rev_))
            .count()
            .execute(&DB_POOL.get().unwrap())
            .unwrap();
        assert_eq!(rev_existed, 1);
    }

    #[actix_rt::test]
    async fn increment_download_for_multiple_versions() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();

        let url = "https://github.com/eadungn/taohe".to_string();
        let rev1 = "30d4792b29330cf701af04b493a38a82102ed4fd".to_string();
        let rev2 = "fe66d6c60a3765c322edbcfa9b63650593971a28".to_string();
        let package_id_ = Package::create_test_package(
            &"Test package".to_string(), &url,
            &"".to_string(),
            &"".to_string(),
            &"".to_string(),
            &rev1,
            20, 100,
            &DB_POOL
        ).await.unwrap();
        PackageVersion::create(
            package_id_, String::from(""), String::from(""),
            rev2.clone(), 40, 200, &DB_POOL
        ).await.unwrap();

        let mut mock_github_service = GithubService::new();

        mock_github_service.expect_fetch_repo_data()
            .returning(|_| Ok(GithubRepoData {
                name: "name".to_string(),
                version: "first_version".to_string(),
                readme_content: "first_readme_content".to_string(),
            }));

        let package_versions_before = PackageVersion
        ::from_package_id(package_id_, &PackageVersionSort::Latest, &DB_POOL)
            .await.unwrap();
        for package_version_before in package_versions_before {
            assert_eq!(package_version_before.downloads_count, 0);
        }
        Package::increase_download_count(&url, &rev1, &mock_github_service, &DB_POOL).await.unwrap();
        Package::increase_download_count(&url, &rev2, &mock_github_service, &DB_POOL).await.unwrap();
        let package_versions_after = PackageVersion
            ::from_package_id(package_id_, &PackageVersionSort::Latest, &DB_POOL)
            .await.unwrap();
        for package_version_after in package_versions_after {
            assert_eq!(package_version_after.downloads_count, 1);
        }
        let package_total_downloads = Package::get(package_id_, &DB_POOL).await.unwrap().total_downloads_count;
        assert_eq!(package_total_downloads, 2);

        Package::increase_download_count(
            &"git@github.com:eadungn/taohe.git".to_string(),
            &rev2,
            &mock_github_service,
            &DB_POOL).await.unwrap();
        let package_versions_after = PackageVersion
            ::from_package_id(package_id_, &PackageVersionSort::Latest, &DB_POOL)
            .await.unwrap();
        let first_package_version_after = package_versions_after.first().unwrap();
        assert_eq!(first_package_version_after.downloads_count, 2);
        let second_package_version_after = package_versions_after.last().unwrap();
        assert_eq!(second_package_version_after.downloads_count, 1);
        let package_total_downloads = Package::get(1, &DB_POOL).await.unwrap().total_downloads_count;
        assert_eq!(package_total_downloads, 3);
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
        pool: &DieselPgPool,
    ) -> Result<i32, Error> {
        let connection = pool.get()?;

        let new_package = NewPackage {
            name: package_name.to_string(),
            description: package_description.to_string(),
            repository_url: repo_url.to_string(),
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
