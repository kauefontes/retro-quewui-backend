// filepath: /home/kaue/developer/quewuicom/retro-quewui-backend/src/routes/github_stats.rs
use actix_web::{get, put, web, HttpResponse, Responder};
use log::{error, info};
use serde::Deserialize;

use crate::auth::AuthenticatedUser;
use crate::config::database::DbPool;
use crate::error::{AppError, AppResult};
use crate::models::github_stats::{GithubStats, TopLanguage, RecentActivity};
use crate::models::github_stats_repository::GithubStatsRepository;
use crate::models::repository::Repository;

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UpdateGithubStatsRequest {
    /// GitHub username
    pub username: Option<String>,
    /// Number of public repositories
    pub repo_count: Option<i32>,
    /// Number of followers
    pub followers: Option<i32>,
    /// Number of contributions in past year
    pub contributions: Option<i32>,
    /// Top programming languages
    pub top_languages: Option<Vec<TopLanguage>>,
    /// Recent GitHub activity
    pub recent_activity: Option<Vec<RecentActivity>>,
}

/// Get GitHub statistics
///
/// Returns the GitHub statistics for the portfolio owner.
#[utoipa::path(
    get,
    path = "/github-stats",
    tag = "github-stats",
    responses(
        (status = 200, description = "GitHub statistics retrieved successfully", body = GithubStats),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/github-stats")]
pub async fn get_github_stats(db: web::Data<DbPool>) -> AppResult<impl Responder> {
    let repo = GithubStatsRepository::new(db.get_ref().clone());
    
    let stats_list = repo.find_all().await
        .map_err(|e| {
            error!("Failed to fetch GitHub stats: {}", e);
            AppError::internal_error(format!("Failed to fetch GitHub stats: {}", e))
        })?;
    
    if stats_list.is_empty() {
        return Err(AppError::not_found("GitHub stats not found"));
    }
    
    // Return the first stats (there should only be one)
    let stats = &stats_list[0];
    
    Ok(HttpResponse::Ok().json(stats))
}

/// Update GitHub statistics
///
/// Updates the GitHub statistics for the portfolio owner.
/// Requires authentication.
#[utoipa::path(
    put,
    path = "/github-stats",
    tag = "github-stats",
    security(
        ("jwt_auth" = [])
    ),
    request_body = UpdateGithubStatsRequest,
    responses(
        (status = 200, description = "GitHub statistics updated successfully", body = GithubStats),
        (status = 401, description = "Unauthorized - Invalid or missing authentication token"),
        (status = 500, description = "Internal server error")
    )
)]
#[put("/github-stats")]
pub async fn update_github_stats(
    stats_req: web::Json<UpdateGithubStatsRequest>,
    db: web::Data<DbPool>,
    _user: AuthenticatedUser, // Require authentication
) -> AppResult<impl Responder> {
    let repo = GithubStatsRepository::new(db.get_ref().clone());
    
    // Get all stats
    let stats_list = repo.find_all().await
        .map_err(|e| {
            error!("Failed to fetch GitHub stats: {}", e);
            AppError::internal_error(format!("Failed to fetch GitHub stats: {}", e))
        })?;
    
    let (stats_id, existing_stats) = if stats_list.is_empty() {
        // Create default stats if none exist
        let new_stats = GithubStats {
            username: stats_req.username.clone().unwrap_or_else(|| "github_user".to_string()),
            repo_count: stats_req.repo_count.unwrap_or(0),
            followers: stats_req.followers.unwrap_or(0),
            contributions: stats_req.contributions.unwrap_or(0),
            top_languages: stats_req.top_languages.clone().unwrap_or_default(),
            recent_activity: stats_req.recent_activity.clone().unwrap_or_default(),
        };
        
        let stats = repo.create(new_stats).await
            .map_err(|e| {
                error!("Failed to create GitHub stats: {}", e);
                AppError::internal_error(format!("Failed to create GitHub stats: {}", e))
            })?;
        
        ("github-stats-1".to_string(), stats)
    } else {
        // Use the first stats
        let stats = &stats_list[0];
        let stats_id = format!("github-stats-{}", 1); // Assuming ID follows a pattern
        (stats_id, stats.clone())
    };
    
    // Update with new values if provided
    let updated_stats = GithubStats {
        username: stats_req.username.clone().unwrap_or_else(|| existing_stats.username.clone()),
        repo_count: stats_req.repo_count.unwrap_or(existing_stats.repo_count),
        followers: stats_req.followers.unwrap_or(existing_stats.followers),
        contributions: stats_req.contributions.unwrap_or(existing_stats.contributions),
        top_languages: stats_req.top_languages.clone().unwrap_or_else(|| existing_stats.top_languages.clone()),
        recent_activity: stats_req.recent_activity.clone().unwrap_or_else(|| existing_stats.recent_activity.clone()),
    };
    
    // Save updated stats
    let result = repo.update(&stats_id, updated_stats.clone()).await
        .map_err(|e| {
            error!("Failed to update GitHub stats: {}", e);
            AppError::internal_error(format!("Failed to update GitHub stats: {}", e))
        })?;
    
    info!("Updated GitHub stats for user: {}", result.username);
    
    Ok(HttpResponse::Ok().json(result))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_github_stats)
       .service(update_github_stats);
}
