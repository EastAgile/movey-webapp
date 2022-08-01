use jelly::prelude::*;
use jelly::Result;

use crate::utils::request_utils;

/// Returns an overview of everything in the system.
pub async fn dashboard(request: HttpRequest) -> Result<HttpResponse> {
    //let user = request.user()?;
    request_utils::renew_token(&request).await?;

    request.render(200, "dashboard/index.html", Context::new())
}
