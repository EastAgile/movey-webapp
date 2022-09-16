use super::super::world::TestWorld;
use super::logout_steps::click_log_out;
use super::signin_steps::visit_sign_in_page;
use crate::features::world::AccountInformation;
use cucumber::{given, then, when};
use jelly::forms::{EmailField, PasswordField};
use mainlib::accounts::forms::NewAccountForm;
use mainlib::accounts::Account;
use mainlib::package_collaborators::package_collaborator::PackageCollaborator;
use mainlib::packages::{Package, PackageVersion};
use mainlib::test::DB_POOL;
use regex::Regex;
use std::fs;
use std::time::Duration;
use thirtyfour::prelude::*;
use tokio::time::sleep;

#[given("I am an owner of a package")]
async fn owner_of_package(world: &mut TestWorld) {
    let conn = DB_POOL.get().unwrap();

    let pid = Package::create_test_package(
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
    .unwrap();

    PackageVersion::create(
        pid,
        "second_version".to_string(),
        "second_readme_content".to_string(),
        "rev_2".to_string(),
        2,
        100,
        None,
        &conn,
    )
    .unwrap();
    PackageCollaborator::new_owner(pid, 1, 1, &DB_POOL.get().unwrap()).unwrap();
    world.first_account.owned_package_name = Some("test package".to_string());
}

#[given("There are other collaborators who work on that package")]
async fn other_owners(world: &mut TestWorld) {
    other_users(world).await;
    let package = Package::get_by_name(
        world.first_account.owned_package_name.as_ref().unwrap(),
        &DB_POOL,
    )
    .unwrap();
    PackageCollaborator::new_collaborator(package.id, 2, 2, &DB_POOL.get().unwrap()).unwrap();
}

#[given("There are other users on Movey")]
async fn other_users(world: &mut TestWorld) {
    let account = AccountInformation {
        email: "collaborator@host.com".to_string(),
        password: "So$trongpas0word!".to_string(),
        owned_package_name: None,
        id: -1,
        slug: "collaborator".to_string(),
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
    let uid = Account::register(&form, &DB_POOL).unwrap();
    Account::mark_verified(uid, &DB_POOL).unwrap();
}

#[when("I access the package detail page of my package")]
async fn access_package_details_page(world: &mut TestWorld) {
    world.go_to_url("packages/test-package").await
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
    assert!(world
        .driver
        .find_element(By::ClassName("new_collaborator_modal"))
        .await
        .is_err());
    let add_btn = world
        .driver
        .find_element(By::ClassName("add_collaborators_btn"))
        .await
        .unwrap();
    add_btn.click().await.unwrap();
}

#[then("I should see an overlay for inviting a collaborator")]
async fn see_an_invitation_overlay(world: &mut TestWorld) {
    assert!(world
        .driver
        .find_element(By::Id("new_collaborator_modal"))
        .await
        .unwrap()
        .is_displayed()
        .await
        .unwrap());
}

#[when("I invite a user to become a collaborator of the package")]
async fn invite_collaborator(world: &mut TestWorld) {
    std::env::set_var("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS", "10");
    fs::remove_dir_all("./emails").unwrap_or(());
    let input_username = world
        .driver
        .find_element(By::ClassName("collaborators_input"))
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

    fs::remove_dir_all("./emails").unwrap_or(());

    invite_btn.click().await.unwrap();
}

#[when("She invite another user to become a collaborator of the package")]
async fn invite_other_collaborator(world: &mut TestWorld) {
    std::env::set_var("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS", "10");
    fs::remove_dir_all("./emails").unwrap_or(());

    let form = NewAccountForm {
        email: EmailField {
            value: "another_user_to_be_invited@host.com".to_string(),
            errors: vec![],
        },
        password: PasswordField {
            value: "So$trongpas0word!".to_string(),
            errors: vec![],
            hints: vec![],
        },
    };
    let uid = Account::register(&form, &DB_POOL).unwrap();
    let _ = Account::mark_verified(uid, &DB_POOL);

    let input_username = world
        .driver
        .find_element(By::ClassName("collaborators_input"))
        .await
        .unwrap();

    input_username.click().await.unwrap();
    input_username
        .send_keys("another_user_to_be_invited@host.com")
        .await
        .unwrap();

    let invite_btn = world
        .driver
        .find_element(By::ClassName("collaborators_btn"))
        .await
        .unwrap();

    fs::remove_dir_all("./emails").unwrap_or(());

    invite_btn.click().await.unwrap();
}

#[when("I invite collaborator with a username that is not in our system")]
async fn invite_user_not_in_system(world: &mut TestWorld) {
    std::env::set_var("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS", "10");
    fs::remove_dir_all("./emails").unwrap_or(());
    let input_username = world
        .driver
        .find_element(By::ClassName("collaborators_input"))
        .await
        .unwrap();

    input_username.click().await.unwrap();
    input_username
        .send_keys("a_username_not_in_the_system")
        .await
        .unwrap();

    let invite_btn = world
        .driver
        .find_element(By::ClassName("collaborators_btn"))
        .await
        .unwrap();

    fs::remove_dir_all("./emails").unwrap_or(());

    invite_btn.click().await.unwrap();
}

#[when("I close the invite modal")]
async fn close_invite_modal(world: &mut TestWorld) {
    let close_modal_btn = world
        .driver
        .find_element(By::ClassName("close-button"))
        .await
        .unwrap();

    close_modal_btn.click().await.unwrap();
}

#[when("I close the modal")]
async fn close_modal(world: &mut TestWorld) {
    let close_modal_btn = world
        .driver
        .find_element(By::ClassName("close-button-icon"))
        .await
        .unwrap();

    close_modal_btn.click().await.unwrap();
}

#[then("She (the collaborator) should receive an invitation email")]
async fn receive_invitation_email(_world: &mut TestWorld) {
    sleep(Duration::from_secs(2)).await;
    let email_dir = fs::read_dir("./emails").unwrap().next();
    let content = fs::read_to_string(email_dir.unwrap().unwrap().path()).unwrap();

    assert!(content.contains("Subject: You have been invited to collaborate on test package"));
    assert!(content.contains("From: movey@eastagile.com"));
    assert!(content.contains("To: collaborator@host.com"));
    assert!(content.contains("New Collaborator Invitation"));
    assert!(content.contains("You got invited as a collaborator on the package test package."));
}

#[then("She (the collaborator) should receive an ownership invitation email")]
async fn receive_ownership_invitation_email(_world: &mut TestWorld) {
    sleep(Duration::from_secs(2)).await;
    let email_dir = fs::read_dir("./emails").unwrap().next();
    let content = fs::read_to_string(email_dir.unwrap().unwrap().path()).unwrap();

    assert!(content.contains("From: movey@eastagile.com"));
    assert!(content.contains("To: collaborator@host.com"));
    assert!(content.contains("New Collaborator Invitation"));
    assert!(content.contains("You got invited as a collaborator on the package test package."));
}

#[when("She is signed in")]
async fn second_user_sign_in(world: &mut TestWorld) {
    click_log_out(world).await;
    visit_sign_in_page(world).await;

    let email_field = world.driver.find_element(By::Name("email")).await.unwrap();
    email_field
        .send_keys(&world.second_account.email)
        .await
        .unwrap();

    let password_field = world
        .driver
        .find_element(By::Name("password"))
        .await
        .unwrap();
    password_field
        .send_keys(&world.second_account.password)
        .await
        .unwrap();
    let login_button = world
        .driver
        .find_element(By::Css(".login-btn"))
        .await
        .unwrap();
    login_button.click().await.unwrap();
}

#[when("She is signed out")]
async fn second_user_sign_out(world: &mut TestWorld) {
    click_log_out(world).await;
    visit_sign_in_page(world).await;
}

#[when("She accesses her invitation page")]
async fn visit_own_invitation_page(world: &mut TestWorld) {
    world.go_to_url("settings/invitations").await;
    sleep(Duration::from_millis(1000)).await;
}

#[then("She should see an invitation in her invitation page")]
async fn see_her_invitation(world: &mut TestWorld) {
    let package_names = world
        .driver
        .find_elements(By::ClassName("package-name-view"))
        .await
        .unwrap();
    assert_eq!(package_names.len(), 1);
    assert_eq!(
        &package_names[0].text().await.unwrap(),
        world.first_account.owned_package_name.as_ref().unwrap()
    );
}

#[then("She should see an ownership invitation in her profile page")]
async fn see_ownership_invitation(world: &mut TestWorld) {
    let package_names = world
        .driver
        .find_elements(By::ClassName("package-name-view"))
        .await
        .unwrap();
    assert_eq!(package_names.len(), 1);
    assert_eq!(
        &package_names[0].text().await.unwrap(),
        world.first_account.owned_package_name.as_ref().unwrap()
    );

    let accept_btns = world
        .driver
        .find_elements(By::ClassName("accept"))
        .await
        .unwrap();
    assert_eq!(accept_btns.len(), 1);
    assert_eq!(&accept_btns[0].text().await.unwrap(), "ACCEPT");
}

#[then("She should see that the invitation is deleted")]
async fn deleted_invitation(world: &mut TestWorld) {
    sleep(Duration::from_millis(1000)).await;
    let package_names = world
        .driver
        .find_elements(By::ClassName("package-name-view"))
        .await
        .unwrap();
    assert!(package_names.is_empty());
}

#[when("She clicks on the link in the email to accept the invitation")]
async fn invitation_link_in_email(world: &mut TestWorld) {
    let email_dir = fs::read_dir("./emails").unwrap().next();
    let content = fs::read_to_string(email_dir.unwrap().unwrap().path()).unwrap();

    let re = Regex::new(r"/owner_invitations/accept/([^ \n]+)".to_string().as_str()).unwrap();
    let caps = re.captures(&content).unwrap();
    let accept_token = caps.get(1).map(|m| m.as_str()).unwrap();
    let url = &format!("owner_invitations/accept/{}", accept_token);
    world.go_to_url(url).await;
}

#[then("She should see that she is a collaborator of the package")]
async fn confirm_in_collaborator_list(world: &mut TestWorld) {
    let collaborator_names = world
        .driver
        .find_elements(By::ClassName("collaborator_name"))
        .await
        .unwrap();

    for name in collaborator_names {
        let collaborator_name = name.text().await.unwrap();
        if collaborator_name.contains(&world.second_account.email) {
            return;
        }
    }
    panic!()
}

#[when("She click on the collaborators tab")]
async fn click_collaborator_tab(world: &mut TestWorld) {
    let tab_owner = world
        .driver
        .find_element(By::ClassName("tab-owner"))
        .await
        .unwrap();
    tab_owner.click().await.unwrap();
}

#[when("Collaborator invitation is expired")]
async fn expired_collaborator_invitation(_world: &mut TestWorld) {
    std::env::set_var("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS", "0");
}

#[when("The transfer ownership invitation is expired")]
async fn expired_owner_invitation(_world: &mut TestWorld) {
    std::env::set_var("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS", "0");
}

#[then("She should see the Invalid or Expired page")]
async fn invalid_page_for_expired_invitation(world: &mut TestWorld) {
    let title = world
        .driver
        .find_element(By::ClassName("title"))
        .await
        .unwrap();
    assert_eq!(&title.text().await.unwrap(), "Invalid Token");
}

#[when("I invite collaborator with a valid email that is not in our system")]
async fn invite_email_not_in_system(world: &mut TestWorld) {
    std::env::set_var("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS", "10");
    let input_username = world
        .driver
        .find_element(By::ClassName("collaborators_input"))
        .await
        .unwrap();

    input_username.click().await.unwrap();
    input_username
        .send_keys("not_in_system@host.com")
        .await
        .unwrap();

    let invite_btn = world
        .driver
        .find_element(By::ClassName("collaborators_btn"))
        .await
        .unwrap();

    fs::remove_dir_all("./emails").unwrap_or(());
    invite_btn.click().await.unwrap();
}

#[then("She (the outsider) should receive an invitation email")]
async fn outsider_receives_invitation_email(_world: &mut TestWorld) {
    sleep(Duration::from_secs(3)).await;
    let email_dir = fs::read_dir("./emails").unwrap().next();
    let content = fs::read_to_string(email_dir.unwrap().unwrap().path()).unwrap();

    assert!(content.contains("Reply-To: movey@eastagile.com"));
    assert!(content.contains("To: not_in_system@host.com"));
    assert!(content.contains("Register To Collaborate"));
    assert!(content.contains("Subject: You have been invited to collaborate on test package"));
    assert!(content.contains("A user on Movey invited you to collaborate on the package test package, but it looks like you haven't sign up yet."));
    assert!(content
        .contains("To start collaborating, please create your account and start working on this."));
    assert!(content.contains("/accounts/register?redirect=/profile"));
}

#[when("She clicks on the link in the email to sign up")]
async fn invited_to_sign_up(world: &mut TestWorld) {
    click_log_out(world).await;
    world
        .go_to_url("/accounts/register?redirect=/profile")
        .await;
}

#[when("She fills in the form and submit")]
async fn fill_in_form(world: &mut TestWorld) {
    let email_field = world.driver.find_element(By::Name("email")).await.unwrap();
    email_field
        .send_keys("not_in_system@host.com")
        .await
        .unwrap();

    let password_field = world
        .driver
        .find_element(By::Name("password"))
        .await
        .unwrap();
    password_field.send_keys("So$trongpas0word!").await.unwrap();

    let i_agree = world
        .driver
        .find_element(By::Name("i_agree"))
        .await
        .unwrap();
    i_agree.click().await.unwrap();

    let create_account_button = world
        .driver
        .find_element(By::ClassName("create_account_btn"))
        .await
        .unwrap();

    fs::remove_dir_all("./emails").unwrap_or(());
    create_account_button.click().await.unwrap();
}

#[when("She verifies her email")]
async fn verify_email(world: &mut TestWorld) {
    sleep(Duration::from_secs(2)).await;

    let email_dir = fs::read_dir("./emails").unwrap().next();
    let content = fs::read_to_string(email_dir.unwrap().unwrap().path()).unwrap();
    assert!(content.contains("To: not_in_system@host.com"));
    assert!(content.contains("Subject: Verify your new Movey account"));
    assert!(content.contains("An account with this email was created just now"));
    let re = Regex::new(r"/accounts/verify/([^ \n]+)".to_string().as_str()).unwrap();
    let caps = re.captures(&content).unwrap();
    let token = caps.get(1).map(|m| m.as_str()).unwrap();
    let url = &format!("accounts/verify/{}", token);
    world.go_to_url(url).await;
}

#[then("She should be redirected to her profile page")]
async fn redirect_to_profile_page(world: &mut TestWorld) {
    assert_eq!(
        world.driver.current_url().await.unwrap(),
        "http://localhost:17002/settings/invitations"
    );
}

#[then("She should see an invitation in her profile page")]
async fn see_invitation_tab(world: &mut TestWorld) {
    let invitation_list = world
        .driver
        .find_elements(By::ClassName("collaborators_content"))
        .await
        .unwrap();
    assert_eq!(invitation_list.len(), 1);

    let test_package_name = world
        .driver
        .find_elements(By::Id("package-name-view"))
        .await
        .unwrap();

    assert_eq!(
        test_package_name[0].text().await.unwrap(),
        "test package".to_string()
    );

    let test_invitor_email = world
        .driver
        .find_elements(By::ClassName("invitations_owners email"))
        .await
        .unwrap();

    assert_eq!(
        test_invitor_email[0].text().await.unwrap(),
        world.first_account.email
    );
}

#[when("She clicks on the Accept button to accept the invitation")]
async fn accept_invitation(world: &mut TestWorld) {
    let accept_btns = world
        .driver
        .find_elements(By::ClassName("accept"))
        .await
        .unwrap();

    assert_eq!(accept_btns.len(), 1);
    accept_btns[0].click().await.unwrap();
}

#[when("She click on the Decline button to decline the invitation")]
async fn decline_invitation(world: &mut TestWorld) {
    let cancel_btns = world
        .driver
        .find_elements(By::ClassName("cancel"))
        .await
        .unwrap();

    assert_eq!(cancel_btns.len(), 1);
    cancel_btns[0].click().await.unwrap();
}

#[then("She should be redirected to the package detail page")]
async fn redirected_to_package_detail_page(world: &mut TestWorld) {
    assert_eq!(
        world.driver.current_url().await.unwrap(),
        "http://localhost:17002/settings/invitations"
    );
}

#[then("She should receive a message that the invitation is expired")]
async fn see_expired_invitation_message(world: &mut TestWorld) {
    let error_message = world
        .driver
        .find_element(By::ClassName("error_message"))
        .await
        .unwrap();
    assert_eq!(
        error_message.text().await.unwrap(),
        "The invitation is expired"
    );
}

#[when("I transfer ownership to a collaborator")]
async fn transfer_ownership(world: &mut TestWorld) {
    std::env::set_var("OWNERSHIP_INVITATIONS_EXPIRATION_DAYS", "10");
    fs::remove_dir_all("./emails").unwrap_or(());
    let transfer_btns = world
        .driver
        .find_elements(By::ClassName("transfer"))
        .await
        .unwrap();

    assert_eq!(transfer_btns.len(), 1);
    transfer_btns[0].click().await.unwrap();

    let confirm_btn = world
        .driver
        .find_element(By::Id("confirm_transfer"))
        .await
        .unwrap();
    confirm_btn.click().await.unwrap();
}

#[when("She clicks on the Accept button to accept the transfer")]
async fn accept_ownership_invitation(world: &mut TestWorld) {
    let accept_btns = world
        .driver
        .find_elements(By::ClassName("accept"))
        .await
        .unwrap();

    assert_eq!(accept_btns.len(), 1);
    accept_btns[0].click().await.unwrap();
}

#[when("She clicks on the Decline button to decline the transfer")]
async fn decline_ownership_invitation(world: &mut TestWorld) {
    let decline_btns = world
        .driver
        .find_elements(By::ClassName("cancel"))
        .await
        .unwrap();

    assert_eq!(decline_btns.len(), 1);
    decline_btns[0].click().await.unwrap();
}

#[then("She should see that she is the owner of the package")]
async fn package_owner(world: &mut TestWorld) {
    let owner_name = world
        .driver
        .find_element(By::ClassName("owner_name"))
        .await
        .unwrap();
    assert_eq!(
        &owner_name.text().await.unwrap(),
        &format!("{}\n(You)",&world.second_account.email)
    );
}

#[then("She should see that she is not the owner of the package")]
async fn not_package_owner(world: &mut TestWorld) {
    let owner_name = world
        .driver
        .find_element(By::ClassName("owner_name"))
        .await
        .unwrap();

    assert_ne!(
        &owner_name.text().await.unwrap(),
        &world.second_account.email
    );
}

#[then("She should see that she is not a collaborator of the package")]
async fn not_collaborator(world: &mut TestWorld) {
    let collaborator_names = world
        .driver
        .find_elements(By::ClassName("collaborator_name"))
        .await
        .unwrap();

    for name in collaborator_names {
        let collaborator_name = name.text().await.unwrap();
        if collaborator_name.contains(&world.second_account.email) {
            panic!()
        }
    }
    let owner_name = world
        .driver
        .find_element(By::ClassName("owner_name"))
        .await
        .unwrap();

    assert_ne!(
        &owner_name.text().await.unwrap(),
        &world.second_account.email
    );
}

#[then("She should see that i am the owner of the package")]
async fn first_account_package_owner(world: &mut TestWorld) {
    let owner_name = world
        .driver
        .find_element(By::ClassName("owner_name"))
        .await
        .unwrap();

    assert_eq!(
        &owner_name.text().await.unwrap(),
        &world.second_account.email
    );
}

#[then("She should see that I am the owner of the package")]
async fn see_owner(world: &mut TestWorld) {
    let owner_name = world
        .driver
        .find_element(By::ClassName("owner_name"))
        .await
        .unwrap();

    assert_eq!(
        &owner_name.text().await.unwrap(),
        &world.first_account.email
    );
}

#[then("She should see that I am a collaborator of the package")]
async fn see_first_user_as_collaborator(world: &mut TestWorld) {
    world
        .go_to_url("packages/test-package/collaborators")
        .await;
    let collaborator_names = world
        .driver
        .find_elements(By::ClassName("collaborator_name"))
        .await
        .unwrap();

    for name in collaborator_names {
        let collaborator_name = name.text().await.unwrap();
        if collaborator_name.contains(&world.first_account.email) {
            return;
        }
    }
    panic!()
}

#[when("She accesses the package detail page")]
async fn visit_package_detail_page(world: &mut TestWorld) {
    world.go_to_url("packages/test-package").await
}

#[then(regex = r"^I should see a modal with text '(.+)'$")]
async fn see_success_message(world: &mut TestWorld, message: String) {
    sleep(Duration::from_millis(100)).await;
    let modal = world
        .driver
        .find_element(By::Id("success_modal_message"))
        .await
        .unwrap();
    let msg = modal.text().await.unwrap();
    assert_eq!(msg, message);
}

#[then(regex = r"^I should see text '(.+)'$")]
async fn see_return_message(world: &mut TestWorld, message: String) {
    sleep(Duration::from_millis(100)).await;
    let modal = world
        .driver
        .find_element(By::Id("return-message"))
        .await
        .unwrap();
    let msg = modal.text().await.unwrap();
    assert_eq!(msg, message);
}

#[then("I should see the invited collaborator email")]
async fn see_invited_collaborator_email(world: &mut TestWorld) {
    let names = world
        .driver
        .find_elements(By::ClassName("collaborator_name"))
        .await
        .unwrap();
    assert_eq!(names.len(), 1);
    assert_eq!(names[0].text().await.unwrap(), world.second_account.email);

    // let sending_statuses = world
    //     .driver
    //     .find_elements(By::ClassName("sending_status"))
    //     .await
    //     .unwrap();
    // assert_eq!(sending_statuses.len(), 1);
    // assert_eq!(sending_statuses[0].text().await.unwrap(), "collaborator invitation sent");

    let rows = world
        .driver
        .find_elements(By::ClassName("collaborator_row"))
        .await
        .unwrap();
    // include the header
    assert_eq!(rows.len(), 3);
}

#[then("I should see the invited external email")]
async fn see_invited_external_email(world: &mut TestWorld) {
    let names = world
        .driver
        .find_elements(By::ClassName("external_name"))
        .await
        .unwrap();
    assert_eq!(names.len(), 1);
    assert_eq!(
        names[0].text().await.unwrap(),
        "not_in_system@host.com".to_string()
    );

    // let sending_statuses = world
    //     .driver
    //     .find_elements(By::ClassName("sending_status"))
    //     .await
    //     .unwrap();
    // assert_eq!(sending_statuses.len(), 1);
    // assert_eq!(sending_statuses[0].text().await.unwrap(), "external invitation sent");

    let rows = world
        .driver
        .find_elements(By::ClassName("collaborator_row"))
        .await
        .unwrap();
    // include the header
    assert_eq!(rows.len(), 3);
}

#[then("I should not see the list of pending collaborators")]
async fn not_see_pending_list(world: &mut TestWorld) {
    let collaborator_names = world
        .driver
        .find_elements(By::ClassName("collaborator_name"))
        .await
        .unwrap();

    assert!(collaborator_names.is_empty());
}

#[then("I should not see the list of external invitations")]
async fn not_see_external_list(world: &mut TestWorld) {
    let external_names = world
        .driver
        .find_elements(By::ClassName("external_name"))
        .await
        .unwrap();

    assert!(external_names.is_empty());
}

#[when("I click the 'Remove' button")]
async fn click_remove_button(world: &mut TestWorld) {
    let remove_btns = world
        .driver
        .find_elements(By::ClassName("remove"))
        .await
        .unwrap();

    assert_eq!(remove_btns.len(), 1);
    remove_btns[0].click().await.unwrap();
}

#[when("I click the 'Confirm' button")]
async fn click_confirm_button(world: &mut TestWorld) {
    let confirm_btn = world
        .driver
        .find_element(By::Id("confirm_delete"))
        .await
        .unwrap();

    confirm_btn.click().await.unwrap();
}

#[then(regex = r"^I should see a remove owner modal with text '(.+)'$")]
async fn see_remove_modal(world: &mut TestWorld, message: String) {
    let modal = world
        .driver
        .find_element(By::Css("#remove_owner_modal .message"))
        .await
        .unwrap();
    let msg = modal.text().await.unwrap();
    assert_eq!(msg, message);
}

#[then("I should see the invitation is deleted")]
async fn deleted_pending_invitation(world: &mut TestWorld) {
    sleep(Duration::from_millis(500)).await;
    let rows = world
        .driver
        .find_elements(By::ClassName("collaborator_row"))
        .await
        .unwrap();
    // include the header
    assert_eq!(rows.len(), 2);
}

#[then("I should see the ownership transfer invitation is deleted")]
async fn deleted_transfer_invitation(world: &mut TestWorld) {
    sleep(Duration::from_millis(500)).await;
    let rows = world
        .driver
        .find_elements(By::ClassName("collaborator_row"))
        .await
        .unwrap();
    // change from PendingOwner to Collaborator so the row is not deleted
    assert_eq!(rows.len(), 3);

    let transfer_btns = world
        .driver
        .find_elements(By::Css(".ownership_btn.transfer"))
        .await
        .unwrap();
    assert_eq!(transfer_btns.len(), 1);
}

#[then("I should not see the add button")]
async fn not_see_add_button(world: &mut TestWorld) {
    assert!(world
        .driver
        .find_element(By::ClassName("new_collaborator_modal"))
        .await
        .is_err());
    assert!(world
        .driver
        .find_element(By::ClassName("add_collaborators_btn"))
        .await
        .is_err());
}

#[given("I am a collaborator of a package")]
async fn collaborator_of_package(world: &mut TestWorld) {
    let pid = Package::create_test_package(
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
    .unwrap();

    PackageVersion::create(
        pid,
        "second_version".to_string(),
        "second_readme_content".to_string(),
        "rev_2".to_string(),
        2,
        100,
        None,
        &DB_POOL.get().unwrap(),
    )
    .unwrap();
    PackageCollaborator::new_collaborator(pid, 1, 1, &DB_POOL.get().unwrap()).unwrap();
    world.first_account.owned_package_name = Some("test package".to_string());
}

#[when("I click the 'Remove' button of the other collaborator")]
async fn click_remove_collaborator_button(world: &mut TestWorld) {
    let remove_btns = world
        .driver
        .find_elements(By::ClassName("remove"))
        .await
        .unwrap();

    assert_eq!(remove_btns.len(), 1);
    remove_btns[0].click().await.unwrap();
}

#[then("I can see the collaborator is removed from table")]
async fn deleted_collaborator(world: &mut TestWorld) {
    sleep(Duration::from_millis(500)).await;
    let rows = world
        .driver
        .find_elements(By::ClassName("collaborator_row"))
        .await
        .unwrap();
    // include the header
    assert_eq!(rows.len(), 2);
}
