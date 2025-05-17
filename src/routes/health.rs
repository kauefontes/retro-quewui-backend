use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};
use utoipa::ToSchema;

use crate::config::database::DbPool;
use crate::error::AppResult;

#[derive(Serialize, ToSchema)]
pub struct HealthResponse {
    /// Service status ("ok" if all systems are operational)
    status: String,
    /// API version from Cargo.toml
    version: String,
    /// Current server timestamp (seconds since UNIX epoch)
    timestamp: u64,
    /// Database connection status
    database: bool,
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
        (status = 500, description = "Service is unhealthy")
    )
)]
#[get("/health")]
pub async fn health_check(db: web::Data<DbPool>) -> AppResult<impl Responder> {
    // Check database connection
    let db_status = sqlx::query("SELECT 1")
        .execute(db.get_ref())
        .await
        .is_ok();
    
    // Get current timestamp
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    // Get version from Cargo.toml
    let version = env!("CARGO_PKG_VERSION").to_string();
    
    let response = HealthResponse {
        status: "ok".to_string(),
        version,
        timestamp,
        database: db_status,
    };
    
    Ok(HttpResponse::Ok().json(response))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
}
