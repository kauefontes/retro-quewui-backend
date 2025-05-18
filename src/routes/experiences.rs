use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use log::{error, info};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::auth::AuthenticatedUser;
use crate::config::database::DbPool;
use crate::error::{AppError, AppResult};
use crate::models::experience::Experience;
use crate::models::experience_repository::ExperienceRepository;
use crate::models::repository::Repository;

/// Get all experiences
///
/// Returns a list of all work experiences sorted by start date (newest first).
#[utoipa::path(
    get,
    path = "/experiences",
    tag = "experiences",
    responses(
        (status = 200, description = "List of all experiences retrieved successfully", body = Vec<Experience>),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/experiences")]
pub async fn get_all_experiences(db: web::Data<DbPool>) -> AppResult<impl Responder> {
    let repo = ExperienceRepository::new(db.get_ref().clone());
    
    let experiences = repo.find_all().await
        .map_err(|e| {
            error!("Failed to fetch experiences: {}", e);
            AppError::internal_error(format!("Failed to fetch experiences: {}", e))
        })?;
    
    info!("Retrieved {} experiences", experiences.len());
    Ok(HttpResponse::Ok().json(experiences))
}

/// Get experience by ID
///
/// Returns a single experience with the specified ID.
#[utoipa::path(
    get,
    path = "/experiences/{id}",
    tag = "experiences",
    params(
        ("id" = String, Path, description = "Experience unique identifier")
    ),
    responses(
        (status = 200, description = "Experience found", body = Experience),
        (status = 404, description = "Experience not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/experiences/{id}")]
pub async fn get_experience_by_id(path: web::Path<String>, db: web::Data<DbPool>) -> AppResult<impl Responder> {
    let id = path.into_inner();
    let repo = ExperienceRepository::new(db.get_ref().clone());
    
    let experience = repo.find_by_id(&id).await
        .map_err(|e| {
            error!("Failed to fetch experience {}: {}", id, e);
            AppError::internal_error(format!("Failed to fetch experience: {}", e))
        })?;
    
    match experience {
        Some(experience) => {
            info!("Retrieved experience with ID: {}", id);
            Ok(HttpResponse::Ok().json(experience))
        },
        None => {
            info!("Experience with ID {} not found", id);
            Err(AppError::not_found(format!("Experience with ID {} not found", id)))
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateExperienceRequest {
    /// Name of the company or organization
    pub company: String,
    /// Job title or position held
    pub position: String,
    /// When the position started (format: YYYY-MM)
    pub start_date: String,
    /// When the position ended (format: YYYY-MM), null if current position
    pub end_date: Option<String>,
    /// Detailed description of the role and responsibilities
    pub description: String,
    /// List of technologies and tools used in this role
    pub technologies: Vec<String>,
    /// Key achievements and notable contributions
    pub highlights: Vec<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateExperienceRequest {
    /// Name of the company or organization
    pub company: Option<String>,
    /// Job title or position held
    pub position: Option<String>,
    /// When the position started (format: YYYY-MM)
    pub start_date: Option<String>,
    /// When the position ended (format: YYYY-MM), null if current position
    pub end_date: Option<String>,
    /// Detailed description of the role and responsibilities
    pub description: Option<String>,
    /// List of technologies and tools used in this role
    pub technologies: Option<Vec<String>>,
    /// Key achievements and notable contributions
    pub highlights: Option<Vec<String>>,
}

/// Create a new experience
///
/// Creates a new work experience entry with the provided details.
/// Requires authentication.
#[utoipa::path(
    post,
    path = "/experiences",
    tag = "experiences",
    security(
        ("jwt_auth" = [])
    ),
    request_body = CreateExperienceRequest,
    responses(
        (status = 201, description = "Experience created successfully", body = Experience),
        (status = 400, description = "Invalid experience data"),
        (status = 401, description = "Unauthorized - Invalid or missing authentication token"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/experiences")]
pub async fn create_experience(
    experience_req: web::Json<CreateExperienceRequest>,
    db: web::Data<DbPool>,
    _user: AuthenticatedUser, // Require authentication
) -> AppResult<impl Responder> {
    let repo = ExperienceRepository::new(db.get_ref().clone());
    
    let experience = Experience::new(
        experience_req.company.clone(),
        experience_req.position.clone(),
        experience_req.start_date.clone(),
        experience_req.end_date.clone(),
        experience_req.description.clone(),
        experience_req.technologies.clone(),
        experience_req.highlights.clone(),
    );
    
    let created_experience = repo.create(experience).await
        .map_err(|e| {
            error!("Failed to create experience: {}", e);
            AppError::internal_error(format!("Failed to create experience: {}", e))
        })?;
    
    info!("Created new experience with ID: {}", created_experience.id);
    Ok(HttpResponse::Created().json(created_experience))
}

/// Update an existing experience
///
/// Updates an existing work experience entry with the specified ID.
/// Requires authentication.
#[utoipa::path(
    put,
    path = "/experiences/{id}",
    tag = "experiences",
    security(
        ("jwt_auth" = [])
    ),
    params(
        ("id" = String, Path, description = "Experience unique identifier")
    ),
    request_body = UpdateExperienceRequest,
    responses(
        (status = 200, description = "Experience updated successfully", body = Experience),
        (status = 400, description = "Invalid experience data"),
        (status = 401, description = "Unauthorized - Invalid or missing authentication token"),
        (status = 404, description = "Experience not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[put("/experiences/{id}")]
pub async fn update_experience(
    path: web::Path<String>,
    experience_req: web::Json<UpdateExperienceRequest>,
    db: web::Data<DbPool>,
    _user: AuthenticatedUser, // Require authentication
) -> AppResult<impl Responder> {
    let id = path.into_inner();
    let repo = ExperienceRepository::new(db.get_ref().clone());
    
    // First, get the existing experience
    let existing_experience = repo.find_by_id(&id).await
        .map_err(|e| {
            error!("Failed to fetch experience {}: {}", id, e);
            AppError::internal_error(format!("Failed to fetch experience: {}", e))
        })?
        .ok_or_else(|| {
            info!("Experience with ID {} not found for update", id);
            AppError::not_found(format!("Experience with ID {} not found", id))
        })?;
    
    // Update the experience with new values, keeping existing values if not provided
    let updated_experience = Experience {
        id: existing_experience.id,
        company: experience_req.company.clone().unwrap_or(existing_experience.company),
        position: experience_req.position.clone().unwrap_or(existing_experience.position),
        start_date: experience_req.start_date.clone().unwrap_or(existing_experience.start_date),
        end_date: experience_req.end_date.clone().or(existing_experience.end_date),
        description: experience_req.description.clone().unwrap_or(existing_experience.description),
        technologies: experience_req.technologies.clone().unwrap_or(existing_experience.technologies),
        highlights: experience_req.highlights.clone().unwrap_or(existing_experience.highlights),
    };
    
    // Save the updated experience
    let result = repo.update(&id, updated_experience.clone()).await
        .map_err(|e| {
            error!("Failed to update experience {}: {}", id, e);
            AppError::internal_error(format!("Failed to update experience: {}", e))
        })?;
    
    info!("Updated experience with ID: {}", id);
    Ok(HttpResponse::Ok().json(result))
}

/// Delete an experience
///
/// Deletes the work experience entry with the specified ID.
/// Requires authentication.
#[utoipa::path(
    delete,
    path = "/experiences/{id}",
    tag = "experiences",
    security(
        ("jwt_auth" = [])
    ),
    params(
        ("id" = String, Path, description = "Experience unique identifier")
    ),
    responses(
        (status = 204, description = "Experience deleted successfully"),
        (status = 401, description = "Unauthorized - Invalid or missing authentication token"),
        (status = 404, description = "Experience not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[delete("/experiences/{id}")]
pub async fn delete_experience(
    path: web::Path<String>,
    db: web::Data<DbPool>,
    _user: AuthenticatedUser, // Require authentication
) -> AppResult<impl Responder> {
    let id = path.into_inner();
    let repo = ExperienceRepository::new(db.get_ref().clone());
    
    // Check if the experience exists
    let experience_exists = repo.find_by_id(&id).await
        .map_err(|e| {
            error!("Failed to fetch experience {}: {}", id, e);
            AppError::internal_error(format!("Failed to fetch experience: {}", e))
        })?
        .is_some();
    
    if !experience_exists {
        info!("Experience with ID {} not found for deletion", id);
        return Err(AppError::not_found(format!("Experience with ID {} not found", id)));
    }
    
    // Delete the experience
    let deleted = repo.delete(&id).await
        .map_err(|e| {
            error!("Failed to delete experience {}: {}", id, e);
            AppError::internal_error(format!("Failed to delete experience: {}", e))
        })?;
    
    if deleted {
        info!("Deleted experience with ID: {}", id);
        Ok(HttpResponse::NoContent().finish())
    } else {
        error!("Failed to delete experience with ID: {}", id);
        Err(AppError::internal_error("Failed to delete experience"))
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_experiences)
       .service(get_experience_by_id)
       .service(create_experience)
       .service(update_experience)
       .service(delete_experience);
}