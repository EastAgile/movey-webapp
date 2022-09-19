use super::*;
use crate::test::util::{create_stub_packages, setup_user};
use crate::{
    packages::models::Package,
    settings::models::token::ApiToken,
    test::{DatabaseTestContext, DB_POOL},
};
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

fn setup_github_account(id_: Option<i64>, login: Option<&str>, email_: Option<&str>) -> Account {
    Account::register_from_github(
        &GithubOauthUser {
            id: id_.unwrap_or(132),
            login: login.unwrap_or("github_name").to_string(),
            email: email_.unwrap_or("email@domain.com").to_string(),
        },
        &DB_POOL,
    )
    .unwrap();
    Account::get_by_github_id(id_.unwrap_or(132), &DB_POOL).unwrap()
}

#[actix_rt::test]
#[should_panic(expected = "Database(NotFound)")]
async fn fetch_name_from_email_returns_error_with_non_existent_email() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    Account::fetch_name_from_email("non-existent@mail.com", &DB_POOL).unwrap();
}

#[actix_rt::test]
async fn fetch_name_from_email_returns_correct_name() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let uid = setup_user(None, None);
    let account_email = Account::get(uid, &DB_POOL).unwrap();
    let account_name = Account::fetch_name_from_email(&account_email.email, &DB_POOL).unwrap();
    assert_eq!(account_name, account_email.name);
}

#[actix_rt::test]
async fn fetch_email_returns_correct_email() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let uid = setup_user(None, None);
    let (name_, email_) = Account::fetch_email(uid, &DB_POOL).unwrap();
    let account_email = Account::get(uid, &DB_POOL).unwrap();

    assert_eq!(name_, account_email.name);
    assert_eq!(email_, account_email.email);
}

#[actix_rt::test]
#[should_panic(expected = "Database(NotFound)")]
async fn fetch_email_returns_error_with_non_existent_uid() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    Account::fetch_email(10, &DB_POOL).unwrap();
}

#[actix_rt::test]
async fn authenticate_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let uid = setup_user(None, None);
    Account::mark_verified(uid, &DB_POOL).unwrap();

    let user = Account::authenticate(&login_form(), &DB_POOL).unwrap();
    assert_eq!(user.id, uid);
}

#[actix_rt::test]
async fn authenticate_with_wrong_email_return_err() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let uid = setup_user(None, None);
    Account::mark_verified(uid, &DB_POOL).unwrap();

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
    match Account::authenticate(&invalid_login_form, &DB_POOL) {
        Err(Error::Database(DBError::NotFound)) => (),
        _ => panic!(),
    }
}

#[actix_rt::test]
async fn authenticate_with_wrong_password_return_err() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let uid = setup_user(None, None);
    Account::mark_verified(uid, &DB_POOL).unwrap();

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
    match Account::authenticate(&invalid_login_form, &DB_POOL) {
        Err(Error::InvalidPassword) => (),
        _ => panic!(),
    }
}

#[actix_rt::test]
async fn authenticate_with_unverified_account_return_err() {
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
    Account::register(&form, &DB_POOL).unwrap();

    match Account::authenticate(&login_form(), &DB_POOL) {
        Err(Generic(e)) => {
            assert_eq!(e, String::from("Your account has not been activated."))
        }
        _ => panic!(),
    }
}

#[actix_rt::test]
async fn register_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let uid = setup_user(None, None);
    let account = Account::get(uid, &DB_POOL).unwrap();
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
    let result = Account::register(&form, &DB_POOL);
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
    let _ = Account::register(&form, &DB_POOL).unwrap();
    let result = Account::register(&form, &DB_POOL);
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
    let uid = setup_user(None, None);
    Account::mark_verified(uid, &DB_POOL).unwrap();

    let new_password = String::from("nEw$trongpas0word!");
    Account::change_password(
        uid,
        String::from("wrong-password!"),
        new_password.clone(),
        &DB_POOL,
    )
    .unwrap();
}

#[actix_rt::test]
async fn change_password_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();
    let uid = setup_user(None, None);
    Account::mark_verified(uid, &DB_POOL).unwrap();

    let new_password = String::from("nEw$trongpas0word!");
    Account::change_password(
        uid,
        String::from("So$trongpas0word!"),
        new_password.clone(),
        &DB_POOL,
    )
    .unwrap();
    let mut login_form = login_form();
    login_form.password.value = new_password.clone();
    match Account::authenticate(&login_form, &DB_POOL) {
        Ok(user) => assert_eq!(user.id, uid),
        _ => panic!(),
    }
}

#[actix_rt::test]
async fn register_with_github_new_account_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    setup_github_account(Some(100_103), Some("git"), Some("a@b.com"));
    let account = Account::get_by_email("a@b.com", &DB_POOL).unwrap();
    assert_eq!(account.name, "");
    assert_eq!(account.email, "a@b.com");
    assert_eq!(account.github_login.unwrap(), "git");
    assert_eq!(account.github_id, Some(100_103));
    assert!(account.has_verified_email);
}

#[actix_rt::test]
async fn register_with_github_existing_account_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    setup_user(Some("email@host.com".to_string()), None);
    setup_github_account(None, None, Some("email@host.com"));
    let account = Account::get_by_email("email@host.com", &DB_POOL).unwrap();
    assert_eq!(account.name, "email");
    assert_eq!(account.email, "email@host.com");
    assert_eq!(account.github_login.unwrap(), "github_name");
    assert_eq!(account.github_id, Some(132));
    assert!(account.has_verified_email);
}

#[actix_rt::test]
async fn new_account_from_form_works() {
    let new_account = NewAccount::from_form(&NewAccountForm {
        email: EmailField {
            value: String::from("a_user_name@a_domain.com"),
            errors: vec![],
        },
        password: PasswordField {
            value: String::from("a_password"),
            errors: vec![],
            hints: vec![],
        },
    });

    assert_eq!(new_account.name, String::from("a_user_name"));
    assert_eq!(new_account.password, String::from(""));
}

#[actix_rt::test]
async fn register_should_populate_name_for_new_account() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let uid = setup_user(Some("email@host.com".to_string()), None);
    let new_account = Account::get(uid, &DB_POOL).unwrap();
    assert_eq!(new_account.name, String::from("email"));
    assert_eq!(new_account.github_login, None);
    assert_eq!(new_account.github_id, None);
}

#[actix_rt::test]
async fn get_by_github_id_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let non_existent_account = Account::get_by_github_id(132, &DB_POOL);
    if let Err(Error::Database(DBError::NotFound)) = non_existent_account {
    } else {
        panic!()
    }

    let account = setup_github_account(None, None, None);
    assert_eq!(account.github_id, Some(132));
    assert_eq!(account.github_login, Some("github_name".to_string()));
    assert_eq!(account.email, "email@domain.com".to_string());
}

#[actix_rt::test]
async fn merge_github_account_and_movey_account_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    // Account that has already been in the database
    let github_account = setup_github_account(None, None, Some("email@github.com"));

    let uid = setup_user(Some("email@host.com".to_string()), None);
    let movey_account = Account::get(uid, &DB_POOL).unwrap();
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
        &DB_POOL,
    )
    .unwrap();
    let movey_account = Account::get(uid, &DB_POOL).unwrap();

    assert_eq!(movey_account.name, "email".to_string());
    assert_eq!(movey_account.email, "email@host.com".to_string());
    assert_eq!(movey_account.github_id, github_account.github_id);
    assert_eq!(movey_account.github_login, github_account.github_login);

    let old_github_account = Account::get(github_account.id, &DB_POOL);
    if let Err(Error::Database(DBError::NotFound)) = old_github_account {
    } else {
        panic!()
    }
}

#[actix_rt::test]
async fn merge_github_account_and_movey_account_should_migrate_api_tokens() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let github_account = setup_github_account(None, None, None);
    ApiToken::insert(&github_account, "old_gh_account_token_1", &DB_POOL).unwrap();
    ApiToken::insert(&github_account, "old_gh_account_token_2", &DB_POOL).unwrap();

    let uid = setup_user(None, None);
    let movey_account = Account::get(uid, &DB_POOL).unwrap();
    assert_eq!(movey_account.github_id, None);
    assert_eq!(movey_account.github_login, None);

    Account::merge_github_account_and_movey_account(
        github_account.id,
        uid,
        github_account.github_id.unwrap(),
        github_account.github_login.as_ref().unwrap().to_string(),
        &DB_POOL,
    )
    .unwrap();
    let movey_account = Account::get(uid, &DB_POOL).unwrap();
    assert_eq!(movey_account.github_id, github_account.github_id);
    assert_eq!(movey_account.github_login, github_account.github_login);

    let movey_account_api_tokens = ApiToken::get_by_account(movey_account.id, &DB_POOL).unwrap();
    assert_eq!(movey_account_api_tokens.len(), 2);

    let movey_account_token_names = movey_account_api_tokens
        .iter()
        .map(|token| token.name.clone())
        .collect::<HashSet<String>>();
    assert_eq!(
        movey_account_token_names,
        HashSet::from_iter([
            "old_gh_account_token_1__github".to_string(),
            "old_gh_account_token_2__github".to_string(),
        ])
    );

    let old_github_account = Account::get(github_account.id, &DB_POOL);
    if let Err(Error::Database(DBError::NotFound)) = old_github_account {
    } else {
        panic!()
    }
}

#[actix_rt::test]
async fn merge_github_account_and_movey_account_should_aggregate_api_tokens() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let github_account = setup_github_account(None, None, None);
    ApiToken::insert(&github_account, "token_1", &DB_POOL).unwrap();
    ApiToken::insert(&github_account, "token_2", &DB_POOL).unwrap();

    let uid = setup_user(None, None);
    let movey_account = Account::get(uid, &DB_POOL).unwrap();
    ApiToken::insert(&movey_account, "token_1", &DB_POOL).unwrap();
    ApiToken::insert(&movey_account, "token_2", &DB_POOL).unwrap();
    ApiToken::insert(&movey_account, "token_3", &DB_POOL).unwrap();

    let movey_account_api_tokens = ApiToken::get_by_account(movey_account.id, &DB_POOL).unwrap();
    assert_eq!(movey_account_api_tokens.len(), 3);

    Account::merge_github_account_and_movey_account(
        github_account.id,
        uid,
        github_account.github_id.unwrap(),
        github_account.github_login.as_ref().unwrap().to_string(),
        &DB_POOL,
    )
    .unwrap();
    let movey_account = Account::get(uid, &DB_POOL).unwrap();
    assert_eq!(movey_account.github_id, github_account.github_id);
    assert_eq!(movey_account.github_login, github_account.github_login);

    let movey_account_api_tokens = ApiToken::get_by_account(movey_account.id, &DB_POOL).unwrap();
    assert_eq!(movey_account_api_tokens.len(), 5);

    let movey_account_token_names = movey_account_api_tokens
        .iter()
        .map(|token| token.name.clone())
        .collect::<HashSet<String>>();
    assert_eq!(
        movey_account_token_names,
        HashSet::from_iter([
            "token_1__movey".to_string(),
            "token_2__movey".to_string(),
            "token_3__movey".to_string(),
            "token_1__github".to_string(),
            "token_2__github".to_string()
        ])
    );

    let old_github_account = Account::get(github_account.id, &DB_POOL);
    if let Err(Error::Database(DBError::NotFound)) = old_github_account {
    } else {
        panic!()
    }
}

#[actix_rt::test]
async fn merge_github_account_and_movey_account_should_migrate_packages_ownership() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let github_account = setup_github_account(None, None, None);
    create_stub_packages(github_account.id, 3);
    let github_account_packages = Package::get_by_account(github_account.id, &DB_POOL).unwrap();
    assert_eq!(github_account_packages.len(), 3);

    let uid = setup_user(None, None);
    let movey_account = Account::get(uid, &DB_POOL).unwrap();
    create_stub_packages(movey_account.id, 7);
    let movey_account_packages = Package::get_by_account(movey_account.id, &DB_POOL).unwrap();
    assert_eq!(movey_account_packages.len(), 7);

    Account::merge_github_account_and_movey_account(
        github_account.id,
        uid,
        github_account.github_id.unwrap(),
        github_account.github_login.as_ref().unwrap().to_string(),
        &DB_POOL,
    )
    .unwrap();
    let movey_account_packages = Package::get_by_account(movey_account.id, &DB_POOL).unwrap();
    assert_eq!(movey_account_packages.len(), 10);

    let old_github_account = Account::get(github_account.id, &DB_POOL);
    if let Err(Error::Database(DBError::NotFound)) = old_github_account {
    } else {
        panic!()
    }
}

#[actix_rt::test]
async fn update_movey_account_with_github_info_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let uid = setup_user(None, None);
    let movey_account = Account::get(uid, &DB_POOL).unwrap();
    assert_eq!(movey_account.name, "email".to_string());
    assert_eq!(movey_account.email, "email@host.com".to_string());
    assert_eq!(movey_account.github_id, None);
    assert_eq!(movey_account.github_login, None);

    Account::update_movey_account_with_github_info(
        uid,
        142_432_554,
        "a_string".to_string(),
        &DB_POOL,
    )
    .unwrap();
    let movey_account = Account::get(uid, &DB_POOL).unwrap();
    assert_eq!(movey_account.name, "email".to_string());
    assert_eq!(movey_account.email, "email@host.com".to_string());
    assert_eq!(movey_account.github_id, Some(142_432_554));
    assert_eq!(movey_account.github_login, Some("a_string".to_string()));
}

#[actix_rt::test]
async fn register_will_check_and_update_slug() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let uid = setup_user(Some("email-to.slug_@host.com".to_string()), None);
    let account = Account::get(uid, &DB_POOL).unwrap();
    assert_eq!(account.slug.unwrap(), "email-to-slug");

    let uid = setup_user(Some("email.to.slug.....@host.com".to_string()), None);
    let account = Account::get(uid, &DB_POOL).unwrap();
    assert!(account.slug.as_ref().unwrap().starts_with("email-to-slug-"));
    assert_eq!(account.slug.unwrap().len(), "email-to-slug-0000".len());
}

#[actix_rt::test]
async fn register_from_github_will_check_and_update_slug() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let github_user = setup_github_account(None, None, None);
    assert_eq!(github_user.github_login.unwrap(), "github_name");
    assert_eq!(github_user.slug.unwrap(), "github-name");

    let another_github_user = setup_github_account(
        Some(145_346),
        Some("-github__name-"),
        Some("another_email@domain.com"),
    );
    assert_ne!(another_github_user.id, github_user.id);
    assert_eq!(another_github_user.github_login.unwrap(), "-github__name-");
    assert!(another_github_user
        .slug
        .as_ref()
        .unwrap()
        .starts_with("github-name-"));
    assert_eq!(
        another_github_user.slug.unwrap().len(),
        "github-name-0000".len()
    );
}

#[actix_rt::test]
async fn register_will_check_and_update_slug_to_avoid_collision() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let github_user = setup_github_account(None, None, None);
    assert_eq!(github_user.slug.as_ref().unwrap(), "github-name");

    let another_github_user = setup_github_account(
        Some(145_346),
        Some("-github__name-"),
        Some("another_email@domain.com"),
    );
    assert!(another_github_user
        .slug
        .as_ref()
        .unwrap()
        .starts_with("github-name"));
    assert_eq!(
        another_github_user.slug.unwrap().len(),
        "github-name-0000".len()
    );

    let uid = setup_user(Some("github+name@host.com".to_string()), None);
    let account = Account::get(uid, &DB_POOL).unwrap();
    assert!(account.slug.as_ref().unwrap().contains("github-name"));
    assert_eq!(account.slug.unwrap().len(), "github-name-0000".len());

    let uid = setup_user(Some("github___name@host.com".to_string()), None);
    let account = Account::get(uid, &DB_POOL).unwrap();
    assert!(account.slug.as_ref().unwrap().starts_with("github-name"));
    assert_eq!(account.slug.unwrap().len(), "github-name-0000".len());
}

#[actix_rt::test]
async fn get_by_slug_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    setup_user(Some("email-to.slug_@host.com".to_string()), None);
    let account = Account::get_by_slug("email-to-slug", &DB_POOL).unwrap();
    assert_eq!(account.email, "email-to.slug_@host.com");
}

#[actix_rt::test]
async fn make_slug_works() {
    crate::test::init();
    let _ctx = DatabaseTestContext::new();

    let uid = setup_user(
        Some(
            "__account._++to.be.trailled-very-important+++***{}!!!!!1111!!!!!@host.com".to_string(),
        ),
        None,
    );
    let account = Account::get(uid, &DB_POOL).unwrap();
    let slug_ = account.make_slug();
    assert_eq!(slug_, "account-to-be-trailled-very-important-1111");

    let account = setup_github_account(Some(10), Some("a_github_username"), Some("aaa@host.com"));
    let slug_ = account.make_slug();
    assert_eq!(slug_, "a-github-username");
}
