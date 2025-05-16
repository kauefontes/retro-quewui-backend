use actix_web::{get, HttpResponse, Responder, web};

use crate::models::skill::{Skill, get_mock_skills};

#[get("/skills")]
pub async fn get_all_skills() -> impl Responder {
    let skills = get_mock_skills();
    HttpResponse::Ok().json(skills)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_skills);
}
