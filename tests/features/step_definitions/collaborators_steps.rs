use std::{fs, thread};
use cucumber::{then, when, given};
use jelly::forms::{EmailField, PasswordField};
use thirtyfour::prelude::*;
use mainlib::accounts::Account;
use mainlib::accounts::forms::NewAccountForm;
use mainlib::packages::{Package, PackageVersion};
use mainlib::test::DB_POOL;
use crate::features::world::AccountInformation;
use super::logout_steps::click_log_out;
use super::signin_steps::visit_sign_in_page;
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
        go_to_url("packages/test%20package")
        .await
}

#[when("I access the package Settings tab")]
async fn access_package_settings_page(world: &mut TestWorld) {
    let collaborator_tab = world
        .driver
        .find_element(By::ClassName("tab-owner"))
        .await
        .unwrap();
    collaborator_tab.click().await.unwrap();
}

#[when("I click on add button")]
async fn click_on_add_collaborator(world: &mut TestWorld) {
    // assert!(world.driver.find_element(By::ClassName("collaborator_input")).await.is_err());
    let add_btn = world
        .driver
        .find_element(By::ClassName("add_collaborators_btn"))
        .await
        .unwrap();
    add_btn.click().await.unwrap();
    // assert!(world.driver.find_element(By::ClassName("collaborator_input")).await.is_ok());
}

#[then("I should see an overlay for inviting a collaborator")]
async fn see_an_invitation_overlay(world: &mut TestWorld) {
    // assert!(world.driver.find_element(By::ClassName("collaborator_input")).await.is_err());
    // assert!(world.driver.find_element(By::ClassName("collaborator_input")).await.is_ok());
}

#[when("I invite a user to become a collaborator of the package")]
async fn invite_collaborator(world: &mut TestWorld) {
    let input_username = world
        .driver
        .find_element(By::ClassName("collaborator_input"))
        .await
        .unwrap();

    input_username.click().await.unwrap();
    input_username
        .send_keys(world.second_account.email.clone())
        .await
        .unwrap();

    let invite_btn = world
        .driver
        .find_element(By::ClassName("collaborators_btn"))
        .await
        .unwrap();

    invite_btn.click().await.unwrap();
}

#[then("She (the collaborator) should receive an invitation email")]
async fn receive_invitation_email(_world: &mut TestWorld) {
    thread::sleep(std::time::Duration::from_millis(1000));
    let email_dir = fs::read_dir("./emails").unwrap().next();
    let email = fs::read_to_string(email_dir.unwrap().unwrap().path()).unwrap();
    let content = fs::read_to_string(email).unwrap();
    if content.contains("Subject: New Contact Request") {
        assert!(content.contains("To: movey@eastagile.com"));
        assert!(content.contains("Hello Admin,"));
        assert!(content.contains("A message was sent from John Doe"));
        assert!(content.contains("Contact Email: email@host.com"));
        assert!(content.contains("Contact for: Account"));
        assert!(content.contains("Description: Hello this is a test."));
    } else if content.contains("Subject: Thank you for contacting us") {
        assert!(content.contains("To: email@host.com"));
        assert!(content.contains("Hello there,"));
        assert!(content.contains(
            "We=E2=80=99ve received your request and will get back to you shortly."
        ));
    } else {
        panic!()
    }
}

#[when("She is signed in")]
async fn second_user_sign_in(world: &mut TestWorld) {
    click_log_out(world).await;
    visit_sign_in_page(world).await;

    let email_field = world.driver.find_element(By::Name("email")).await.unwrap();
    email_field.send_keys(&world.second_account.email).await.unwrap();

    let password_field = world
        .driver
        .find_element(By::Name("password"))
        .await
        .unwrap();
    password_field.send_keys(&world.second_account.password).await.unwrap();
    let login_button = world
        .driver
        .find_element(By::ClassName("login-btn"))
        .await
        .unwrap();
    login_button.click().await.unwrap();
}

#[when("She access her own invitation page")]
async fn visit_own_invitation_page(world: &mut TestWorld) {
    world
        .go_to_url("settings/invitations")
        .await
}

#[when("She should see an invitation in her invitation page")]
async fn see_her_invitation(world: &mut TestWorld) {
    let package_name = world
        .driver
        .find_element(By::ClassName("package-name"))
        .await
        .unwrap();
    assert_eq!(&package_name.text().await.unwrap(), world.first_account.owned_package_name.as_ref().unwrap());
}
