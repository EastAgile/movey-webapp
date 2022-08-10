use cucumber::{then, when, given};
use jelly::forms::{EmailField, PasswordField};
use thirtyfour::prelude::*;
use mainlib::accounts::Account;
use mainlib::accounts::forms::NewAccountForm;
use mainlib::packages::{Package, PackageVersion};
use mainlib::test::DB_POOL;
use crate::features::world::AccountInformation;

use super::super::world::TestWorld;

#[given("I am an owner of a package")]
async fn owner_of_package(world: &mut TestWorld) {
    let uid = Package::create_test_package( 
        &"test package".to_string(),
        &"https://github.com/Elements-Studio/starswap-core".to_string(),
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
    world.first_account.owned_package_name = Some("test-package".to_string());
}

#[given("There are other users on Movey")]
async fn other_users(world: &mut TestWorld) {
    let account = AccountInformation {
        email: "collaborator@host.com".to_string(),
        password: "So$trongpas0word!".to_string(),
        owned_package_name: None 
    }; 
    let form = NewAccountForm {
        email: EmailField {
            value: account.email.clone(),
            errors: vec![],
        },
        password: PasswordField {
            value: account.password.clone(),
            errors: vec![],
            hints: vec![],
        },
    };
    world.second_account = account;
    let uid = Account::register(&form, &DB_POOL).await.unwrap();
    Account::mark_verified(uid, &DB_POOL).await.unwrap();
}

#[when("I access the package detail page of my package")]
async fn access_package_details_page(world: &mut TestWorld) {
    world.
        go_to_url("package/test%20package")
        .await
}

#[when("I access the package Settings tab")]
async fn access_package_settings_page(world: &mut TestWorld) {
    world.
    go_to_url("package/test%20package")
    .await
}

#[when("I click on add button")]
async fn click_on_add_collaborator(world: &mut TestWorld) {
    assert!(world.driver.find_element(By::Css(".username_input")).await.is_err());

    let add_btn = world
        .driver
        .find_element(By::Css(".add_collaborator .btn"))
        .await
        .unwrap();

    assert!(world.driver.find_element(By::Css(".username_input")).await.is_ok());

    add_btn.click().await.unwrap();
}

#[when("I click on add button")]
async fn click_on_add_btn(world: &mut TestWorld) {
    let add_btn = world
        .driver
        .find_element(By::Css(".add_collaborator .btn"))
        .await
        .unwrap();

    assert!(world.driver.find_element(By::Css(".username_input")).await.is_ok());

    add_btn.click().await.unwrap();
}

#[when("I invite a user to become a collaborator of the package")]
async fn invite_collaborator(world: &mut TestWorld) {
    let input_username = world
        .driver
        .find_element(By::Css(".input .username"))
        .await
        .unwrap();

    input_username.click().await.unwrap();
    input_username
        .send_keys(world.second_account.email.clone());

    let invite_btn = world
    .driver
    .find_element(By::Css(".invite_collaborator .btn"))
    .await
    .unwrap();

    invite_btn.click().await.unwrap();
}

