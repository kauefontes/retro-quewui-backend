use actix_web::{get, put, web, HttpResponse, Responder};
use log::{info, error};
use serde::Deserialize;

use crate::auth::AuthenticatedUser;
use crate::error::{AppError, AppResult};
use crate::models::github_stats::{GithubStats, TopLanguage, RecentActivity, get_mock_github_stats};

/// Get GitHub statistics
///
/// Returns GitHub statistics including repository count, followers, contributions,
/// top languages, and recent activity.
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
pub async fn get_github_stats() -> impl Responder {
    let stats = get_mock_github_stats();
    HttpResponse::Ok().json(stats)
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UpdateGithubStatsRequest {
    /// GitHub username
    pub username: Option<String>,
    /// Number of public repositories
    pub repo_count: Option<i32>,
    /// Number of followers
    pub followers: Option<i32>,
    /// Number of contributions in the last year
    pub contributions: Option<i32>,
    /// List of top programming languages and their percentages
    pub top_languages: Option<Vec<TopLanguage>>,
    /// List of recent GitHub activity
    pub recent_activity: Option<Vec<RecentActivity>>,
}

/// Update GitHub statistics
///
/// Updates the GitHub statistics with the provided data.
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
        (status = 400, description = "Invalid GitHub stats data"),
        (status = 401, description = "Unauthorized - Invalid or missing authentication token"),
        (status = 500, description = "Internal server error")
    )
)]
#[put("/github-stats")]
pub async fn update_github_stats(
    stats_req: web::Json<UpdateGithubStatsRequest>,
    _user: AuthenticatedUser, // Require authentication
) -> AppResult<impl Responder> {
    let existing_stats = get_mock_github_stats();
    
    // Create updated stats
    let updated_stats = GithubStats {
        username: stats_req.username.clone().unwrap_or_else(|| existing_stats.username.clone()),
        repo_count: stats_req.repo_count.unwrap_or(existing_stats.repo_count),
        followers: stats_req.followers.unwrap_or(existing_stats.followers),
        contributions: stats_req.contributions.unwrap_or(existing_stats.contributions),
        top_languages: stats_req.top_languages.clone().unwrap_or_else(|| existing_stats.top_languages.clone()),
        recent_activity: stats_req.recent_activity.clone().unwrap_or_else(|| existing_stats.recent_activity.clone()),
    };
    
    // In a real application, you would update this in a database
    info!("Updated GitHub stats for user: {}", updated_stats.username);
    
    Ok(HttpResponse::Ok().json(updated_stats))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_github_stats)
       .service(update_github_stats);
}
