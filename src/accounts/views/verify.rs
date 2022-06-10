use crate::accounts::views::utils::validate_token;
use crate::accounts::Account;
use jelly::accounts::User;
use jelly::actix_session::UserSession;
use jelly::actix_web::web::Query;
use jelly::actix_web::{web, web::Path, HttpRequest};
use jelly::prelude::*;
use jelly::request::DatabasePool;
use jelly::Result;
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

    return request.render(200, "accounts/invalid_token.html", Context::new());
}

#[derive(Debug, serde::Deserialize)]
pub struct GithubOauthUser {
    pub name: String,
    pub login: String,
    pub email: String
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
                        .send()
                        .unwrap();

                    let oauth_user: GithubOauthUser = response.json().unwrap();
                    let db = request.db_pool()?;
                    let user = Account::register_from_github(&oauth_user, &db).await?;
                    Account::update_last_login(user.id, &db).await?;
                    request.set_user(user)?;
                    request.redirect("/settings/profile")
                }
                Err(_) => request.redirect("/accounts/register/"),
            }
        }
        _ => request.redirect("/accounts/register/"),
    };
}

pub async fn callback_google(
    request: HttpRequest,
    user: web::Query<GithubOauthUser>,
) -> Result<HttpResponse> {
    request.set_user(User {
        id: 0,
        name: user.name.clone(),
        is_admin: false,
        is_anonymous: false,
    })?;
    request.redirect("/settings/profile")
}
