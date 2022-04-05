use jelly::anyhow::anyhow;
use jelly::accounts::User;
use jelly::actix_session::UserSession;
use jelly::actix_web::{web::Path, HttpRequest, web};
use jelly::actix_web::web::Query;
use jelly::prelude::*;
use jelly::request::DatabasePool;
use jelly::Result;
use oauth2::{AuthorizationCode, Client, CsrfToken, ErrorResponse, RevocableToken, TokenIntrospectionResponse, TokenResponse, TokenType};
use oauth2::basic::BasicClient;
use oauth2::reqwest::{async_http_client, http_client};
use crate::accounts::views::utils::validate_token;
use crate::accounts::Account;

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

        return request.redirect("/dashboard/");
    }

    return request.render(200, "accounts/invalid_token.html", Context::new());
}

pub async fn callback(request: HttpRequest, params: Query<AuthRequest>, client: web::Data<BasicClient>) -> Result<HttpResponse> {
    return match request.get_session().get::<String>("state") {
        Ok(Some(state)) if state.eq(&params.state) => {
            let code = AuthorizationCode::new(params.code.clone());
            let state = CsrfToken::new(params.state.clone());
            match client.exchange_code(code).request(http_client) {
                Ok(token) => {
                    info!("Github returned the following token:\n{:?}\n", token.access_token().secret());
                    let html = format!(
                        r#"<html>
                <head><title>OAuth2 Test</title></head>
                <body>
                    Google returned the following state:
                    <pre>{}</pre>
                    Google returned the following token:
                    <pre>{:?}</pre>
                </body>
                </html>"#,
                        state.secret(),
                        token
                    );
                    Ok(HttpResponse::Ok().body(html))
                },
                Err(e) => {
                    Err(Error::Anyhow(anyhow!("{:?}", e)))
                }
            }
        },
        _ => {
            Err(Error::Anyhow(anyhow!("asdasdsadsdsd")))
        }
    }
}
