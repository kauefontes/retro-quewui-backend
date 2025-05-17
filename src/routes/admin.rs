use actix_web::{get, web, HttpResponse, Responder};
use log::info;
use serde::Serialize;

use crate::auth::AuthenticatedUser;
use crate::error::AppResult;

#[derive(Serialize)]
struct AdminResponse {
    message: String,
    user: String,
}

// Protected route that requires authentication
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
