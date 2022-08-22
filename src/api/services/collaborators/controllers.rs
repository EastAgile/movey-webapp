use crate::accounts::jobs::{SendCollaboratorInvitationEmail, SendRegisterToCollabEmail};
use crate::accounts::Account;
use crate::api::services::collaborators::views::{CollaboratorJson, InvitationResponse};
use crate::package_collaborators::models::owner_invitation::OwnerInvitation;
use crate::package_collaborators::models::external_invitation::ExternalInvitation;
use crate::package_collaborators::package_collaborator::{PackageCollaborator, Role};
use crate::packages::Package;
use crate::utils::request_utils;
use diesel::result::Error as DBError;
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
    if !request_utils::is_authenticated(&request).await? {
        return Ok(request_utils::clear_cookie(&request));
    }
    let db = request.db_pool().map_err(|e| ApiServerError(Box::new(e)))?;
    let conn = db.get().map_err(|e| ApiServerError(Box::new(e)))?;

    let package = Package::get_by_name(&package_name, db)
        .await
        .map_err(|e| ApiNotFound(MSG_PACKAGE_NOT_FOUND, Box::new(e)))?;

    let user = request.user().map_err(|e| ApiServerError(Box::new(e)))?;
    let collaborator = PackageCollaborator::get(package.id, user.id, &conn)
        .map_err(|e| ApiForbidden(MSG_UNAUTHORIZED_TO_ADD_COLLABORATOR, Box::new(e)))?;
    if collaborator.role != Role::Owner as i32 {
        return Err(ApiForbidden(
            MSG_UNAUTHORIZED_TO_ADD_COLLABORATOR,
            Box::new(Error::Generic(format!(
                "Non-owner is trying to add collaborator. uid: {}, package id: {}",
                collaborator.account_id, collaborator.package_id
            ))),
        ));
    }

    let invited_account = Account::get_by_email_or_gh_login(&json.user, db).await;
    let invited_account = match invited_account {
        Ok(account) => account,
        Err(e) => {
            if matches!(e, Error::Database(DBError::NotFound))
                && json.user.contains('@')
                && ExternalInvitation::create(&json.user, user.id, package.id, &conn).is_ok()
            {
                // TODO: Handle error for this line
                let _ = request.queue(SendRegisterToCollabEmail {
                    to: json.user.clone(),
                    package_name: package.name.clone(),
                });
            }
            // Inviting email is not in system, return a message that will send email to them.
            return Ok(HttpResponse::Ok().json(json!({
                "ok": false,
                "msg": MSG_ACCOUNT_NOT_FOUND_INVITING
            })));
        }
    };

    if PackageCollaborator::get(package.id, invited_account.id, &conn).is_ok() {
        return Err(ApiBadRequest(
            MSG_COLLABORATOR_ALREADY_EXISTED,
            Box::new(Error::Generic(format!(
                "Collaborator already existed. uid: {}, package id: {}",
                invited_account.id, package.id
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
    if !request_utils::is_authenticated(&request).await? {
        return Ok(request_utils::clear_cookie(&request));
    }
    let db = request.db_pool().map_err(|e| ApiServerError(Box::new(e)))?;
    let conn = db.get().map_err(|e| ApiServerError(Box::new(e)))?;

    let package = Package::get_by_name(&package_name, db)
        .await
        .map_err(|e| ApiNotFound(MSG_PACKAGE_NOT_FOUND, Box::new(e)))?;

    let invited_account = Account::get_by_email_or_gh_login(&json.user, db)
        .await
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
    if !request_utils::is_authenticated(&request).await? {
        return Ok(request_utils::clear_cookie(&request));
    }

    let db = request.db_pool().map_err(|e| ApiServerError(Box::new(e)))?;
    let conn = db.get().map_err(|e| ApiServerError(Box::new(e)))?;

    let user = request.user().map_err(|e| ApiServerError(Box::new(e)))?;
    let invitation = OwnerInvitation::find_by_id(user.id, json.package_id, &conn)
        .map_err(|e| ApiNotFound(MSG_PACKAGE_NOT_FOUND, Box::new(e)))?;
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
        invitation
            .accept(&conn)
            .map_err(|e| ApiUnauthorized(MSG_UNEXPECTED_ERROR, Box::new(e)))?;
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
    if !request_utils::is_authenticated(&request).await? {
        return Ok(request_utils::clear_cookie(&request));
    }
    let db = request.db_pool().map_err(|e| ApiServerError(Box::new(e)))?;
    let conn = db.get().map_err(|e| ApiServerError(Box::new(e)))?;

    let package = Package::get_by_name(&package_name, db)
        .await
        .map_err(|e| ApiNotFound(MSG_PACKAGE_NOT_FOUND, Box::new(e)))?;
    let user = request.user().map_err(|e| ApiServerError(Box::new(e)))?;
    let collaborator = PackageCollaborator::get(package.id, user.id, &conn)
        .map_err(|e| ApiForbidden(MSG_UNAUTHORIZED_TO_ADD_COLLABORATOR, Box::new(e)))?;
    if collaborator.role != Role::Owner as i32 {
        return Err(ApiForbidden(
            MSG_UNAUTHORIZED_TO_REMOVE_COLLABORATOR,
            Box::new(Error::Generic(format!(
                "Non-owner is trying to remove a collaborator. uid: {}, package id: {}",
                collaborator.account_id, collaborator.package_id
            ))),
        ));
    }
    let removed_account = Account::get_by_email_or_gh_login(
        &json.user,
        db
    )
        .await;
    match removed_account {
        Ok(account) => {
            // if account is a PendingOwner, only delete the invitation
            let res = OwnerInvitation::delete_by_id(account.id, package.id, &conn)
                .map_err(|e| ApiServerError(Box::new(e)));
            if let Ok(0) = res {
                PackageCollaborator::delete_by_id(account.id, package.id, &conn)
                    .map_err(|e| ApiNotFound(MSG_ACCOUNT_NOT_FOUND, Box::new(e)))?;
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
