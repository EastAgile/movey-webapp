use jelly::actix_web::web;
use jelly::actix_web::web::Path;
use jelly::prelude::*;
use serde_json::json;
use crate::accounts::Account;
use crate::accounts::jobs::SendCollaboratorInvitationEmail;
use crate::package_collaborators::models::owner_invitation::OwnerInvitation;
use crate::package_collaborators::package_collaborator;
use crate::package_collaborators::package_collaborator::{PackageCollaborator, Role};
use crate::packages::Package;
use jelly::Result;
use crate::api::services::collaborators::views::{AddCollaboratorJson, InvitationResponse};
use crate::utils::owner_invitation::accept_invitation;
use crate::utils::request_utils;

pub async fn add_collaborators(
    request: HttpRequest,
    Path(package_name): Path<String>,
    json: web::Json<AddCollaboratorJson>,
) -> Result<HttpResponse> {
    if !request_utils::is_authenticated(&request).await? {
        return Ok(request_utils::clear_cookie(&request));
    }
    let user = request.user()?;
    let db = request.db_pool()?;
    let conn = db.get()?;

    let package = Package::get_by_name(&package_name, db).await;
    if let Err(e) = package {
        warn!("add_collaborators failed, error: {}", e);
        return Ok(HttpResponse::NotFound().json(json!({
                "ok": false,
                "msg": "package not found"
            })));
    }
    let package = package.unwrap();
    let collaborator = PackageCollaborator::get(
        package.id,
        user.id,
        &conn,
    ).await;
    if let Err(e) = collaborator {
        warn!("add_collaborators failed, error: {}", e);
        return Ok(HttpResponse::Unauthorized().json(json!({
                "ok": false,
                "msg": "user is not allowed to add collaborator to this package"
            })));
    }
    let collaborator = collaborator.unwrap();
    if collaborator.role != Role::Owner as i32 {
        return Ok(HttpResponse::Forbidden().json(json!({
                "ok": false,
                "msg": "forbidden"
            })))
    }

    let invited_account =
        match Account::get_by_email_or_gh_login(&json.user, db).await {
            Ok(account) => account,
            Err(e) => {
                warn!("add_collaborators failed, error: {}", e);
                return Ok(HttpResponse::BadRequest().json(json!({
                    "ok": false,
                    "msg": "account not found"
                    })))
            }
        };

    match OwnerInvitation::create(
        invited_account.id,
        user.id,
        package.id,
        &conn).await
    {
        Ok(token) => {
            if !invited_account.is_generated_email() {
                request.queue(SendCollaboratorInvitationEmail {
                    to: invited_account.email
                })?;
            }
        }
        Err(e) => {
            warn!("add_collaborators failed, error: {}", e);
            return Ok(HttpResponse::BadRequest().json(json!({
                    "ok": false,
                    "msg": ""
                })))
        }
    }

    Ok(HttpResponse::Ok().json(&json!({ "ok": true, "msg": "asd" })))
}

pub async fn handle_invite(
    request: HttpRequest,
    json: web::Json<InvitationResponse>
) -> Result<HttpResponse> {
    if !request_utils::is_authenticated(&request).await? {
        return Ok(request_utils::clear_cookie(&request));
    }
    let user = request.user()?;

    let conn = request.db_pool()?.get()?;
    let invitation = OwnerInvitation::find_by_id(user.id, json.package_id, &conn)?;
    if invitation.is_expired() {
        return Ok(HttpResponse::NotFound().json(json!({
                "ok": false,
                "msg": "invitation expired"
            })));
    }
    if json.accepted {
        if let Err(e) = invitation.accept(&conn) {
            warn!("handle_invite failed, error: {}", e);
            return Ok(HttpResponse::Unauthorized().json(json!({
                "ok": false,
                "msg": "unexpected error"
            })));
        }
    } else {
        if let Err(e) = invitation.delete(&conn).await {
            warn!("handle_invite failed, error: {}", e);
            return Ok(HttpResponse::Unauthorized().json(json!({
                "ok": false,
                "msg": "unexpected error"
            })));
        }
    }

    Ok(HttpResponse::Ok().json(json!({ "ok": true, "msg": "aa" })))
}
