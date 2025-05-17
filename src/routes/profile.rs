use actix_web::{get, put, web, HttpResponse, Responder};
use log::info;
use serde::Deserialize;

use crate::auth::AuthenticatedUser;
use crate::error::AppResult;
use crate::models::profile::{Profile, SocialLink, Education, Language, get_mock_profile};

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
pub async fn get_profile() -> impl Responder {
    info!("Profile data requested");
    
    // In a real application, you would:
    // 1. Fetch profile data from a database
    // 2. Handle potential errors
    
    // For now, we'll just return mock data
    let profile = get_mock_profile();
    
    HttpResponse::Ok().json(profile)
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
    _user: AuthenticatedUser, // Require authentication
) -> AppResult<impl Responder> {
    let existing_profile = get_mock_profile();
    
    // Create updated profile
    let updated_profile = Profile {
        bio: profile_req.bio.clone().unwrap_or_else(|| existing_profile.bio.clone()),
        social_links: profile_req.social_links.clone().unwrap_or_else(|| existing_profile.social_links.clone()),
        education: profile_req.education.clone().unwrap_or_else(|| existing_profile.education.clone()),
        languages: profile_req.languages.clone().unwrap_or_else(|| existing_profile.languages.clone()),
    };
    
    // In a real application, you would update this in a database
    info!("Updated user profile");
    
    Ok(HttpResponse::Ok().json(updated_profile))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_profile)
       .service(update_profile);
}
