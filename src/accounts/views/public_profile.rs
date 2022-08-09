use jelly::actix_web::web::Path;
use jelly::actix_web::web::Query;
use jelly::prelude::*;
use jelly::Result;

use crate::packages::models::PackageSortField;
use crate::packages::models::PackageSortOrder;
use crate::packages::models::PACKAGES_PER_PAGE;
use crate::packages::views::controller::PackageIndexParams;
use crate::packages::Package;

pub async fn get(
    request: HttpRequest,
    Path(user_slug): Path<i32>,
    mut params: Query<PackageIndexParams>,
) -> Result<HttpResponse> {
    let db = request.db_pool()?;

    if params.field.is_none() {
        params.field = Some(PackageSortField::Name);
    }
    if params.order.is_none() {
        params.order = if let Some(PackageSortField::Name) = params.field {
            Some(PackageSortOrder::Asc)
        } else {
            Some(PackageSortOrder::Desc)
        }
    }

    let (packages, total_count, total_pages) = Package::get_by_account_paginated(
        user_slug,
        params.field.as_ref().unwrap(),
        params.order.as_ref().unwrap(),
        params.page,
        None,
        db,
    )
    .await?;

    let current_page = params.page.unwrap_or(1);
    if current_page < 1 {
        return Err(Error::Generic(String::from("Invalid page number.")));
    }
    let field_name = match &params.field {
        Some(f) => f.to_string(),
        None => "".to_string(),
    };
    let display_pagination_start = (current_page - 1) * PACKAGES_PER_PAGE + 1;
    let display_pagination_end: usize = (display_pagination_start as usize) + packages.len() - 1;

    request.render(200, "accounts/public_profile.html", {
        let mut ctx = Context::new();
        ctx.insert("account", "dung.ngo");
        ctx.insert("account_slug", "2");
        ctx.insert("packages", &packages);
        ctx.insert("sort_type", &field_name);
        ctx.insert("current_page", &current_page);
        ctx.insert("display_pagination_start", &display_pagination_start);
        ctx.insert("display_pagination_end", &display_pagination_end);
        ctx.insert("total_count", &total_count);
        ctx.insert("total_pages", &total_pages);
        ctx
    })
}
