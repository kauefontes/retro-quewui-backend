use actix_web::{web, HttpResponse, Responder, get};
use crate::models::github_profile_repository::GitHubProfileRepository;

#[get("/github/profile")]
pub async fn get_github_profile(github_repo: web::Data<GitHubProfileRepository>) -> impl Responder {
    match github_repo.get_github_profile().await {
        Ok(profile) => HttpResponse::Ok().json(profile),
        Err(e) => {
            log::error!("Error fetching GitHub profile: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch GitHub profile data"
            }))
        }
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_github_profile);
}
