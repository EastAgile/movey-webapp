use crate::accounts::jobs::SendCollaboratorInvitationEmail;
use crate::accounts::Account;
use crate::api::services::collaborators::views::{AddCollaboratorJson, InvitationResponse};
use crate::constants::*;
use crate::package_collaborators::models::owner_invitation::OwnerInvitation;
use crate::package_collaborators::package_collaborator::{PackageCollaborator, Role};
use crate::packages::Package;
use crate::utils::request_utils;
use jelly::actix_web::web;
use jelly::actix_web::web::Path;
use jelly::prelude::*;
use jelly::Result;
use serde_json::json;

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
            "msg": MSG_PACKAGE_NOT_FOUND,
        })));
    }
    let package = package.unwrap();
    let collaborator = PackageCollaborator::get(package.id, user.id, &conn).await;
    if let Err(e) = collaborator {
        warn!("add_collaborators failed, error: {}", e);
        return Ok(HttpResponse::Unauthorized().json(json!({
            "ok": false,
            "msg": MSG_UNAUTHORIZED_TO_ADD_COLLABORATOR
        })));
    }
    let collaborator = collaborator.unwrap();
    if collaborator.role != Role::Owner as i32 {
        return Ok(HttpResponse::Forbidden().json(json!({
            "ok": false,
            "msg": MSG_UNAUTHORIZED_TO_ADD_COLLABORATOR
        })));
    }

    let invited_account = match Account::get_by_email_or_gh_login(&json.user, db).await {
        Ok(account) => account,
        Err(e) => {
            warn!("add_collaborators failed, error: {}", e);
            return Ok(HttpResponse::BadRequest().json(json!({
                "ok": false,
                "msg": MSG_ACCOUNT_NOT_FOUND,
            })));
        }
    };

    match OwnerInvitation::create(invited_account.id, user.id, package.id, &conn).await {
        Ok(_) => {
            if !invited_account.is_generated_email() {
                request.queue(SendCollaboratorInvitationEmail {
                    to: invited_account.email,
                })?;
            }
        }
        Err(e) => {
            warn!("add_collaborators failed, error: {}", e);
            return Ok(HttpResponse::BadRequest().json(json!({
                "ok": false,
                "msg": MSG_FAILURE_INVITING_COLLABORATOR,
            })));
        }
    }

    Ok(HttpResponse::Ok().json(&json!({
        "ok": true,
        "msg": MSG_SUCCESSFULLY_INVITED_COLLABORATOR,
    })))
}

pub async fn handle_invite(
    request: HttpRequest,
    json: web::Json<InvitationResponse>,
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
            "msg": MSG_INVITATION_EXPIRED
        })));
    }
    if json.accepted {
        if let Err(e) = invitation.accept(&conn) {
            warn!("handle_invite failed, error: {}", e);
            return Ok(HttpResponse::Unauthorized().json(json!({
                "ok": false,
                "msg": MSG_UNEXPECTED_ERROR
            })));
        }
    } else {
        if let Err(e) = invitation.delete(&conn).await {
            warn!("handle_invite failed, error: {}", e);
            return Ok(HttpResponse::Unauthorized().json(json!({
                "ok": false,
                "msg": MSG_UNEXPECTED_ERROR
            })));
        }
    }

    Ok(HttpResponse::Ok().json(json!({
        "ok": true,
        "msg": MSG_SUCCESSFULLY_ADDED_COLLABORATOR
    })))
}
