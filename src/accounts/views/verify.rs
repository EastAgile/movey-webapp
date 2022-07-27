use crate::accounts::views::utils::validate_token;
use crate::accounts::Account;
use diesel::result::Error as DBError;
use jelly::accounts::User;
use jelly::actix_session::UserSession;
use jelly::actix_web::web::Query;
use jelly::actix_web::{web, web::Path, HttpRequest};
use jelly::request::DatabasePool;
use jelly::Result;
use jelly::{prelude::*, DieselPgPool};
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{AuthorizationCode, TokenResponse};

#[derive(serde::Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
}

/// Just renders a standard "Check your email and verify" page.
pub async fn verify(request: HttpRequest) -> Result<HttpResponse> {
    request.render(200, "accounts/verify/index.html", Context::new())
}

/// Given a link (of form {uidb64}-{ts}-{token}), verifies the
/// token and user, signs them in, and redirects to the dashboard.
///
/// In general, we do not want to leak information, so any errors here
/// should simply report as "invalid or expired".
pub async fn with_token(
    request: HttpRequest,
    Path((uidb64, ts, token)): Path<(String, String, String)>,
) -> Result<HttpResponse> {
    if let Ok(account) = validate_token(&request, &uidb64, &ts, &token).await {
        let db = request.db_pool()?;
        Account::mark_verified(account.id, db).await?;

        request.set_user(User {
            id: account.id,
            name: account.name,
            is_admin: account.is_admin,
            is_anonymous: false,
        })?;

        return request.redirect("/settings/profile");
    }

    request.render(200, "accounts/invalid_token.html", Context::new())
}

#[derive(Debug, serde::Deserialize)]
pub struct GithubOauthResponse {
    pub id: i64,
    pub login: String,
    pub email: Option<String>,
}

#[derive(Debug)]
pub struct GithubOauthUser {
    pub id: i64,
    pub login: String,
    pub email: String,
}

pub async fn callback_github(
    request: HttpRequest,
    params: Query<AuthRequest>,
    client: web::Data<BasicClient>,
) -> Result<HttpResponse> {
    return match request.get_session().get::<String>("oauth_state") {
        Ok(Some(state)) if state.eq(&params.state) => {
            request.get_session().remove("oauth_state");
            let code = AuthorizationCode::new(params.code.clone());
            match client.exchange_code(code).request(http_client) {
                Ok(token) => {
                    let client = reqwest::blocking::Client::new();
                    let response = client
                        .get("https://api.github.com/user")
                        .bearer_auth(token.access_token().secret())
                        .header("User-Agent", "Movey")
                        .send()?;

                    let oauth_response: GithubOauthResponse = response.json().unwrap();
                    let db = request.db_pool()?;

                    let updated_account =
                        link_github_to_movey_account(request.user()?, &oauth_response, db).await?;
                    let user = if updated_account.is_none() {
                        create_default_account_for_github_user(oauth_response, db).await?
                    } else {
                        User {
                            id: updated_account.as_ref().unwrap().id,
                            is_admin: updated_account.as_ref().unwrap().is_admin,
                            name: updated_account.unwrap().name,
                            is_anonymous: false,
                        }
                    };

                    request.set_user(user)?;
                    request.redirect("/settings/profile")
                }
                Err(e) => {
                    error!("Error getting information from Github: {:?}", e);
                    request.redirect("/accounts/register/")
                }
            }
        }
        Ok(_) => {
            error!("Unable to get oauth state.");
            request.redirect("/accounts/register/")
        }
        Err(e) => {
            error!("Invalid Github callback: {:?}", e);
            request.redirect("/accounts/register/")
        }
    };
}

async fn link_github_to_movey_account(
    current_user: User,
    oauth_response: &GithubOauthResponse,
    pool: &DieselPgPool,
) -> Result<Option<Account>> {
    // If id == 0, user is not signed in, therefore we don't need to link
    // we just need to get the account that the user is expecting
    if current_user.is_anonymous || current_user.id == 0 {
        return Ok(Account::get_by_github_id(oauth_response.id, pool)
            .await
            .ok());
    }

    let movey_account = match Account::get(current_user.id, pool).await {
        Ok(account) => {
            if account.github_id.is_some() || account.github_login.is_some() {
                error!(
                    "This user has already linked to Github account. 
                    uid: {}, current Github id: {}, new Github id: {}",
                    account.id,
                    account.github_id.unwrap(),
                    oauth_response.id
                );
                return Err(Error::Generic(
                    "This user has already linked to Github account.".to_string(),
                ));
            }
            account
        }
        Err(e) => {
            error!("A valid user id is expected, but cannot be found: {:?}", e);
            return Err(e);
        }
    };

    let current_github_account = Account::get_by_github_id(oauth_response.id, pool).await;
    match current_github_account {
        Ok(current_github_account) => {
            if current_github_account.name != *"" {
                error!("This Github account has already been linked to a Movey account. current uid: {}, incoming uid: {}, Github id: {}", 
                    current_github_account.id, movey_account.id, current_github_account.github_id.unwrap());
                return Err(Error::Generic(
                    "This Github account has already been linked to a Movey account.".to_string(),
                ));
            }

            info!("Existing Github account: {:?}", current_github_account);

            Account::merge_github_account_and_movey_account(
                current_github_account.id,
                movey_account.id,
                oauth_response.id,
                oauth_response.login.clone(),
                pool,
            )
            .await?;
        }
        Err(Error::Database(DBError::NotFound)) => {
            Account::update_movey_account_with_github_info(
                movey_account.id,
                oauth_response.id,
                oauth_response.login.clone(),
                pool,
            )
            .await?;
        }
        Err(e) => {
            error!(
                "Error linking Movey account with Github. uid: {}, error: {:?}",
                movey_account.id, e
            );
            return Err(e);
        }
    }
    Account::update_last_login(movey_account.id, pool).await?;

    let updated_account = Account::get(movey_account.id, pool).await?;
    Ok(Some(updated_account))
}

async fn create_default_account_for_github_user(
    mut oauth_response: GithubOauthResponse,
    pool: &DieselPgPool,
) -> Result<User> {
    if oauth_response.email.is_none() {
        // user does not expose email, create mock email
        let email_domain =
            std::env::var("NO_REPLY_EMAIL_DOMAIN").expect("NO_REPLY_EMAIL_DOMAIN is not set!");
        let mock_email = format!(
            "{}+{}@{}",
            &(oauth_response.id).to_string(),
            oauth_response.login.clone(),
            email_domain
        );
        oauth_response.email = Some(mock_email);
    }

    let oauth_user = GithubOauthUser {
        id: oauth_response.id,
        login: oauth_response.login,
        email: oauth_response.email.unwrap(),
    };

    let user = Account::register_from_github(&oauth_user, pool).await?;
    Account::update_last_login(user.id, pool).await?;

    Ok(user)
}

#[cfg(test)]
mod tests {
    use jelly::forms::{EmailField, PasswordField};

    use super::*;
    use crate::{
        accounts::forms::NewAccountForm,
        test::{DatabaseTestContext, DB_POOL},
    };

    fn get_oauth_response() -> GithubOauthResponse {
        GithubOauthResponse {
            id: 143_543,
            login: "a_gh_username".to_string(),
            email: Some("a_email@github.com".to_string()),
        }
    }

    async fn get_new_movey_account() -> Account {
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
        Account::get(uid, &DB_POOL).await.unwrap()
    }

    fn get_user_from_account(account: &Account) -> User {
        User {
            id: account.id,
            name: account.name.clone(),
            is_admin: account.is_admin,
            is_anonymous: false,
        }
    }

    #[actix_rt::test]
    async fn link_github_to_movey_account_return_none_if_not_signed_in_with_any_account() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();
        let oauth_stub = get_oauth_response();

        let result = link_github_to_movey_account(User::default(), &oauth_stub, &DB_POOL).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[actix_rt::test]
    async fn link_github_to_movey_account_return_user_if_sign_in_via_github() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();

        let oauth_stub = get_oauth_response();
        let account = get_new_movey_account().await;
        assert!(account.github_id.is_none());
        assert!(account.github_login.is_none());

        let user = get_user_from_account(&account);
        let result = link_github_to_movey_account(user, &oauth_stub, &DB_POOL).await;
        assert!(result.is_ok());
        assert!(result.as_ref().unwrap().is_some());
        assert_eq!(result.unwrap().unwrap().id, account.id);

        let user = User {
            id: 0,
            is_anonymous: true,
            name: "a_gh_username".to_string(),
            is_admin: false,
        };
        let result = link_github_to_movey_account(user, &oauth_stub, &DB_POOL).await;
        assert!(result.is_ok());
        assert!(result.as_ref().unwrap().is_some());
        assert_eq!(result.unwrap().unwrap().id, account.id);
    }

    #[actix_rt::test]
    async fn link_github_to_movey_account_return_user_if_movey_user_link_to_github_for_the_first_time(
    ) {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();

        let oauth_stub = get_oauth_response();
        let account = get_new_movey_account().await;
        assert!(account.github_id.is_none());
        assert!(account.github_login.is_none());

        let user = get_user_from_account(&account);
        let result = link_github_to_movey_account(user, &oauth_stub, &DB_POOL).await;
        assert!(result.is_ok());
        assert!(result.as_ref().unwrap().is_some());
        assert_eq!(result.unwrap().unwrap().id, account.id);

        let account = Account::get(account.id, &DB_POOL).await.unwrap();
        assert_eq!(account.github_id, Some(143_543));
        assert_eq!(account.github_login, Some("a_gh_username".to_string()));
    }

    #[actix_rt::test]
    async fn link_github_to_movey_account_return_user_if_movey_user_merge_with_existing_github_account(
    ) {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();

        let existing_gh_account = Account::register_from_github(
            &GithubOauthUser {
                id: 143_543,
                login: "a_gh_username".to_string(),
                email: "a_email@github.com".to_string(),
            },
            &DB_POOL,
        )
        .await
        .unwrap();

        assert_eq!(existing_gh_account.name, "");

        let account = get_new_movey_account().await;
        assert!(account.github_id.is_none());
        assert!(account.github_login.is_none());

        let user = get_user_from_account(&account);
        let oauth_stub = get_oauth_response();
        let result = link_github_to_movey_account(user, &oauth_stub, &DB_POOL).await;
        assert!(result.is_ok());
        assert!(result.as_ref().unwrap().is_some());
        assert_eq!(result.unwrap().unwrap().id, account.id);

        let account = Account::get(account.id, &DB_POOL).await.unwrap();
        assert_eq!(account.name, "email".to_string());
        assert_eq!(account.email, "email@host.com".to_string());
        assert_eq!(account.github_id, Some(143_543));
        assert_eq!(account.github_login, Some("a_gh_username".to_string()));
    }

    #[actix_rt::test]
    async fn link_github_to_movey_account_return_error_if_movey_user_already_linked_to_github_account(
    ) {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();

        let oauth_stub = get_oauth_response();

        let account = get_new_movey_account().await;
        assert!(account.github_id.is_none());
        assert!(account.github_login.is_none());

        let user = get_user_from_account(&account);
        let result = link_github_to_movey_account(user, &oauth_stub, &DB_POOL).await;
        assert!(result.is_ok());
        assert!(result.as_ref().unwrap().is_some());
        assert_eq!(result.unwrap().unwrap().id, account.id);

        let account = Account::get(account.id, &DB_POOL).await.unwrap();
        let user = get_user_from_account(&account);
        let result = link_github_to_movey_account(user, &oauth_stub, &DB_POOL).await;
        assert!(result.is_err());
        if let Err(Error::Generic(message)) = result {
            assert_eq!(
                message,
                "This user has already linked to Github account.".to_string()
            );
        }
    }

    #[actix_rt::test]
    async fn link_github_to_movey_account_return_error_if_github_account_already_linked_to_another_movey_account(
    ) {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();

        let oauth_stub = get_oauth_response();

        let account = get_new_movey_account().await;
        assert!(account.github_id.is_none());
        assert!(account.github_login.is_none());

        let user = get_user_from_account(&account);
        let result = link_github_to_movey_account(user, &oauth_stub, &DB_POOL).await;
        assert!(result.is_ok());
        assert!(result.as_ref().unwrap().is_some());
        assert_eq!(result.unwrap().unwrap().id, account.id);

        let form = NewAccountForm {
            email: EmailField {
                value: "email@thief.com".to_string(),
                errors: vec![],
            },
            password: PasswordField {
                value: "So$trongpas0word!".to_string(),
                errors: vec![],
                hints: vec![],
            },
        };
        let uid = Account::register(&form, &DB_POOL).await.unwrap();
        let account = Account::get(uid, &DB_POOL).await.unwrap();
        let user = get_user_from_account(&account);
        let result = link_github_to_movey_account(user, &oauth_stub, &DB_POOL).await;
        assert!(result.is_err());
        if let Err(Error::Generic(message)) = result {
            assert_eq!(
                message,
                "This Github account has already been linked to a Movey account.".to_string()
            );
        }
    }

    #[actix_rt::test]
    async fn create_default_account_for_github_user_works() {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();
        let oauth_stub = get_oauth_response();

        let result = create_default_account_for_github_user(oauth_stub, &DB_POOL).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "".to_string());

        let account = Account::get_by_github_id(143_543, &DB_POOL).await.unwrap();
        assert_eq!(account.name, "".to_string());
        assert_eq!(account.email, "a_email@github.com".to_string());
        assert_eq!(account.github_login, Some("a_gh_username".to_string()));
    }

    #[actix_rt::test]
    async fn create_default_account_for_github_user_will_create_default_email_if_github_email_is_private(
    ) {
        crate::test::init();
        let _ctx = DatabaseTestContext::new();
        let oauth_stub = GithubOauthResponse {
            id: 143_543,
            login: "a_gh_username".to_string(),
            email: None,
        };

        let result = create_default_account_for_github_user(oauth_stub, &DB_POOL).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "".to_string());

        let account = Account::get_by_github_id(143_543, &DB_POOL).await.unwrap();
        let domain = std::env::var("NO_REPLY_EMAIL_DOMAIN").unwrap();
        assert_eq!(account.name, "".to_string());
        assert_eq!(account.email, "143543+a_gh_username@".to_string() + &domain);
        assert_eq!(account.github_login, Some("a_gh_username".to_string()));
    }
}
