use cucumber::{then, when};
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
        &"test-package".to_string(),
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
    world.first_account.owned_package_id = Some(uid)
}

#[given("There are other users on Movey")]
async fn other_users(world: &mut TestWorld) {
    let account = AccountInformation {
        email: "email@host.com".to_string(),
        password: "So$trongpas0word!".to_string(),
        owned_package_id: None
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
    world
        .driver
        .get("http://localhost:17002/accounts/login/")
        .await
        .unwrap()
}

#[then("I should see the header search overlay")]
async fn see_header_search_overlay(world: &mut TestWorld) {
    let search_overlay_element = world
        .driver
        .find_element(By::Id("search-bar"))
        .await
        .unwrap();
    let displayed = search_overlay_element.is_displayed().await.unwrap();
    assert!(displayed);
}
