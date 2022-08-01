use crate::package_collaborators::models::owner_invitation::OwnerInvitation;
use jelly::actix_web::web::Path;
use jelly::prelude::*;
use jelly::Result;

pub async fn handle_invite_with_token(
    request: HttpRequest,
    Path(token): Path<String>,
) -> Result<HttpResponse> {
    let conn = request.db_pool()?.get()?;

    let invitation = OwnerInvitation::find_by_token(&token, &conn)?;
    if invitation.is_expired() {
        return request.render(200, "dashboard/index.html", Context::new());
    }
    invitation.accept(&conn)?;
    request.render(200, "dashboard/index.html", Context::new())
}
