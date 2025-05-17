use actix_web::{get, web, HttpResponse, Responder};
use log::{error, info};

use crate::config::database::DbPool;
use crate::error::{AppError, AppResult};
use crate::models::project_repository::{ProjectRepository, seed_projects};
use crate::models::repository::Repository;

#[get("/projects")]
pub async fn get_all_projects(db: web::Data<DbPool>) -> AppResult<impl Responder> {
    let repo = ProjectRepository::new(db.get_ref().clone());
    
    // Seed database with mock data if empty (for development)
    if let Err(e) = seed_projects(&repo).await {
        error!("Failed to seed projects: {}", e);
        // Continue even if seeding fails
    }
    
    let projects = repo.find_all().await
        .map_err(|e| {
            error!("Failed to fetch projects: {}", e);
            AppError::internal_error(format!("Failed to fetch projects: {}", e))
        })?;
    
    info!("Retrieved {} projects", projects.len());
    Ok(HttpResponse::Ok().json(projects))
}

#[get("/projects/{id}")]
pub async fn get_project_by_id(path: web::Path<String>, db: web::Data<DbPool>) -> AppResult<impl Responder> {
    let id = path.into_inner();
    let repo = ProjectRepository::new(db.get_ref().clone());
    
    let project = repo.find_by_id(&id).await
        .map_err(|e| {
            error!("Failed to fetch project {}: {}", id, e);
            AppError::internal_error(format!("Failed to fetch project: {}", e))
        })?;
    
    match project {
        Some(project) => {
            info!("Retrieved project with ID: {}", id);
            Ok(HttpResponse::Ok().json(project))
        },
        None => {
            info!("Project with ID {} not found", id);
            Err(AppError::not_found(format!("Project with ID {} not found", id)))
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_projects)
       .service(get_project_by_id);
}
