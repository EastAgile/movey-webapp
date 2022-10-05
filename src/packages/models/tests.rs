use regex::Regex;
use crate::test::{DatabaseTestContext, DB_POOL};

use crate::github_service::GithubRepoData;
use crate::packages::models::*;
use crate::test::util::{create_stub_packages, setup_user};

fn setup(account_id_: Option<i32>) -> Result<()> {
    let pool = &DB_POOL;
    Package::create_test_package(
        &"The first package".to_string(),
        &"https://github.com/EastAgile/ea-movey".to_string(),
        &"description 1".to_string(),
        &"1.0.0".to_string(),
        &"".to_string(),
        &"".to_string(),
        0,
        0,
        account_id_,
        pool,
    )?;
    Package::create_test_package(
        &"The first Diva".to_string(),
        &"".to_string(),
        &"randomly picked, and changes some".to_string(),
        &"1.0.0".to_string(),
        &"".to_string(),
        &"".to_string(),
        0,
        0,
        account_id_,
        pool,
    )?;
    Package::create_test_package(
        &"Charles Diya".to_string(),
        &"".to_string(),
        &"randomly picked, and changes some".to_string(),
        &"1.0.0".to_string(),
        &"".to_string(),
        &"".to_string(),
        0,
        0,
        account_id_,
        pool,
    )?;
    Ok(())
}

#[actix_rt::test]
async fn delete_package_version_by_package_id_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let uid = setup_user(None, None);
    create_stub_packages(uid, 1);
    assert_eq!(1, Package::count(&DB_POOL).unwrap());
    assert_eq!(1, PackageVersion::count(&DB_POOL).unwrap());
    let package_ = Package::get_by_account(uid, &DB_POOL).unwrap();
    PackageVersion::delete_by_package_id(package_.get(0).unwrap().id, &DB_POOL).unwrap();
    assert_eq!(0, PackageVersion::count(&DB_POOL).unwrap());
}

#[actix_rt::test]
async fn delete_package_version_by_package_id_returns_error_if_not_existed() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let result = PackageVersion::delete_by_package_id(-1, &DB_POOL).unwrap();
    assert_eq!(0, result);
}

#[actix_rt::test]
async fn search_by_single_word_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    setup(None).unwrap();
    let pool = &DB_POOL;
    let search_query = "package";
    let (search_result, total_count, total_pages) = Package::search(
        search_query,
        &PackageSortField::Name,
        &PackageSortOrder::Desc,
        Some(1),
        None,
        pool,
    )
    .unwrap();
    assert_eq!(total_count, 1);
    assert_eq!(total_pages, 1);
    assert_eq!(search_result[0].name, "The first package");
}

#[actix_rt::test]
async fn search_by_multiple_words_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    setup(None).unwrap();
    let pool = &DB_POOL;
    let search_query = "the package";
    let (search_result, total_count, total_pages) = Package::search(
        search_query,
        &PackageSortField::Name,
        &PackageSortOrder::Desc,
        Some(1),
        None,
        pool,
    )
    .unwrap();
    assert_eq!(total_count, 1);
    assert_eq!(total_pages, 1);
    assert_eq!(search_result[0].name, "The first package");
}

#[actix_rt::test]
async fn search_return_multiple_result() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    setup(None).unwrap();
    let pool = &DB_POOL;
    let search_query = "first";
    let (search_result, total_count, total_pages) = Package::search(
        search_query,
        &PackageSortField::Name,
        &PackageSortOrder::Desc,
        Some(1),
        Some(1),
        pool,
    )
    .unwrap();
    assert_eq!(total_count, 2);
    assert_eq!(total_pages, 2);

    assert_eq!(search_result.len(), 1);
    assert_eq!(search_result[0].name, "The first package");

    let (search_result, _total_count, _total_pages) = Package::search(
        search_query,
        &PackageSortField::Name,
        &PackageSortOrder::Desc,
        Some(2),
        Some(1),
        pool,
    )
    .unwrap();
    assert_eq!(search_result.len(), 1);
    assert_eq!(search_result[0].name, "The first Diva");
}

#[actix_rt::test]
async fn search_by_partial_name_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    setup(None).unwrap();
    let pool = &DB_POOL;
    let search_query = "charl";
    let (search_result, total_count, total_pages) = Package::search(
        search_query,
        &PackageSortField::Name,
        &PackageSortOrder::Desc,
        Some(1),
        Some(2),
        pool,
    )
    .unwrap();
    assert_eq!(total_count, 1);
    assert_eq!(total_pages, 1);
    assert_eq!(search_result.len(), 1);
    assert_eq!(search_result[0].name, "Charles Diya");
}

#[actix_rt::test]
async fn search_by_partial_description_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    setup(None).unwrap();
    let pool = &DB_POOL;
    let search_query = "random pick";
    let (search_result, total_count, total_pages) = Package::search(
        search_query,
        &PackageSortField::Name,
        &PackageSortOrder::Asc,
        Some(1),
        Some(2),
        pool,
    )
    .unwrap();
    assert_eq!(total_count, 2);
    assert_eq!(total_pages, 1);
    assert_eq!(search_result.len(), 2);
    assert!(search_result[0].name == "Charles Diya");
    assert!(search_result[1].name == "The first Diva");
}

#[actix_rt::test]
async fn search_sorted_by_newly_added_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    setup(None).unwrap();
    let pool = &DB_POOL;
    let search_query = "random";
    let (search_result, total_count, total_pages) = Package::search(
        search_query,
        &PackageSortField::NewlyAdded,
        &PackageSortOrder::Desc,
        Some(1),
        Some(2),
        pool,
    )
    .unwrap();
    assert_eq!(total_count, 2);
    assert_eq!(total_pages, 1);
    assert_eq!(search_result.len(), 2);
    assert!(search_result[0].name == "Charles Diya");
    assert!(search_result[1].name == "The first Diva");
}

#[actix_rt::test]
async fn search_sorted_by_recently_updated_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    setup(None).unwrap();
    let conn = DB_POOL.get().unwrap();

    let the_first_package = Package::get_by_name(&"The first package".to_string(), &DB_POOL).unwrap();
    assert!(the_first_package.name.contains("The first package"));
    assert!(the_first_package.description.contains("description 1"));

    PackageVersion::create(
        the_first_package.id,
        "second_version".to_string(),
        "".to_string(),
        "".to_string(),
        25,
        500,
        None,
        &conn,
    )
    .unwrap();
    let total_packages = Package::count(&DB_POOL).unwrap();
    assert_eq!(total_packages, 3);
    let total_versions = PackageVersion::count(&DB_POOL).unwrap();
    assert_eq!(total_versions, 4);

    let search_query = "first";
    let (search_result, total_count, total_pages) = Package::search(
        search_query,
        &PackageSortField::RecentlyUpdated,
        &PackageSortOrder::Desc,
        Some(1),
        Some(2),
        &DB_POOL,
    )
    .unwrap();
    assert_eq!(total_count, 2);
    assert_eq!(total_pages, 1);
    assert_eq!(search_result.len(), 2);
    assert_eq!(search_result[0].name, "The first package");
    assert_eq!(search_result[1].name, "The first Diva");
}

#[actix_rt::test]
async fn all_packages_with_pagination() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    setup(None).unwrap();
    let pool = &DB_POOL;
    let (search_result, total_count, total_pages) = Package::all_packages(
        &PackageSortField::Name,
        &PackageSortOrder::Desc,
        Some(1),
        Some(2),
        pool,
    )
    .unwrap();
    assert_eq!(total_count, 3);
    assert_eq!(total_pages, 2);

    assert_eq!(search_result.len(), 2);
    assert_eq!(search_result[0].name, "The first package");
    assert_eq!(search_result[1].name, "The first Diva");

    let (search_result, _total_count, _total_pages) = Package::all_packages(
        &PackageSortField::Name,
        &PackageSortOrder::Desc,
        Some(2),
        Some(2),
        pool,
    )
    .unwrap();
    assert_eq!(search_result.len(), 1);
    assert_eq!(search_result[0].name, "Charles Diya");
}

#[actix_rt::test]
async fn all_packages_with_pagination_and_sort_by_recently_updated() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    setup(None).unwrap();
    let pool = &DB_POOL;
    let conn = pool.get().unwrap();

    let the_first_package = Package::get_by_name(&"The first package".to_string(), pool).unwrap();
    assert!(the_first_package.name.contains("The first package"));
    assert!(the_first_package.description.contains("description 1"));

    PackageVersion::create(
        the_first_package.id,
        "second_version".to_string(),
        "".to_string(),
        "".to_string(),
        25,
        500,
        None,
        &conn,
    )
    .unwrap();
    let total_packages = Package::count(pool).unwrap();
    assert_eq!(total_packages, 3);
    let total_versions = PackageVersion::count(pool).unwrap();
    assert_eq!(total_versions, 4);

    let (search_result, total_count, total_pages) = Package::all_packages(
        &PackageSortField::RecentlyUpdated,
        &PackageSortOrder::Desc,
        Some(1),
        Some(2),
        pool,
    )
    .unwrap();
    assert_eq!(total_count, 3);
    assert_eq!(total_pages, 2);
    assert_eq!(search_result.len(), 2);
    assert_eq!(search_result[0].name, "The first package");
    assert_eq!(search_result[1].name, "Charles Diya");

    let (search_result, _total_count, _total_pages) = Package::all_packages(
        &PackageSortField::RecentlyUpdated,
        &PackageSortOrder::Desc,
        Some(2),
        Some(2),
        pool,
    )
    .unwrap();
    assert_eq!(search_result.len(), 1);
    assert_eq!(search_result[0].name, "The first Diva");
}

#[actix_rt::test]
async fn all_packages_with_pagination_and_sort_by_newly_added() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    setup(None).unwrap();
    let pool = &DB_POOL;

    let (search_result, total_count, total_pages) = Package::all_packages(
        &PackageSortField::NewlyAdded,
        &PackageSortOrder::Desc,
        Some(1),
        Some(2),
        pool,
    )
    .unwrap();
    assert_eq!(total_count, 3);
    assert_eq!(total_pages, 2);

    assert_eq!(search_result.len(), 2);
    assert_eq!(search_result[0].name, "Charles Diya");
    assert_eq!(search_result[1].name, "The first Diva");

    let (search_result, _total_count, _total_pages) = Package::all_packages(
        &PackageSortField::NewlyAdded,
        &PackageSortOrder::Desc,
        Some(2),
        Some(2),
        pool,
    )
    .unwrap();
    assert_eq!(search_result.len(), 1);
    assert_eq!(search_result[0].name, "The first package");
}

#[actix_rt::test]
async fn create_package_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let uid = setup_user(None, None);

    let mut mock_github_service = GithubService::new();
    mock_github_service
        .expect_fetch_repo_data()
        .withf(|x: &str, y: &Option<String>, z: &Option<String>| {
            x == "repo_url" && y.is_none() && z.is_none()
        })
        .returning(|_, _, _| {
            Ok(GithubRepoData {
                name: "My Test String!!!1!1".to_string(),
                version: "version".to_string(),
                readme_content: "readme_content".to_string(),
                description: "".to_string(),
                size: 0,
                stars_count: 0,
                forks_count: 0,
                url: "".to_string(),
                rev: "".to_string(),
            })
        });

    let uid = Package::create(
        "repo_url",
        "package_description",
        "1",
        2,
        100,
        Some(uid),
        &mock_github_service,
        None,
        &DB_POOL,
    )
    .unwrap().id;

    let package = Package::get(uid, &DB_POOL).unwrap();
    assert_eq!(package.name, "My Test String!!!1!1");
    assert_eq!(package.description, "package_description");
    assert_eq!(package.slug, "my-test-string-1-1");

    let mut mock_github_service = GithubService::new();
    mock_github_service
        .expect_fetch_repo_data()
        .withf(|x: &str, y: &Option<String>, z: &Option<String>| {
            x == "repo_url2" && y.is_none() && z.is_none()
        })
        .returning(|_, _, _| {
            Ok(GithubRepoData {
                name: "My Test String!!!1!1".to_string(),
                version: "version".to_string(),
                readme_content: "readme_content".to_string(),
                description: "".to_string(),
                size: 0,
                stars_count: 0,
                forks_count: 0,
                url: "".to_string(),
                rev: "".to_string(),
            })
        });
    let uid2 = Package::create(
        "repo_url2",
        "package_description",
        "1",
        2,
        100,
        Some(uid),
        &mock_github_service,
        None,
        &DB_POOL,
    )
        .unwrap().id;
    let package = Package::get(uid2, &DB_POOL).unwrap();
    assert_eq!(package.name, "My Test String!!!1!1");
    assert_eq!(package.description, "package_description");
    let re = Regex::new(r"^my-test-string-1-1-[a-zA-Z0-9]{4}$").unwrap();
    assert!(re.is_match(&package.slug));

    let package_version =
        &PackageVersion::from_package_id(uid, &PackageVersionSort::Latest, &DB_POOL).unwrap()[0];
    assert_eq!(package_version.version, "version");
    match &package_version.readme_content {
        Some(content) => {
            assert_eq!(content, "readme_content");
        }
        None => {
            panic!("readme content is wrong")
        }
    }

    // Asserts that no new version is created with different account id
    let mut mock_github_service_2 = GithubService::new();
    mock_github_service_2
        .expect_fetch_repo_data()
        .withf(|x: &str, y: &Option<String>, z: &Option<String>| {
            x == "repo_url" && y.is_none() && z.is_none()
        })
        .returning(|_, _, _| {
            Ok(GithubRepoData {
                name: "My Test String!!!1!1".to_string(),
                version: "version_2".to_string(),
                readme_content: "readme_content".to_string(),
                description: "".to_string(),
                size: 0,
                stars_count: 0,
                forks_count: 0,
                url: "".to_string(),
                rev: "".to_string(),
            })
        });

    let result = Package::create(
        "repo_url",
        "package_description",
        "1",
        2,
        100,
        None,
        &mock_github_service_2,
        None,
        &DB_POOL,
    );
    assert!(result.is_err());

    let versions =
        PackageVersion::from_package_id(uid, &PackageVersionSort::Latest, &DB_POOL).unwrap();

    assert_eq!(versions.len(), 1);
}

#[actix_rt::test]
async fn get_versions_by_latest() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let conn = &DB_POOL.get().unwrap();

    let mut mock_github_service = GithubService::new();
    mock_github_service
        .expect_fetch_repo_data()
        .returning(|_, _, _| {
            Ok(GithubRepoData {
                name: "name".to_string(),
                version: "first_version".to_string(),
                readme_content: "first_readme_content".to_string(),
                description: "".to_string(),
                size: 0,
                stars_count: 0,
                forks_count: 0,
                url: "".to_string(),
                rev: "".to_string(),
            })
        });

    let uid = Package::create(
        "repo_url",
        "package_description",
        "1",
        2,
        100,
        None,
        &mock_github_service,
        None,
        &DB_POOL,
    )
    .unwrap().id;

    PackageVersion::create(
        uid,
        "second_version".to_string(),
        "second_readme_content".to_string(),
        "1".to_string(),
        2,
        100,
        None,
        &conn,
    )
    .unwrap();

    let versions =
        PackageVersion::from_package_id(uid, &PackageVersionSort::Latest, &DB_POOL).unwrap();

    assert_eq!(versions.len(), 2);
    assert_eq!(versions[0].version, "second_version");
    assert_eq!(versions[1].version, "first_version");
}

#[actix_rt::test]
async fn get_versions_by_oldest() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let conn = DB_POOL.get().unwrap();

    let mut mock_github_service = GithubService::new();
    mock_github_service
        .expect_fetch_repo_data()
        .returning(|_, _, _| {
            Ok(GithubRepoData {
                name: "name".to_string(),
                version: "first_version".to_string(),
                readme_content: "first_readme_content".to_string(),
                description: "".to_string(),
                size: 0,
                stars_count: 0,
                forks_count: 0,
                url: "".to_string(),
                rev: "".to_string(),
            })
        });

    let uid = Package::create(
        "repo_url",
        "package_description",
        "1",
        2,
        3,
        None,
        &mock_github_service,
        None,
        &DB_POOL,
    )
    .unwrap().id;

    PackageVersion::create(
        uid,
        "second_version".to_string(),
        "second_readme_content".to_string(),
        "1".to_string(),
        2,
        3,
        None,
        &conn,
    )
    .unwrap();

    let versions =
        PackageVersion::from_package_id(uid, &PackageVersionSort::Oldest, &DB_POOL).unwrap();

    assert_eq!(versions.len(), 2);
    assert_eq!(versions[0].version, "first_version");
    assert_eq!(versions[1].version, "second_version");
}

#[actix_rt::test]
async fn get_versions_by_most_downloads() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let mut mock_github_service = GithubService::new();
    mock_github_service
        .expect_fetch_repo_data()
        .returning(|_, _, _| {
            Ok(GithubRepoData {
                name: "name".to_string(),
                version: "first_version".to_string(),
                readme_content: "first_readme_content".to_string(),
                description: "".to_string(),
                size: 0,
                stars_count: 0,
                forks_count: 0,
                url: "".to_string(),
                rev: "".to_string(),
            })
        });

    let uid = Package::create(
        "repo_url",
        "package_description",
        "1",
        2,
        3,
        None,
        &mock_github_service,
        None,
        &DB_POOL,
    )
    .unwrap().id;

    let mut version_2 = PackageVersion::create(
        uid,
        "second_version".to_string(),
        "second_readme_content".to_string(),
        "5".to_string(),
        2,
        3,
        None,
        &DB_POOL.get().unwrap(),
    )
    .unwrap();
    version_2.downloads_count = 5;
    let _ = &version_2
        .save_changes::<PackageVersion>(&*(DB_POOL.get().unwrap()))
        .unwrap();

    let versions =
        PackageVersion::from_package_id(uid, &PackageVersionSort::MostDownloads, &DB_POOL).unwrap();

    assert_eq!(versions.len(), 2);
    assert_eq!(versions[0].version, "second_version");
    assert_eq!(versions[1].version, "first_version");
}

#[actix_rt::test]
async fn count_package_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    assert_eq!(Package::count(&DB_POOL).unwrap(), 0);
    assert_eq!(PackageVersion::count(&DB_POOL).unwrap(), 0);
    setup(None).unwrap();

    assert_eq!(Package::count(&DB_POOL).unwrap(), 3);
    assert_eq!(PackageVersion::count(&DB_POOL).unwrap(), 3);

    PackageVersion::create(
        1,
        "second_version".to_string(),
        "second_readme_content".to_string(),
        "rev_2".to_string(),
        2,
        100,
        None,
        &DB_POOL.get().unwrap(),
    )
    .unwrap();
    assert_eq!(PackageVersion::count(&DB_POOL).unwrap(), 4);
}

#[actix_rt::test]
async fn increase_download_count_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let uid = setup_user(None, None);
    let mut no_downloads = Package::get_downloads(uid, &DB_POOL);
    assert_eq!(0, no_downloads.unwrap());

    let url = &"https://github.com/eadungn/taohe".to_string();
    let rev_ = &"30d4792b29330cf701af04b493a38a82102ed4fd".to_string();
    let package_id_ = Package::create_test_package(
        &"Test package".to_string(),
        url,
        &"".to_string(),
        &"1.0.0".to_string(),
        &"".to_string(),
        rev_,
        20,
        100,
        Some(uid),
        &DB_POOL,
    )
    .unwrap();

    let package_versions_before =
        PackageVersion::from_package_id(package_id_, &PackageVersionSort::Latest, &DB_POOL)
            .unwrap();
    let package_version_before = package_versions_before.first().unwrap();
    assert_eq!(package_version_before.downloads_count, 0);

    let mut mock_github_service = GithubService::new();

    mock_github_service
        .expect_fetch_repo_data()
        .returning(|_, _, _| {
            Ok(GithubRepoData {
                name: "name".to_string(),
                version: "1.0.0".to_string(),
                readme_content: "first_readme_content".to_string(),
                description: "".to_string(),
                size: 0,
                stars_count: 0,
                forks_count: 0,
                url: "".to_string(),
                rev: "".to_string(),
            })
        });

    Package::increase_download_count(url, rev_, &String::new(), &mock_github_service, &DB_POOL)
        .unwrap();
    no_downloads = Package::get_downloads(uid, &DB_POOL);
    assert_eq!(1, no_downloads.unwrap());

    Package::increase_download_count(url, rev_, &String::new(), &mock_github_service, &DB_POOL)
        .unwrap();
    no_downloads = Package::get_downloads(uid, &DB_POOL);
    assert_eq!(2, no_downloads.unwrap());
    let package_versions_after =
        PackageVersion::from_package_id(package_id_, &PackageVersionSort::Latest, &DB_POOL)
            .unwrap();
    let package_version_after = package_versions_after.first().unwrap();
    assert_eq!(package_version_after.downloads_count, 2);

    let _ = Package::increase_download_count(
        &"git@github.com:eadungn/taohe.git".to_string(),
        rev_,
        &String::new(),
        &mock_github_service,
        &DB_POOL,
    );

    no_downloads = Package::get_downloads(uid, &DB_POOL);
    assert_eq!(3, no_downloads.unwrap());
    let package_versions_after =
        PackageVersion::from_package_id(package_id_, &PackageVersionSort::Latest, &DB_POOL)
            .unwrap();
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
    mock_github_service
        .expect_fetch_repo_data()
        .returning(|_, _, _| {
            Ok(GithubRepoData {
                name: "name".to_string(),
                version: "first_version".to_string(),
                readme_content: "first_readme_content".to_string(),
                description: "".to_string(),
                size: 0,
                stars_count: 0,
                forks_count: 0,
                url: "".to_string(),
                rev: "".to_string(),
            })
        });

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

    Package::increase_download_count(url, rev_, &String::new(), &mock_github_service, &DB_POOL)
        .unwrap();
    Package::increase_download_count(url, rev_, &String::new(), &mock_github_service, &DB_POOL)
        .unwrap();

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
async fn increase_download_count_for_multiple_versions() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let url = "https://github.com/eadungn/taohe".to_string();
    let rev1 = "30d4792b29330cf701af04b493a38a82102ed4fd".to_string();
    let rev2 = "fe66d6c60a3765c322edbcfa9b63650593971a28".to_string();
    let package_id_ = Package::create_test_package(
        &"Test package".to_string(),
        &url,
        &"".to_string(),
        &"1.0.0".to_string(),
        &"".to_string(),
        &rev1,
        20,
        100,
        None,
        &DB_POOL,
    )
    .unwrap();
    PackageVersion::create(
        package_id_,
        String::from("1.0.0"),
        String::from(""),
        rev2.clone(),
        40,
        200,
        None,
        &DB_POOL.get().unwrap(),
    )
    .unwrap();

    let mut mock_github_service = GithubService::new();

    mock_github_service
        .expect_fetch_repo_data()
        .returning(|_, _, _| {
            Ok(GithubRepoData {
                name: "name".to_string(),
                version: "1.0.0".to_string(),
                readme_content: "first_readme_content".to_string(),
                description: "".to_string(),
                size: 0,
                stars_count: 0,
                forks_count: 0,
                url: "".to_string(),
                rev: "".to_string(),
            })
        });

    let package_versions_before =
        PackageVersion::from_package_id(package_id_, &PackageVersionSort::Latest, &DB_POOL)
            .unwrap();
    for package_version_before in package_versions_before {
        assert_eq!(package_version_before.downloads_count, 0);
    }
    Package::increase_download_count(&url, &rev1, &String::new(), &mock_github_service, &DB_POOL)
        .unwrap();
    Package::increase_download_count(&url, &rev2, &String::new(), &mock_github_service, &DB_POOL)
        .unwrap();
    let package_versions_after =
        PackageVersion::from_package_id(package_id_, &PackageVersionSort::Latest, &DB_POOL)
            .unwrap();
    for package_version_after in package_versions_after {
        assert_eq!(package_version_after.downloads_count, 1);
    }
    let package_total_downloads = Package::get(package_id_, &DB_POOL)
        .unwrap()
        .total_downloads_count;
    assert_eq!(package_total_downloads, 2);

    Package::increase_download_count(
        &"git@github.com:eadungn/taohe.git".to_string(),
        &rev2,
        &String::new(),
        &mock_github_service,
        &DB_POOL,
    )
    .unwrap();
    let package_versions_after =
        PackageVersion::from_package_id(package_id_, &PackageVersionSort::Latest, &DB_POOL)
            .unwrap();
    let first_package_version_after = package_versions_after.first().unwrap();
    assert_eq!(first_package_version_after.downloads_count, 2);
    let second_package_version_after = package_versions_after.last().unwrap();
    assert_eq!(second_package_version_after.downloads_count, 1);
    let package_total_downloads = Package::get(1, &DB_POOL).unwrap().total_downloads_count;
    assert_eq!(package_total_downloads, 3);
}

#[actix_rt::test]
async fn get_badge_info() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let search_query = "The first package";
    Package::create_test_package_with_multiple_versions(
        &"The first package".to_string(),
        &"".to_string(),
        &"description 1".to_string(),
        1500,
        10,
        2,
        &DB_POOL,
    )
    .unwrap();
    let mut expected: Vec<(String, i32, String, i32)> = vec![(
        "The first package".to_string(),
        1500,
        "0.0.1".to_string(),
        500,
    )];
    expected.push((
        "The first package".to_string(),
        1500,
        "0.0.2".to_string(),
        1000,
    ));
    let result = Package::get_badge_info(search_query, &DB_POOL).unwrap();
    assert_eq!(result.len(), 2);
    assert_eq!(result, expected);
}

#[actix_rt::test]
async fn get_by_name_case_insensitive_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    setup(None).unwrap();

    let packages_list = Package::get_by_name_case_insensitive("the FIRST package", &DB_POOL);
    assert!(packages_list.is_ok());
    let packages_list = packages_list.unwrap();
    assert_eq!(packages_list.len(), 1);
    assert_eq!(packages_list[0].name, "The first package");

    let packages_list = Package::get_by_name_case_insensitive("ChArLeS DiYa", &DB_POOL);
    assert!(packages_list.is_ok());
    let packages_list = packages_list.unwrap();
    assert_eq!(packages_list.len(), 1);
    assert_eq!(packages_list[0].name, "Charles Diya");

    let packages_list = Package::get_by_name_case_insensitive("Charles D1ya", &DB_POOL);
    assert!(packages_list.is_ok());
    let packages_list = packages_list.unwrap();
    assert_eq!(packages_list.len(), 0);
}

#[actix_rt::test]
async fn get_by_account_with_pagination() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let pool = &DB_POOL;
    let uid = setup_user(None, None);
    setup(Some(uid)).unwrap();
    let (search_result, total_count, total_pages) = Package::get_by_account_paginated(
        uid,
        &PackageSortField::Name,
        &PackageSortOrder::Desc,
        Some(1),
        Some(2),
        pool,
    )
    .unwrap();
    assert_eq!(total_count, 3);
    assert_eq!(total_pages, 2);

    assert_eq!(search_result.len(), 2);
    assert_eq!(search_result[0].name, "The first package");
    assert_eq!(search_result[1].name, "The first Diva");

    let (search_result, _total_count, _total_pages) = Package::all_packages(
        &PackageSortField::Name,
        &PackageSortOrder::Desc,
        Some(2),
        Some(2),
        pool,
    )
    .unwrap();
    assert_eq!(search_result.len(), 1);
    assert_eq!(search_result[0].name, "Charles Diya");
}

#[actix_rt::test]
async fn get_package() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let uid = setup_user(None, None);
    setup(Some(uid)).unwrap();

    let res = Package::get_by_name_and_repo_url(
        "The first package",
        "https://github.com/EastAgile/ea-movey",
        &DB_POOL.get().unwrap(),
    );
    assert!(res.is_ok());
    let res = res.unwrap();
    assert_eq!(res.name, "The first package");
    assert_eq!(res.repository_url, "https://github.com/EastAgile/ea-movey");

    let res = Package::get_by_slug(
        "the-first-package",
        &DB_POOL.get().unwrap(),
    );
    assert!(res.is_ok());
    let res = res.unwrap();
    assert_eq!(res.slug, "the-first-package");
    assert_eq!(res.name, "The first package");
}
