use actix_web::{get, web, HttpResponse, Responder};

use crate::models::experience::{Experience, get_mock_experiences};

#[get("/experiences")]
pub async fn get_all_experiences() -> impl Responder {
    let experiences = get_mock_experiences();
    HttpResponse::Ok().json(experiences)
}

#[get("/experiences/{id}")]
pub async fn get_experience_by_id(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    let experiences = get_mock_experiences();
    
    match experiences.iter().find(|e| e.id == id) {
        Some(experience) => HttpResponse::Ok().json(experience),
        None => HttpResponse::NotFound().body(format!("Experience with ID {} not found", id)),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_experiences)
       .service(get_experience_by_id);
}
