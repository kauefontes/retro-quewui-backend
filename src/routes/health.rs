use actix_web::{get, web, HttpResponse, Responder};
use log::error;
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};
use utoipa::ToSchema;

use crate::config::database::DbPool;

#[derive(Serialize, ToSchema)]
pub struct HealthResponse {
    /// Service status ("ok" if all systems are operational)
    pub status: String,
    /// API version from Cargo.toml
    pub version: String,
    /// Current server timestamp (seconds since UNIX epoch)
    pub timestamp: u64,
    /// Database connection status
    pub database: bool,
}

/// Health check endpoint
///
/// Returns the current status of the API and its components.
/// Use this endpoint to verify that the service is running correctly.
#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse),
        (status = 500, description = "Service is unhealthy", body = HealthResponse)
    )
)]
#[get("/health")]
pub async fn health_check(db: web::Data<DbPool>) -> impl Responder {
    // Get current timestamp
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    // Get version from Cargo.toml
    let version = env!("CARGO_PKG_VERSION").to_string();
    
    // Check database connection
    let db_result = sqlx::query("SELECT 1")
        .execute(db.get_ref())
        .await;
    
    let db_status = db_result.is_ok();
    
    // If database check failed, log the error
    if let Err(ref e) = db_result {
        error!("Database health check failed: {}", e);
    }
    
    let response = HealthResponse {
        status: if db_status { "ok" } else { "degraded" }.to_string(),
        version,
        timestamp,
        database: db_status,
    };
    
    // Always return 200 OK, but with degraded status in the JSON if something is wrong
    // This helps Swagger UI correctly display the response
    HttpResponse::Ok().json(response)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
}
