use crate::accounts::jobs::{SendCollaboratorInvitationEmail, SendRegisterToCollabEmail};
use crate::accounts::Account;
use crate::api::services::collaborators::views::{AddCollaboratorJson, InvitationResponse};
use crate::constants::*;
use crate::package_collaborators::models::owner_invitation::OwnerInvitation;
use crate::package_collaborators::models::pending_invitation::PendingInvitation;
use crate::package_collaborators::package_collaborator::{PackageCollaborator, Role};
use crate::packages::Package;
use crate::utils::request_utils;
use diesel::result::{DatabaseErrorKind, Error as DBError};
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

    let package = match Package::get_by_name(&package_name, db).await {
        Ok(package) => package,
        Err(Error::Database(DBError::NotFound)) => {
            return Ok(HttpResponse::NotFound().json(json!({
                "ok": false,
                "msg": MSG_PACKAGE_NOT_FOUND,
            })));
        }
        Err(e) => {
            warn!("add_collaborators failed, error: {:?}", e);
            return Ok(HttpResponse::InternalServerError().json(json!({
                "ok": false,
                "msg": MSG_FAILURE_INVITING_COLLABORATOR,
            })));
        }
    };

    let invited_account = match Account::get_by_email_or_gh_login(&json.user, db).await {
        Ok(account) => account,
        Err(Error::Database(DBError::NotFound)) => {
            if json.user.contains("@") {
                PendingInvitation::create(&json.user, user.id, package.id, &conn)?;
                request.queue(SendRegisterToCollabEmail {
                    to: json.user.clone(),
                    package_name: package.name,
                })?;
            }
            return Ok(HttpResponse::NotFound().json(json!({
                "ok": false,
                "msg": MSG_ACCOUNT_NOT_FOUND_INVITING,
            })));
        }
        Err(e) => {
            warn!("add_collaborators failed, error: {:?}", e);
            return Ok(HttpResponse::InternalServerError().json(json!({
                "ok": false,
                "msg": MSG_FAILURE_INVITING_COLLABORATOR,
            })));
        }
    };

    match PackageCollaborator::get(package.id, user.id, &conn) {
        Ok(collaborator) => {
            if collaborator.role != Role::Owner as i32 {
                return Ok(HttpResponse::Forbidden().json(json!({
                    "ok": false,
                    "msg": MSG_UNAUTHORIZED_TO_ADD_COLLABORATOR
                })));
            }
        }
        Err(e) => {
            warn!("add_collaborators failed, error: {:?}", e);
            return Ok(HttpResponse::Unauthorized().json(json!({
                "ok": false,
                "msg": MSG_UNAUTHORIZED_TO_ADD_COLLABORATOR
            })));
        }
    };

    if let Ok(_) = PackageCollaborator::get(package.id, invited_account.id, &conn) {
        return Ok(HttpResponse::BadRequest().json(json!({
            "ok": false,
            "msg": MSG_COLLABORATOR_ALREADY_EXISTED
        })));
    }

    match OwnerInvitation::create(invited_account.id, user.id, package.id, None, &conn) {
        Ok(invitation) => {
            if !invited_account.is_generated_email() {
                request.queue(SendCollaboratorInvitationEmail {
                    to: invited_account.email,
                    package_name: package.name,
                    token: invitation.token,
                })?;
            }
        }
        Err(Error::Database(DBError::NotFound)) => {
            return Ok(HttpResponse::BadRequest().json(json!({
                "ok": false,
                "msg": MSG_INVITATION_ALREADY_EXISTED,
            })));
        }
        Err(e) => {
            warn!("add_collaborators failed, error: {:?}", e);
            return Ok(HttpResponse::InternalServerError().json(json!({
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

pub async fn transfer_ownership(
    request: HttpRequest,
    Path(package_name): Path<String>,
    json: web::Json<AddCollaboratorJson>,
) -> Result<HttpResponse> {
    if !request_utils::is_authenticated(&request).await? {
        return Ok(request_utils::clear_cookie(&request));
    }
    let db = request.db_pool()?;
    let conn = db.get()?;

    let package = match Package::get_by_name(&package_name, db).await {
        Ok(package) => package,
        Err(Error::Database(DBError::NotFound)) => {
            return Ok(HttpResponse::NotFound().json(json!({
                "ok": false,
                "msg": MSG_PACKAGE_NOT_FOUND,
            })));
        }
        Err(e) => {
            warn!("transfer_ownership failed, error: {:?}", e);
            return Ok(HttpResponse::InternalServerError().json(json!({
                "ok": false,
                "msg": MSG_FAILURE_INVITING_COLLABORATOR,
            })));
        }
    };

    let invited_account = match Account::get_by_email_or_gh_login(&json.user, db).await {
        Ok(account) => account,
        Err(Error::Database(DBError::NotFound)) => {
            return Ok(HttpResponse::NotFound().json(json!({
                "ok": false,
                "msg": MSG_ACCOUNT_NOT_FOUND_INVITING,
            })));
        }
        Err(e) => {
            warn!("transfer_ownership failed, error: {:?}", e);
            return Ok(HttpResponse::InternalServerError().json(json!({
                "ok": false,
                "msg": MSG_FAILURE_INVITING_COLLABORATOR,
            })));
        }
    };

    let user = request.user()?;
    let ids = vec![user.id, invited_account.id];
    match PackageCollaborator::get_in_bulk(package.id, ids, &conn) {
        Ok(collaborators) => {
            if collaborators.len() != 2 {
                return Ok(HttpResponse::BadRequest().json(json!({
                "ok": false,
                "msg": MSG_UNAUTHORIZED_TO_ADD_COLLABORATOR
            })));
            }
            if collaborators.get(0).unwrap().role != Role::Owner as i32 {
                return Ok(HttpResponse::Forbidden().json(json!({
                    "ok": false,
                    "msg": MSG_UNAUTHORIZED_TO_ADD_COLLABORATOR
                })));
            }
            if collaborators.get(1).unwrap().role != Role::Collaborator as i32 {
                return Ok(HttpResponse::Forbidden().json(json!({
                    "ok": false,
                    "msg": MSG_UNAUTHORIZED_TO_ADD_COLLABORATOR
                })));
            }
        }
        Err(e) => {
            warn!("transfer_ownership failed, error: {:?}", e);
            return Ok(HttpResponse::Unauthorized().json(json!({
                "ok": false,
                "msg": MSG_UNAUTHORIZED_TO_ADD_COLLABORATOR
            })));
        }
    };

    match OwnerInvitation::create(invited_account.id, user.id, package.id, Some(true), &conn) {
        Ok(invitation) => {
            if !invited_account.is_generated_email() {
                // TODO: need a new email for transferring ownership
                request.queue(SendCollaboratorInvitationEmail {
                    to: invited_account.email,
                    package_name: package.name,
                    token: invitation.token,
                })?;
            }
        }
        Err(Error::Database(DBError::NotFound)) => {
            return Ok(HttpResponse::BadRequest().json(json!({
                "ok": false,
                "msg": MSG_INVITATION_ALREADY_EXISTED,
            })));
        }
        Err(e) => {
            warn!("add_collaborators failed, error: {:?}", e);
            return Ok(HttpResponse::InternalServerError().json(json!({
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
            warn!("handle_invite failed, error: {:?}", e);
            return Ok(HttpResponse::Unauthorized().json(json!({
                "ok": false,
                "msg": MSG_UNEXPECTED_ERROR
            })));
        }
    } else {
        if let Err(e) = invitation.delete(&conn) {
            warn!("handle_invite failed, error: {:?}", e);
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

pub async fn accept_invite_with_token(
    request: HttpRequest,
    Path(token): web::Path<String>,
) -> Result<HttpResponse> {
    let conn = request.db_pool()?.get()?;
    let invitation = OwnerInvitation::find_by_token(&token, &conn)?;
    if invitation.is_expired() {
        return request.render(410, "accounts/invalid_token.html", Context::new());
    }
    if let Err(e) = invitation.accept(&conn) {
        warn!("handle_invite failed, error: {:?}", e);
        return request.render(503, "503.html", Context::new());
    }
    let package = Package::get(invitation.package_id, request.db_pool()?).await?;
    request.redirect(&format!("/packages/{}", package.name))
}
