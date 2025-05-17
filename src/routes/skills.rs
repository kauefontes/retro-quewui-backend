use actix_web::{get, post, put, delete, HttpResponse, Responder, web};
use log::info;
use serde::Deserialize;

use crate::auth::AuthenticatedUser;
use crate::error::{AppError, AppResult};
use crate::models::skill::{Skill, get_mock_skills};

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
pub async fn get_all_skills() -> impl Responder {
    let skills = get_mock_skills();
    HttpResponse::Ok().json(skills)
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
) -> AppResult<impl Responder> {
    let skill = Skill {
        category: skill_req.category.clone(),
        items: skill_req.items.clone(),
    };
    
    // In a real application, you would save this to a database
    info!("Created new skill category: {}", skill.category);
    
    Ok(HttpResponse::Created().json(skill))
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
) -> AppResult<impl Responder> {
    let category_name = path.into_inner();
    let skills = get_mock_skills();
    
    // Find the skill category to update
    let existing_skill = skills.iter().find(|s| s.category == category_name)
        .ok_or_else(|| {
            info!("Skill category '{}' not found for update", category_name);
            AppError::not_found(format!("Skill category '{}' not found", category_name))
        })?;
    
    // Create updated skill
    let updated_skill = Skill {
        category: skill_req.category.clone().unwrap_or_else(|| existing_skill.category.clone()),
        items: skill_req.items.clone().unwrap_or_else(|| existing_skill.items.clone()),
    };
    
    // In a real application, you would update this in a database
    info!("Updated skill category: {}", updated_skill.category);
    
    Ok(HttpResponse::Ok().json(updated_skill))
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
) -> AppResult<impl Responder> {
    let category_name = path.into_inner();
    let skills = get_mock_skills();
    
    // Check if the skill category exists
    let skill_exists = skills.iter().any(|s| s.category == category_name);
    
    if !skill_exists {
        info!("Skill category '{}' not found for deletion", category_name);
        return Err(AppError::not_found(format!("Skill category '{}' not found", category_name)));
    }
    
    // In a real application, you would delete this from a database
    info!("Deleted skill category: {}", category_name);
    
    Ok(HttpResponse::NoContent().finish())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_skills)
       .service(create_skill)
       .service(update_skill)
       .service(delete_skill);
}
