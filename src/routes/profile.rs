use actix_web::{get, web, HttpResponse, Responder};
use log::info;

use crate::models::profile::{Profile, get_mock_profile};

#[get("/profile")]
pub async fn get_profile() -> impl Responder {
    info!("Profile data requested");
    
    // In a real application, you would:
    // 1. Fetch profile data from a database
    // 2. Handle potential errors
    
    // For now, we'll just return mock data
    let profile = get_mock_profile();
    
    HttpResponse::Ok().json(profile)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_profile);
}
