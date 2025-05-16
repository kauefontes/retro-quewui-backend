use actix_web::{get, web, HttpResponse, Responder};

use crate::models::github_stats::get_mock_github_stats;

#[get("/github-stats")]
pub async fn get_github_stats() -> impl Responder {
    let stats = get_mock_github_stats();
    HttpResponse::Ok().json(stats)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_github_stats);
}
