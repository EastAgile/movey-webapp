use crate::package_collaborators::models::owner_invitation::OwnerInvitation;
use crate::packages::Package;
use jelly::actix_web::web::Path;
use jelly::prelude::*;
use jelly::Result;

pub async fn accept_invite_with_token(
    request: HttpRequest,
    Path(token): Path<String>,
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
    let package = Package::get(invitation.package_id, request.db_pool()?)?;
    request.redirect(&format!("/packages/{}", package.slug))
}
