use actix_web::{get, web, HttpResponse, Responder};

use crate::models::project::get_mock_projects;

#[get("/projects")]
pub async fn get_all_projects() -> impl Responder {
    let projects = get_mock_projects();
    HttpResponse::Ok().json(projects)
}

#[get("/projects/{id}")]
pub async fn get_project_by_id(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    let projects = get_mock_projects();
    
    match projects.iter().find(|p| p.id == id) {
        Some(project) => HttpResponse::Ok().json(project),
        None => HttpResponse::NotFound().body(format!("Project with ID {} not found", id)),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_projects)
       .service(get_project_by_id);
}
