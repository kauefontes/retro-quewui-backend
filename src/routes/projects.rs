use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use log::{error, info};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::auth::AuthenticatedUser;
use crate::config::database::DbPool;
use crate::error::{AppError, AppResult};
use crate::models::project::Project;
use crate::models::project_repository::ProjectRepository;
use crate::models::repository::Repository;

/// Get all projects
///
/// Returns a list of all projects in the system.
#[utoipa::path(
    get,
    path = "/projects",
    tag = "projects",
    responses(
        (status = 200, description = "List of all projects retrieved successfully", body = Vec<Project>),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/projects")]
pub async fn get_all_projects(db: web::Data<DbPool>) -> AppResult<impl Responder> {
    let repo = ProjectRepository::new(db.get_ref().clone());
    
    let projects = repo.find_all().await
        .map_err(|e| {
            error!("Failed to fetch projects: {}", e);
            AppError::internal_error(format!("Failed to fetch projects: {}", e))
        })?;
    
    info!("Retrieved {} projects", projects.len());
    Ok(HttpResponse::Ok().json(projects))
}

/// Get project by ID
///
/// Returns a single project with the specified ID.
#[utoipa::path(
    get,
    path = "/projects/{id}",
    tag = "projects",
    params(
        ("id" = String, Path, description = "Project unique identifier")
    ),
    responses(
        (status = 200, description = "Project found", body = Project),
        (status = 404, description = "Project not found"),
        (status = 500, description = "Internal server error")
    )
)]
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

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProjectRequest {
    /// Project title
    pub title: String,
    /// Detailed description of the project
    pub description: String,
    /// List of technologies used in the project
    pub technologies: Vec<String>,
    /// Optional link to GitHub repository
    pub github_url: Option<String>,
    /// Optional link to live demo
    pub live_url: Option<String>,
    /// Optional URL to project image/screenshot
    pub image_url: Option<String>,
    /// Year the project was completed
    pub year: i32,
    /// Key highlights or features of the project
    pub highlights: Vec<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateProjectRequest {
    /// Project title
    pub title: Option<String>,
    /// Detailed description of the project
    pub description: Option<String>,
    /// List of technologies used in the project
    pub technologies: Option<Vec<String>>,
    /// Optional link to GitHub repository
    pub github_url: Option<String>,
    /// Optional link to live demo
    pub live_url: Option<String>,
    /// Optional URL to project image/screenshot
    pub image_url: Option<String>,
    /// Year the project was completed
    pub year: Option<i32>,
    /// Key highlights or features of the project
    pub highlights: Option<Vec<String>>,
}

/// Create a new project
///
/// Creates a new project with the provided details.
/// Requires authentication.
#[utoipa::path(
    post,
    path = "/projects",
    tag = "projects",
    security(
        ("jwt_auth" = [])
    ),
    request_body = CreateProjectRequest,
    responses(
        (status = 201, description = "Project created successfully", body = Project),
        (status = 400, description = "Invalid project data"),
        (status = 401, description = "Unauthorized - Invalid or missing authentication token"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/projects")]
pub async fn create_project(
    project_req: web::Json<CreateProjectRequest>,
    db: web::Data<DbPool>,
    _user: AuthenticatedUser, // Require authentication
) -> AppResult<impl Responder> {
    let repo = ProjectRepository::new(db.get_ref().clone());
    
    let project = Project::new(
        project_req.title.clone(),
        project_req.description.clone(),
        project_req.technologies.clone(),
        project_req.github_url.clone(),
        project_req.live_url.clone(),
        project_req.image_url.clone(),
        project_req.year,
        project_req.highlights.clone(),
    );
    
    let created_project = repo.create(project).await
        .map_err(|e| {
            error!("Failed to create project: {}", e);
            AppError::internal_error(format!("Failed to create project: {}", e))
        })?;
    
    info!("Created new project with ID: {}", created_project.id);
    Ok(HttpResponse::Created().json(created_project))
}

/// Update an existing project
///
/// Updates an existing project with the specified ID.
/// Requires authentication.
#[utoipa::path(
    put,
    path = "/projects/{id}",
    tag = "projects",
    security(
        ("jwt_auth" = [])
    ),
    params(
        ("id" = String, Path, description = "Project unique identifier")
    ),
    request_body = UpdateProjectRequest,
    responses(
        (status = 200, description = "Project updated successfully", body = Project),
        (status = 400, description = "Invalid project data"),
        (status = 401, description = "Unauthorized - Invalid or missing authentication token"),
        (status = 404, description = "Project not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[put("/projects/{id}")]
pub async fn update_project(
    path: web::Path<String>,
    project_req: web::Json<UpdateProjectRequest>,
    db: web::Data<DbPool>,
    _user: AuthenticatedUser, // Require authentication
) -> AppResult<impl Responder> {
    let id = path.into_inner();
    let repo = ProjectRepository::new(db.get_ref().clone());
    
    // First, get the existing project
    let existing_project = repo.find_by_id(&id).await
        .map_err(|e| {
            error!("Failed to fetch project {}: {}", id, e);
            AppError::internal_error(format!("Failed to fetch project: {}", e))
        })?
        .ok_or_else(|| {
            info!("Project with ID {} not found for update", id);
            AppError::not_found(format!("Project with ID {} not found", id))
        })?;
    
    // Update the project with new values, keeping existing values if not provided
    let updated_project = Project {
        id: existing_project.id,
        title: project_req.title.clone().unwrap_or(existing_project.title),
        description: project_req.description.clone().unwrap_or(existing_project.description),
        technologies: project_req.technologies.clone().unwrap_or(existing_project.technologies),
        github_url: project_req.github_url.clone().or(existing_project.github_url),
        live_url: project_req.live_url.clone().or(existing_project.live_url),
        image_url: project_req.image_url.clone().or(existing_project.image_url),
        year: project_req.year.unwrap_or(existing_project.year),
        highlights: project_req.highlights.clone().unwrap_or(existing_project.highlights),
    };
    
    // Save the updated project
    let result = repo.update(&id, updated_project.clone()).await
        .map_err(|e| {
            error!("Failed to update project {}: {}", id, e);
            AppError::internal_error(format!("Failed to update project: {}", e))
        })?;
    
    info!("Updated project with ID: {}", id);
    Ok(HttpResponse::Ok().json(result))
}

/// Delete a project
///
/// Deletes the project with the specified ID.
/// Requires authentication.
#[utoipa::path(
    delete,
    path = "/projects/{id}",
    tag = "projects",
    security(
        ("jwt_auth" = [])
    ),
    params(
        ("id" = String, Path, description = "Project unique identifier")
    ),
    responses(
        (status = 204, description = "Project deleted successfully"),
        (status = 401, description = "Unauthorized - Invalid or missing authentication token"),
        (status = 404, description = "Project not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[delete("/projects/{id}")]
pub async fn delete_project(
    path: web::Path<String>,
    db: web::Data<DbPool>,
    _user: AuthenticatedUser, // Require authentication
) -> AppResult<impl Responder> {
    let id = path.into_inner();
    let repo = ProjectRepository::new(db.get_ref().clone());
    
    // Check if the project exists
    let project_exists = repo.find_by_id(&id).await
        .map_err(|e| {
            error!("Failed to fetch project {}: {}", id, e);
            AppError::internal_error(format!("Failed to fetch project: {}", e))
        })?
        .is_some();
    
    if !project_exists {
        info!("Project with ID {} not found for deletion", id);
        return Err(AppError::not_found(format!("Project with ID {} not found", id)));
    }
    
    // Delete the project
    let deleted = repo.delete(&id).await
        .map_err(|e| {
            error!("Failed to delete project {}: {}", id, e);
            AppError::internal_error(format!("Failed to delete project: {}", e))
        })?;
    
    if deleted {
        info!("Deleted project with ID: {}", id);
        Ok(HttpResponse::NoContent().finish())
    } else {
        error!("Failed to delete project with ID: {}", id);
        Err(AppError::internal_error("Failed to delete project"))
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_projects)
       .service(get_project_by_id)
       .service(create_project)
       .service(update_project)
       .service(delete_project);
}
