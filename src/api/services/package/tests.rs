use crate::api::services::package::controller::{
    increase_download_count, register_package, DownloadInfo, PackageRequest,
};
use crate::packages::{Package, PackageVersion};
use crate::test::util::create_test_token;
use crate::test::{mock, DatabaseTestContext, DB_POOL};
use crate::utils::presenter::validate_name_and_version;

use jelly::actix_web::body::Body;
use jelly::actix_web::http::StatusCode;
use jelly::actix_web::web;
use jelly::error::Error;

fn init_form() -> web::Form<DownloadInfo> {
    web::Form(DownloadInfo {
        url: "https://github.com/move-language/move".to_string(),
        rev: "a8383d88fa48f4e1e0e91264cffbbd27136e4732".to_string(),
        subdir: "/tools".to_string(),
    })
}

async fn package_request() -> web::Json<PackageRequest> {
    web::Json(PackageRequest {
        github_repo_url: "".to_string(),
        total_files: 0,
        token: create_test_token().await,
        subdir: "".to_string(),
    })
}

#[actix_rt::test]
async fn register_package_create_new_packages() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let mut mock_http_request = mock::MockHttpRequest::new();
    mock_http_request
        .expect_db_pool()
        .returning(|| Ok(&DB_POOL));
    let package_request = package_request().await;
    assert_eq!(Package::count(&DB_POOL).await.unwrap(), 0);

    let response = register_package(mock_http_request, package_request).await;
    assert!(response.is_ok());
    let resp = response.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(resp.body().as_ref().unwrap(), &Body::from("name1"));
    assert_eq!(Package::count(&DB_POOL).await.unwrap(), 1);
}

#[actix_rt::test]
async fn register_package_returns_error_with_invalid_token() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let mut mock_http_request = mock::MockHttpRequest::new();
    mock_http_request
        .expect_db_pool()
        .returning(|| Ok(&DB_POOL));
    let mut package_request = package_request().await;
    package_request.token = "".to_string();
    assert_eq!(Package::count(&DB_POOL).await.unwrap(), 0);

    let response = register_package(mock_http_request, package_request).await;
    assert!(response.is_ok());
    let resp = response.unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        resp.body().as_ref().unwrap(),
        &Body::from("Invalid API token.")
    );
    assert_eq!(Package::count(&DB_POOL).await.unwrap(), 0);
}

#[actix_rt::test]
async fn increase_download_count_creates_shadow_package_when_package_version_not_existed() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let mut mock_http_request = mock::MockHttpRequest::new();
    mock_http_request
        .expect_db_pool()
        .returning(|| Ok(&DB_POOL));
    let form = init_form();
    increase_download_count(mock_http_request, form)
        .await
        .unwrap();
    let package = Package::get_by_name(&"name1".to_string(), &DB_POOL)
        .await
        .unwrap();
    PackageVersion::delete_by_package_id(package.id, &DB_POOL)
        .await
        .unwrap();
    assert_eq!(Package::count(&DB_POOL).await.unwrap(), 1);
    assert_eq!(PackageVersion::count(&DB_POOL).await.unwrap(), 0);

    let mut mock_http_request = mock::MockHttpRequest::new();
    mock_http_request
        .expect_db_pool()
        .returning(|| Ok(&DB_POOL));
    let form = init_form();
    increase_download_count(mock_http_request, form)
        .await
        .unwrap();
    assert_eq!(Package::count(&DB_POOL).await.unwrap(), 1);
    assert_eq!(PackageVersion::count(&DB_POOL).await.unwrap(), 1);
}

#[actix_rt::test]
async fn increase_download_count_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let mut mock_http_request = mock::MockHttpRequest::new();
    mock_http_request
        .expect_db_pool()
        .returning(|| Ok(&DB_POOL));
    let form = init_form();
    assert_eq!(Package::count(&DB_POOL).await.unwrap(), 0);
    assert_eq!(PackageVersion::count(&DB_POOL).await.unwrap(), 0);

    let response = increase_download_count(mock_http_request, form).await;
    assert!(response.is_ok());
    let resp = response.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(resp.body().as_ref().unwrap(), &Body::from("2"));
    assert_eq!(Package::count(&DB_POOL).await.unwrap(), 1);
    assert_eq!(PackageVersion::count(&DB_POOL).await.unwrap(), 1);
}

#[actix_rt::test]
async fn increase_download_count_returns_error_with_empty_rev() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let mut mock_http_request = mock::MockHttpRequest::new();
    mock_http_request
        .expect_db_pool()
        .returning(|| Ok(&DB_POOL));
    let mut form = init_form();
    form.rev = "".to_string();

    let response = increase_download_count(mock_http_request, form).await;
    assert!(response.is_ok());
    let resp = response.unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        resp.body().as_ref().unwrap(),
        &Body::from("Invalid git info.")
    );
    assert_eq!(Package::count(&DB_POOL).await.unwrap(), 0);
    assert_eq!(PackageVersion::count(&DB_POOL).await.unwrap(), 0);
}

#[actix_rt::test]
async fn increase_download_count_returns_error_with_empty_url() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let mut mock_http_request = mock::MockHttpRequest::new();
    mock_http_request
        .expect_db_pool()
        .returning(|| Ok(&DB_POOL));
    let mut form = init_form();
    form.url = "".to_string();

    let response = increase_download_count(mock_http_request, form).await;
    assert!(response.is_ok());
    let resp = response.unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        resp.body().as_ref().unwrap(),
        &Body::from("invalid git info")
    );
    assert_eq!(Package::count(&DB_POOL).await.unwrap(), 0);
    assert_eq!(PackageVersion::count(&DB_POOL).await.unwrap(), 0);
}

#[actix_rt::test]
async fn register_package_returns_error_when_database_is_not_available() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let mut mock_http_request = mock::MockHttpRequest::new();
    mock_http_request
        .expect_db_pool()
        .returning(|| Err(Error::Generic("Cannot get db pool".to_string())));
    let package_request = package_request().await;

    let response = register_package(mock_http_request, package_request).await;
    assert!(response.is_ok());
    let resp = response.unwrap();
    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(
        resp.body().as_ref().unwrap(),
        &Body::from("Something went wrong, please try again later.")
    );
}

#[actix_rt::test]
async fn increase_download_count_returns_error_when_database_is_not_available() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let mut mock_http_request = mock::MockHttpRequest::new();
    mock_http_request
        .expect_db_pool()
        .returning(|| Err(Error::Generic("Cannot get db pool".to_string())));
    let form = init_form();

    let response = increase_download_count(mock_http_request, form).await;
    assert!(response.is_ok());
    let resp = response.unwrap();
    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(
        resp.body().as_ref().unwrap(),
        &Body::from("Something went wrong, please try again later.")
    );
}

#[actix_rt::test]
async fn validate_name_and_version_works_for_names() {
    let valid_package_names = vec![
        "MoveStdLib",
        "A_Certain_Package",
        "package103",
        "package-in-kebab-case",
        "101-dalmatians",
        "up-and_down__and--up",
    ];
    for name in valid_package_names {
        let hints = validate_name_and_version(name, "0.1.0");
        assert!(hints.is_empty());
    }
    let invalid_package_names = vec![
        "special_package!",
        "package_歷要人",
        "@MystenLabs/Sui",
        "new.package.dot.com",
        "package-1.0.3",
        "invalid/package",
        "-package-name-",
        "-_-_-zigzag-package-name-_-_-",
    ];
    for name in invalid_package_names {
        let hints = validate_name_and_version(name, "0.1.0");
        assert_eq!(hints.len(), 1);
        assert_eq!(
            hints[0],
            "Package name should only contain alphanumeric characters connected by hyphens or underscores"
        );
    }
}

#[actix_rt::test]
async fn validate_name_and_version_works_for_versions() {
    let valid_versions = vec![
        "1.0.0-alpha",
        "1.0.0-alpha.1",
        "1.0.0-alpha.beta",
        "1.0.0-beta",
        "1.0.0-beta.2",
        "1.0.0-beta.11",
        "1.0.0-rc.1",
        "1.0.0",
    ];
    for version in valid_versions {
        let hints = validate_name_and_version("valid_name", version);
        assert!(hints.is_empty());
    }
    let invalid_versions = vec![
        "1.0.0-",
        "1.0.0-alpha+pre+release",
        "1.0.0*beta",
        "1.01.100",
        "1.0",
        "new.version",
    ];
    for version in invalid_versions {
        let hints = validate_name_and_version("valid_name", version);
        assert_eq!(hints.len(), 1, "version: {}", version);
        assert_eq!(
            hints[0],
            "Package version should adhere to semantic versioning (see https://semver.org)",
            "version: {}",
            version
        );
    }
}
