use crate::api::services::package::controller::{
    increase_download_count, register_package, DownloadInfo, PackageRequest,
};
use crate::packages::{Package, PackageVersion};
use crate::test::{mock, DatabaseTestContext, DB_POOL};

use crate::test::util::create_test_token;
use jelly::actix_web::body::Body;
use jelly::actix_web::http::StatusCode;
use jelly::actix_web::web;

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
        &Body::from("Invalid Api Token")
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
        &Body::from("invalid git info")
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
