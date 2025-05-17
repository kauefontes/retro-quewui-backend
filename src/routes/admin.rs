use actix_web::{get, web, HttpResponse, Responder};
use log::info;
use serde::Serialize;
use utoipa::ToSchema;

use crate::auth::AuthenticatedUser;
use crate::error::AppResult;

#[derive(Serialize, ToSchema)]
struct AdminResponse {
    /// Welcome message for the admin
    message: String,
    /// Username of the authenticated admin
    user: String,
}

/// Admin dashboard endpoint
///
/// Protected route that requires authentication with admin privileges.
/// Returns a welcome message and the authenticated user's name.
#[utoipa::path(
    get,
    path = "/admin/dashboard",
    tag = "admin",
    security(
        ("jwt_auth" = [])
    ),
    responses(
        (status = 200, description = "Successfully accessed admin dashboard", body = AdminResponse),
        (status = 401, description = "Unauthorized - Invalid or missing authentication token"),
        (status = 403, description = "Forbidden - User does not have admin privileges")
    )
)]
#[get("/admin/dashboard")]
pub async fn admin_dashboard(user: AuthenticatedUser) -> AppResult<impl Responder> {
    info!("Admin dashboard accessed by: {}", user.0.name);
    
    Ok(HttpResponse::Ok().json(AdminResponse {
        message: "Welcome to the admin dashboard".to_string(),
        user: user.0.name,
    }))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(admin_dashboard);
}
