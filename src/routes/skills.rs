use actix_web::{get, post, put, delete, HttpResponse, Responder, web};
use log::{error, info};
use serde::Deserialize;

use crate::auth::AuthenticatedUser;
use crate::config::database::DbPool;
use crate::error::{AppError, AppResult};
use crate::models::skill::Skill;
use crate::models::skill_repository::SkillRepository;
use crate::models::repository::Repository;

/// Get all skills
///
/// Returns a list of all skills grouped by category.
#[utoipa::path(
    get,
    path = "/skills",
    tag = "skills",
    responses(
        (status = 200, description = "List of all skills retrieved successfully", body = Vec<Skill>),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/skills")]
pub async fn get_all_skills(db: web::Data<DbPool>) -> AppResult<impl Responder> {
    let repo = SkillRepository::new(db.get_ref().clone());
    
    let skills = repo.find_all().await
        .map_err(|e| {
            error!("Failed to fetch skills: {}", e);
            AppError::internal_error(format!("Failed to fetch skills: {}", e))
        })?;
    
    info!("Retrieved {} skill categories", skills.len());
    Ok(HttpResponse::Ok().json(skills))
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct CreateSkillRequest {
    /// Skill category (e.g., "Languages", "Frontend", "Backend")
    pub category: String,
    /// List of skills in this category
    pub items: Vec<String>,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UpdateSkillRequest {
    /// Skill category (e.g., "Languages", "Frontend", "Backend")
    pub category: Option<String>,
    /// List of skills in this category
    pub items: Option<Vec<String>>,
}

/// Create a new skill category
///
/// Creates a new skill category with the provided items.
/// Requires authentication.
#[utoipa::path(
    post,
    path = "/skills",
    tag = "skills",
    security(
        ("jwt_auth" = [])
    ),
    request_body = CreateSkillRequest,
    responses(
        (status = 201, description = "Skill category created successfully", body = Skill),
        (status = 400, description = "Invalid skill data"),
        (status = 401, description = "Unauthorized - Invalid or missing authentication token"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/skills")]
pub async fn create_skill(
    skill_req: web::Json<CreateSkillRequest>,
    _user: AuthenticatedUser, // Require authentication
    db: web::Data<DbPool>,
) -> AppResult<impl Responder> {
    let repo = SkillRepository::new(db.get_ref().clone());
    
    let skill = Skill {
        category: skill_req.category.clone(),
        items: skill_req.items.clone(),
    };
    
    let created_skill = repo.create(skill).await
        .map_err(|e| {
            error!("Failed to create skill category: {}", e);
            AppError::internal_error(format!("Failed to create skill category: {}", e))
        })?;
    
    info!("Created new skill category: {}", created_skill.category);
    
    Ok(HttpResponse::Created().json(created_skill))
}

/// Get a skill category by name
///
/// Returns a single skill category with the specified name.
#[utoipa::path(
    get,
    path = "/skills/{category}",
    tag = "skills",
    params(
        ("category" = String, Path, description = "Skill category name")
    ),
    responses(
        (status = 200, description = "Skill category found", body = Skill),
        (status = 404, description = "Skill category not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/skills/{category}")]
pub async fn get_skill_by_category(
    path: web::Path<String>,
    db: web::Data<DbPool>,
) -> AppResult<impl Responder> {
    let category = path.into_inner();
    let repo = SkillRepository::new(db.get_ref().clone());
    
    // Find the skill with the matching category
    let skills = repo.find_all().await
        .map_err(|e| {
            error!("Failed to fetch skills: {}", e);
            AppError::internal_error(format!("Failed to fetch skills: {}", e))
        })?;
    
    match skills.iter().find(|s| s.category == category) {
        Some(skill) => {
            info!("Retrieved skill category: {}", category);
            Ok(HttpResponse::Ok().json(skill))
        },
        None => {
            info!("Skill category '{}' not found", category);
            Err(AppError::not_found(format!("Skill category '{}' not found", category)))
        }
    }
}

/// Update an existing skill category
///
/// Updates an existing skill category by name.
/// Requires authentication.
#[utoipa::path(
    put,
    path = "/skills/{category}",
    tag = "skills",
    security(
        ("jwt_auth" = [])
    ),
    params(
        ("category" = String, Path, description = "Skill category name")
    ),
    request_body = UpdateSkillRequest,
    responses(
        (status = 200, description = "Skill category updated successfully", body = Skill),
        (status = 400, description = "Invalid skill data"),
        (status = 401, description = "Unauthorized - Invalid or missing authentication token"),
        (status = 404, description = "Skill category not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[put("/skills/{category}")]
pub async fn update_skill(
    path: web::Path<String>,
    skill_req: web::Json<UpdateSkillRequest>,
    _user: AuthenticatedUser, // Require authentication
    db: web::Data<DbPool>,
) -> AppResult<impl Responder> {
    let category = path.into_inner();
    let repo = SkillRepository::new(db.get_ref().clone());
    
    // Find the skill to update
    let skills = repo.find_all().await
        .map_err(|e| {
            error!("Failed to fetch skills: {}", e);
            AppError::internal_error(format!("Failed to fetch skills: {}", e))
        })?;
    
    let skill_entry = skills.iter().enumerate()
        .find(|(_, s)| s.category == category)
        .ok_or_else(|| {
            info!("Skill category '{}' not found for update", category);
            AppError::not_found(format!("Skill category '{}' not found", category))
        })?;
    
    let (index, existing_skill) = skill_entry;
    
    // Create updated skill
    let updated_skill = Skill {
        category: skill_req.category.clone().unwrap_or_else(|| existing_skill.category.clone()),
        items: skill_req.items.clone().unwrap_or_else(|| existing_skill.items.clone()),
    };
    
    // Get the ID based on index (simplified approach)
    let id = format!("skill-{}", index + 1);
    
    let result = repo.update(&id, updated_skill.clone()).await
        .map_err(|e| {
            error!("Failed to update skill category: {}", e);
            AppError::internal_error(format!("Failed to update skill category: {}", e))
        })?;
    
    info!("Updated skill category: {}", category);
    
    Ok(HttpResponse::Ok().json(result))
}

/// Delete a skill category
///
/// Deletes the skill category with the specified name.
/// Requires authentication.
#[utoipa::path(
    delete,
    path = "/skills/{category}",
    tag = "skills",
    security(
        ("jwt_auth" = [])
    ),
    params(
        ("category" = String, Path, description = "Skill category name")
    ),
    responses(
        (status = 204, description = "Skill category deleted successfully"),
        (status = 401, description = "Unauthorized - Invalid or missing authentication token"),
        (status = 404, description = "Skill category not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[delete("/skills/{category}")]
pub async fn delete_skill(
    path: web::Path<String>,
    _user: AuthenticatedUser, // Require authentication
    db: web::Data<DbPool>,
) -> AppResult<impl Responder> {
    let category = path.into_inner();
    let repo = SkillRepository::new(db.get_ref().clone());
    
    // Find the skill to delete
    let skills = repo.find_all().await
        .map_err(|e| {
            error!("Failed to fetch skills: {}", e);
            AppError::internal_error(format!("Failed to fetch skills: {}", e))
        })?;
    
    let skill_entry = skills.iter().enumerate()
        .find(|(_, s)| s.category == category)
        .ok_or_else(|| {
            info!("Skill category '{}' not found for deletion", category);
            AppError::not_found(format!("Skill category '{}' not found", category))
        })?;
    
    let (index, _) = skill_entry;
    
    // Get the ID based on index (simplified approach)
    let id = format!("skill-{}", index + 1);
    
    repo.delete(&id).await
        .map_err(|e| {
            error!("Failed to delete skill category: {}", e);
            AppError::internal_error(format!("Failed to delete skill category: {}", e))
        })?;
    
    info!("Deleted skill category: {}", category);
    
    Ok(HttpResponse::NoContent().finish())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_skills)
       .service(get_skill_by_category)
       .service(create_skill)
       .service(update_skill)
       .service(delete_skill);
}
