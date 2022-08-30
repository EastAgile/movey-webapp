use super::super::world::TestWorld;
use super::signin_steps;
use cucumber::{given, then, when};
use mainlib::packages::{Package, PackageVersion};
use mainlib::test::DB_POOL;
use thirtyfour::prelude::*;

#[given("There are packages in the system")]
async fn package_in_system(world: &mut TestWorld) {
    signin_steps::an_user(world).await;
    let uid = Package::create_test_package(
        &"test-package".to_string(),
        &"https://github.com/Elements-Studio/starswap-core".to_string(),
        &"package_description".to_string(),
        &"first_version".to_string(),
        &"first_readme_content".to_string(),
        &"rev".to_string(),
        2,
        100,
        Some(world.account.id),
        &DB_POOL,
    )
    .await
    .unwrap();
    let uid2 = Package::create_test_package(
        &"rand".to_string(),
        &"https://github.com/Elements-Studio/rand".to_string(),
        &"package_description".to_string(),
        &"first_version".to_string(),
        &"first_readme_content".to_string(),
        &"rev".to_string(),
        2,
        100,
        None,
        &DB_POOL,
    )
    .await
    .unwrap();
    let uid3 = Package::create_test_package(
        &"random_derive".to_string(),
        &"https://github.com/Elements-Studio/random_derive".to_string(),
        &"package_description".to_string(),
        &"first_version".to_string(),
        &"first_readme_content".to_string(),
        &"rev".to_string(),
        2,
        100,
        None,
        &DB_POOL,
    )
    .await
    .unwrap();
    PackageVersion::create(
        uid,
        "second_version".to_string(),
        "second_readme_content".to_string(),
        "rev_2".to_string(),
        2,
        100,
        None,
        &DB_POOL,
    )
    .await
    .unwrap();
    PackageVersion::create(
        uid2,
        "second_version".to_string(),
        "second_readme_content".to_string(),
        "rev_2".to_string(),
        2,
        100,
        None,
        &DB_POOL,
    )
    .await
    .unwrap();
    PackageVersion::create(
        uid3,
        "second_version".to_string(),
        "second_readme_content".to_string(),
        "rev_2".to_string(),
        2,
        100,
        None,
        &DB_POOL,
    )
    .await
    .unwrap();
}

#[given("There is a package that is in a subdir")]
async fn package_in_subdir(world: &mut TestWorld) {
    signin_steps::an_user(world).await;
    let uid = Package::create_test_package(
        &"long_url_package".to_string(),
        &"https://github.com/666lcz/toy-move-packages/tree/main/fungible-tokens".to_string(),
        &"package_description".to_string(),
        &"first_version".to_string(),
        &"first_readme_content".to_string(),
        &"rev".to_string(),
        2,
        100,
        None,
        &DB_POOL,
    )
    .await
    .unwrap();
    PackageVersion::create(
        uid,
        "second_version".to_string(),
        "second_readme_content".to_string(),
        "rev_2".to_string(),
        2,
        100,
        None,
        &DB_POOL,
    )
    .await
    .unwrap();
}

#[when("I access the package details page")]
async fn visit_package_page(world: &mut TestWorld) {
    world
        .driver
        .get("http://localhost:17002/packages/test-package")
        .await
        .unwrap();
}

#[when("I access the package details page of a package that was crawled")]
async fn visit_crawled_package_page(world: &mut TestWorld) {
    world
        .driver
        .get("http://localhost:17002/packages/rand")
        .await
        .unwrap();
}

#[when("I upload a new package to Movey")]
async fn upload_a_package(_world: &mut TestWorld) {
    Package::create_test_package(
        &"test_move_package".to_string(),
        &"https://github.com/ea-dungn/test_move_package".to_string(),
        &"package_description".to_string(),
        &"first_version".to_string(),
        &"first_readme_content".to_string(),
        &"rev".to_string(),
        2,
        100,
        None,
        &DB_POOL,
    )
    .await
    .unwrap();
}

#[then("I should see latest information of that package")]
async fn see_package_latest_info(world: &mut TestWorld) {
    let package_name_element = world
        .driver
        .find_element(By::ClassName("package-name"))
        .await
        .unwrap();
    let package_name = package_name_element.text().await.unwrap();
    assert_eq!(package_name, "test-package");

    let package_description_element = world
        .driver
        .find_element(By::ClassName("package-description"))
        .await
        .unwrap();
    let package_description = package_description_element.text().await.unwrap();
    assert_eq!(package_description, "package_description");

    let package_version_element = world
        .driver
        .find_element(By::ClassName("package-version"))
        .await
        .unwrap();
    let package_version = package_version_element.text().await.unwrap();
    assert_eq!(package_version, "second_version");

    let package_instruction_element = world
        .driver
        .find_element(By::ClassName("package-install-instruction"))
        .await
        .unwrap();
    let package_instruction = package_instruction_element.text().await.unwrap();

    let expected_result = format!(
        "{} = {{ git = \"{}\", rev = \"{}\" }}",
        "test-package", "https://github.com/Elements-Studio/starswap-core.git", "rev_2"
    );
    assert_eq!(package_instruction, expected_result);
}

#[then("I should see the owner information")]
async fn see_owner_info(world: &mut TestWorld) {
    let package_info_title = world
        .driver
        .find_element(By::Css(".package-owners .package-information-title"))
        .await
        .unwrap();
    assert_eq!(package_info_title.text().await.unwrap(), "Owners");

    let package_owner_info = world
        .driver
        .find_element(By::ClassName("package-owners-info"))
        .await
        .unwrap();
    let name_from_email = world.account.email.split('@').next().unwrap();
    assert_eq!(package_owner_info.text().await.unwrap(), name_from_email);
}

#[then("I should see a default owner name")]
async fn not_see_owner_info(world: &mut TestWorld) {
    let package_info_title = world
        .driver
        .find_element(By::Css(".package-owners .package-information-title"))
        .await;
    match package_info_title {
        Ok(_) => {}
        Err(_) => panic!(),
    }
    let package_owner_info = world
        .driver
        .find_element(By::ClassName("package-owners-info"))
        .await;
    match package_owner_info {
        Ok(element) => assert_eq!(element.text().await.unwrap(), "Elements-Studio"),
        Err(_) => panic!(),
    }
}

#[then("I should see a banner saying that this package is crawled")]
async fn see_banner(world: &mut TestWorld) {
    let package_banner_content = world
        .driver
        .find_element(By::ClassName("package-banner-content"))
        .await;
    assert!(package_banner_content.is_ok());
    let package_banner_content = package_banner_content.unwrap().text().await.unwrap();
    assert!(package_banner_content
        .contains("This package was crawled and has not been assigned owners yet."))
}

#[then("I should see that banner tell me to create an account")]
async fn see_register_cta(world: &mut TestWorld) {
    let package_banner_content = world
        .driver
        .find_element(By::ClassName("package-banner-content"))
        .await;
    assert!(package_banner_content.is_ok());

    let package_banner_cta = package_banner_content
        .unwrap()
        .find_element(By::Tag("a"))
        .await;
    assert!(package_banner_cta.is_ok());

    let package_banner_cta_url = package_banner_cta
        .as_ref()
        .unwrap()
        .get_attribute("href")
        .await
        .unwrap();
    assert!(package_banner_cta_url.is_some());
    assert_eq!(package_banner_cta_url.unwrap(), "/accounts/register");
    
    let package_banner_cta_content = package_banner_cta
        .unwrap()
        .text()
        .await
        .unwrap();
    assert!(package_banner_cta_content.contains("create an account"));
}

#[then("I should see that banner tell me to contact for ownership request")]
async fn see_contact_us_cta(world: &mut TestWorld) {
    let package_banner_content = world
        .driver
        .find_element(By::ClassName("package-banner-content"))
        .await;
    assert!(package_banner_content.is_ok());
    
    let package_banner_cta = package_banner_content
        .unwrap()
        .find_element(By::Tag("a"))
        .await;
    assert!(package_banner_cta.is_ok());

    let package_banner_cta_url = package_banner_cta
        .as_ref()
        .unwrap()
        .get_attribute("href")
        .await
        .unwrap();
    assert!(package_banner_cta_url.is_some());
    assert_eq!(package_banner_cta_url.unwrap(), "/contact");
    
    let package_banner_cta_content = package_banner_cta
        .unwrap()
        .text()
        .await
        .unwrap();
    assert!(package_banner_cta_content.contains("claim your package ownership"));
}

#[when("I click on versions of that package")]
async fn click_on_versions_tab(world: &mut TestWorld) {
    let versions_tab_element = world
        .driver
        .find_element(By::ClassName("tab-versions"))
        .await
        .unwrap();
    versions_tab_element.click().await.unwrap();
}

#[then("I should see the versions of that package by latest")]
async fn see_latest_versions(world: &mut TestWorld) {
    let version_item_elements = world
        .driver
        .find_elements(By::ClassName("package-version-number"))
        .await
        .unwrap();

    let first_version_item_element = version_item_elements.first().unwrap();
    let first_version_text = first_version_item_element.text().await.unwrap();
    assert_eq!(first_version_text, "second_version");

    let second_version_item_element = version_item_elements.last().unwrap();
    let second_version_text = second_version_item_element.text().await.unwrap();
    assert_eq!(second_version_text, "first_version");
}

#[when("I sort the package versions by oldest")]
async fn sort_versions_by_oldest(world: &mut TestWorld) {
    let select_element = world
        .driver
        .find_element(By::ClassName("select2-container"))
        .await
        .unwrap();
    select_element.click().await.unwrap();

    let dropdown_element = world
        .driver
        .find_element(By::ClassName("select2-dropdown"))
        .await
        .unwrap();
    let option_elements = dropdown_element
        .find_elements(By::ClassName("select2-results__option"))
        .await
        .unwrap();
    option_elements[1].click().await.unwrap();
}

#[then("I should see the versions of that package by oldest")]
async fn see_oldest_versions(world: &mut TestWorld) {
    let version_item_elements = world
        .driver
        .find_elements(By::ClassName("package-version-number"))
        .await
        .unwrap();

    let first_version_item_element = version_item_elements.first().unwrap();
    let first_version_text = first_version_item_element.text().await.unwrap();
    assert_eq!(first_version_text, "first_version");

    let second_version_item_element = version_item_elements.last().unwrap();
    let second_version_text = second_version_item_element.text().await.unwrap();
    assert_eq!(second_version_text, "second_version");
}

#[when("I click on an older version of the package")]
async fn click_on_old_version(world: &mut TestWorld) {
    let version_item_elements = world
        .driver
        .find_elements(By::ClassName("package-version-number"))
        .await
        .unwrap();

    let first_version_item_element = version_item_elements.first().unwrap();
    first_version_item_element.click().await.unwrap();
}

#[then("I should see the older version of the package")]
async fn see_older_version(world: &mut TestWorld) {
    let package_version_element = world
        .driver
        .find_element(By::ClassName("package-version"))
        .await
        .unwrap();
    let package_version = package_version_element.text().await.unwrap();
    assert_eq!(package_version, "first_version");
}

#[then("I should see the correct number of packages and package versions")]
async fn see_stats(world: &mut TestWorld) {
    let stats = world
        .driver
        .find_elements(By::ClassName("stat-no"))
        .await
        .unwrap();

    assert_eq!(stats.len(), 2);
    assert_eq!(stats[0].text().await.unwrap(), "3");
    assert_eq!(stats[1].text().await.unwrap(), "6");
}

#[then("I should see the number of packages and package versions increase by 1")]
async fn stats_after_upload_package(world: &mut TestWorld) {
    let stats = world
        .driver
        .find_elements(By::ClassName("stat-no"))
        .await
        .unwrap();

    assert_eq!(stats.len(), 2);
    assert_eq!(stats[0].text().await.unwrap(), "4");
    assert_eq!(stats[1].text().await.unwrap(), "7");
}

#[when("I access the package details page of a package that is in a subdir")]
async fn visit_subdir_package(world: &mut TestWorld) {
    world
        .driver
        .get("http://localhost:17002/packages/long_url_package")
        .await
        .unwrap();
}

#[then("I should see correct install instruction for that package")]
async fn see_install_instruction(world: &mut TestWorld) {
    let package_name_element = world
        .driver
        .find_element(By::ClassName("package-name"))
        .await
        .unwrap();
    let package_name = package_name_element.text().await.unwrap();
    assert_eq!(package_name, "long_url_package");

    let package_version_element = world
        .driver
        .find_element(By::ClassName("package-version"))
        .await
        .unwrap();
    let package_version = package_version_element.text().await.unwrap();
    assert_eq!(package_version, "second_version");

    let package_instruction_element = world
        .driver
        .find_element(By::ClassName("package-install-instruction"))
        .await
        .unwrap();
    let package_instruction = package_instruction_element.text().await.unwrap();
    let expected_result = format!(
        "{} = {{ git = \"{}\", rev = \"{}\", subdir = \"{}\" }}",
        "long_url_package",
        "https://github.com/666lcz/toy-move-packages.git",
        "rev_2",
        "fungible-tokens"
    );
    assert_eq!(package_instruction, expected_result);
}

#[then("I should see correct install instruction for older version of that package")]
async fn see_older_install_instruction(world: &mut TestWorld) {
    let package_name_element = world
        .driver
        .find_element(By::ClassName("package-name"))
        .await
        .unwrap();
    let package_name = package_name_element.text().await.unwrap();
    assert_eq!(package_name, "long_url_package");

    let package_version_element = world
        .driver
        .find_element(By::ClassName("package-version"))
        .await
        .unwrap();
    let package_version = package_version_element.text().await.unwrap();
    assert_eq!(package_version, "first_version");

    let package_instruction_element = world
        .driver
        .find_element(By::ClassName("package-install-instruction"))
        .await
        .unwrap();
    let package_instruction = package_instruction_element.text().await.unwrap();
    let expected_result = format!(
        "{} = {{ git = \"{}\", rev = \"{}\", subdir = \"{}\" }}",
        "long_url_package",
        "https://github.com/666lcz/toy-move-packages.git",
        "rev",
        "fungible-tokens"
    );
    assert_eq!(package_instruction, expected_result);
}
