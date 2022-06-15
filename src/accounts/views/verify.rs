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
pub struct GithubOauthResponse {
    pub id: i64,
    pub login: String,
    pub email: Option<String>
}

#[derive(Debug)]
pub struct GithubOauthUser {
    pub id: i64,
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
                        .send()?;

                    let mut oauth_response: GithubOauthResponse = response.json().unwrap();
                    if oauth_response.email.is_none() {
                        // user does not expose email, create mock email
                        let email_domain = std::env::var("NO_REPLY_EMAIL_DOMAIN")
                            .expect("NO_REPLY_EMAIL_DOMAIN is not set!");
                        let mock_email = format!("{}+{}@{}",
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

                    let oauth_user: GithubOauthUser = response.json()?;
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
