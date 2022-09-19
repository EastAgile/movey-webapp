use crate::accounts::jobs::{SendCollaboratorInvitationEmail, SendRegisterToCollabEmail};
use crate::accounts::Account;
use crate::api::collaborators::views::{CollaboratorJson, InvitationResponse};
use crate::package_collaborators::models::external_invitation::ExternalInvitation;
use crate::package_collaborators::models::owner_invitation::OwnerInvitation;
use crate::package_collaborators::package_collaborator::{PackageCollaborator, Role};
use crate::packages::Package;
use crate::utils::request_utils;
use diesel::result::Error as DBError;
use diesel::Connection;
use jelly::actix_web::web;
use jelly::actix_web::web::Path;
use jelly::prelude::Error::*;
use jelly::prelude::*;
use jelly::utils::error_constants::*;
use jelly::Result;
use serde_json::json;

pub async fn add_collaborators(
    request: HttpRequest,
    Path(package_name): Path<String>,
    json: web::Json<CollaboratorJson>,
) -> Result<HttpResponse> {
    if !request_utils::is_authenticated(&request)? {
        return Ok(request_utils::clear_cookie(&request));
    }
    let db = request.db_pool().map_err(|e| ApiServerError(Box::new(e)))?;
    let conn = db.get().map_err(|e| ApiServerError(Box::new(e)))?;

    let package = Package::get_by_name(&package_name, db)
        .map_err(|e| ApiNotFound(MSG_PACKAGE_NOT_FOUND, Box::new(e)))?;

    let user = request.user().map_err(|e| ApiServerError(Box::new(e)))?;
    PackageCollaborator::get(package.id, user.id, &conn)
        .map_err(|e| ApiForbidden(MSG_UNAUTHORIZED_TO_ADD_COLLABORATOR, Box::new(e)))?;

    let invited_account = match Account::get_by_email_or_gh_login(&json.user, db) {
        Ok(account) => account,
        Err(e) => {
            if matches!(e, Error::Database(DBError::NotFound)) && json.user.contains('@') {
                ExternalInvitation::create(&json.user, user.id, package.id, &conn)
                    .map_err(|e| ApiBadRequest(MSG_INVITATION_ALREADY_EXISTED, Box::new(e)))?;
                // TODO: Handle error for this line
                let _ = request.queue(SendRegisterToCollabEmail {
                    to: json.user.clone(),
                    package_name: package.name.clone(),
                });
                // Inviting email is not in system, return a message that will send email to them.
                return Ok(HttpResponse::Ok().json(json!({
                    "ok": false,
                    "msg": MSG_ACCOUNT_NOT_FOUND_INVITING
                })));
            } else {
                return Err(ApiNotFound(MSG_ACCOUNT_NOT_FOUND_DONT_INVITE, Box::new(e)));
            }
        }
    };

    if PackageCollaborator::get(package.id, invited_account.id, &conn).is_ok() {
        return Err(ApiBadRequest(
            MSG_COLLABORATOR_ALREADY_EXISTED,
            Box::new(Error::Generic(format!(
                "Collaborator already existed. uid: {}, package id: {}",
                invited_account.id, package.id,
            ))),
        ));
    }

    let invitation =
        OwnerInvitation::create(invited_account.id, user.id, package.id, None, None, &conn)
            .map_err(|e| ApiBadRequest(MSG_INVITATION_ALREADY_EXISTED, Box::new(e)))?;
    if !invited_account.is_generated_email() {
        request.queue(SendCollaboratorInvitationEmail {
            to: invited_account.email,
            package_name: package.name,
            token: invitation.token,
        })?;
    }

    Ok(HttpResponse::Ok().json(&json!({
        "ok": true,
        "msg": MSG_SUCCESSFULLY_INVITED_COLLABORATOR,
    })))
}

pub async fn transfer_ownership(
    request: HttpRequest,
    Path(package_name): Path<String>,
    json: web::Json<CollaboratorJson>,
) -> Result<HttpResponse> {
    if !request_utils::is_authenticated(&request)? {
        return Ok(request_utils::clear_cookie(&request));
    }
    let db = request.db_pool().map_err(|e| ApiServerError(Box::new(e)))?;
    let conn = db.get().map_err(|e| ApiServerError(Box::new(e)))?;

    let package = Package::get_by_name(&package_name, db)
        .map_err(|e| ApiNotFound(MSG_PACKAGE_NOT_FOUND, Box::new(e)))?;

    let invited_account = Account::get_by_email_or_gh_login(&json.user, db)
        .map_err(|e| ApiNotFound(MSG_ACCOUNT_NOT_FOUND, Box::new(e)))?;

    let user = request.user().map_err(|e| ApiServerError(Box::new(e)))?;
    let ids = vec![user.id, invited_account.id];
    let collaborators = PackageCollaborator::get_in_bulk_order_by_role(package.id, ids, &conn)
        .map_err(|e| ApiForbidden(MSG_UNAUTHORIZED_TO_ADD_COLLABORATOR, Box::new(e)))?;
    let unauthorized_error = Box::new(Error::Generic(String::from(
        "Unauthorized to transfer ownership.",
    )));
    if collaborators.len() != 2 {
        return Err(ApiBadRequest(
            MSG_UNAUTHORIZED_TO_TRANSFER_OWNERSHIP,
            unauthorized_error,
        ));
    }
    if collaborators.get(0).unwrap().role != Role::Owner as i32
        || collaborators.get(1).unwrap().role != Role::Collaborator as i32
    {
        return Err(ApiForbidden(
            MSG_UNAUTHORIZED_TO_TRANSFER_OWNERSHIP,
            unauthorized_error,
        ));
    }

    let invitation = OwnerInvitation::create(
        invited_account.id,
        user.id,
        package.id,
        Some(true),
        None,
        &conn,
    )
    .map_err(|e| ApiBadRequest(MSG_INVITATION_ALREADY_EXISTED, Box::new(e)))?;

    if !invited_account.is_generated_email() {
        // TODO: need a new email for transferring ownership
        request.queue(SendCollaboratorInvitationEmail {
            to: invited_account.email,
            package_name: package.name,
            token: invitation.token,
        })?;
    }

    Ok(HttpResponse::Ok().json(&json!({
        "ok": true,
        "msg": MSG_SUCCESSFULLY_TRANSFER_OWNERSHIP,
    })))
}

pub async fn handle_invite(
    request: HttpRequest,
    json: web::Json<InvitationResponse>,
) -> Result<HttpResponse> {
    if !request_utils::is_authenticated(&request)? {
        return Ok(request_utils::clear_cookie(&request));
    }

    let db = request.db_pool().map_err(|e| ApiServerError(Box::new(e)))?;
    let conn = db.get().map_err(|e| ApiServerError(Box::new(e)))?;

    let user = request.user().map_err(|e| ApiServerError(Box::new(e)))?;
    let invitation = OwnerInvitation::find_by_id(user.id, json.package_id, &conn)
        .map_err(|e| ApiNotFound(MSG_INVITATION_NOT_FOUND, Box::new(e)))?;
    if invitation.is_expired() {
        return Err(ApiBadRequest(
            MSG_INVITATION_EXPIRED,
            Box::new(Error::Generic(format!(
                "Invitation is expired. invited id: {}, package id: {}",
                invitation.invited_user_id, invitation.package_id
            ))),
        ));
    }
    if json.accepted {
        conn.transaction(|| -> Result<()> {
            invitation
                .accept(&conn)
                .map_err(|e| ApiUnauthorized(MSG_UNEXPECTED_ERROR, Box::new(e)))?;
            if invitation.is_transferring {
                Package::change_owner(invitation.package_id, invitation.invited_user_id, &conn)?;
            }
            Ok(())
        })?
    } else {
        invitation
            .delete(&conn)
            .map_err(|e| ApiUnauthorized(MSG_UNEXPECTED_ERROR, Box::new(e)))?;
    }
    Ok(HttpResponse::Ok().json(json!({
        "ok": true,
        "msg": MSG_SUCCESSFULLY_ADDED_COLLABORATOR
    })))
}

pub async fn remove_collaborator(
    request: HttpRequest,
    Path(package_name): Path<String>,
    json: web::Json<CollaboratorJson>,
) -> Result<HttpResponse> {
    if !request_utils::is_authenticated(&request)? {
        return Ok(request_utils::clear_cookie(&request));
    }
    let db = request.db_pool().map_err(|e| ApiServerError(Box::new(e)))?;
    let conn = db.get().map_err(|e| ApiServerError(Box::new(e)))?;

    let package = Package::get_by_name(&package_name, db)
        .map_err(|e| ApiNotFound(MSG_PACKAGE_NOT_FOUND, Box::new(e)))?;

    let user = request.user().map_err(|e| ApiServerError(Box::new(e)))?;
    PackageCollaborator::get(package.id, user.id, &conn)
        .map_err(|e| ApiForbidden(MSG_UNAUTHORIZED_TO_ADD_COLLABORATOR, Box::new(e)))?;

    let target_account = Account::get_by_email_or_gh_login(&json.user, db);
    match target_account {
        Ok(account) => {
            // if account is a PendingOwner, only delete the invitation
            let num_deleted_invitations =
                OwnerInvitation::delete_by_id(account.id, package.id, &conn)
                    .map_err(|e| ApiServerError(Box::new(e)))?;
            if num_deleted_invitations == 0 {
                let num_deleted_collaborators =
                    PackageCollaborator::delete_collaborator_by_id(account.id, package.id, &conn)
                        .map_err(|e| ApiServerError(Box::new(e)))?;
                if num_deleted_collaborators == 0 {
                    return Err(ApiNotFound(
                        MSG_COLLABORATOR_NOT_FOUND,
                        Box::new(Error::Generic(format!(
                            "Failure trying to remove collaborator from package. requester id: {}, target id: {}, package id: {}",
                            user.id, account.id, package.id
                        ))),
                    ));
                }
            }
        }
        Err(e) => {
            // an external account must have a valid email address
            if !json.user.contains('@') {
                return Err(ApiNotFound(MSG_ACCOUNT_NOT_FOUND, Box::new(e)));
            }
            ExternalInvitation::delete_by_id(&json.user, package.id, &conn)
                .map_err(|e| ApiNotFound(MSG_ACCOUNT_NOT_FOUND, Box::new(e)))?;
        }
    }

    Ok(HttpResponse::Ok().json(&json!({
        "ok": true,
        "msg": MSG_SUCCESSFULLY_REMOVED_COLLABORATOR,
    })))
}
