use super::*;
use crate::{test::{DatabaseTestContext, DB_POOL}, settings::models::token::ApiToken,
            packages::models::Package};
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::DatabaseError;
use jelly::forms::{EmailField, PasswordField};
use std::{collections::HashSet, iter::FromIterator};

fn login_form() -> LoginForm {
    LoginForm {
        email: EmailField {
            value: "email@host.com".to_string(),
            errors: vec![],
        },
        password: PasswordField {
            value: "So$trongpas0word!".to_string(),
            errors: vec![],
            hints: vec![],
        },
        remember_me: "off".to_string(),
        redirect: "".to_string(),
    }
}

async fn setup_user() -> i32 {
    let form = NewAccountForm {
        email: EmailField {
            value: "email@host.com".to_string(),
            errors: vec![],
        },
        password: PasswordField {
            value: "So$trongpas0word!".to_string(),
            errors: vec![],
            hints: vec![],
        },
    };
    Account::register(&form, &DB_POOL).await.unwrap()
}

async fn setup_github_account() -> Account {
    Account::register_from_github(& GithubOauthUser {
        id: 132,
        login: "github_name".to_string(),
        email: "email@domain.com".to_string(),
    }, &DB_POOL).await.unwrap();
    Account::get_by_github_id(132, &DB_POOL).await.unwrap()
}

async fn create_stub_packages(account_id: i32, num_of_packages: i32) {
    for idx in 0..num_of_packages {
        Package::create_test_package(
            &format!("package_{}_{}", idx, account_id),
            &"repo_url".to_string(),
            &"package_description".to_string(),
            &"0.0.0".to_string(),
            &"".to_string(),
            &"".to_string(),
            10,
            200,
            Some(account_id),
            &DB_POOL)
            .await.unwrap();
    }
}

#[actix_rt::test]
#[should_panic(expected = "Database(NotFound)")]
async fn fetch_name_from_email_returns_error_with_non_existent_email() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    Account::fetch_name_from_email(&"non-existent@mail.com", &DB_POOL).await.unwrap();
}

#[actix_rt::test]
async fn fetch_name_from_email_returns_correct_name() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let uid = setup_user().await;
    let account_email = Account::get(uid, &DB_POOL).await.unwrap();
    let account_name =
        Account::fetch_name_from_email(&account_email.email, &DB_POOL).await.unwrap();
    assert_eq!(account_name, account_email.name);
}

#[actix_rt::test]
async fn fetch_email_returns_correct_email() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let uid = setup_user().await;
    let (name_, email_) = Account::fetch_email(uid, &DB_POOL).await.unwrap();
    let account_email = Account::get(uid, &DB_POOL).await.unwrap();

    assert_eq!(name_, account_email.name);
    assert_eq!(email_, account_email.email);
}

#[actix_rt::test]
#[should_panic(expected = "Database(NotFound)")]
async fn fetch_email_returns_error_with_non_existent_uid() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    Account::fetch_email(10, &DB_POOL).await.unwrap();
}

#[actix_rt::test]
async fn authenticate_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let uid = setup_user().await;
    Account::mark_verified(uid, &DB_POOL).await.unwrap();

    let user = Account::authenticate(&login_form(), &DB_POOL)
        .await
        .unwrap();
    assert_eq!(user.id, uid);
}

#[actix_rt::test]
async fn authenticate_with_wrong_email_return_err() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let uid = setup_user().await;
    Account::mark_verified(uid, &DB_POOL).await.unwrap();

    let invalid_login_form = LoginForm {
        email: EmailField {
            value: "wrong@host.com".to_string(),
            errors: vec![],
        },
        password: PasswordField {
            value: "So$trongpas0word!".to_string(),
            errors: vec![],
            hints: vec![],
        },
        remember_me: "off".to_string(),
        redirect: "".to_string(),
    };
    match Account::authenticate(&invalid_login_form, &DB_POOL).await {
        Err(Error::Database(DBError::NotFound)) => (),
        _ => panic!(),
    }
}

#[actix_rt::test]
async fn authenticate_with_wrong_password_return_err() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let uid = setup_user().await;
    Account::mark_verified(uid, &DB_POOL).await.unwrap();

    let invalid_login_form = LoginForm {
        email: EmailField {
            value: "email@host.com".to_string(),
            errors: vec![],
        },
        password: PasswordField {
            value: "wrongpassword".to_string(),
            errors: vec![],
            hints: vec![],
        },
        remember_me: "off".to_string(),
        redirect: "".to_string(),
    };
    match Account::authenticate(&invalid_login_form, &DB_POOL).await {
        Err(Error::InvalidPassword) => (),
        _ => panic!(),
    }
}

#[actix_rt::test]
async fn authenticate_with_unverified_account_return_err() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    setup_user().await;

    match Account::authenticate(&login_form(), &DB_POOL).await {
        Err(Error::Generic(e)) => {
            assert_eq!(e, String::from("Your account has not been activated."))
        }
        _ => panic!(),
    }
}

#[actix_rt::test]
async fn register_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let uid = setup_user().await;
    let account = Account::get(uid, &DB_POOL).await.unwrap();
    assert_eq!(account.email, "email@host.com");
}

#[actix_rt::test]
async fn register_with_empty_email_throws_exception() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let form = NewAccountForm {
        email: EmailField {
            value: "".to_string(),
            errors: vec![],
        },
        password: PasswordField {
            value: "xxyyzz12".to_string(),
            errors: vec![],
            hints: vec![],
        },
    };
    let result = Account::register(&form, &DB_POOL).await;
    assert!(result.is_err());
    match result {
        Err(Error::Database(DatabaseError(DatabaseErrorKind::__Unknown, _))) => (),

        _ => panic!(),
    }
}

#[actix_rt::test]
async fn register_with_duplicate_email_throws_exception() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let form = NewAccountForm {
        email: EmailField {
            value: "email@host.com".to_string(),
            errors: vec![],
        },
        password: PasswordField {
            value: "xxyyzz123".to_string(),
            errors: vec![],
            hints: vec![],
        },
    };
    let _ = Account::register(&form, &DB_POOL).await.unwrap();
    let result = Account::register(&form, &DB_POOL).await;
    assert!(result.is_err());
    match result {
        Err(Error::Database(DatabaseError(DatabaseErrorKind::UniqueViolation, _))) => (),
        _ => panic!(),
    }
}

#[actix_rt::test]
#[should_panic(expected = "InvalidPassword")]
async fn change_password_returns_error_if_wrong_current_password() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let uid = setup_user().await;
    Account::mark_verified(uid, &DB_POOL).await.unwrap();

    let new_password = String::from("nEw$trongpas0word!");
    Account::change_password(
        uid,
        String::from("wrong-password!"),
        new_password.clone(),
        &DB_POOL,
    )
        .await
        .unwrap();
}

#[actix_rt::test]
async fn change_password_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let uid = setup_user().await;
    Account::mark_verified(uid, &DB_POOL).await.unwrap();

    let new_password = String::from("nEw$trongpas0word!");
    Account::change_password(
        uid,
        String::from("So$trongpas0word!"),
        new_password.clone(),
        &DB_POOL,
    )
    .await
    .unwrap();
    let mut login_form = login_form();
    login_form.password.value = new_password.clone();
    match Account::authenticate(&login_form, &DB_POOL).await {
        Ok(user) => assert_eq!(user.id, uid),
        _ => panic!(),
    }
}

#[actix_rt::test]
async fn register_with_github_new_account_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let oauth_user = GithubOauthUser {
        email: "a@b.com".to_string(),
        login: "git".to_string(),
        id: 100_103,
    };
    Account::register_from_github(&oauth_user, &DB_POOL).await.unwrap();

    let account = Account::get_by_email(&oauth_user.email, &DB_POOL).await.unwrap();
    assert_eq!(account.name, "");
    assert_eq!(account.email, "a@b.com");
    assert_eq!(account.github_login.unwrap(), "git");
    assert_eq!(account.github_id, Some(100_103));
    assert_eq!(account.has_verified_email, true);
}

#[actix_rt::test]
async fn register_with_github_existing_account_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    setup_user().await;
    let oauth_user = GithubOauthUser {
        email: "email@host.com".to_string(),
        login: "git".to_string(),
        id: 100_103,
    };
    Account::register_from_github(&oauth_user, &DB_POOL).await.unwrap();

    let account = Account::get_by_email(&oauth_user.email, &DB_POOL).await.unwrap();
    assert_eq!(account.name, "email");
    assert_eq!(account.email, "email@host.com");
    assert_eq!(account.github_login.unwrap(), "git");
    assert_eq!(account.github_id, Some(100_103));
    assert_eq!(account.has_verified_email, true);
}

#[actix_rt::test]
async fn new_account_from_form_works() {
    let user_email = String::from("a_user_name@a_domain.com");
    let new_account = NewAccount::from_form(&NewAccountForm { 
        email: EmailField { value: user_email, errors: vec![] }, 
        password: PasswordField { value: String::from("a_password"), errors: vec![], hints: vec![] }
    });

    assert_eq!(new_account.name, String::from("a_user_name"));
    assert_eq!(new_account.password, String::from(""));
}

#[actix_rt::test]
async fn register_should_populate_name_for_new_account() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let form = NewAccountForm {
        email: EmailField {
            value: "email@host.com".to_string(),
            errors: vec![],
        },
        password: PasswordField {
            value: "So$trongpas0word!".to_string(),
            errors: vec![],
            hints: vec![],
        },
    };
    let uid = Account::register(&form, &DB_POOL).await.unwrap();
    let new_account = Account::get(uid, &DB_POOL).await.unwrap();

    assert_eq!(new_account.name, String::from("email"));
    assert_eq!(new_account.github_login, None);
    assert_eq!(new_account.github_id, None);
}

#[actix_rt::test]
async fn get_by_github_id_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let non_existent_account = 
        Account::get_by_github_id(132, &DB_POOL).await;
    if let Err(Error::Database(DBError::NotFound)) = non_existent_account {
    } else {
        panic!()
    }

    let account = setup_github_account().await;

    assert_eq!(account.github_id, Some(132));
    assert_eq!(account.github_login, Some("github_name".to_string()));
    assert_eq!(account.email, "email@domain.com".to_string());
}

#[actix_rt::test]
async fn merge_github_account_and_movey_account_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    // Information of the github account that signed in via OAuth
    let _ = Account::register_from_github(& GithubOauthUser {
        id: 132,
        login: "github_name".to_string(),
        email: "email@github.com".to_string(),
    }, &DB_POOL).await.unwrap();

    // Account that has already been in the database
    let github_account = 
        Account::get_by_github_id(132, &DB_POOL).await.unwrap();

    let uid = Account::register(&NewAccountForm {
        email: EmailField {
            value: "email@host.com".to_string(),
            errors: vec![],
        },
        password: PasswordField {
            value: "So$trongpas0word!".to_string(),
            errors: vec![],
            hints: vec![],
        },
    }, &DB_POOL).await.unwrap();

    let movey_account = Account::get(uid, &DB_POOL).await.unwrap();
    assert_eq!(movey_account.name, "email".to_string());
    assert_eq!(movey_account.email, "email@host.com".to_string());
    assert_eq!(movey_account.github_id, None);
    assert_eq!(movey_account.github_login, None);
    assert_ne!(uid as i64, github_account.github_id.unwrap());

    Account::merge_github_account_and_movey_account(
        github_account.id,
        uid, 
        github_account.github_id.unwrap(), 
        github_account.github_login.as_ref().unwrap().to_string(), 
        &DB_POOL
    ).await.unwrap();
    let movey_account = Account::get(uid, &DB_POOL).await.unwrap();

    assert_eq!(movey_account.name, "email".to_string());
    assert_eq!(movey_account.email, "email@host.com".to_string());
    assert_eq!(movey_account.github_id, github_account.github_id);
    assert_eq!(movey_account.github_login, github_account.github_login);

    let old_github_account = Account::get(github_account.id, &DB_POOL).await;
    if let Err(Error::Database(DBError::NotFound)) = old_github_account {
    } else {
        panic!()
    }
}

#[actix_rt::test]
async fn merge_github_account_and_movey_account_should_migrate_api_tokens() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let github_account = setup_github_account().await;
    ApiToken::insert(&github_account, "old_gh_account_token_1", &DB_POOL).await.unwrap();
    ApiToken::insert(&github_account, "old_gh_account_token_2", &DB_POOL).await.unwrap();

    let uid = setup_user().await;
    let movey_account = Account::get(uid, &DB_POOL).await.unwrap();
    assert_eq!(movey_account.github_id, None);
    assert_eq!(movey_account.github_login, None);

    Account::merge_github_account_and_movey_account(
        github_account.id,
        uid, 
        github_account.github_id.unwrap(), 
        github_account.github_login.as_ref().unwrap().to_string(), 
        &DB_POOL
    ).await.unwrap();
    let movey_account = Account::get(uid, &DB_POOL).await.unwrap();
    assert_eq!(movey_account.github_id, github_account.github_id);
    assert_eq!(movey_account.github_login, github_account.github_login);

    let movey_account_api_tokens = ApiToken::get_by_account(movey_account.id, &DB_POOL).await.unwrap();
    assert_eq!(movey_account_api_tokens.len(), 2);

    let movey_account_token_names = movey_account_api_tokens
        .iter().map(|token| token.name.clone()).collect::<HashSet<String>>();
    assert_eq!(
        movey_account_token_names, 
        HashSet::from_iter([
            "old_gh_account_token_1__github".to_string(), "old_gh_account_token_2__github".to_string(),
        ]));


    let old_github_account = Account::get(github_account.id, &DB_POOL).await;
    if let Err(Error::Database(DBError::NotFound)) = old_github_account {
    } else {
        panic!()
    }
}

#[actix_rt::test]
async fn merge_github_account_and_movey_account_should_aggregate_api_tokens() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let github_account = setup_github_account().await;
    ApiToken::insert(&github_account, "token_1", &DB_POOL).await.unwrap();
    ApiToken::insert(&github_account, "token_2", &DB_POOL).await.unwrap();

    let uid = setup_user().await;
    let movey_account = Account::get(uid, &DB_POOL).await.unwrap();
    ApiToken::insert(&movey_account, "token_1", &DB_POOL).await.unwrap();
    ApiToken::insert(&movey_account, "token_2", &DB_POOL).await.unwrap();
    ApiToken::insert(&movey_account, "token_3", &DB_POOL).await.unwrap();
    
    let movey_account_api_tokens = ApiToken::get_by_account(movey_account.id, &DB_POOL).await.unwrap();
    assert_eq!(movey_account_api_tokens.len(), 3);

    Account::merge_github_account_and_movey_account(
        github_account.id,
        uid, 
        github_account.github_id.unwrap(), 
        github_account.github_login.as_ref().unwrap().to_string(), 
        &DB_POOL
    ).await.unwrap();
    let movey_account = Account::get(uid, &DB_POOL).await.unwrap();
    assert_eq!(movey_account.github_id, github_account.github_id);
    assert_eq!(movey_account.github_login, github_account.github_login);

    let movey_account_api_tokens = ApiToken::get_by_account(movey_account.id, &DB_POOL).await.unwrap();
    assert_eq!(movey_account_api_tokens.len(), 5);

    let movey_account_token_names = movey_account_api_tokens
        .iter().map(|token| token.name.clone()).collect::<HashSet<String>>();
    assert_eq!(
        movey_account_token_names, 
        HashSet::from_iter([
            "token_1__movey".to_string(), "token_2__movey".to_string(), "token_3__movey".to_string(),
            "token_1__github".to_string(), "token_2__github".to_string()
        ]));

    let old_github_account = Account::get(github_account.id, &DB_POOL).await;
    if let Err(Error::Database(DBError::NotFound)) = old_github_account {
    } else {
        panic!()
    }
}

#[actix_rt::test]
async fn merge_github_account_and_movey_account_should_migrate_packages_ownership() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let github_account = setup_github_account().await;
    create_stub_packages(github_account.id, 3).await;
    let github_account_packages = Package::get_by_account(github_account.id, &DB_POOL).await.unwrap();
    assert_eq!(github_account_packages.len(), 3);

    let uid = setup_user().await;
    let movey_account = Account::get(uid, &DB_POOL).await.unwrap();
    create_stub_packages(movey_account.id, 7).await;
    let movey_account_packages = Package::get_by_account(movey_account.id, &DB_POOL).await.unwrap();
    assert_eq!(movey_account_packages.len(), 7);

    Account::merge_github_account_and_movey_account(
        github_account.id,
        uid,
        github_account.github_id.unwrap(), 
        github_account.github_login.as_ref().unwrap().to_string(), 
        &DB_POOL
    ).await.unwrap();
    let movey_account_packages = Package::get_by_account(movey_account.id, &DB_POOL).await.unwrap();
    assert_eq!(movey_account_packages.len(), 10);

    let old_github_account = Account::get(github_account.id, &DB_POOL).await;
    if let Err(Error::Database(DBError::NotFound)) = old_github_account {
    } else {
        panic!()
    }
}

#[actix_rt::test]
async fn update_movey_account_with_github_info_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let uid = Account::register(&NewAccountForm {
        email: EmailField {
            value: "email@host.com".to_string(),
            errors: vec![],
        },
        password: PasswordField {
            value: "So$trongpas0word!".to_string(),
            errors: vec![],
            hints: vec![],
        },
    }, &DB_POOL).await.unwrap();

    let movey_account = Account::get(uid, &DB_POOL).await.unwrap();
    assert_eq!(movey_account.name, "email".to_string());
    assert_eq!(movey_account.email, "email@host.com".to_string());
    assert_eq!(movey_account.github_id, None);
    assert_eq!(movey_account.github_login, None);

    Account::update_movey_account_with_github_info(
        uid, 
        142_432_554, 
        "a_string".to_string(), 
        &DB_POOL
    ).await.unwrap();
    let movey_account = Account::get(uid, &DB_POOL).await.unwrap();
    assert_eq!(movey_account.name, "email".to_string());
    assert_eq!(movey_account.email, "email@host.com".to_string());
    assert_eq!(movey_account.github_id, Some(142_432_554));
    assert_eq!(movey_account.github_login, Some("a_string".to_string()));
}
