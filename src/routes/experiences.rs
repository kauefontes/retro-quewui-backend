use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use log::info;
use serde::Deserialize;

use crate::auth::AuthenticatedUser;
use crate::error::{AppError, AppResult};
use crate::models::experience::{Experience, get_mock_experiences};

/// Get all experiences
///
/// Returns a list of all professional experiences.
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
pub async fn get_all_experiences() -> impl Responder {
    let experiences = get_mock_experiences();
    HttpResponse::Ok().json(experiences)
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
pub async fn get_experience_by_id(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    let experiences = get_mock_experiences();
    
    match experiences.iter().find(|e| e.id == id) {
        Some(experience) => HttpResponse::Ok().json(experience),
        None => HttpResponse::NotFound().body(format!("Experience with ID {} not found", id)),
    }
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
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

#[derive(Debug, Deserialize, utoipa::ToSchema)]
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
/// Creates a new professional experience entry.
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
    _user: AuthenticatedUser, // Require authentication
) -> AppResult<impl Responder> {
    let experience = Experience::new(
        experience_req.company.clone(),
        experience_req.position.clone(),
        experience_req.start_date.clone(),
        experience_req.end_date.clone(),
        experience_req.description.clone(),
        experience_req.technologies.clone(),
        experience_req.highlights.clone(),
    );
    
    // In a real application, you would save this to a database
    // For now, we'll just return the created experience
    info!("Created new experience for {}: {}", experience.company, experience.position);
    
    Ok(HttpResponse::Created().json(experience))
}

/// Update an existing experience
///
/// Updates an existing experience with the specified ID.
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
    _user: AuthenticatedUser, // Require authentication
) -> AppResult<impl Responder> {
    let id = path.into_inner();
    let experiences = get_mock_experiences();
    
    // Find the experience to update
    let existing_experience = experiences.iter().find(|e| e.id == id)
        .ok_or_else(|| {
            info!("Experience with ID {} not found for update", id);
            AppError::not_found(format!("Experience with ID {} not found", id))
        })?;
    
    // Create updated experience
    let updated_experience = Experience {
        id: existing_experience.id.clone(),
        company: experience_req.company.clone().unwrap_or_else(|| existing_experience.company.clone()),
        position: experience_req.position.clone().unwrap_or_else(|| existing_experience.position.clone()),
        start_date: experience_req.start_date.clone().unwrap_or_else(|| existing_experience.start_date.clone()),
        end_date: experience_req.end_date.clone().or_else(|| existing_experience.end_date.clone()),
        description: experience_req.description.clone().unwrap_or_else(|| existing_experience.description.clone()),
        technologies: experience_req.technologies.clone().unwrap_or_else(|| existing_experience.technologies.clone()),
        highlights: experience_req.highlights.clone().unwrap_or_else(|| existing_experience.highlights.clone()),
    };
    
    // In a real application, you would update this in a database
    info!("Updated experience with ID: {}", id);
    
    Ok(HttpResponse::Ok().json(updated_experience))
}

/// Delete an experience
///
/// Deletes the experience with the specified ID.
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
    _user: AuthenticatedUser, // Require authentication
) -> AppResult<impl Responder> {
    let id = path.into_inner();
    let experiences = get_mock_experiences();
    
    // Check if the experience exists
    let experience_exists = experiences.iter().any(|e| e.id == id);
    
    if !experience_exists {
        info!("Experience with ID {} not found for deletion", id);
        return Err(AppError::not_found(format!("Experience with ID {} not found", id)));
    }
    
    // In a real application, you would delete this from a database
    info!("Deleted experience with ID: {}", id);
    
    Ok(HttpResponse::NoContent().finish())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_experiences)
       .service(get_experience_by_id)
       .service(create_experience)
       .service(update_experience)
       .service(delete_experience);
}
