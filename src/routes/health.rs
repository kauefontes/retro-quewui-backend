use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::database::DbPool;
use crate::error::AppResult;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
    timestamp: u64,
    database: bool,
}

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
