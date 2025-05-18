use actix_web::{get, put, web, HttpResponse, Responder};
use log::{error, info};
use serde::Deserialize;

use crate::auth::AuthenticatedUser;
use crate::config::database::DbPool;
use crate::error::{AppError, AppResult};
use crate::models::profile::{Profile, SocialLink, Education, Language};
use crate::models::profile_repository::ProfileRepository;
use crate::models::repository::Repository;

/// Get user profile
///
/// Returns the user profile information including bio, social links, education, and languages.
#[utoipa::path(
    get,
    path = "/profile",
    tag = "profile",
    responses(
        (status = 200, description = "Profile retrieved successfully", body = Profile),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/profile")]
pub async fn get_profile(db: web::Data<DbPool>) -> AppResult<impl Responder> {
    info!("Profile data requested");
    
    let repo = ProfileRepository::new(db.get_ref().clone());
    
    let profiles = repo.find_all().await
        .map_err(|e| {
            error!("Failed to fetch profile: {}", e);
            AppError::internal_error(format!("Failed to fetch profile: {}", e))
        })?;
    
    if profiles.is_empty() {
        return Err(AppError::not_found("Profile not found"));
    }
    
    // Return the first profile (there should only be one)
    let profile = &profiles[0];
    
    Ok(HttpResponse::Ok().json(profile))
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UpdateProfileRequest {
    /// Bio paragraphs
    pub bio: Option<Vec<String>>,
    /// Social media links
    pub social_links: Option<Vec<SocialLink>>,
    /// Education history
    pub education: Option<Vec<Education>>,
    /// Languages spoken
    pub languages: Option<Vec<Language>>,
}

/// Update user profile
///
/// Updates the user profile with the provided information.
/// Requires authentication.
#[utoipa::path(
    put,
    path = "/profile",
    tag = "profile",
    security(
        ("jwt_auth" = [])
    ),
    request_body = UpdateProfileRequest,
    responses(
        (status = 200, description = "Profile updated successfully", body = Profile),
        (status = 400, description = "Invalid profile data"),
        (status = 401, description = "Unauthorized - Invalid or missing authentication token"),
        (status = 500, description = "Internal server error")
    )
)]
#[put("/profile")]
pub async fn update_profile(
    profile_req: web::Json<UpdateProfileRequest>,
    db: web::Data<DbPool>,
    _user: AuthenticatedUser, // Require authentication
) -> AppResult<impl Responder> {
    let repo = ProfileRepository::new(db.get_ref().clone());
    
    // Get all profiles
    let profiles = repo.find_all().await
        .map_err(|e| {
            error!("Failed to fetch profile: {}", e);
            AppError::internal_error(format!("Failed to fetch profile: {}", e))
        })?;
    
    let (profile_id, existing_profile) = if profiles.is_empty() {
        // Create a new profile if none exists
        let new_profile = Profile {
            bio: Vec::new(),
            social_links: Vec::new(),
            education: Vec::new(),
            languages: Vec::new(),
        };
        
        let profile = repo.create(new_profile).await
            .map_err(|e| {
                error!("Failed to create profile: {}", e);
                AppError::internal_error(format!("Failed to create profile: {}", e))
            })?;
        
        ("profile-1".to_string(), profile)
    } else {
        // Use the first profile from the actual database
        let profile = &profiles[0];
        
        // Query to get the ID of the first profile
        let query = "SELECT id FROM profiles LIMIT 1";
        let row = sqlx::query_as::<_, (String,)>(query)
            .fetch_optional(db.get_ref())
            .await
            .map_err(|e| {
                error!("Failed to fetch profile ID: {}", e);
                AppError::internal_error(format!("Failed to fetch profile ID: {}", e))
            })?;
        
        let profile_id = match row {
            Some((id,)) => {
                info!("Found profile ID in database: {}", id);
                id
            },
            None => {
                error!("No profile ID found in the database, this should not happen");
                return Err(AppError::internal_error("No profile ID found in the database"));
            }
        };
        
        (profile_id, profile.clone())
    };
    
    // Create updated profile
    let updated_profile = Profile {
        bio: profile_req.bio.clone().unwrap_or_else(|| existing_profile.bio.clone()),
        social_links: profile_req.social_links.clone().unwrap_or_else(|| existing_profile.social_links.clone()),
        education: profile_req.education.clone().unwrap_or_else(|| existing_profile.education.clone()),
        languages: profile_req.languages.clone().unwrap_or_else(|| existing_profile.languages.clone()),
    };
    
    // Save updated profile
    info!("Updating profile with ID: {}", profile_id);
    info!("Updated profile data: {:?}", updated_profile);
    
    let result = repo.update(&profile_id, updated_profile.clone()).await
        .map_err(|e| {
            error!("Failed to update profile: {}", e);
            AppError::internal_error(format!("Failed to update profile: {}", e))
        })?;
    
    // Verify the update by fetching the profile again
    let updated = repo.find_by_id(&profile_id).await
        .map_err(|e| {
            error!("Failed to fetch updated profile: {}", e);
            AppError::internal_error(format!("Failed to fetch updated profile: {}", e))
        })?;
    
    info!("Profile after update: {:?}", updated);
    
    info!("Updated user profile");
    
    Ok(HttpResponse::Ok().json(result))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_profile)
       .service(update_profile);
}
