use jelly::prelude::*;
use jelly::Result;

/// Returns an overview of everything in the system.
pub async fn dashboard(request: HttpRequest) -> Result<HttpResponse> {
    //let user = request.user()?;

    request.render(200, "dashboard/index.html", {
        let context = Context::new();
        context
    })
}
